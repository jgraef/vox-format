//! Contains trait for for reading voxel data, and a simple implementation for
//! it.

#[cfg(feature = "serialize")]
use serde::{
    Deserialize,
    Serialize,
};

use crate::types::{
    Model,
    Palette,
    Size,
    Version,
    Voxel,
};

/// A simple implementation of [`VoxBuffer`] that collects voxels into `Vec`s.
pub type VoxData = VoxModels<Model>;

impl VoxModelBuffer for Model {
    fn new(size: Size) -> Self {
        Model {
            size,
            voxels: vec![],
        }
    }

    fn set_voxel(&mut self, voxel: Voxel, _palette: &Palette) {
        self.voxels.push(voxel);
    }
}

/// A trait for data structures that can be constructed from a VOX file.
/// [`crate::vox::VoxData`] implements this for convienience, but you can also
/// implement this for your own voxel model types.
///
/// These are always called in this order:
/// 1. `set_version`
/// 2. `set_palette`
/// 3. `set_num_models`
/// 4. `set_model_size`
///   1. `set_voxel`
///
/// `set_model_size` is always called before the voxels from this model are
/// passed via `set_voxel`. `set_model_size` is called for each model, and
/// `set_voxel` is called for each voxel in a model.
pub trait VoxBuffer {
    /// Called after the file version was read.
    ///
    /// The reader checks if the file version is supported, so most likely you
    /// can ignore this.
    fn set_version(&mut self, _version: Version) {}

    /// Called after the number of models was detected.
    fn set_num_models(&mut self, _num_models: usize) {}

    /// Called for each model before its voxels are being passed with
    /// [`VoxBuffer::set_voxel`].
    fn set_model_size(&mut self, _model_size: Size) {}

    /// Called for each voxel.
    fn set_voxel(&mut self, voxel: Voxel);

    /// Called when the color palette was read. This will be read before any
    /// calls to [`Self::set_voxel`].
    fn set_palette(&mut self, palette: Palette);
}

/// Trait for reading a single model.
pub trait VoxModelBuffer {
    fn new(size: Size) -> Self;
    fn set_voxel(&mut self, voxel: Voxel, palette: &Palette);
}

/// A [`VoxBuffer`] implementation that collects the models into a `Vec` and is
/// generic over the kind of voxel data.
#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct VoxModels<V> {
    pub version: Version,
    pub models: Vec<V>,
    pub palette: Palette,
}

impl<V> Default for VoxModels<V> {
    fn default() -> Self {
        Self {
            version: Version::default(),
            models: vec![],
            palette: Palette::default(),
        }
    }
}

impl<V: VoxModelBuffer> VoxBuffer for VoxModels<V> {
    fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    fn set_num_models(&mut self, num_models: usize) {
        self.models.reserve_exact(num_models);
    }

    fn set_model_size(&mut self, model_size: Size) {
        self.models.push(V::new(model_size));
    }

    fn set_voxel(&mut self, voxel: Voxel) {
        let model = self.models.last_mut().expect("model");
        model.set_voxel(voxel, &self.palette);
    }

    fn set_palette(&mut self, palette: Palette) {
        self.palette = palette;
    }
}
