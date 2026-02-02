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
    ])
}

pub fn export_svg<W: Write>(aln: &Alignment, opts: &ExportOpts, out: &mut W) -> Result<()> {
    //let _ = (aln, region, opts);
    let _ = (aln, opts);
    out.write_all(svg_open(200.0, 80.0).as_bytes())?;
    write_aln(aln, opts, out)?;
    out.write_all(svg_close().as_bytes())?;
    Ok(())
}

fn svg_open(width: f32, height: f32) -> String {
    format!("<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}'>", width, height)
}

fn svg_aln_cell(c: char, x: f32, y: f32) -> String {
    format!("<text x='{}' y='{}'>{}</text>", x, y, c)
}

fn svg_sequence(seq: &str, x: f32, y: f32, opts: &ExportOpts) -> String {
    let colormap = jalview_colormap(); // TODO: pass a ref
    let def_color: String = String::from("none");
    let frame_color = if opts.cell_frames { String::from("black") } else { String::from("none") };
    let backgrounds = seq.chars().enumerate().map(|(i, c)| {
        let color_string = colormap.get(&c).unwrap_or(&def_color);
        format!("<rect x='{}' y='{}' width='{}' height='{}' fill='{}' stroke='{}'/>",
            x + (i as f32 * opts.cell_width),
            y,
            opts.cell_width,
            opts.cell_height,
            color_string,
            frame_color,
        )
    }).join("");
    let hdr = "<text font-family='mono'>";
    let residues = seq.chars().enumerate().map(|(i, c)| 
        format!("<tspan x='{}' y='{}'>{}</tspan>", x + (i as f32 * opts.cell_width), y + opts.ascent_corr, c)
        ).join("");
    format!("{}{}{}</text>", backgrounds, hdr, residues)
}

fn write_aln<W: Write>(aln: &Alignment, opts: &ExportOpts, out: &mut W) -> Result<()> {
    //writeln!(out, r#"<text x="0" y="0" font-family="mono">"#)?; // TODO: make a group, not a text

    writeln!(out, "{}", svg_sequence("GAATTC", 10.0, 20.0, opts))?;
    //writeln!(out, "</text>")?;
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

    fn test_cell() {
        assert_eq!(svg_aln_cell('A', 5.2, 6.4), "<text x=\"5.2\" y=\"6.4\">A</text>");
    }
}
