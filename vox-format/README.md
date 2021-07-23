[![crates.io](https://img.shields.io/crates/v/vox-format.svg)](https://crates.io/crates/vox-format)
[![docs.rs](https://docs.rs/vox-format/badge.svg)](https://docs.rs/vox-format)
[![MIT license](https://img.shields.io/badge/license-MIT-brightgreen)](https://opensource.org/licenses/MIT)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# `vox-format`

`vox-format` is a parser and encoder for MagicaVoxel's VOX files.

## Usage

In your `Cargo.toml` add:

```toml
[dependencies]
vox-format = "0.1"
```

## Example

```rust
let vox_data = vox_format::from_file("test_files/glider.vox")?;
println!("{:#?}", vox_data);
```
