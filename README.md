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

# `vox-tool`

`vox-tool` is a command-line tool to inspect and manipulate VOX files.

If you want some functionality for this tool, please open an issue or pull request.

## Usage

```
vox-tool 0.1.0
Tools for inspection and manipulation of MagicaVoxel VOX files

USAGE:
    vox-tool <SUBCOMMAND>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


SUBCOMMANDS:
    export-palette    Exports a palette as image
    help              Prints this message or the help of the given subcommand(s)
    print-info        Prints info about a VOX file
    set-palette       Replaces the palette in a VOX file
    strip             Strips chunks from the VOX file
```


In the repository run:

```sh
cargo run -- --help
```

or if you want to install the binary, run:

```sh
cargo install -p vox-tool --help
```

Then from any directory, run

```sh
vox-tool --help
```

# TODO

 - [ ] Finish implementing undocumented chunk IDs.
 - [ ] Function and trait to write arbitrary voxel data, and not only `VoxData`.
 - [ ] Move `copy_map_chunks` to `vox_format::chunk` and implement copying children.
 - [ ] Write tests:
   - [ ] Read materials
   - [ ] Material ID behaviour (is it just a `ColorIndex`?)
 - [ ] Support for `INFO`, `rOBJ` and `rCAM` chunks.

# License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
