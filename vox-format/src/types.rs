//! Basic types

use std::{
    collections::HashMap,
    convert::{
        TryFrom,
        TryInto,
    },
    fmt,
    io::{
        Read,
        Write,
    },
    ops::Index,
};

use byteorder::{
    ReadBytesExt,
    WriteBytesExt,
    LE,
};
#[cfg(feature = "serialize")]
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

use crate::{
    default_palette::DEFAULT_PALETTE,
    reader::Error as ReadError,
    writer::Error as WriteError,
};

/// The version of a `.VOX` file. This is a wrapper around a `u32` and
/// implements `Default` and the [`Version::is_supported`] method.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, Deserialize),
    serde(transparent)
)]
pub struct Version(pub u32);

impl Version {
    /// Returns whether this version is supported.
    pub fn is_supported(&self) -> bool {
        *self == Self::default()
    }
}

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

impl Version {
    /// Reads a version from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        Ok(Self(reader.read_u32::<LE>()?))
    }

    /// Writes the version to a [`std::io::Write`].
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        writer.write_u32::<LE>(self.0)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// A collection of voxels with a specific size.
///
/// Note, that the voxels are
/// stored as a `Vec`, like they're stored in-file. Therefore there is no
/// efficient point-query for voxels. If you need your models to support
/// point-queries, you must implement your own [`crate::data::VoxBuffer`] or
/// [`crate::data::VoxModelBuffer`]. For testing and convienience there is
/// [`Model::get_voxel`] to perform a point-query with a linear search.
pub struct Model {
    /// Size of the model in voxels.
    pub size: Size,
    pub voxels: Vec<Voxel>,
}

impl Model {
    /// Looks up the voxel with the coordinates of `point`. Since [`Model`]
    /// stores [`Voxel`]s in a `Vec`, this performs a linear search, and should
    /// be avoided if possible.
    pub fn get_voxel(&self, point: Vector<i8>) -> Option<&Voxel> {
        self.voxels.iter().find(|voxel| voxel.point == point)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Voxel {
    pub point: Point,
    pub color_index: ColorIndex,
}

impl Voxel {
    /// Creates a new voxel
    ///
    /// # Arguments
    ///
    /// - `point`: The point location of the voxel.
    /// - `color_index`: The color index for this voxel.
    ///
    /// # Returns
    ///
    /// The newly created voxel.
    pub fn new(point: impl Into<Point>, color_index: impl Into<ColorIndex>) -> Self {
        Self {
            point: point.into(),
            color_index: color_index.into(),
        }
    }

    /// Reads a voxel from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        Ok(Self {
            point: Point::read(&mut reader)?,
            color_index: ColorIndex::read(&mut reader)?,
        })
    }

    /// Writes the voxel to a [`std::io::Write`].
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        self.point.write(&mut writer)?;
        self.color_index.write(&mut writer)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T> {
    /// Creates a vector from its components.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Vector<i8> {
    /// Reads a vector from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        Ok(Self {
            x: reader.read_i8()?,
            y: reader.read_i8()?,
            z: reader.read_i8()?,
        })
    }

    /// Writes the vector to a [`std::io::Write`].
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        writer.write_i8(self.x)?;
        writer.write_i8(self.y)?;
        writer.write_i8(self.z)?;
        Ok(())
    }
}

impl Vector<u32> {
    /// Reads a vector from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        Ok(Self {
            x: reader.read_u32::<LE>()?,
            y: reader.read_u32::<LE>()?,
            z: reader.read_u32::<LE>()?,
        })
    }

    /// Writes the vector to a [`std::io::Write`].
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        writer.write_u32::<LE>(self.x)?;
        writer.write_u32::<LE>(self.y)?;
        writer.write_u32::<LE>(self.z)?;
        Ok(())
    }
}

impl<T> From<[T; 3]> for Vector<T> {
    fn from(v: [T; 3]) -> Self {
        let [x, y, z] = v;
        Self::new(x, y, z)
    }
}

