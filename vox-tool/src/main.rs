use std::{
    collections::HashSet,
    convert::TryInto,
    fs::{
        File,
        OpenOptions,
    },
    io::{
        Read,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
};

use color_eyre::eyre::{
    bail,
    Error,
};
use image::io::Reader as ImageReader;
use structopt::StructOpt;
use vox_format::{
    chunk::{
        read_main_chunk,
        Chunk,
        ChunkId,
        ChunkWriter,
    },
    default_palette::DEFAULT_PALETTE,
    from_file,
    types::{
        Model,
        Palette,
    },
    writer::main_chunk_writer,
};

/// Tools for inspection and manipulation of MagicaVoxel VOX files.
#[derive(Debug, StructOpt)]
enum Args {
    /// Strips chunks from the VOX file.
    Strip {
        /// Selects chunks to strip. If omitted, everything except chunks
        /// specified with `--keep` will be stripped.
        #[structopt(short = "s", long = "strip")]
        strip: Vec<String>,

        /// Keep the specified chunks. If this and `--strip` is omitted, every
        /// chunk except `PACK`, `XYZI`, `RGBA` and `MATT` is stripped.
        #[structopt(short = "k", long = "keep")]
        keep: Vec<String>,

        /// Output file. Defaults to `INPUT.stripped.vox` where `INPUT` is the
        /// input file path without `.vox` file extension.
        #[structopt(short = "o", long = "output")]
        output: Option<PathBuf>,

        /// The input file from which the chunks will be stripped.
        input: PathBuf,
    },
    /// Prints info about a VOX file.
    PrintInfo {
        /// The input file from which information willbe displayed
        input: PathBuf,

        /// Prints the palette. This will not print the palette if it's the
        /// default palette. If you want this, use `-P` instead.
        #[structopt(short = "p", long = "palette")]
        print_palette: bool,

        /// Prints the palette - even if it's the default palette.
        #[structopt(short = "P")]
        print_palette_even_if_default: bool,

        /// Print all models. This overrides anything set with `-m` or
        /// `--model`.
        #[structopt(short = "M", long = "all-models")]
        print_all_models: bool,

        /// Print only the model with the specified index.
        #[structopt(short = "m", long = "model")]
        model_index: Option<usize>,
    },
    /// Exports a palette as image.
    ExportPalette {
        /// The input file from which the palette will be exported. If omitted,
        /// the default palette will be exported.
        input: Option<PathBuf>,

        /// The path for the output file. The file format will be guessed using
        /// the file extension.
        #[structopt(short = "o", long = "output")]
        output: PathBuf,
    },
    /* /// Exports a slice of the volume as image.
    ExportSlice {
        #[structopt(short = "x")]
        x: Option<i8>,

        #[structopt(short = "y")]
        y: Option<i8>,

        #[structopt(short = "z")]
        z: Option<i8>,

        #[structopt(short = "o", long = "output")]
        ouptut: PathBuf,
    },*/
    /// Replaces the palette in a VOX file.
    ///
    /// The palette is specified with `--palette` option and must be an image.
    /// Regardless of the image's shape, the first 256 pixels will be used
    /// for the palette.
    ///
    /// The images will be converted to RGBA values to be used in the palette.
    ///
    /// Note that entry 0 in the palette is special, in that it's always
    /// transparent. If you set another color for that pixel, it will be
    /// ignored.
    SetPalette {
        /// The input file that will have it's palette changed.
        input: PathBuf,

        /// Path to image containing the palette. This his compatible with
        /// `export-palette`. If omitted, the default palette
        /// will be used.
        #[structopt(short = "p", long = "palette")]
        palette: Option<PathBuf>,

        /// The path for the output file. The file format will be guessed using
        /// the file extension.
        #[structopt(short = "o", long = "output")]
        output: Option<PathBuf>,
    },
}

#[derive(Debug)]
pub enum StripChunks {
    Strip(HashSet<ChunkId>),
    Keep(HashSet<ChunkId>),
}

impl Default for StripChunks {
    fn default() -> Self {
        let mut s = HashSet::new();
        s.insert("PACK".parse().unwrap());
        s.insert("SIZE".parse().unwrap());
        s.insert("XYZI".parse().unwrap());
        s.insert("RGBA".parse().unwrap());
        s.insert("MATT".parse().unwrap());

        Self::Keep(s)
    }
}

impl StripChunks {
    fn strip(&self, chunk_id: ChunkId) -> bool {
        match self {
            Self::Strip(s) => s.contains(&chunk_id),
            Self::Keep(s) => !s.contains(&chunk_id),
        }
    }
}

impl Args {
    fn run(self) -> Result<(), Error> {
        match self {
            Self::Strip {
                strip,
                keep,
                input,
                output,
            } => {
                let strip = if !strip.is_empty() && !keep.is_empty() {
                    bail!("The options `--strip` and `--keep` are exlusive");
                }
                else if !strip.is_empty() {
                    StripChunks::Strip(strip.iter().map(|s| s.parse()).collect::<Result<_, _>>()?)
                }
                else if !keep.is_empty() {
                    StripChunks::Keep(keep.iter().map(|s| s.parse()).collect::<Result<_, _>>()?)
                }
                else {
                    StripChunks::default()
                };

                log::debug!("Stripping: {:?}", strip);

                let output = output.unwrap_or_else(|| default_output_path(&input, "stripped"));

                copy_map_chunks(&input, &output, |_reader, chunk, _writer| {
                    if !strip.strip(chunk.id()) {
                        Ok(true)
                    }
                    else {
                        log::trace!("Stripping chunk: {:?}", chunk.id());
                        Ok(false)
                    }
                })?;
            }
            Self::PrintInfo {
                input,
                print_palette,
                print_palette_even_if_default,
                print_all_models,
                model_index,
            } => {
                let vox = from_file(input)?;

                println!("VOX version: {}", vox.version);

                if print_all_models {
                    for (i, model) in vox.models.iter().enumerate() {
                        print_model(i, model);
                    }
                }
                else if let Some(model_index) = model_index {
                    if let Some(model) = vox.models.get(model_index) {
                        print_model(model_index, model);
                    }
                    else {
                        eprintln!("Model with index {} does not exists. There are {} models in this file.", model_index, vox.models.len());
                    }
                }

                if print_palette {
                    if vox.palette.is_default() && !print_palette_even_if_default {
                        println!("Palette: default");
                    }
                    else {
                        println!("Palette:");
                        for (index, color) in vox.palette.iter() {
                            println!("  - #{}: {:?}", index, color);
                        }
                    }
                }
            }
            Self::ExportPalette { input, output } => {
                let vox;
                let palette = if let Some(input) = input {
                    vox = from_file(input)?;
                    &vox.palette
                }
                else {
                    &DEFAULT_PALETTE
                };

                let image = palette.as_image();
                image.save(output)?;
            }
            Self::SetPalette {
                input,
                palette,
                output,
            } => {
                let palette = if let Some(palette) = palette {
                    let image = ImageReader::open(palette)?.decode()?;

                    // TODO: It would be nicer to pass an `ImageBuffer` with any pixel format and
                    // then just convert the pixels we need.
                    let image = image.into_rgba8();

                    Palette::from_image(&image)
                }
                else {
                    DEFAULT_PALETTE
                };

                let output = output.unwrap_or_else(|| default_output_path(&input, "new-palette"));

                copy_map_chunks(&input, &output, |_reader, chunk, writer| {
                    if matches!(chunk.id(), ChunkId::Rgba) {
                        // Replace RGBA chunk
                        writer
                            .child_content_writer(ChunkId::Rgba, |writer| palette.write(writer))?;

                        Ok(false)
                    }
                    else {
                        Ok(true)
                    }
                })?;
            }
        }

        Ok(())
    }
}

fn print_model(i: usize, model: &Model) {
    println!("Model #{}: {:?}", i, model.size);

    for voxel in &model.voxels {
        println!("  - {:?}: #{}", voxel.point, voxel.color_index);
    }
}

fn default_output_path<P: AsRef<Path>>(input: P, postfix: &str) -> PathBuf {
    let input = input.as_ref();
    let ext = input.extension().and_then(|s| s.to_str()).unwrap_or("vox");
    input.with_extension(format!("{}.{}", postfix, ext))
}

fn copy_map_chunks<
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(&mut File, &Chunk, &mut ChunkWriter<File>) -> Result<bool, vox_format::writer::Error>,
>(
    input: P,
    output: Q,
    mut f: F,
) -> Result<(), Error> {
    let input = input.as_ref();
    let output = output.as_ref();

    log::debug!("Reading input: {}", input.display());
    let mut reader = File::open(&input)?;
    let (main_chunk, version) = read_main_chunk(&mut reader)?;

    log::debug!("Writing output: {}", output.display());
    let writer = OpenOptions::new().create(true).write(true).open(output)?;

    let mut chunks = vec![];

    for r in main_chunk.children(&mut reader) {
        let chunk = r?;
        chunks.push(chunk);
    }

    main_chunk_writer(writer, version, |chunk_writer| {
        let mut buf = vec![];

        for chunk in &chunks {
            if f(&mut reader, chunk, chunk_writer)? {
                // Copy chunk

                buf.clear();
                buf.reserve(chunk.content_len().try_into()?);

                chunk.content(&mut reader)?.read_to_end(&mut buf)?;

                chunk_writer.child_content_writer(chunk.id(), |writer| {
                    writer.write_all(&buf)?;
                    assert_eq!(writer.len(), chunk.content_len());
                    Ok(())
                })?;

                // TODO: If we move the copy function into `vox-format`, we can make use of the
                // fact that we can read/write the children as a blob.
                if chunk.children_len() != 0 {
                    todo!("TODO: Copy children. This is not implemented, because at this point all supported chunk types (except `MAIN`) have no children. Please open an issue, if you need this feature.");
                }
            }
        }

        Ok(())
    })?;

    Ok(())
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    color_eyre::install()?;
    pretty_env_logger::init();

    let args = Args::from_args();

    args.run()?;

    Ok(())
}
