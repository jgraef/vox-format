use std::{
    fs::File,
    io::{
        Cursor,
        Read,
        Seek,
    },
    path::Path,
    usize,
};

use byteorder::{
    ReadBytesExt,
    LE,
};
use thiserror::Error;

use crate::{
    chunk::{
        read_main_chunk,
        Chunk,
        ChunkId,
    },
    vox::{
        Color,
        ColorIndex,
        Palette,
        Vector,
        Version,
        VoxDataBuffer,
        Voxel,
    },
    VoxData,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected file header to start with b'VOX ', but got: {got:?}.")]
    InvalidMagic { got: [u8; 4] },

    #[error("Expected MAIN chunk, but read chunk with ID: {0:?}", got.id())]
    ExpectedMainChunk { got: Chunk },

    #[error("Found multiple PACK chunks (at {} and {}).", .chunks[0].offset(), chunks[1].offset())]
    MultiplePackChunks { chunks: [Chunk; 2] },

    #[error("Found {} SIZE chunks, {} XYZI chunks, and PACK said there are {} models.", .size_chunks.len(), .xyzi_chunks.len(), num_models)]
    InvalidNumberOfSizeAndXyziChunks {
        size_chunks: Vec<Chunk>,
        xyzi_chunks: Vec<Chunk>,
        num_models: usize,
    },

    #[error("Found multiple RGBA chunks (at {} and {}).", .chunks[0].offset(), chunks[1].offset())]
    MultipleRgbaChunks { chunks: [Chunk; 2] },

    #[error("IO error")]
    Io(#[from] std::io::Error),
}

pub trait VoxBuffer {
    fn set_version(&mut self, version: Version);

    fn set_num_models(&mut self, num_models: usize);

    fn set_model_size(&mut self, model_size: Vector);

    fn set_voxel(&mut self, voxel: Voxel);

    fn set_palette(&mut self, palette: Palette);
}

pub fn read_vox_into<R: Read + Seek, B: VoxBuffer>(
    mut reader: R,
    buffer: &mut B,
) -> Result<(), Error> {
    let (main_chunk, version) = read_main_chunk(&mut reader)?;

    buffer.set_version(version);

    //print_chunk(&main_chunk, &mut self.reader, 0)?;
    log::debug!("main chunk: {:#?}", main_chunk);

    let mut pack_chunk = None;
    let mut size_chunks = vec![];
    let mut xyzi_chunks = vec![];
    let mut rgba_chunk = None;

    for r in main_chunk.children(&mut reader) {
        let chunk = r?;
        match chunk.id() {
            ChunkId::Pack => {
                if pack_chunk.is_some() {
                    return Err(Error::MultiplePackChunks {
                        chunks: [pack_chunk.take().unwrap(), chunk],
                    });
                }
                pack_chunk = Some(chunk);
            }
            ChunkId::Size => size_chunks.push(chunk),
            ChunkId::Xyzi => xyzi_chunks.push(chunk),
            ChunkId::Rgba => {
                if rgba_chunk.is_some() {
                    return Err(Error::MultipleRgbaChunks {
                        chunks: [rgba_chunk.take().unwrap(), chunk],
                    });
                }
            }
            id => log::trace!("Skipping unsupported chunk: {:?}", id),
        }
    }

    let num_models = pack_chunk
        .map(|pack| Ok::<_, Error>(pack.content(&mut reader)?.read_u32::<LE>()? as usize))
        .transpose()?
        .unwrap_or(1);
    log::debug!("num_models = {}", num_models);

    if num_models != size_chunks.len() || num_models != xyzi_chunks.len() {
        return Err(Error::InvalidNumberOfSizeAndXyziChunks {
            size_chunks,
            xyzi_chunks,
            num_models,
        });
    }
    buffer.set_num_models(num_models);

    for (size_chunk, xyzi_chunk) in size_chunks.into_iter().zip(xyzi_chunks) {
        let model_size = Vector::read(&mut size_chunk.content(&mut reader)?)?;
        log::debug!("model_size = {:?}", model_size);
        buffer.set_model_size(model_size);

        let mut reader = xyzi_chunk.content(&mut reader)?;

        let num_voxels = reader.read_u32::<LE>()?;
        log::debug!("num_voxels = {}", num_voxels);

        for _ in 0..num_voxels {
            let voxel = Voxel::read(&mut reader)?;
            log::debug!("voxel = {:?}", voxel);
            buffer.set_voxel(voxel);
        }
    }

    if let Some(rgba_chunk) = rgba_chunk {
        let palette = Palette::read(&mut rgba_chunk.content(&mut reader)?)?;
        buffer.set_palette(palette);
    }

    Ok(())
}

impl Version {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(reader.read_u32::<LE>()?))
    }
}

impl Voxel {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            point: Vector::read(&mut reader)?,
            color_index: ColorIndex::read(&mut reader)?,
        })
    }
}

impl Palette {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        let mut palette = Palette::default();

        for i in 0..254 {
            palette.colors[i + 1] = Color::read(&mut reader)?;
        }

        Ok(palette)
    }
}

impl Color {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            r: reader.read_u8()?,
            g: reader.read_u8()?,
            b: reader.read_u8()?,
            a: reader.read_u8()?,
        })
    }
}

impl ColorIndex {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        Ok(Self(reader.read_u8()?))
    }
}

impl Vector {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, Error> {
        Ok(Self {
            x: reader.read_i8()?,
            y: reader.read_i8()?,
            z: reader.read_i8()?,
        })
    }
}

#[cfg(test)]
mod tests {
    // TODO: Write some proper test with some better test files.

    use std::io::Cursor;

    pub use super::*;
    use crate::vox::VoxDataBuffer;

    #[test]
    fn it_works_perfectly_as_intended_lol() {
        dotenv::dotenv().ok();
        pretty_env_logger::init();

        let raw = include_bytes!("../../test_files/ore_small.vox");

        let mut buffer = VoxDataBuffer::default();

        match read_vox_into(Cursor::new(raw), &mut buffer) {
            Ok(()) => {
                println!("{:#?}", buffer.build());
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}

pub fn from_reader<R: Read + Seek>(reader: R) -> Result<VoxData, Error> {
    let mut buffer = VoxDataBuffer::default();
    read_vox_into(reader, &mut buffer)?;
    Ok(buffer.build())
}

pub fn from_slice(slice: &[u8]) -> Result<VoxData, Error> {
    from_reader(Cursor::new(slice))
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<VoxData, Error> {
    from_reader(File::open(path)?)
}