//! This modules provides utilities to read and write VOX files as chunks, but
//! is agnostic to the content of these chunks.
//!
//! VOX files use a binary format that is similar to that of RIFF files.
//! Unfortunately it's different enough that the `riff` crate can't be used.
//!
//! The file format consists of so-called chunks, which contain specific data
//! (e.g. palette data). Chunks have IDs consisting of 4 characters. A file
//! starts with a root-chunk `MAIN`. The `MAIN` chunk then contains other chunks
//! that contain the voxel data. The format is specified [here](https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox.txt), but not all chunk IDs are described.

use std::{
    convert::TryInto,
    io::{
        Error as IoError,
        ErrorKind,
        Read,
        Seek,
        SeekFrom,
        Write,
    },
    str::FromStr,
    u64,
};

use byteorder::{
    ReadBytesExt,
    WriteBytesExt,
    LE,
};
use thiserror::Error;

use crate::{
    reader::Error as ReadError,
    types::Version,
    writer::Error as WriteError,
};

#[derive(Debug, Error)]
#[error("Failed to parse chunk ID: {0}")]
pub struct ChunkIdParseError(String);

/// A chunk ID. It's either a pre-defined ID (that is used for VOX), or
/// `Unsupported`.
///
/// The best way to find out which chunk IDs MagicaVoxel uses, is by looking at
/// its [source](https://github.com/aiekick/MagicaVoxel_File_Writer/blob/master/VoxWriter.cpp)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChunkId {
    // These are actually defined in the spec.
    Main,
    Pack,
    Size,
    Xyzi,
    Rgba,
    Matt, // There's also a `MATL`?

    /// TODO: What format does this have?
    Note,

    // From source `VoxWriter::VoxWriter` (line 482)
    Vox,
    NTrn,
    NGrp,
    NShp,
    Layr,
    Matl,
    RObj,
    RCam,

    /// Unsupported chunk ID
    Unsupported([u8; 4]),
}

impl ChunkId {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        let mut id = [0u8; 4];
        reader.read_exact(&mut id)?;
        Ok(ChunkId::from(id))
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        let id: [u8; 4] = (*self).into();
        Ok(writer.write_all(&id)?)
    }

    /// Returns whether this chunk ID is currently supported by this crate.
    pub fn is_supported(&self) -> bool {
        !matches!(self, ChunkId::Unsupported(_))
    }
}

