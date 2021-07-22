use mint::Vector3;

use crate::types::Vector;

impl<T> From<Vector<T>> for Vector3<T> {
    fn from(v: Vector<T>) -> Self {
        <[T; 3]>::from(v).into()
    }
}

impl<T> From<Vector3<T>> for Vector<T> {
    fn from(v: Vector3<T>) -> Self {
        Into::<[T; 3]>::into(v).into()
    }
}
