use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use termal_alignment::{
    Alignment,
    rgb::ColorMapName,
    rgb::ResidueColorMap,
    seq::{fasta, file},
};

// A wrapper for rgb::ColorMapName. This allows us to decouple that struct from Clap::ValueEnum.
// NOTE: the capitalisation differs slightly from ColorMapName's because of the way Clap derives
// valid CLI values.
#[derive(Copy, Clone, Debug, ValueEnum)]
enum ColorMapArg {
    AALesk,
    AAClustalx,
    DNAJalview,
    Monochrome,
}

// allows .into()
impl From<ColorMapArg> for ColorMapName {
    fn from(v: ColorMapArg) -> Self {
        match v {
            ColorMapArg::AALesk => ColorMapName::AALesk,
            ColorMapArg::AAClustalx => ColorMapName::AAClustalX,
            ColorMapArg::DNAJalview => ColorMapName::DNAJalView,
            ColorMapArg::Monochrome => ColorMapName::Monochrome,
        }
    }
}

// This crate's lib.rs, also used by termal-msa (the TUI app)
use termal_export::{compute_layout, export_svg, ExportOpts}; 

#[derive(Parser, Debug)]
#[command(name = "termal-export")]
#[command(about = "Export a (region of a) multiple sequence alignment as graphics", long_about = None)]
struct Args {
    /// Input alignment file
    input: PathBuf,

    /// Output SVG file ("-" for stdout)
    #[arg(short, long, default_value = "-")]
    output: String,

    /// Colormap 
    #[arg(short, long, value_enum, default_value_t = ColorMapArg::AAClustalx)]
    colormap_name: ColorMapArg,

    /// Row range as START:END (0-based, END exclusive). Example: 0:50
    #[arg(long)]
    rows: Option<String>,

    /// Column range as START:END (0-based, END exclusive). Example: 0:200
    #[arg(long)]
    cols: Option<String>,

    /// Cell width in px
    #[arg(long, default_value_t = 11.0)]
    cell_width: f32,

    /// Cell height in px
    #[arg(long, default_value_t = 16.0)]
    cell_height: f32,

    /// Font size
    #[arg(long, default_value_t = 14)]
    residue_font_size: u32,

    /// Ascent correction
    #[arg(long, default_value_t = 12.0)]
    ascent_corr: f32,

    /// Character width
    #[arg(long, default_value_t = 8.0)]
    char_width: f32,

    /// Margin x in px
    #[arg(long, default_value_t = 10.0)]
    margin_x: f32,

    /// Margin y in px
    #[arg(long, default_value_t = 10.0)]
    margin_y: f32,

    /// Show cell frames
    #[arg(long)]
    cell_frames: bool
}

fn parse_range(s: &str) -> Result<std::ops::Range<usize>> {
    let (a, b) = s
        .split_once(':')
        .with_context(|| format!("invalid range '{s}', expected START:END"))?;
    let start: usize = a.parse().with_context(|| format!("invalid START in '{s}'"))?;
    let end: usize = b.parse().with_context(|| format!("invalid END in '{s}'"))?;
    anyhow::ensure!(start <= end, "range START must be <= END in '{s}'");
    Ok(start..end)
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Read alignment 
    let aln_file: file::SeqFile = fasta::read_fasta_file(&args.input)?;
    let aln: Alignment = Alignment::from_file(aln_file);

    let colormap_name: ColorMapName = args.colormap_name.into();
    let colormap = ResidueColorMap::by_name(colormap_name);

    // Region (defaults: all)
    let _row_range = match &args.rows {
        Some(r) => parse_range(r)?,
        None => 0..aln.num_seq(), 
    };
    let _col_range = match &args.cols {
        Some(r) => parse_range(r)?,
        None => 0..aln.aln_len(), 
    };

    // let region = Region { rows: row_range, cols: col_range }; // adjust type/fields

    // Export options
    let opts = ExportOpts {
        cell_width: args.cell_width,
        cell_height: args.cell_height,
        residue_font_size: args.residue_font_size,
        colormap: colormap,
        ascent_corr: args.ascent_corr,
        margin_x: args.margin_x,
        margin_y: args.margin_y,
        cell_frames: args.cell_frames,
        ..Default::default()
    };


    // Layout
    let layout = compute_layout(&aln, &opts);

    if args.output == "-" {
        let mut out = io::BufWriter::new(io::stdout().lock());
        export_svg(&aln, &opts, &layout, &mut out)?;
        out.flush()?;
    } else {
        let mut out = fs::File::create(&args.output)?;
        export_svg(&aln, &opts, &layout, &mut out)?;
    }

    Ok(())
}

