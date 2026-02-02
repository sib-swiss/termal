use std::{
    collections::HashMap,
    io::{Result, Write}
};

use itertools::Itertools;

use termal_alignment::Alignment;

use crate::ExportOpts;


fn jalview_colormap() -> HashMap<char, String> {
    HashMap::from([
        ('A', "#64F73F".to_string()),
        ('C', "#FFB340".to_string()),
        ('G', "#EB413C".to_string()),
        ('T', "#3C88EE".to_string()),
        ('a', "#64F73F".to_string()),
        ('c', "#FFB340".to_string()),
        ('g', "#EB413C".to_string()),
        ('t', "#3C88EE".to_string()),
    ])
}

pub fn export_svg<W: Write>(aln: &Alignment, opts: &ExportOpts, out: &mut W) -> Result<()> {
    let grid_width = aln.aln_len() as f32 * opts.cell_width;
    let grid_height = aln.num_seq() as f32 * opts.cell_height;
    out.write_all(svg_open(grid_width, grid_height).as_bytes())?;
    write_aln(aln, opts, out)?;
    out.write_all(svg_close().as_bytes())?;
    Ok(())
}

fn svg_open(width: f32, height: f32) -> String {
    format!("<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}'>", width, height)
}

fn svg_sequence(seq: &str, opts: &ExportOpts) -> String {
    let colormap = jalview_colormap(); // TODO: pass a ref
    let def_color: String = String::from("none");
    let frame_color = if opts.cell_frames { String::from("black") } else { String::from("none") };
    let backgrounds = seq.chars().enumerate().map(|(i, c)| {
        let color_string = colormap.get(&c).unwrap_or(&def_color);
        format!("<rect x='{}' y='0' width='{}' height='{}' fill='{}' stroke='{}'/>",
            i as f32 * opts.cell_width,
            opts.cell_width,
            opts.cell_height,
            color_string,
            frame_color,
        )
    }).join("");
    let hdr = format!(
        "<text font-family='ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"DejaVu Sans Mono\", \"Courier New\", monospace' font-size='{}'>", opts.residue_font_size);
    let residues = seq.chars().enumerate().map(|(i, c)| 
        format!("<tspan x='{}' y='{}'>{}</tspan>", i as f32 * opts.cell_width, opts.ascent_corr, c)
        ).join("");
    format!("{}{}{}</text>", backgrounds, hdr, residues)
}

fn write_aln<W: Write>(aln: &Alignment, opts: &ExportOpts, out: &mut W) -> Result<()> {
    for (i, seq) in aln.sequences.iter().enumerate() {
        writeln!(out, "<g transform='translate(0,{})'>{}</g>",
            i as f32 * opts.cell_height,
            svg_sequence(seq, opts)
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
        assert_eq!(svg_open(200.0, 80.0), r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="80">"#);
    }

    #[test]
    fn test_svg_close() { assert_eq!(svg_close(), "</svg>"); }
}
