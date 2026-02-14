use std::{
    collections::HashMap,
    io::{Result, Write},
    iter::zip,
};

use itertools::Itertools;

use termal_alignment::{
    Alignment,
    rgb::{
        Rgb,
        ResidueColorMap,
    },
};

use crate::{ExportOpts, Layout};

const RESIDUE_FONT_FAMILY: &str = "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"DejaVu Sans Mono\", \"Courier New\", monospace";

pub fn export_svg<W: Write>(aln: &Alignment, opts: &ExportOpts,
        layout: &Layout, out: &mut W) -> Result<()> {
    out.write_all(svg_open(layout.grid_width, layout.grid_height).as_bytes())?;
    write_aln(aln, opts, layout, out)?;
    out.write_all(svg_close().as_bytes())?;
    Ok(())
}

fn svg_open(width: f32, height: f32) -> String {
    format!("<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' viewBox='0 0 {} {}'>",
        width, height, width, height)
}

fn svg_header(hdr: &str, opts: &ExportOpts) -> String {
    format!("<text x='0' y='{}' fill='black' font-family='{}' font-size='{}'>{}</text>",
        opts.ascent_corr,
        RESIDUE_FONT_FAMILY,
        opts.residue_font_size,
        hdr,
    )
}

fn svg_sequence(seq: &str, opts: &ExportOpts) -> String {
    let def_color: String = String::from("none");
    let frame_color = if opts.cell_frames { String::from("black") } else { String::from("none") };
    let backgrounds = seq.chars().enumerate().map(|(i, c)| {
        let color_string = opts.colormap.rgb(c as u8).to_hex();
        format!("<rect x='{}' y='0' width='{}' height='{}' fill='{}' stroke='{}'/>",
            i as f32 * opts.cell_width,
            opts.cell_width,
            opts.cell_height,
            color_string,
            frame_color,
        )
    }).join("");
    let hdr = format!(
        "<text font-family='{}' font-size='{}'>",
        RESIDUE_FONT_FAMILY,
        opts.residue_font_size);
    let residues = seq.chars().enumerate().map(|(i, c)| 
        format!("<tspan x='{}' y='{}'>{}</tspan>",
            i as f32 * opts.cell_width,
            opts.ascent_corr,
            c)
        ).join("");
    format!("{}{}{}</text>", backgrounds, hdr, residues)
}


fn write_aln<W: Write>(aln: &Alignment, opts: &ExportOpts,
        layout: &Layout, out: &mut W) -> Result<()> {
    let zipped_aln = zip( aln.headers.iter(), aln.sequences.iter());
    for (i, (hdr, seq)) in zipped_aln.enumerate() {
        writeln!(out, "<g transform='translate(0,{})'>{}<g transform='translate({},0)'>{}</g></g>",
            i as f32 * opts.cell_height,
            svg_header(hdr, opts),
            layout.hdr_txt_width,
            svg_sequence(seq, opts),
        )?;
    }
    Ok(())
}

fn svg_close() -> String {
    "</svg>".to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_svg_open() {
        assert_eq!(svg_open(200.0, 80.0), r"<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='200' height='80' viewBox='0 0 200 80'>");
    }

    #[test]
    fn test_svg_close() { assert_eq!(svg_close(), "</svg>"); }
}
