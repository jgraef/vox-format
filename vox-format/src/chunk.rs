use std::{
    convert::TryInto,
    io::{Error as IoError, Read, Seek, SeekFrom, Write},
    str::FromStr,
    u64,
};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use thiserror::Error;

use crate::{reader::Error as ReadError, vox::Version, writer::Error as WriteError};

#[derive(Debug, Error)]
#[error("Failed to parse chunk ID: {0}")]
pub struct ChunkIdParseError(String);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChunkId {
    Main,
    Pack,
    Size,
    Xyzi,
    Rgba,
    Matt,
    //Ntrn,
    //NGrp,
    Unsupported([u8; 4]),
}

impl ChunkId {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        let mut id = [0u8; 4];
        reader.read_exact(&mut id)?;
        Ok(ChunkId::from(id))
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        let id: [u8; 4] = self.clone().into();
        Ok(writer.write_all(&id)?)
    }
}

///
/// # TODO
///
///  - Implement undocumented types[1]
///
/// [1] https://github.com/aiekick/MagicaVoxel_File_Writer/blob/master/VoxWriter.cpp
///
impl From<[u8; 4]> for ChunkId {
    fn from(value: [u8; 4]) -> Self {
        match &value {
            b"MAIN" => Self::Main,
            b"PACK" => Self::Pack,
            b"SIZE" => Self::Size,
            b"XYZI" => Self::Xyzi,
            b"RGBA" => Self::Rgba,
            b"MATT" => Self::Matt,
            //b"nTRN" => Self::Ntrn,
            //b"nGRP" => Self::NGrp,
            //b"nSHP" => Self::NShp,
            _ => Self::Unsupported(value),
        }
    }
}

impl From<ChunkId> for [u8; 4] {
    fn from(chunk_id: ChunkId) -> Self {
        match chunk_id {
            ChunkId::Main => *b"MAIN",
            ChunkId::Pack => *b"PACK",
            ChunkId::Size => *b"SIZE",
            ChunkId::Xyzi => *b"XYZI",
            ChunkId::Rgba => *b"RGBA",
            ChunkId::Matt => *b"MATT",
            ChunkId::Unsupported(value) => value,
        }
    }
}

impl FromStr for ChunkId {
    type Err = ChunkIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes().len() != 4 {
            return Err(ChunkIdParseError(s.to_owned()));
        }

        let mut id = [0u8; 4];
        id.copy_from_slice(s.as_bytes());
        Ok(ChunkId::from(id))
    }
}

#[derive(Clone, Debug)]
pub struct Chunk {
    offset: u32,
    id: ChunkId,
    content_len: u32,
    children_len: u32,
}

impl Chunk {
    pub fn read<R: Read + Seek>(mut reader: R) -> Result<Self, ReadError> {
        let offset = reader.stream_position()? as u32;

        let id = ChunkId::read(&mut reader)?;
        log::trace!("id = {:?}", id);

        let content_len = reader.read_u32::<LE>()?;
        let children_len = reader.read_u32::<LE>()?;
        log::trace!("content_len = {}, children_len = {}", content_len, children_len);

        Ok(Chunk {
            offset,
            id,
            content_len,
            children_len,
        })
    }

    pub fn content<'r, R: Read + Seek>(&self, reader: &'r mut R) -> Result<ContentReader<'r, R>, ReadError> {
        let offset = self.content_offset();
        reader.seek(SeekFrom::Start(offset as u64))?;
        Ok(ContentReader {
            reader,
            start_offset: offset,
            offset,
            end: offset + self.content_len,
        })
    }

    pub fn children<'r, R: Read + Seek + 'r>(&self, reader: &'r mut R) -> ChildrenReader<'r, R> {
        let offset = self.children_offset();

        ChildrenReader {
            reader,
            offset,
            end: offset + self.children_len,
        }
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn id(&self) -> ChunkId {
        self.id
    }

    pub fn content_offset(&self) -> u32 {
        self.offset + 12
    }

    pub fn content_len(&self) -> u32 {
        self.content_len
    }

    pub fn children_offset(&self) -> u32 {
        self.offset + 12 + self.content_len
    }

    pub fn children_len(&self) -> u32 {
        self.children_len
    }

    pub fn len(&self) -> u32 {
        self.content_len + self.children_len + 12
    }
}

pub struct ContentReader<'r, R> {
    reader: &'r mut R,
    #[allow(dead_code)]
    start_offset: u32,
    offset: u32,
    end: u32,
}

impl<'r, R: Read> Read for ContentReader<'r, R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        if self.offset < self.end {
            let n_at_most = ((self.end - self.offset) as usize).min(buf.len());
            let n_read = self.reader.read(&mut buf[..n_at_most])?;
            self.offset += n_read as u32;
            Ok(n_read)
        } else {
            Ok(0)
        }
    }
}

