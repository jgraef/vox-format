use std::{
    borrow::Cow,
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
    pub palette: Cow<'static, Palette>,
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
            palette: Cow::Borrowed(&DEFAULT_PALETTE),
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: [Color; 256],
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            colors: [Color::default(); 256],
        }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorIndex(pub u8);

#[derive(Debug, Default)]
pub struct VoxDataBuffer {
    version: Version,
    models: Vec<Model>,
    palette: Option<Palette>,
}

impl VoxDataBuffer {
    pub fn build(self) -> VoxData {
        VoxData {
            version: self.version,
            models: self.models,
            palette: self
                .palette
                .map(|palette| Cow::Owned(palette))
                .unwrap_or_else(|| Cow::Borrowed(&DEFAULT_PALETTE)),
        }
    }
}

impl VoxBuffer for VoxDataBuffer {
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
        self.palette = Some(palette);
    }
}
