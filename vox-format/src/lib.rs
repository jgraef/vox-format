//! # Overview
//!
//! This crate provides a reader and write for [MagicaVoxel's](https://ephtracy.github.io/) VOX files. It was designed with the
//! in mind to keep copying to a minimum. While you can read a full VOX file
//! into a [`VoxData`] and then read or manipulate its data, there's also an
//! interface to construct your own voxel data from a VOX file.
//!
//! If you're looking for a command-line utility to manipulate VOX files, take a look at [`vox-tool`](https://docs.rs/vox-tool).
//! `vox-tool` is written using `vox-format`. so everything it does, can also be
//! achieved using only this crate in Rust.
//!
//! ## Features
//!
//! # `image` support
//!
//! This crate has support for some conversion between its types and [`image`](https://docs.rs/image/0.23.14/image/index.html)
//! types. Specifically between [`Color`] and `Rgba<u8>`.  But it also provides
//! methods to read and write palettes from images.
//!
//! # This crate is work-in-progress
//!
//! Although this crate has a very limited scope and already mostly implements
//! it, it is missing some features and isn't well-tested yet. This will change
//! soon, but if you just can't wait, you're welcome to contribute 🥺.
//!
//! # Examples
//!
//! Reads `VoxData` from path.
//!
//! ```rust
//! let vox_data = vox_format::from_file("../test_files/test_single_model_default_palette.vox").unwrap();
//! println!("{:#?}", vox_data);
//! ```

pub mod chunk;
pub mod data;
pub mod default_palette;
pub mod reader;
pub mod types;
pub mod writer;

pub use crate::{
    data::VoxData,
    reader::{
        from_file,
        from_reader,
        from_slice,
    },
    writer::{
        to_file,
        to_vec,
        to_writer,
    },
};

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "building-blocks")]
pub mod building_blocks;