impl<T> From<Vector<T>> for [T; 3] {
    fn from(v: Vector<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

pub type Point = Vector<i8>;
pub type Size = Vector<u32>;

/// A color palette. This contains colors indexec by `u8`. It is used to look up
/// colors of a voxel.
///
/// If you need MagicaVoxel's default palette, you can either use
/// [`crate::default_palette::DEFAULT_PALETTE`], or [`Palette::default`].
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, Deserialize),
    serde(transparent)
)]
pub struct Palette {
    /// The colors of the palette.
    ///
    /// MagicaVoxel always set color 0 to be fully transparent. We don't enfore
    /// this, but this palette entry will never be written to a `.VOX` file.
    #[cfg_attr(feature = "serialize", serde(with = "serde_big_array::BigArray"))]
    pub colors: [Color; 256],
}

impl Default for Palette {
    fn default() -> Self {
        DEFAULT_PALETTE.clone()
    }
}

impl Palette {
    /// Tests whether this is the [`crate::default_palette::DEFAULT_PALETTE`].
    pub fn is_default(&self) -> bool {
        self.colors == DEFAULT_PALETTE.colors
    }

    /// Returns the color for an index. Since all color indices are valid, this
    /// always returns a value. This is equivalent to `palette[index]`.
    pub fn get(&self, color_index: ColorIndex) -> Color {
        self.colors[color_index.0 as usize]
    }

    /// Creates an iterator over all colors.
    ///
    /// ```
    /// # let palette = vox_format::types::Palette::default();
    /// for (index, color) in palette.iter() {
    ///     println!("{} -> {:?}", index, color);
    /// }
    /// ```
    pub fn iter(&self) -> PaletteIter {
        PaletteIter {
            inner: self.colors.iter().enumerate(),
        }
    }

    /// Reads a color palette from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        let mut palette = Palette::default();

        for i in 0..255 {
            palette.colors[i + 1] = Color::read(&mut reader)?;
        }

        Ok(palette)
    }

    /// Writes the color palette to a [`std::io::Write`].
    ///
    /// MagicaVoxel fixes color 0 to be always fully transparent, and doesn't
    /// include it in the file. Therefore this method will ignore the value
    /// for color 0.
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        for color in &self.colors[1..] {
            color.write(&mut writer)?;
        }

        Ok(())
    }
}

/// An iterator over entries in a [`Palette`]. This is created with
/// [`Palette::iter`].
#[derive(Debug)]
pub struct PaletteIter<'a> {
    inner: std::iter::Enumerate<std::slice::Iter<'a, Color>>,
}

impl<'a> Iterator for PaletteIter<'a> {
    type Item = (ColorIndex, Color);

    fn next(&mut self) -> Option<Self::Item> {
        let (index, color) = self.inner.next()?;
        Some((ColorIndex(index as u8), *color))
    }
}

impl Index<ColorIndex> for Palette {
    type Output = Color;

    fn index(&self, color_index: ColorIndex) -> &Self::Output {
        &self.colors[color_index.0 as usize]
    }
}

/// A palette of materials
///
/// # Work-in-Progress
///
/// This interface his likely to change in the future and is not fully
/// implemented yet.
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, Deserialize),
    serde(transparent)
)]
pub struct MaterialPalette {
    /// TODO: Does the material ID correspond to a ColorIndex?
    materials: HashMap<ColorIndex, Material>,
}

