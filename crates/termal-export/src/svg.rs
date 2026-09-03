// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use std::{
    io::{Result, Write},
    iter::zip,
};

use itertools::Itertools;

use termal_alignment::Alignment;

use crate::{ExportOpts, Layout};

const RESIDUE_FONT_FAMILY: &str = "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \"Liberation Mono\", \"DejaVu Sans Mono\", \"Courier New\", monospace";

pub fn export_svg<W: Write>(
    aln: &Alignment,
    opts: &ExportOpts,
    layout: &Layout,
    out: &mut W,
) -> Result<()> {
    out.write_all(svg_open(layout.grid_width, layout.grid_height).as_bytes())?;
    write_aln(aln, opts, layout, out)?;
    out.write_all(svg_close().as_bytes())?;
    Ok(())
}

fn svg_open(width: f32, height: f32) -> String {
    format!(
        "<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' viewBox='0 0 {} {}'>",
        width, height, width, height
    )
}

fn svg_header(hdr: &str, opts: &ExportOpts) -> String {
    format!(
        "<text x='0' y='{}' fill='black' font-family='{}' font-size='{}'>{}</text>",
        opts.ascent_corr, RESIDUE_FONT_FAMILY, opts.residue_font_size, hdr,
    )
}

fn svg_sequence(seq: &str, opts: &ExportOpts) -> String {
    // let def_color: String = String::from("none");
    let frame_color = if opts.cell_frames {
        String::from("black")
    } else {
        String::from("none")
    };
    let backgrounds = seq
        .chars()
        .enumerate()
        .skip(opts.region.cols.start)
        .take(opts.region.cols.end - opts.region.cols.start)
        .map(|(i, c)| {
            let color_string = opts.colormap.rgb(c as u8).to_hex();
            format!(
                "<rect x='{}' y='0' width='{}' height='{}' fill='{}' stroke='{}'/>",
                i as f32 * opts.cell_width,
                opts.cell_width,
                opts.cell_height,
                color_string,
                frame_color,
            )
        })
        .join("");
    let hdr = format!(
        "<text font-family='{}' font-size='{}'>",
        RESIDUE_FONT_FAMILY, opts.residue_font_size
    );
    let residues = seq
        .chars()
        .enumerate()
        .map(|(i, c)| {
            format!(
                "<tspan x='{}' y='{}'>{}</tspan>",
                i as f32 * opts.cell_width,
                opts.ascent_corr,
                c
            )
        })
        .join("");
    format!("{}{}{}</text>", backgrounds, hdr, residues)
}

fn write_aln<W: Write>(
    aln: &Alignment,
    opts: &ExportOpts,
    layout: &Layout,
    out: &mut W,
) -> Result<()> {
    let zipped_aln = zip(aln.headers.iter(), aln.sequences.iter());
    for (i, (hdr, seq)) in zipped_aln
        .enumerate()
        .skip(opts.region.rows.start)
        .take(opts.region.rows.end - opts.region.rows.start) {
            writeln!(
                out,
                "<g transform='translate(0,{})'>{}<g transform='translate({},0)'>{}</g></g>",
                i as f32 * opts.cell_height,
                svg_header(hdr, opts),
                layout.hdr_txt_width,
                svg_sequence(seq, opts),
            )?;
    };
    Ok(())
}

fn svg_close() -> String {
    "</svg>".to_string()
}

#[cfg(test)]
mod tests {
    use termal_alignment::{rgb::ResidueColorMap, Alignment};

    use super::*;

    fn test_alignment() -> Alignment {
        Alignment::from_vecs(
            vec!["seq1".to_string(), "longer".to_string()],
            vec!["ACG".to_string(), "TTA".to_string()],
        )
    }

    fn export_to_string(opts: &ExportOpts) -> String {
        let aln = test_alignment();
        let layout = crate::compute_layout(&aln, opts);
        let mut out = Vec::new();
        export_svg(&aln, opts, &layout, &mut out).unwrap();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn test_svg_open() {
        assert_eq!(
            svg_open(200.0, 80.0),
            r"<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='200' height='80' viewBox='0 0 200 80'>"
        );
    }

    #[test]
    fn test_svg_close() {
        assert_eq!(svg_close(), "</svg>");
    }

    #[test]
    fn export_svg_includes_svg_root_and_closing_tag() {
        let svg = export_to_string(&ExportOpts::default());

        assert!(svg.starts_with("<?xml version='1.0' encoding='UTF-8'?>\n<svg "));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn export_svg_writes_headers_and_residues() {
        let svg = export_to_string(&ExportOpts::default());

        assert!(svg.contains(">seq1</text>"));
        assert!(svg.contains(">longer</text>"));
        assert!(svg.contains(">A</tspan>"));
        assert!(svg.contains(">C</tspan>"));
        assert!(svg.contains(">G</tspan>"));
        assert!(svg.contains(">T</tspan>"));
    }

    #[test]
    fn export_svg_writes_one_rect_per_residue() {
        let svg = export_to_string(&ExportOpts::default());

        assert_eq!(svg.matches("<rect ").count(), 6);
    }

    #[test]
    fn export_svg_cell_frames_toggle_stroke() {
        let no_frames = export_to_string(&ExportOpts::default());

        let with_frames = export_to_string(&ExportOpts {
            cell_frames: true,
            colormap: ResidueColorMap::aa_lesk(),
            ..ExportOpts::default()
        });

        assert!(no_frames.contains("stroke='none'"));
        assert!(with_frames.contains("stroke='black'"));
    }
    #[test]
    fn export_svg_uses_header_offset_from_layout() {
        let aln = test_alignment();
        let opts = ExportOpts::default();
        let layout = crate::compute_layout(&aln, &opts);
        let svg = export_to_string(&opts);

        assert!(svg.contains(&format!(
            "<g transform='translate({},0)'>",
            layout.hdr_txt_width
        )));
    }

    #[test]
    fn export_svg_emits_one_group_per_sequence() {
        let aln = test_alignment();
        let svg = export_to_string(&ExportOpts::default());

        assert_eq!(
            svg.matches("<g transform='translate(0,").count(),
            aln.num_seq()
        );
    }

    #[test]
    fn export_svg_positions_rows_by_cell_height() {
        let opts = ExportOpts::default();
        let svg = export_to_string(&opts);

        assert!(svg.contains("<g transform='translate(0,0)'>"));
        assert!(svg.contains(&format!(
            "<g transform='translate(0,{})'>",
            opts.cell_height
        )));
    }

    #[test]
    fn export_svg_uses_residue_x_positions_from_cell_width() {
        let opts = ExportOpts::default();
        let svg = export_to_string(&opts);

        assert!(svg.contains(&format!("<tspan x='0' y='{}'>A</tspan>", opts.ascent_corr)));
        assert!(svg.contains(&format!(
            "<tspan x='{}' y='{}'>C</tspan>",
            opts.cell_width, opts.ascent_corr
        )));
        assert!(svg.contains(&format!(
            "<tspan x='{}' y='{}'>G</tspan>",
            2.0 * opts.cell_width,
            opts.ascent_corr
        )));
    }

    #[test]
    fn export_svg_uses_residue_font_size_and_ascent() {
        let opts = ExportOpts {
            residue_font_size: 17,
            ascent_corr: 9.5,
            ..ExportOpts::default()
        };
        let svg = export_to_string(&opts);

        assert!(svg.contains("font-size='17'"));
        assert!(svg.contains("<text x='0' y='9.5'"));
        assert!(svg.contains("<tspan x='0' y='9.5'>A</tspan>"));
    }
}