impl From<[u8; 4]> for ChunkId {
    fn from(value: [u8; 4]) -> Self {
        match &value {
            b"MAIN" => Self::Main,
            b"PACK" => Self::Pack,
            b"SIZE" => Self::Size,
            b"XYZI" => Self::Xyzi,
            b"RGBA" => Self::Rgba,
            b"MATT" => Self::Matt,
            b"NOTE" => Self::Note,
            b"VOX " => Self::Vox,
            b"nTRN" => Self::NTrn,
            b"nGRP" => Self::NGrp,
            b"nSHP" => Self::NShp,
            b"LAYR" => Self::Layr,
            b"MATL" => Self::Matl,
            b"rOBJ" => Self::RObj,
            b"rCAM" => Self::RCam,
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
            ChunkId::Note => *b"NOTE",
            ChunkId::Vox => *b"VOX ",
            ChunkId::NTrn => *b"nTRN",
            ChunkId::NGrp => *b"nGRP",
            ChunkId::NShp => *b"nSHP",
            ChunkId::Layr => *b"LAYR",
            ChunkId::Matl => *b"MATL",
            ChunkId::RObj => *b"rOBJ",
            ChunkId::RCam => *b"rCAM",
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

/// Chunk meta-data. This doesn't contain contents or children, but information
/// needed to read the chunk from a file. You will still need a reader to read
/// the contents though.
#[derive(Clone, Debug)]
pub struct Chunk {
    offset: u32,
    id: ChunkId,
    content_len: u32,
    children_len: u32,
}

impl Chunk {
    /// Reads the chunk header and returns the information needed to read its
    /// contents or children.
    pub fn read<R: Read + Seek>(mut reader: R) -> Result<Self, ReadError> {
        let offset = reader.stream_position()? as u32;

        let id = ChunkId::read(&mut reader)?;
        log::trace!("read chunk at {}: {:?}", offset, id);

        let content_len = reader.read_u32::<LE>()?;
        let children_len = reader.read_u32::<LE>()?;
        log::trace!(
            "content_len = {}, children_len = {}",
            content_len,
            children_len
        );

        Ok(Chunk {
            offset,
            id,
            content_len,
            children_len,
        })
    }

    /// Creates a reader for its contents.
    pub fn content<R: Read + Seek>(&self, mut reader: R) -> Result<ContentReader<R>, ReadError> {
        let offset = self.content_offset();
        log::trace!("content reader: id={:?}, offset={}", self.id, offset);
        reader.seek(SeekFrom::Start(offset as u64))?;
        Ok(ContentReader {
            reader,
            start: offset,
            offset,
            end: offset + self.content_len,
        })
    }

    pub fn read_content_to_vec<R: Read + Seek>(&self, reader: R) -> Result<Vec<u8>, ReadError> {
        let mut buf = vec![];
        self.content(reader)?.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// Creates an iterator over its children. The iterator yields
    /// `Result<Chunk, _>`, so you'll need to handle the error first.
    /// Each child then is another `Chunk` struct that can be used to read
    /// contents or children.
    pub fn children<R: Read + Seek>(&self, reader: R) -> ChildrenReader<R> {
        let offset = self.children_offset();

        log::trace!(
            "children reader: offset={}, end={}",
            offset,
            offset + self.children_len
        );

        ChildrenReader {
            reader,
            offset,
            end: offset + self.children_len,
        }
    }

    /// Returns the offset at which the chunk starts. This is relative to the
    /// start of the reader. Note that for children chunks, this is relative
    /// to the start of the child data, since they basically use a
    /// `ContentReader` to read children chunks.
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// Returns the chunk IDs
    pub fn id(&self) -> ChunkId {
        self.id
    }

    /// Returns the offset to the content data. See [`Self::offset`] for further
    /// information.
    pub fn content_offset(&self) -> u32 {
        self.offset + 12
    }

    /// Returns the length of the content data.
    pub fn content_len(&self) -> u32 {
        self.content_len
    }

    /// Returns the offset to the children data. See [`Self::offset`] for
    /// further information.
    pub fn children_offset(&self) -> u32 {
        self.offset + 12 + self.content_len
    }

    /// Returns the length of the children data.
    pub fn children_len(&self) -> u32 {
        self.children_len
    }

    /// Returns the length of this chunk. That is the length of its contents,
    /// children and header.
    pub fn len(&self) -> u32 {
        self.content_len + self.children_len + 12
    }

    /// Returns `true` if the chunks has neither content nor children, `false`
    /// otherwise.
    pub fn is_empty(&self) -> bool {
        self.content_len == 0 && self.children_len == 0
    }
}

/// A reader for a chunk's contents.
pub struct ContentReader<R> {
    reader: R,
    offset: u32,
    start: u32,
    end: u32,
}

impl<R: Read> Read for ContentReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        log::trace!(
            "read: offset={}, start={}, end={}",
            self.offset,
            self.start,
            self.end
        );
        if self.offset < self.end {
            let n_at_most = ((self.end - self.offset) as usize).min(buf.len());
            log::trace!(
                "read: offset={}, end={}, n_at_most={}",
                self.offset,
                self.end,
                n_at_most
            );
            let n_read = self.reader.read(&mut buf[..n_at_most])?;
            log::trace!("read: n_read={}", n_read);
            self.offset += n_read as u32;
            Ok(n_read)
        }
        else {
            Ok(0)
        }
    }
}

impl<R: Seek> Seek for ContentReader<R> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, IoError> {
        let new_offset = seek_to(self.offset, self.start, self.end, pos)?;

        if new_offset != self.offset {
            log::trace!(
                "seek: offset={}, start={}, end={}, pos={:?}, new_offset={}",
                self.offset,
                self.start,
                self.end,
                pos,
                new_offset
            );

            self.offset = new_offset;

            self.reader.seek(SeekFrom::Start(self.offset.into()))?;
        }

        Ok((self.offset - self.start).into())
    }
}

/// An iterator over a chunk's children.
pub struct ChildrenReader<R> {
    reader: R,
    offset: u32,
    end: u32,
}

impl<R: Read + Seek> Iterator for ChildrenReader<R> {
    type Item = Result<Chunk, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        log::trace!("read next child: offset={}, end={}", self.offset, self.end);
        if self.offset < self.end {
            Some(read_chunk_at(&mut self.reader, &mut self.offset))
        }
        else {
            None
        }
    }
}

/// Reads a chunk from `reader` at the specified offset.
pub fn read_chunk_at<R: Read + Seek>(mut reader: R, offset: &mut u32) -> Result<Chunk, ReadError> {
    log::trace!("reading chunk at {}", offset);
    reader.seek(SeekFrom::Start((*offset).into()))?;
    let chunk = Chunk::read(reader)?;
    *offset += chunk.len();
    Ok(chunk)
}

