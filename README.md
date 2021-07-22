[![MIT license](https://img.shields.io/badge/license-MIT-brightgreen)](https://opensource.org/licenses/MIT)
![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)


**This is in development. The README is not accurate, as there is no release on crates.io yet.**


# `vox-format`

`vox-format` is a parser and encoder for MagicaVoxel's VOX files.

## Usage

In your `Cargo.toml` add:

```toml
vox-format = "0.1"
```

## Example

```rust
let vox_data = vox_format::from_file("test_files/glider.vox")?;
println!("{:#?}, vox_data);
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

 - [_] Support for undocumented chunk IDs. `PACK` is not used as it seems.
 - [_] Integrations:
   - [X]: `building-blocks`
   - [X]: `image`
   - [_]: `palette`
   - [_]: `mint` or `nalgebra`?
   - [_]: `serde`
 - [_] Write tests:
   - [X] Read single model
   - [X] Read multiple models
   - [X] Read custom palette
   - [_] Read materials
   - [_] ColorIndex behaviour.
 - [_] Is material ID a color index?
 - [_] Move `copy_map_chunks` to `vox_format::chunk`.
   - [_] Also copy children with `copy_map_chunks`.
 - [_] More documentation:
   - [_] docs for all items: `#![deny(missing_docs)]`
   - [_] more examples
 - [_] Some methods in `Chunk` take `&mut W` instead of `W` as argument.

# License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