impl<'r, R: Seek> Seek for ContentReader<'r, R> {
    fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
        todo!()
    }
}

pub struct ChildrenReader<'r, R> {
    reader: &'r mut R,
    offset: u32,
    end: u32,
}

impl<'r, R: Read + Seek> Iterator for ChildrenReader<'r, R> {
    type Item = Result<Chunk, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset < self.end {
            Some(read_chunk_at(self.reader, &mut self.offset))
        } else {
            None
        }
    }
}

fn read_chunk_at<R: Read + Seek>(reader: &mut R, offset: &mut u32) -> Result<Chunk, ReadError> {
    log::trace!("reading chunk at {}", offset);
    reader.seek(SeekFrom::Start((*offset).into()))?;
    let chunk = Chunk::read(reader)?;
    *offset += chunk.len();
    Ok(chunk)
}

/// Verifies the VOX file's signature and reads the MAIN chunk and file version.
pub fn read_main_chunk<R: Read + Seek>(mut reader: R) -> Result<(Chunk, Version), ReadError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    log::trace!("magic = {:?}", buf);
    if &buf != b"VOX " {
        return Err(ReadError::InvalidMagic { got: buf });
    }

    let version = Version::read(&mut reader)?;
    log::trace!("version = {:?}", version);

    let main_chunk = Chunk::read(reader)?;

    if main_chunk.id() != ChunkId::Main {
        return Err(ReadError::ExpectedMainChunk { got: main_chunk });
    }

    Ok((main_chunk, version))
}

#[derive(Debug)]
pub struct ChunkWriter<W> {
    writer: W,
    offset: u64,
    content_len: u32,
    children_len: u32,
}

impl<W: Write + Seek> ChunkWriter<W> {
    fn new(mut writer: W, chunk_id: ChunkId) -> Result<Self, WriteError> {
        chunk_id.write(&mut writer)?;

        // Leave 8 bytes for `content_len` and `children_len`. Remember offset to write values later.
        let offset = writer.seek(SeekFrom::Current(4))?;

        Ok(Self {
            writer,
            offset,
            content_len: 0,
            children_len: 0,
        })
    }

    pub fn content_writer<'w, F: FnMut(&mut ContentWriter<&'w mut W>) -> Result<(), WriteError>>(&'w mut self, mut f: F) -> Result<(), WriteError> {
        /*if self.content_len != 0 {
            panic!("Chunk content already written: content_len = {}", self.content_len);
        }*/
        if self.children_len != 0 {
            panic!("Chunk children already written: children_len = {}", self.children_len);
        }

        let mut content_writer = ContentWriter {
            writer: &mut self.writer,
            len: 0,
        };

        f(&mut content_writer)?;

        self.content_len = content_writer.len.try_into()?;

        Ok(())
    }

    pub fn write_content(&mut self, data: &[u8]) -> Result<(), WriteError> {
        self.content_writer(|content_writer| {
            content_writer.write(data)?;
            Ok(())
        })
    }

    pub fn child_writer<'w, F: FnMut(&mut ChildWriter<'w, W>) -> Result<(), WriteError>>(&'w mut self, chunk_id: ChunkId, mut f: F) -> Result<(), WriteError> {
        if self.children_len != 0 {
            panic!("Chunk children already written: children_len = {}", self.children_len);
        }

        let mut child_writer = ChildWriter::new(
            ContentWriter {
                writer: &mut self.writer,
                len: 0,
            },
            chunk_id,
        )?;

        f(&mut child_writer)?;

        child_writer.write_header()?;

        Ok(())
    }

    fn write_header(&mut self) -> Result<(), WriteError> {
        let old_pos = self.writer.seek(SeekFrom::Start(self.offset as u64 + 4))?;

        self.writer.write_u32::<LE>(self.content_len)?;
        self.writer.write_u32::<LE>(self.children_len)?;

        self.writer.seek(SeekFrom::Start(old_pos))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ContentWriter<W> {
    writer: W,
    len: usize,
}

impl<W: Write> Write for ContentWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let n_written = self.writer.write(buf)?;
        self.len += n_written;
        Ok(n_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl<W: Seek> Seek for ContentWriter<W> {
    fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
        todo!()
    }
}

pub type ChildWriter<'w, W> = ChunkWriter<ContentWriter<&'w mut W>>;

pub fn chunk_writer<W: Write + Seek, F: FnMut(&mut ChunkWriter<W>) -> Result<(), WriteError>>(
    writer: W,
    mut f: F,
    chunk_id: ChunkId,
) -> Result<(), WriteError> {
    let mut chunk_writer = ChunkWriter::new(writer, chunk_id)?;

    f(&mut chunk_writer)?;

    chunk_writer.write_header()?;

    Ok(())
}
