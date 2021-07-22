use palette::Srgba;

use crate::types::Color;

impl From<Color> for Srgba<u8> {
    fn from(color: Color) -> Self {
        Srgba::from_components((color.r, color.g, color.b, color.a))
    }
}

impl From<Srgba<u8>> for Color {
    fn from(color: Srgba<u8>) -> Self {
        Self::new(
            color.color.red,
            color.color.green,
            color.color.blue,
            color.alpha,
        )
    }
}
