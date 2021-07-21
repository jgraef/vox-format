use std::{
    collections::HashSet,
    convert::TryInto,
    fs::{
        File,
        OpenOptions,
    },
    io::Read,
    path::{
        Path,
        PathBuf,
    },
};

use color_eyre::eyre::{
    bail,
    Error,
};
use structopt::StructOpt;
use vox_format::{chunk::{
        read_main_chunk,
        ChunkId,
    }, from_file, writer::main_chunk_writer};

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
    /// Print all VOX data as debug output.
    PrintDebug {
        /// The input file from which the palette will be read.
        input: PathBuf,        
    },
    /// Print the color palette of a VOX file.
    PrintPalette {
        /// The input file from which the palette will be read.
        input: PathBuf,
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

fn default_output_path(input: &Path) -> PathBuf {
    let ext = input.extension().and_then(|s| s.to_str()).unwrap_or("vox");
    input.with_extension(format!("stripped.{}", ext))
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

                log::debug!("Reading input: {}", input.display());
                let mut reader = File::open(&input)?;
                let (main_chunk, version) = read_main_chunk(&mut reader)?;

                let output = output.unwrap_or_else(|| default_output_path(&input));
                log::debug!("Writing output: {}", output.display());
                let writer = OpenOptions::new().create(true).write(true).open(output)?;

                let mut chunks = vec![];

                for r in main_chunk.children(&mut reader) {
                    let chunk = r?;

                    if !strip.strip(chunk.id()) {
                        log::debug!("Keeping chunk: {:?}, content_len={}, children_len={}", chunk.id(), chunk.content_len(), chunk.children_len());
                        chunks.push(chunk);
                    }
                    else {
                        log::trace!("Stripping chunk: {:?}", chunk.id());
                    }
                }

                main_chunk_writer(writer, version, |chunk_writer| {
                    let mut buf = vec![];

                    for chunk in &chunks {
                        buf.clear();
                        buf.reserve(chunk.content_len().try_into()?);

                        // FIXME: Handle the error
                        chunk.content(&mut reader).unwrap().read_to_end(&mut buf)?;

                        chunk_writer.child_writer(chunk.id(), |child_writer| {
                            child_writer.write_content(&buf)?;
                            assert_eq!(child_writer.content_len(), chunk.content_len());
                            Ok(())
                        })?;

                        if chunk.children_len() != 0 {
                            todo!("At this point all supported chunk types (except `MAIN`) have no children");
                        }
                    }

                    Ok(())
                })?;
            }
            Self::PrintDebug { input } => {
                let vox = from_file(input)?;
                println!("{:#?}", vox);
            }
            Self::PrintPalette { .. } => {
                todo!()
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    color_eyre::install()?;
    pretty_env_logger::init();

    let args = Args::from_args();

    args.run()?;

    Ok(())
}
