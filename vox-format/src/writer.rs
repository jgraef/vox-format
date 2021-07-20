use std::{
    fs::OpenOptions,
    io::{Cursor, Seek, Write},
    path::Path,
};

use byteorder::{WriteBytesExt, LE};
use thiserror::Error;

use crate::{
    chunk::{chunk_writer, ChunkId, ChunkWriter},
    vox::{Version, VoxData},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Integer overflow")]
    Overflow(#[from] std::num::TryFromIntError),
}

impl Version {
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), Error> {
        writer.write_u32::<LE>(self.0)?;
        Ok(())
    }
}

pub fn write_file_header<W: Write>(mut writer: W, version: Version) -> Result<(), Error> {
    writer.write_all(b"VOX ")?;
    version.write(writer)?;
    Ok(())
}

pub fn main_chunk_writer<W: Write + Seek, F: FnMut(&mut ChunkWriter<W>) -> Result<(), Error>>(mut writer: W, version: Version, f: F) -> Result<(), Error> {
    write_file_header(&mut writer, version)?;

    chunk_writer(writer, f, ChunkId::Main)
}

pub fn to_writer<W: Write>(_writer: W, _vox: &VoxData) -> Result<(), Error> {
    todo!()
}

pub fn to_vec(vox: &VoxData) -> Result<Vec<u8>, Error> {
    //let mut buf = Vec::with_capacity(vox.size_hint());
    let mut buf = Vec::with_capacity(1024);
    to_writer(Cursor::new(&mut buf), vox)?;
    buf.shrink_to_fit();
    Ok(buf)
}

pub fn to_file<P: AsRef<Path>>(path: P, vox: &VoxData) -> Result<(), Error> {
    to_writer(OpenOptions::new().write(true).open(path)?, vox)
}
