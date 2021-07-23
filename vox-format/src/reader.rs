///! Provides functions to read VOX files.
use std::{
    fs::File,
    io::{
        Cursor,
        Read,
        Seek,
    },
    path::Path,
    str::from_utf8,
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
    data::{
        VoxBuffer,
        VoxData,
    },
    types::{
        Palette,
        Size,
        Version,
        Voxel,
    },
};

/// Error type returned when reading a VOX file fails.
#[derive(Debug, Error)]
pub enum Error {
    /// The file signature is incorrect.
    #[error("Expected file header to start with b'VOX ', but got: {got:?}.")]
    InvalidMagic { got: [u8; 4] },

    /// The file uses an unsupported version.
    #[error("Unsupported file version: {version}")]
    UnsupportedFileVersion { version: Version },

    /// We expected to read the `MAIN` chunk, but instead read another chunk.
    #[error("Expected MAIN chunk, but read chunk with ID: {0:?}", got.id())]
    ExpectedMainChunk { got: Chunk },

    /// `SIZE` and `XYZI` chunks must appear in pairs. This error is returned if
    /// the number of `SIZE` chunks doesn't match the number of `XYZI` chunks.
    #[error("Found {} SIZE chunks, {} XYZI chunks.", .size_chunks.len(), .xyzi_chunks.len())]
    InvalidNumberOfSizeAndXyziChunks {
        size_chunks: Vec<Chunk>,
        xyzi_chunks: Vec<Chunk>,
    },

    /// Multiple `RGBA` chunks (color palette) were found.
    #[error("Found multiple RGBA chunks (at {} and {}).", .chunks[0].offset(), chunks[1].offset())]
    MultipleRgbaChunks { chunks: [Chunk; 2] },

    /// Unknown material type.
    #[error("Invalid material type: {material_type}")]
    InvalidMaterial { material_type: u8 },

