//! Integration with `image` crate.

use image::{
    Rgba,
    RgbaImage,
};

use crate::{
    Color,
    Palette,
};

impl From<Color> for Rgba<u8> {
    fn from(color: Color) -> Self {
        Rgba(color.into())
    }
}

impl From<Rgba<u8>> for Color {
    fn from(color: Rgba<u8>) -> Self {
        color.0.into()
    }
}

impl Palette {
    /// Converts the palette into an RGBA image. The image will have dimensions
    /// 256 (width) x 1 (height).
    pub fn as_image(&self) -> RgbaImage {
        RgbaImage::from_fn(256, 1, |x, _| {
            let color = self.colors[x as usize];
            color.into()
        })
    }

    /// Converts a RGBA image to a color palette. Regardless of shape of the
    /// image, the first 256 pixels (row-first order) will be used. If there
    /// aren't enough pixels, the rest of the palette will be transparent.
    ///
    /// # Note
    ///
    /// VOX palettes have 256 colors, but VOX files only store the last 255
    /// colors. The first color is always assumed to be fully transparent. Thus
    /// any color for the first palette entry will be lost once the palette is
    /// written to file.
    pub fn from_image(image: &RgbaImage) -> Palette {
        let mut colors = [Color::default(); 256];
        image
            .pixels()
            .take(256)
            .copied()
            .enumerate()
            .for_each(|(i, px)| colors[i] = px.into());

        Palette { colors }
    }
}