/// Reads the VOX file's header, verifies it, and then reads the MAIN chunk.
pub fn read_main_chunk<R: Read + Seek>(mut reader: R) -> Result<(Chunk, Version), ReadError> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    log::trace!("magic = {:?}", buf);
    if &buf != b"VOX " {
        return Err(ReadError::InvalidMagic { got: buf });
    }

    let version = Version::read(&mut reader)?;
    log::trace!("version = {:?}", version);
    if !version.is_supported() {
        return Err(ReadError::UnsupportedFileVersion { version });
    }

    let main_chunk = Chunk::read(reader)?;

    if main_chunk.id() != ChunkId::Main {
        return Err(ReadError::ExpectedMainChunk { got: main_chunk });
    }

    Ok((main_chunk, version))
}

/// This struct is used to write out chunks to a file.
#[derive(Debug)]
pub struct ChunkWriter<W> {
    chunk_id: ChunkId,

    writer: W,

    /// Offset for the chunk. Points at the Chunk ID.
    offset: u64,

    content_len: u32,

    children_len: u32,
}

impl<W: Write + Seek> ChunkWriter<W> {
    fn new(mut writer: W, chunk_id: ChunkId) -> Result<Self, WriteError> {
        chunk_id.write(&mut writer)?;

        // Leave 8 bytes for `content_len` and `children_len`. Remember offset to write
        // values later.
        let offset = writer.seek(SeekFrom::Current(8))? - 12;

        Ok(Self {
            chunk_id,
            writer,
            offset,
            content_len: 0,
            children_len: 0,
        })
    }

    /// Returns the chunk ID
    pub fn id(&self) -> ChunkId {
        self.chunk_id
    }

    /// Returns the offset at which this chunk is written in the underlying
    /// writer. Similar to [`Chunk::offset`], this is is usually 0 for child
    /// chunks, since the use a [`ContentWriter`], which only sees the children
    /// data.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Returns the current length of the contents.
    pub fn content_len(&self) -> u32 {
        self.content_len
    }

    /// Returns the current length of the children data. This is the total
    /// number of bytes written for children chunks.
    pub fn children_len(&self) -> u32 {
        self.children_len
    }

    /// Writes data to the chunks content.
    ///
    /// Note, that this must be called before any calls to `child_writer`.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::io::Cursor;
    /// # use vox_format::chunk::*;
    /// # vox_format::writer::main_chunk_writer(Cursor::new(vec![]), Default::default(), |chunk_writer| {
    /// use std::io::Write;
    ///
    /// chunk_writer.content_writer(|writer| {
    ///     // `writer` implements `std::io::Write + std::io::Seek`.
    ///     writer.write_all(b"Hello World")?;
    ///     Ok(())
    /// })
    /// # }).unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics, if children have been written already.
    pub fn content_writer<'w, F: FnMut(&mut ContentWriter<&'w mut W>) -> Result<(), WriteError>>(
        &'w mut self,
        mut f: F,
    ) -> Result<(), WriteError> {
        /*if self.content_len != 0 {
            panic!("Chunk content already written: content_len = {}", self.content_len);
        }*/
        if self.children_len != 0 {
            panic!(
                "Chunk children already written: children_len = {}",
                self.children_len
            );
        }

        let mut content_writer = ContentWriter::new(&mut self.writer)?;

        f(&mut content_writer)?;

        self.content_len = content_writer.len();

        Ok(())
    }

    /// Writes the given slice to the chunk's data.
    pub fn write_content(&mut self, data: &[u8]) -> Result<(), WriteError> {
        self.content_writer(|content_writer| {
            content_writer.write_all(data)?;
            Ok(())
        })
    }

    /// Writes children chunks to this chunk. Note, that after a child has been
    /// written to a chunk, you can't write any more data to its contents.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::io::Cursor;
    /// # use vox_format::chunk::*;
    /// # vox_format::writer::main_chunk_writer(Cursor::new(vec![]), Default::default(), |chunk_writer| {
    /// chunk_writer.child_writer(ChunkId::Main, |child_writer| {
    ///     // `child_writer` is just another `ChunkWriter`.
    ///     child_writer.write_content(b"Hello World")?;
    ///     Ok(())
    /// })
    /// # }).unwrap();
    /// ```
    pub fn child_writer<'w, F: FnMut(&mut ChildWriter<'w, W>) -> Result<(), WriteError>>(
        &'w mut self,
        chunk_id: ChunkId,
        mut f: F,
    ) -> Result<(), WriteError> {
        let mut child_writer = ChildWriter::new(ContentWriter::new(&mut self.writer)?, chunk_id)?;

        f(&mut child_writer)?;

        self.children_len = child_writer.writer.len();

        child_writer.write_header()?;

        Ok(())
    }

    /// Short-hand to opening and child-writer and then a content-writer to that
    /// child. Useful, if you want to write a child with only content data.
    pub fn child_content_writer<
        'w,
        F: FnMut(&mut ContentWriter<&mut ContentWriter<&'w mut W>>) -> Result<(), WriteError>,
    >(
        &'w mut self,
        chunk_id: ChunkId,
        mut f: F,
    ) -> Result<(), WriteError> {
        self.child_writer(chunk_id, |child_writer| {
            child_writer.content_writer(|content_writer| f(content_writer))
        })
    }

    fn write_header(&mut self) -> Result<(), WriteError> {
        log::trace!(
            "Write header for chunk {:?} to offset {}: content_len = {}, children_len = {}",
            self.chunk_id,
            self.offset,
            self.content_len,
            self.children_len
        );

        let old_pos = self.writer.seek(SeekFrom::Current(0))?;
        self.writer.seek(SeekFrom::Start(self.offset + 4))?;

        self.writer.write_u32::<LE>(self.content_len)?;
        self.writer.write_u32::<LE>(self.children_len)?;

        self.writer.seek(SeekFrom::Start(old_pos))?;

        Ok(())
    }
}

