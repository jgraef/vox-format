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

## For first release

 - [x] Reading of undocumented chunk IDs.
 - [x] Integrations:
   - [x] `image`
   - [x] `palette`
   - [x] `mint`
   - [x] `nalgebra`
   - [x] `serde`
 - [x] Write tests:
   - [x] Read single model
   - [x] Read multiple models
   - [x] Read custom palette
   - [x] ColorIndex behaviour.
 - [ ] More documentation:
   - [ ] docs for all items: `#![deny(missing_docs)]`
   - [ ] Some more examples for function docs
   - [ ] proof-read docs
   - [ ] Example programs (`cargo run --example`)
   - [ ] Check `vox-tool` usage
 - [x] Some methods in `Chunk` take `&mut W` instead of `W` as argument. And some calls unnecessarily use `&mut writer`.
 - [x] Proper iterator types, instead of `impl Iterator`.
 - [x] Remove `dev-dependencies`. They're only occasionaly used during debugging.
 - [x] Is `vox-tool` ready?
 - [ ] Update README
 - [ ] Last: grep for TODOs, run clippy, tests, fmt.


## After first release

 - [_] Finish implementing undocumented chunk IDs.
 - [_] Function and trait to write arbitrary voxel data, and not only `VoxData`.
 - [ ] Move `copy_map_chunks` to `vox_format::chunk` and implement copying children.
 - [_] Write tests:
   - [ ] Read materials
   - [ ] Material ID behaviour (is it just a `ColorIndex`?)

# License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)