impl MaterialPalette {
    /// Tests if the material palette is empty.
    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
    }

    /// Returns the  material with ID `material_id` from the palette. Returns
    /// `None`, if there is no material with this ID. This is equivalent to
    /// `material_palette[material_id]`.
    pub fn get(&self, material_id: ColorIndex) -> Option<&Material> {
        self.materials.get(&material_id)
    }

    /// Creates an iterator over all materials.
    ///
    /// ```
    /// # let material_palette = vox_format::types::MaterialPalette::default();
    /// for (id, material) in material_palette.iter() {
    ///     println!("{} -> {:#?}", id, material);
    /// }
    /// ```
    ///
    /// # Work-in-Progress
    ///
    /// This interface his likely to change in the future and is not fully
    /// implemented yet.
    pub fn iter(&self) -> MaterialPaletteIter {
        MaterialPaletteIter {
            inner: self.materials.iter(),
        }
    }
}

/// An iterator over entries in a [`MaterialPalette`]. This is created with
/// [`MaterialPalette::iter`].
#[derive(Debug)]
pub struct MaterialPaletteIter<'a> {
    inner: std::collections::hash_map::Iter<'a, ColorIndex, Material>,
}

impl<'a> Iterator for MaterialPaletteIter<'a> {
    type Item = (ColorIndex, &'a Material);

    fn next(&mut self) -> Option<Self::Item> {
        let (index, material) = self.inner.next()?;
        Some((*index, material))
    }
}

/// An 8-bit RGBA color.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Color {
    /// Red channel
    pub r: u8,

    /// Green channel
    pub g: u8,

    /// Blue channel
    pub b: u8,

    /// Alpha channel. `0` is fully transparent, `255` is fully opaque.
    pub a: u8,
}

impl Color {
    /// Creates a new color from its channels.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Reads a color from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        // FIXME: I think color is stored in ABGR format.
        Ok(Self {
            r: reader.read_u8()?,
            g: reader.read_u8()?,
            b: reader.read_u8()?,
            a: reader.read_u8()?,
        })
    }

    /// Writes the color to a [`std::io::Write`].
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        // FIXME: I think color is stored in ABGR format.
        writer.write_u8(self.r)?;
        writer.write_u8(self.g)?;
        writer.write_u8(self.b)?;
        writer.write_u8(self.a)?;
        Ok(())
    }

    /// A light-blue color. This is the color that is selected by MagicaVoxel by
    /// default. It has index `79` in the [default
    /// palette](`crate::default_palette::DEFAULT_PALETTE`)
    pub fn light_blue() -> Self {
        Self {
            r: 153,
            g: 204,
            b: 255,
            a: 255,
        }
    }
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, Deserialize),
    serde(transparent)
)]
pub struct ColorIndex(pub u8);

impl ColorIndex {
    /// Reads a color index from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        Ok(Self(reader.read_u8()?))
    }

    /// Writes the color index to a [`std::io::Write`].
    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        writer.write_u8(self.0)?;
        Ok(())
    }

    /// The index selected by default (79). With the default palette this is a
    /// [light-blue](`Color::light_blue`).
    pub fn default_index() -> Self {
        Self(79)
    }
}

impl From<u8> for ColorIndex {
    fn from(x: u8) -> Self {
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

/// A material definition.
///
/// # Work-in-Progress
///
/// This interface his likely to change in the future and is not fully
/// implemented yet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Material {
    /// The type of material.
    pub ty: MaterialType,

    /// The mateiral weight. This has a different meaning depending on the
    /// material type:
    ///  - [`MaterialType::Diffuse`]: Always `1.0`.
    ///  - [`MaterialType::Metal`]: Blends between metal and diffuse material.
    ///    Must be in interval `(0.0, 1.0]`.
    ///  - [`MaterialType::Glass`]: Blends between glass and diffuse material.
    ///    Must be in interval `(0.0, 1.0]`.
    ///  - [`MaterialType::Emissive`]: The intensity of emitted light. Must be
    ///    in interval `(0.0, 1.0]`.
    pub weight: f32,

    pub plastic: Option<f32>,
    pub roughness: Option<f32>,
    pub specular: Option<f32>,
    pub ior: Option<f32>,
    pub attenuation: Option<f32>,
    pub power: Option<f32>,
    pub glow: Option<f32>,
    pub is_total_power: bool,
}

impl Material {
    /// Reads a material definition from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        let ty = MaterialType::read(&mut reader)?;
        let weight = reader.read_f32::<LE>()?;
        let flags = reader.read_u32::<LE>()?;

