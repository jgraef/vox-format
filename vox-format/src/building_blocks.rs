use building_blocks_core::{
    Extent3i,
    Point3i,
    PointN,
};
use building_blocks_storage::{
    Array3x1,
    GetMut,
};

use crate::{
    data::VoxModelBuf,
    types::{
        Color,
        ColorIndex,
        Palette,
        Vector,
        Voxel,
    },
};

impl From<Vector> for Point3i {
    fn from(v: Vector) -> Self {
        PointN([v.x as i32, v.y as i32, v.z as i32])
    }
}

impl VoxModelBuf for Array3x1<ColorIndex> {
    fn new(size: Vector) -> Self {
        Array3x1::fill_with(
            Extent3i::from_min_and_max(Point3i::default(), size.into()),
            |_point| ColorIndex::default(),
        )
    }

    fn set_voxel(&mut self, voxel: Voxel, _palette: &Palette) {
        let point = Point3i::from(voxel.point);
        *self.get_mut(point) = voxel.color_index;
    }
}

impl VoxModelBuf for Array3x1<Color> {
    fn new(size: Vector) -> Self {
        Array3x1::fill_with(
            Extent3i::from_min_and_max(Point3i::default(), size.into()),
            |_point| Color::default(),
        )
    }

    fn set_voxel(&mut self, voxel: Voxel, palette: &Palette) {
        let point = Point3i::from(voxel.point);
        *self.get_mut(point) = palette[voxel.color_index];
    }
}
