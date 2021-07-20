
** This is in development. The README is not accurate, as there is no release on crates.io yet. **


[![MIT license](https://img.shields.io/badge/license-MIT-brightgreen)](https://opensource.org/licenses/MIT)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)

# `vox-format`

`vox-format` is a parser and encoder for MagicaVoxel's VOX files.

## Usage

In your `Cargo.toml` add:

```toml
vox-format = "0.1"
```

## Example

```rust
let vox_data = vox_format::from_file("test_files/ore_small.vox")?;

println!("{:#?}, vox_data);

```



# `vox-tool`

`vox-tool` is a command-line tool to inspect and manipulate VOX files.

If you want some functionality for this tool, please open an issue or pull request.



[![MIT license](https://img.shields.io/badge/license-MIT-brightgreen)](https://opensource.org/licenses/MIT)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)


# TODO

 - Implement `Seek` for `ContentReader` and `ContentWriter`.
 - Integrate into `building-blocks`.


## License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