        let plastic = (flags & 1 != 0)
            .then(|| reader.read_f32::<LE>())
            .transpose()?;
        let roughness = (flags & 2 != 0)
            .then(|| reader.read_f32::<LE>())
            .transpose()?;
        let specular = (flags & 4 != 0)
            .then(|| reader.read_f32::<LE>())
            .transpose()?;
        let ior = (flags & 8 != 0)
            .then(|| reader.read_f32::<LE>())
            .transpose()?;
        let attenuation = (flags & 16 != 0)
            .then(|| reader.read_f32::<LE>())
            .transpose()?;
        let power = (flags & 32 != 0)
            .then(|| reader.read_f32::<LE>())
            .transpose()?;
        let glow = (flags & 64 != 0)
            .then(|| reader.read_f32::<LE>())
            .transpose()?;

        Ok(Material {
            ty,
            weight,
            plastic,
            roughness,
            specular,
            ior,
            attenuation,
            power,
            glow,
            is_total_power: (flags & 128 != 0),
        })
    }
}

/// A material type.
///
/// # Work-in-Progress
///
/// This interface his likely to change in the future and is not fully
/// implemented yet.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum MaterialType {
    Diffuse,
    Metal,
    Glass,
    Emissive,
}

impl TryFrom<u8> for MaterialType {
    type Error = MaterialTryFromError;

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            0 => Ok(MaterialType::Diffuse),
            1 => Ok(MaterialType::Metal),
            2 => Ok(MaterialType::Glass),
            3 => Ok(MaterialType::Emissive),
            x => Err(MaterialTryFromError(x)),
        }
    }
}

impl From<MaterialType> for u8 {
    fn from(ty: MaterialType) -> Self {
        match ty {
            MaterialType::Diffuse => 0,
            MaterialType::Metal => 1,
            MaterialType::Glass => 2,
            MaterialType::Emissive => 3,
        }
    }
}

impl MaterialType {
    /// Reads a material type from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        reader
            .read_u8()?
            .try_into()
            .map_err(|e: MaterialTryFromError| ReadError::InvalidMaterial { material_type: e.0 })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), WriteError> {
        writer.write_u8((*self).into())?;
        Ok(())
    }
}

/// A transform node.
///
/// # Work-in-Progress
///
/// This interface his likely to change in the future and is not fully
/// implemented yet.
#[derive(Debug, Error)]
#[error("Invalid material type: {0}")]
pub struct MaterialTryFromError(pub u8);

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Transform {
    pub node_id: u32,
    pub attributes: Attributes,
    pub child_node_id: u32,
    pub reserved_id: Option<u32>,
    pub layer_id: Option<u32>,
    pub frames: Vec<Attributes>,
}

impl Transform {
    /// Reads a transform node from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        let node_id = reader.read_u32::<LE>()?;
        let attributes = Attributes::read(&mut reader)?;
        let child_node_id = reader.read_u32::<LE>()?;
        let reserved_id = read_id_opt(&mut reader)?;
        let layer_id = read_id_opt(&mut reader)?;

        let num_frames = reader.read_u32::<LE>()?;
        let mut frames = vec![];
        for _ in 0..num_frames {
            frames.push(Attributes::read(&mut reader)?);
        }

        Ok(Self {
            node_id,
            attributes,
            child_node_id,
            reserved_id,
            layer_id,
            frames,
        })
    }

    pub fn get_transform(&self, frame: usize) -> Option<Vector<i32>> {
        let mut parts = self.frames.get(frame)?.get("_t")?.split_whitespace();
        let x = parts.next()?.parse().ok()?;
        let y = parts.next()?.parse().ok()?;
        let z = parts.next()?.parse().ok()?;

        parts.next().is_none().then(|| Vector::new(x, y, z))
    }
}

