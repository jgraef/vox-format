//! Provides functions to write VOX files. This is work-in-progress.

use std::{
    convert::TryInto,
    fs::OpenOptions,
    io::{
        Cursor,
        Seek,
        Write,
    },
    path::Path,
};

use byteorder::{
    WriteBytesExt,
    LE,
};
use thiserror::Error;

use crate::{
    chunk::{
        chunk_writer,
        ChunkId,
        ChunkWriter,
    },
    data::VoxData,
    types::Version,
};

/// Error type returned when writing fails.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    /// An integer overflowed.
    #[error("Integer overflow")]
    Overflow(#[from] std::num::TryFromIntError),

    /// This is a work-around,since sometimes we want to read VOX files in a
    /// chunk-writer closure.
    #[error("Reader error")]
    Reader(#[from] crate::reader::Error),
}

/// Writes the file header for a VOX file.
fn write_file_header<W: Write>(mut writer: W, version: Version) -> Result<(), Error> {
    writer.write_all(b"VOX ")?;
    version.write(writer)?;
    Ok(())
}

/// Writes the `MAIN` chunk including the file signature. The closure you pass,
/// will be called with the [`ChunkWriter`] for the `MAIN` chunk.
pub fn main_chunk_writer<W: Write + Seek, F: FnMut(&mut ChunkWriter<W>) -> Result<(), Error>>(
    mut writer: W,
    version: Version,
    f: F,
) -> Result<(), Error> {
    write_file_header(&mut writer, version)?;

    chunk_writer(writer, ChunkId::Main, f)
}

/// Writes [`crate::data::VoxData`] to a [`std::io::Write`].
pub fn to_writer<W: Write + Seek>(writer: W, vox: &VoxData) -> Result<(), Error> {
    main_chunk_writer(writer, Version::default(), |chunk_writer| {
        // Write PACK, if there is more than 1 model.
        // FIXME: Apparently PACK is not used anymore.
        if vox.models.len() > 1 {
            chunk_writer.child_content_writer(ChunkId::Pack, |writer| {
                writer.write_u32::<LE>(vox.models.len().try_into()?)?;
                Ok(())
            })?;
        }

        // Write models
        for model in &vox.models {
            // Write SIZE chunk
            chunk_writer.child_content_writer(ChunkId::Size, |writer| {
                model.size.write(writer)?;
                Ok(())
            })?;

            // Write XYZI chunk
            chunk_writer.child_content_writer(ChunkId::Xyzi, |mut writer| {
                writer.write_u32::<LE>(model.voxels.len().try_into()?)?;
                for voxel in &model.voxels {
                    voxel.write(&mut writer)?;
                }
                Ok(())
            })?;
        }

        // Write palette
        if !vox.palette.is_default() {
            chunk_writer.child_content_writer(ChunkId::Rgba, |writer| {
                vox.palette.write(writer)?;
                Ok(())
            })?;
        }

        Ok(())
    })
}

/// Encode [`VoxData`] and return bytes as `Vec<u8>`.
pub fn to_vec(vox: &VoxData) -> Result<Vec<u8>, Error> {
    //let mut buf = Vec::with_capacity(vox.size_hint());
    let mut buf = Vec::with_capacity(1024);
    to_writer(Cursor::new(&mut buf), vox)?;
    buf.shrink_to_fit();
    Ok(buf)
}

/// Writes VOX data to the specified path.
pub fn to_file<P: AsRef<Path>>(path: P, vox: &VoxData) -> Result<(), Error> {
    to_writer(OpenOptions::new().create(true).write(true).open(path)?, vox)
}
