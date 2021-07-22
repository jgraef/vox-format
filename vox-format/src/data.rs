use crate::types::{
    ColorIndex,
    Material,
    MaterialPalette,
    Model,
    Palette,
    Size,
    Version,
    Voxel,
};

#[derive(Clone, Debug)]
pub struct VoxData {
    pub version: Version,
    pub models: Vec<Model>,
    pub palette: Palette,
    pub materials: MaterialPalette,
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
            materials: MaterialPalette::default(),
        }
    }
}

impl VoxBuffer for VoxData {
    fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    fn set_num_models(&mut self, num_models: usize) {
        self.models.reserve_exact(num_models);
    }

    fn set_model_size(&mut self, size: Size) {
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

    fn set_material(&mut self, material_id: ColorIndex, material: Material) {
        self.materials.insert(material_id, material);
    }
}

/// A trait for data structures that can constructed from a VOX file.
/// `[crate::vox::VoxData]` implements this for convienience, but you can also
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
    fn set_version(&mut self, version: Version);

    fn set_num_models(&mut self, num_models: usize);

    fn set_model_size(&mut self, model_size: Size);

    fn set_voxel(&mut self, voxel: Voxel);

    fn set_palette(&mut self, palette: Palette);

    fn set_material(&mut self, material_id: ColorIndex, material: Material);
}

/// Trait for reading a single model.
pub trait VoxModelBuf {
    fn new(size: Size) -> Self;
    fn set_voxel(&mut self, voxel: Voxel, palette: &Palette);
}

/// A [`VoxBuffer`] implementation that collects the models into a `Vec` and is
/// generic over the kind of voxel data.
#[derive(Debug)]
pub struct VoxModels<V> {
    pub models: Vec<V>,
    pub palette: Palette,
    pub materials: MaterialPalette,
}

impl<V: VoxModelBuf> VoxBuffer for VoxModels<V> {
    fn set_version(&mut self, _version: Version) {}

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

    fn set_material(&mut self, material_id: ColorIndex, material: Material) {
        self.materials.insert(material_id, material);
    }
}