/// This implements `Write` and `Seek` on a restricted range of offsets in the
/// underlying reader of the chunk you're writing to.
#[derive(Debug)]
pub struct ContentWriter<W> {
    writer: W,
    offset: u32,
    start: u32,
    end: u32,
}

impl<W: Seek> ContentWriter<W> {
    fn new(mut writer: W) -> Result<Self, WriteError> {
        let offset = writer.stream_position()?.try_into()?;
        Ok(Self {
            writer,
            offset,
            start: offset,
            end: offset,
        })
    }

    fn len(&self) -> u32 {
        self.end - self.start
    }
}

impl<W: Write> Write for ContentWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, IoError> {
        let n_written = self.writer.write(buf)?;

        self.offset = n_written
            .try_into()
            .ok()
            .and_then(|n| self.offset.checked_add(n))
            .ok_or_else(|| IoError::from(ErrorKind::Other))?;

        if self.offset > self.end {
            log::trace!(
                "write {} bytes: offset={}, end={}",
                n_written,
                self.offset,
                self.end
            );
            self.end = self.offset;
        }

        Ok(n_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Seek> Seek for ContentWriter<W> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, IoError> {
        let new_offset = seek_to(self.offset, self.start, self.end, pos)?;

        if new_offset != self.offset {
            log::trace!(
                "seek: offset={}, start={}, end={}, pos={:?}, new_offset={}",
                self.offset,
                self.start,
                self.end,
                pos,
                new_offset
            );
            self.writer.seek(SeekFrom::Start(new_offset.into()))?;
            self.offset = new_offset;
        }

        Ok((self.offset - self.start).into())
    }
}

/// Short-hand for a [`ChunkWriter`] wrapping a [`ContentWriter`]. The
/// underlying content-writer writes to the chunk's children data blob.
pub type ChildWriter<'w, W> = ChunkWriter<ContentWriter<&'w mut W>>;

/// This creates a `ChunkWriter` and will the closure you passed in with it.
/// This allows you to write contents and children to the `ChunkWriter`. After
/// you return from the closure this function will make sure that the chunk
/// headers are upates.
pub fn chunk_writer<W: Write + Seek, F: FnMut(&mut ChunkWriter<W>) -> Result<(), WriteError>>(
    writer: W,
    chunk_id: ChunkId,
    mut f: F,
) -> Result<(), WriteError> {
    let mut chunk_writer = ChunkWriter::new(writer, chunk_id)?;

    f(&mut chunk_writer)?;

    chunk_writer.write_header()?;

    Ok(())
}

#[derive(Debug, Error)]
#[error("The argument {pos:?} to seek is invalid.")]
struct InvalidSeek {
    current: u32,
    start: u32,
    end: u32,
    pos: SeekFrom,
}

fn seek_to(current: u32, start: u32, end: u32, pos: SeekFrom) -> Result<u32, IoError> {
    let (offset, delta) = match pos {
        SeekFrom::Current(pos) => (current, Some(pos)),
        SeekFrom::Start(pos) => (start, pos.try_into().ok()),
        SeekFrom::End(pos) => (end, Some(pos)),
    };

    let offset = i64::from(offset);
    let new_pos: Option<u32> = delta
        .and_then(|d| offset.checked_add(d))
        .and_then(|p| p.try_into().ok());

    new_pos.ok_or_else(|| {
        IoError::new(
            ErrorKind::Other,
            InvalidSeek {
                current,
                start,
                end,
                pos,
            },
        )
    })
}
