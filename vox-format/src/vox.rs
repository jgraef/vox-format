//! Types used by reader and writer. Also contains [`VoxData`] which is a
//! built-in [`crate::reader::VoxBuffer`].

use std::{
    fmt,
    ops::Index,
};

use crate::{
    default_palette::DEFAULT_PALETTE,
    reader::VoxBuffer,
};

#[derive(Clone, Debug)]
pub struct VoxData {
    pub version: Version,
    pub models: Vec<Model>,
    pub palette: Palette,
}

impl Default for VoxData {
    fn default() -> Self {
        Self::new(Version::default())
    }
}

impl VoxData {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            models: vec![],
            palette: Palette::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(pub u32);

impl Default for Version {
    fn default() -> Self {
        Self(150)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct Model {
    pub size: Vector,
    pub voxels: Vec<Voxel>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Voxel {
    pub point: Vector,
    pub color_index: ColorIndex,
}

impl Voxel {
    pub fn new(point: impl Into<Vector>, color_index: impl Into<ColorIndex>) -> Self {
        Self {
            point: point.into(),
            color_index: color_index.into(),
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Vector {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }
}

impl From<[i8; 3]> for Vector {
    fn from(v: [i8; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
}

impl From<Vector> for [i8; 3] {
    fn from(v: Vector) -> Self {
        [v.x, v.y, v.z]
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: [Color; 256],
}

impl Default for Palette {
    fn default() -> Self {
        DEFAULT_PALETTE.clone()
    }
}

impl Palette {
    pub fn is_default(&self) -> bool {
        self.colors == DEFAULT_PALETTE.colors
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (ColorIndex, Color)> + 'a {
        self.colors[1..]
            .iter()
            .enumerate()
            .map(|(i, color)| (ColorIndex(i as u8), *color))
    }
}

impl Index<ColorIndex> for Palette {
    type Output = Color;

    fn index(&self, index: ColorIndex) -> &Self::Output {
        &self.colors[index.0 as usize + 1]
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<Color> for [u8; 4] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

impl From<[u8; 4]> for Color {
    fn from(color: [u8; 4]) -> Self {
        Self {
            r: color[0],
            g: color[0],
            b: color[0],
            a: color[0],
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorIndex(pub u8);

impl From<u8> for ColorIndex {
    fn from(x: u8) -> Self {
        if x == 255 {
            panic!("Invalid color index: 255");
        }
        Self(x)
    }
}

impl From<ColorIndex> for u8 {
    fn from(x: ColorIndex) -> Self {
        x.0
    }
}

impl fmt::Display for ColorIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl VoxBuffer for VoxData {
    fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    fn set_num_models(&mut self, num_models: usize) {
        self.models.reserve_exact(num_models);
    }

    fn set_model_size(&mut self, size: Vector) {
        self.models.push(Model {
            size,
            voxels: Vec::with_capacity((size.x * size.y * size.z) as usize),
        })
    }

    fn set_voxel(&mut self, voxel: Voxel) {
        let model = self
            .models
            .last_mut()
            .expect("Expected to have set_model_size called first.");
        model.voxels.push(voxel);
    }

    fn set_palette(&mut self, palette: Palette) {
        self.palette = palette;
    }
}
