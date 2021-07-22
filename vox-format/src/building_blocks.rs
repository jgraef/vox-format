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
        Point,
        Size,
        Vector,
        Voxel,
    },
};

impl<T> From<Vector<T>> for PointN<[T; 3]> {
    fn from(v: Vector<T>) -> Self {
        PointN(v.into())
    }
}

impl From<Size> for Extent3i {
    fn from(size: Size) -> Self {
        // Note: This can fail, if the component is greater than `i32::MAX`
        Extent3i::from_min_and_shape(
            Default::default(),
            PointN([size.x as i32, size.y as i32, size.z as i32]),
        )
    }
}

impl From<Point> for Point3i {
    fn from(point: Point) -> Self {
        PointN([point.x as i32, point.y as i32, point.z as i32])
    }
}

impl VoxModelBuf for Array3x1<ColorIndex> {
    fn new(size: Size) -> Self {
        Array3x1::fill_with(size.into(), |_point| ColorIndex::default())
    }

    fn set_voxel(&mut self, voxel: Voxel, _palette: &Palette) {
        let point = Point3i::from(voxel.point);
        *self.get_mut(point) = voxel.color_index;
    }
}

impl VoxModelBuf for Array3x1<Color> {
    fn new(size: Size) -> Self {
        Array3x1::fill_with(size.into(), |_point| Color::default())
    }

    fn set_voxel(&mut self, voxel: Voxel, palette: &Palette) {
        let point = Point3i::from(voxel.point);
        *self.get_mut(point) = palette[voxel.color_index];
    }
}
