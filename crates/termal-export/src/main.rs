use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use termal_alignment::Alignment;

use termal_export::{export_svg, ExportOpts}; // this crate's lib.rs, also used by termal-msa (the
                                             // TUI app)

#[derive(Parser, Debug)]
#[command(name = "termal-export")]
#[command(about = "Export a region of a multiple sequence alignment as graphics", long_about = None)]
struct Args {
    /// Input alignment file
    input: PathBuf,

    /// Output SVG file ("-" for stdout)
    #[arg(short, long, default_value = "-")]
    output: String,

    /// Row range as START:END (0-based, END exclusive). Example: 0:50
    #[arg(long)]
    rows: Option<String>,

    /// Column range as START:END (0-based, END exclusive). Example: 0:200
    #[arg(long)]
    cols: Option<String>,

    /// Cell width in px
    #[arg(long, default_value_t = 12.0)]
    cell_w: f32,

    /// Cell height in px
    #[arg(long, default_value_t = 14.0)]
    cell_h: f32,

    /// Margin x in px
    #[arg(long, default_value_t = 10.0)]
    margin_x: f32,

    /// Margin y in px
    #[arg(long, default_value_t = 10.0)]
    margin_y: f32,
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

    // 1) Read alignment 
    let aln: Alignment = Alignment::from_file(&args.input)
        .with_context(|| format!("failed to read alignment from {}", args.input.display()))?;

    // 2) Region (defaults: all)
    let row_range = match &args.rows {
        Some(r) => parse_range(r)?,
        None => 0..aln.nseq(), // adjust method name
    };
    let col_range = match &args.cols {
        Some(r) => parse_range(r)?,
        None => 0..aln.ncol(), // adjust method name
    };

    // let region = Region { rows: row_range, cols: col_range }; // adjust type/fields

    // 3) Export options
    let opts = ExportOpts {
        cell_w: args.cell_w,
        cell_h: args.cell_h,
        margin_x: args.margin_x,
        margin_y: args.margin_y,
        ..Default::default()
    };

    // 4) Export SVG
    // let svg = export_svg(&aln, &region, &opts)?;
    let svg = export_svg(&aln, &opts)?;

    // 5) Write
    if args.output == "-" {
        let mut out = io::BufWriter::new(io::stdout().lock());
        out.write_all(svg.as_bytes())?;
        out.flush()?;
    } else {
        fs::write(&args.output, svg).with_context(|| format!("failed to write {}", args.output))?;
    }

    Ok(())
}