/// A group node.
///
/// # Work-in-Progress
///
/// This interface his likely to change in the future and is not fully
/// implemented yet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Group {
    pub node_id: u32,
    pub attributes: Attributes,
    pub children: Vec<u32>,
}

impl Group {
    /// Reads a group from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        let node_id = reader.read_u32::<LE>()?;
        let attributes = Attributes::read(&mut reader)?;
        let num_children = reader.read_u32::<LE>()?;
        let mut children = Vec::with_capacity(num_children as usize);

        for _ in 0..num_children {
            children.push(reader.read_u32::<LE>()?);
        }

        Ok(Self {
            node_id,
            attributes,
            children,
        })
    }
}

/// A shape node.
///
/// # Work-in-Progress
///
/// This interface his likely to change in the future and is not fully
/// implemented yet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Shape {
    pub node_id: u32,
    pub attributes: Attributes,
}

impl Shape {
    /// Reads a shape node from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        Ok(Self {
            node_id: reader.read_u32::<LE>()?,
            attributes: Attributes::read(reader)?,
        })
    }
}

/// A layer node.
///
/// # Work-in-Progress
///
/// This interface his likely to change in the future and is not fully
/// implemented yet.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Layer {
    pub node_id: u32,
    pub attributes: Attributes,
    pub reserved_id: Option<u32>,
}

impl Layer {
    /// Reads a layer node from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        Ok(Self {
            node_id: reader.read_u32::<LE>()?,
            attributes: Attributes::read(&mut reader)?,
            reserved_id: read_id_opt(reader)?,
        })
    }
}

/// Node attributes. These contain meta-data for nodes, such as [`Transform`] or
/// [`Layer`].
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, Deserialize),
    serde(transparent)
)]
pub struct Attributes {
    inner: HashMap<String, String>,
}

impl Attributes {
    /// Reads attributes from a [`std::io::Read`].
    pub fn read<R: Read>(mut reader: R) -> Result<Self, ReadError> {
        // An array of key value pairs, where key and value are strings prefixed with
        // length as u32

        let mut inner = HashMap::new();
        let num_items = reader.read_u32::<LE>()?;
        log::trace!("Attributes::read: num_items={}", num_items);
        for _ in 0..num_items {
            let key = Self::read_string(&mut reader)?;
            let value = Self::read_string(&mut reader)?;
            log::trace!("Attributes::read: key={}, value={}", key, value);
            inner.insert(key, value);
        }

        Ok(Attributes { inner })
    }

    fn read_string<R: Read>(mut reader: R) -> Result<String, ReadError> {
        let len = reader.read_u32::<LE>()?;
        log::trace!("Attributes::read_string: len={}", len);
        let mut buf = vec![0; len.try_into().expect("int overflow")];
        reader.read_exact(&mut buf)?;
        log::trace!("Attributes::read_string: buf={:?}", buf);
        Ok(String::from_utf8(buf)?)
    }

    /// Returns the attribute with the given key, or `None`, if no such
    /// attribute exists.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        Some(self.inner.get(key.as_ref())?.as_str())
    }

    /// Creates an iterator over the attributes. The iterator returns items
    /// `(&str, &str)`.
    pub fn iter(&self) -> AttributesIter {
        AttributesIter {
            inner: self.inner.iter(),
        }
    }
}

/// An interator over attributes. Created with [`Attributes::iter`].
#[derive(Debug)]
pub struct AttributesIter<'a> {
    inner: std::collections::hash_map::Iter<'a, String, String>,
}

impl<'a> Iterator for AttributesIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let (key, value) = self.inner.next()?;
        Some((key.as_ref(), value.as_ref()))
    }
}

fn read_id_opt<R: Read>(mut reader: R) -> Result<Option<u32>, ReadError> {
    Ok(reader.read_i32::<LE>()?.try_into().ok())
}