    /// An error of the underlying IO
    #[error("IO error")]
    Io(#[from] std::io::Error),

    /// An error while decoding strings to UTF-8.
    #[error("Failed to decode UTF-8 string")]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// Reads a VOX file from the reader into the [`VoxBuffer`]. This function is
/// useful, if you want to provide your own [`VoxBuffer`].
///
/// As an example, this `VoxBuffer` only counts the number of models in the
/// file:
///
/// ```
/// # use vox_format::{reader::read_vox_into, data::VoxBuffer, types::{Version, Size, Voxel, Palette}};
/// # let mut vox_file = std::fs::File::open("../test_files/test_multiple_models.vox").unwrap();
///
/// #[derive(Default)]
/// pub struct CountModels {
///   num_models: usize
/// }
///
/// impl VoxBuffer for CountModels {
///   fn set_voxel(&mut self, _voxel: Voxel) {}
///   fn set_palette(&mut self, _palette: Palette) {}
///   fn set_num_models(&mut self, num_models: usize) {
///     self.num_models += 1
///   }
/// }
///
/// let mut counter = CountModels::default();
/// read_vox_into(vox_file, &mut counter).unwrap();
/// println!("{}", counter.num_models);
/// ```
pub fn read_vox_into<R: Read + Seek, B: VoxBuffer>(
    mut reader: R,
    buffer: &mut B,
) -> Result<(), Error> {
    let (main_chunk, version) = read_main_chunk(&mut reader)?;

    buffer.set_version(version);

    //print_chunk(&main_chunk, &mut self.reader, 0)?;
    log::trace!("main chunk: {:#?}", main_chunk);

    //let mut pack_chunk = None;
    let mut size_chunks = vec![];
    let mut xyzi_chunks = vec![];
    let mut rgba_chunk = None;
    let mut transform_chunks = vec![];
    let mut group_chunks = vec![];
    let mut shape_chunks = vec![];
    let mut layer_chunks = vec![];

    for r in main_chunk.children(&mut reader) {
        let chunk = r?;

        if chunk.children_len() > 0 {
            log::warn!("Chunk {:?} has children o.O", chunk.id());
        }

        match chunk.id() {
            /*ChunkId::Pack => {
                log::debug!("read PACK chunk: {:?}", chunk);
                if pack_chunk.is_some() {
                    return Err(Error::MultiplePackChunks {
                        chunks: [pack_chunk.take().unwrap(), chunk],
                    });
                }
                pack_chunk = Some(chunk);
            }*/
            ChunkId::Size => size_chunks.push(chunk),
            ChunkId::Xyzi => xyzi_chunks.push(chunk),
            ChunkId::Rgba => {
                if rgba_chunk.is_some() {
                    return Err(Error::MultipleRgbaChunks {
                        chunks: [rgba_chunk.take().unwrap(), chunk],
                    });
                }
                rgba_chunk = Some(chunk);
            }
            /*ChunkId::Note => {
                let data = chunk.read_content_to_vec(&mut reader)?;
                log::error!("{:#?}", data);
                todo!();
            },*/
            ChunkId::NTrn => transform_chunks.push(chunk),
            ChunkId::NGrp => group_chunks.push(chunk),
            ChunkId::NShp => shape_chunks.push(chunk),
            ChunkId::Layr => layer_chunks.push(chunk),
            ChunkId::Unsupported(raw) => {
                let str_opt = from_utf8(&raw).ok();
                log::debug!("Skipping unsupported chunk: {:?} ({:?})", raw, str_opt);
            }
            id => log::trace!("Skipping unimplemented chunk: {:?}", id),
        }
    }

    /*
    for chunk in &transform_chunks {
        let transform = Transform::read(chunk.content(&mut reader)?)?;
        log::debug!("{:#?}", transform);
    }

    for chunk in &group_chunks {
        let group = Group::read(chunk.content(&mut reader)?)?;
        log::debug!("{:#?}", group);
    }

    for chunk in &shape_chunks {
        let shape = Shape::read(chunk.content(&mut reader)?)?;
        log::debug!("{:#?}", shape);
    }

    for chunk in &layer_chunks {
        let layer = Layer::read(chunk.content(&mut reader)?)?;
        log::debug!("{:#?}", layer);
    }
    */

    // Call `set_palette` first, so the trait impl has the palette data already when
    // reading the voxels.
    if let Some(rgba_chunk) = rgba_chunk {
        log::trace!("read RGBA chunk");
        let palette = Palette::read(rgba_chunk.content(&mut reader)?)?;
        buffer.set_palette(palette);
    }
    else {
        log::trace!("no RGBA chunk found");
    }

    /*let num_models = pack_chunk
        .map(|pack| Ok::<_, Error>(pack.content(&mut reader)?.read_u32::<LE>()? as usize))
        .transpose()?
        .unwrap_or(1);
    log::trace!("num_models = {}", num_models);*/

    if xyzi_chunks.len() != size_chunks.len() {
        return Err(Error::InvalidNumberOfSizeAndXyziChunks {
            size_chunks,
            xyzi_chunks,
        });
    }
    let num_models = size_chunks.len();
    log::trace!("num_models = {}", num_models);
    buffer.set_num_models(num_models);

    for (size_chunk, xyzi_chunk) in size_chunks.into_iter().zip(xyzi_chunks) {
        let model_size = Size::read(size_chunk.content(&mut reader)?)?;
        log::trace!("model_size = {:?}", model_size);
        buffer.set_model_size(model_size);

        let mut reader = xyzi_chunk.content(&mut reader)?;

        let num_voxels = reader.read_u32::<LE>()?;
        log::trace!("num_voxels = {}", num_voxels);

        for _ in 0..num_voxels {
            let voxel = Voxel::read(&mut reader)?;
            log::trace!("voxel = {:?}", voxel);
            buffer.set_voxel(voxel);
        }
    }

    Ok(())
}

/// Reads a VOX file from a reader into [`crate::data::VoxData`].
pub fn from_reader<R: Read + Seek>(reader: R) -> Result<VoxData, Error> {
    let mut buffer = VoxData::default();
    read_vox_into(reader, &mut buffer)?;
    Ok(buffer)
}

/// Reads a VOX file from a slice into [`crate::data::VoxData`].
pub fn from_slice(slice: &[u8]) -> Result<VoxData, Error> {
    from_reader(Cursor::new(slice))
}

/// Reads a VOX file from the specified path into [`crate::data::VoxData`].
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<VoxData, Error> {
    from_reader(File::open(path)?)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::from_slice;
    use crate::types::{
        Color,
        ColorIndex,
        Model,
        Point,
        Vector,
        Voxel,
    };

    fn glider() -> Vec<Voxel> {
        vec![
            Voxel::new([0, 0, 1], 79),
            Voxel::new([1, 0, 0], 79),
            Voxel::new([2, 0, 0], 79),
            Voxel::new([2, 0, 1], 69),
            Voxel::new([2, 0, 2], 69),
        ]
    }

    fn glider2() -> Vec<Voxel> {
        vec![
            Voxel::new([0, 2, 0], 79),
            Voxel::new([1, 1, 0], 79),
            Voxel::new([2, 1, 0], 79),
            Voxel::new([1, 0, 0], 79),
            Voxel::new([0, 0, 0], 79),
        ]
    }

    fn assert_voxels(model: &Model, expected: &[Voxel]) {
        let voxels = model
            .voxels
            .iter()
            .map(|voxel| (voxel.point, voxel.color_index))
            .collect::<HashMap<Point, ColorIndex>>();

        for expected_voxel in expected {
            let voxel = voxels.get(&Vector::from(expected_voxel.point)).copied();
            assert_eq!(
                voxel,
                Some(expected_voxel.color_index),
                "Expected right at {:?}",
                expected_voxel.point
            );
        }
    }

    #[test]
    fn it_reads_files_without_models() {
        let vox = from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../test_files/test_no_models.vox"
        )))
        .unwrap();
        assert!(vox.models.is_empty());
    }

    #[test]
    fn it_reads_a_single_model() {
        let vox = from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../test_files/test_single_model_default_palette.vox"
        )))
        .unwrap();

        assert_eq!(vox.models.len(), 1);
        assert!(vox.palette.is_default());

        let model = &vox.models[0];
        assert_eq!(model.size, Vector::new(3, 1, 3));
        assert_voxels(model, &glider());
    }

    #[test]
    fn it_reads_multiple_models() {
        let vox = from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../test_files/test_multiple_models.vox"
        )))
        .unwrap();

        assert_eq!(vox.models.len(), 2);

        let model2 = &vox.models[0];
        assert_eq!(model2.size, Vector::new(3, 3, 1));
        assert_voxels(model2, &glider2());

        let model1 = &vox.models[1];
        assert_eq!(model1.size, Vector::new(3, 1, 3));
        assert_voxels(model1, &glider());
    }

    #[test]
    fn it_reads_a_custom_palette() {
        let vox = from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../test_files/test_custom_palette.vox"
        )))
        .unwrap();

        assert!(!vox.palette.is_default());
        assert_eq!(vox.palette[79.into()], Color::light_blue());
        assert_eq!(vox.palette[69.into()], Color::new(108, 0, 204, 255));
    }

    #[test]
    fn color_indices_work_as_expected() {
        let vox = from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../test_files/test_single_model_default_palette.vox"
        )))
        .unwrap();

        let color_index = vox
            .models
            .get(0)
            .unwrap()
            .voxels
            .first()
            .unwrap()
            .color_index;
        assert_eq!(vox.palette[color_index], Color::light_blue());
    }
}
