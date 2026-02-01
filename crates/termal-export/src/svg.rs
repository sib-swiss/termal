use std::io::{Result, Write};

use termal_alignment::Alignment;

use crate::ExportOpts;

pub fn export_svg<W: Write>(aln: &Alignment, opts: &ExportOpts, out: &mut W) -> Result<()> {
    //let _ = (aln, region, opts);
    let _ = (aln, opts);
    out.write_all(svg_open(200.0, 80.0).as_bytes())?;
    write_aln(aln, opts, out)?;
    out.write_all(svg_close().as_bytes())?;
    Ok(())
}

fn svg_open(width: f32, height: f32) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, width, height)
}

fn svg_aln_cell(c: char, x: f32, y: f32) -> String {
    format!(r#"<text x="{}" y="{}">{}</text>"#, x, y, c)
}

fn svg_sequence(seq: &str, x: f32, y: f32) -> String {
    format!(r#"<text font-family="mono" x="{}" y="{}">{}</text>"#, x, y, seq)
}

fn write_aln<W: Write>(aln: &Alignment, opts: &ExportOpts, out: &mut W) -> Result<()> {
    writeln!(out, r#"<text x="0", y="0" font-family="mono">"#)?;
    writeln!(out, "</text>")?;
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
