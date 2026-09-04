// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod svg;

use std::ops::Range;

use termal_alignment::{rgb::ResidueColorMap, Alignment};

pub use svg::export_svg;

#[derive(Clone, Debug)]
pub struct Region {
    pub rows: Range<usize>,
    pub cols: Range<usize>,
}

const GUTTER_WIDTH: f32 = 20.0; // h space between headers and sequences

#[derive(Clone, Debug)]
pub struct ExportOpts {
    pub region: Region,
    pub cell_width: f32,
    pub cell_height: f32,
    pub residue_font_size: u32,
    pub ascent_corr: f32,
    pub char_width: f32,
    pub colormap: ResidueColorMap,
    pub margin_x: f32,
    pub margin_y: f32,
    pub cell_frames: bool,
    pub hdr_pane_width_corr: f32,
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub grid_width: f32,
    pub grid_height: f32,
    pub hdr_txt_width: f32,
}

pub fn compute_layout(aln: &Alignment, opts: &ExportOpts) -> Layout {
    let max_hdr_len = aln.headers
            .iter()
            .skip(opts.region.rows.start)
            .take(opts.region.rows.end - opts.region.rows.start)
            .map(|h| h.len()).max().unwrap_or(0);
    let hdr_txt_width = max_hdr_len as f32 * opts.char_width * opts.hdr_pane_width_corr + GUTTER_WIDTH;

    Layout {
        grid_width: hdr_txt_width + (opts.region.cols.end - opts.region.cols.start) as f32 * opts.cell_width,
        grid_height: (opts.region.rows.end - opts.region.rows.start) as f32 * opts.cell_height,
        hdr_txt_width,
    }
}

// NOTE: this is mainly intended for tests, in which Clap may not be available.

impl Default for ExportOpts {
    fn default() -> Self {
        let colormap: ResidueColorMap = ResidueColorMap::aa_lesk();
        Self {
            region: Region { rows: (0..10), cols: (0..20) },
            cell_width: 11.0,
            cell_height: 12.0,
            residue_font_size: 14,
            ascent_corr: 12.0,
            char_width: 8.0,
            colormap,
            margin_x: 10.0,
            margin_y: 10.0,
            cell_frames: false,
            hdr_pane_width_corr: 1.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{compute_layout, export_svg, ExportOpts};
    use termal_alignment::Alignment;
    use crate::Region;

    #[test]
    fn compute_layout_uses_longest_header_and_alignment_size() {
        let aln = Alignment::from_vecs(
            vec!["a".to_string(), "long_hdr".to_string()],
            vec!["ACG".to_string(), "TTA".to_string()],
        );

        let mut opts = ExportOpts::default();
        opts.region = Region {
            rows: 0..aln.num_seq(),
            cols: 0..aln.aln_len(),
        };

        let layout = compute_layout(&aln, &opts);
        assert_eq!(layout.hdr_txt_width, 84.0);
        assert_eq!(layout.grid_width, 117.0);
        assert_eq!(layout.grid_height, 24.0);
    }

    fn test_alignment() -> Alignment {
        Alignment::from_vecs(
            vec!["seq0".to_string(), "seq1".to_string(), "seq2".to_string(), "seq3".to_string()],
            vec!["ACGTACGTACGT".to_string(), "TGCATGCATGCA".to_string(),
                 "GGGGGGGGGGGG".to_string(), "TTTTTTTTTTTT".to_string()],
        )
    }

    fn count_svg_rows(svg: &str) -> usize {
        svg.matches("<g transform='translate(0,").count()
    }

    fn count_svg_cols(svg: &str) -> usize {
        svg.lines().find(|l| l.contains("<rect"))
            .map(|l| l.matches("<rect").count())
            .unwrap_or(0)
    }

    #[test]
    fn region_full_alignment_exports_all_rows_and_cols() {
        let aln = test_alignment();
        let mut opts = ExportOpts::default();
        opts.region = Region {
            rows: 0..aln.num_seq(),
            cols: 0..aln.aln_len(),
        };

        let layout = compute_layout(&aln, &opts);
        let mut out = Vec::new();
        export_svg(&aln, &opts, &layout, &mut out).unwrap();
        let svg = String::from_utf8(out).unwrap();

        assert_eq!(count_svg_rows(&svg), 4, "should export all 4 sequences");
        assert_eq!(count_svg_cols(&svg), 12, "should export all 12 columns");
    }

    #[test]
    fn region_row_subset_exports_only_specified_rows() {
        let aln = test_alignment();
        let mut opts = ExportOpts::default();
        opts.region = Region {
            rows: 1..3,  // seq1, seq2
            cols: 0..aln.aln_len(),
        };

        let layout = compute_layout(&aln, &opts);
        let mut out = Vec::new();
        export_svg(&aln, &opts, &layout, &mut out).unwrap();
        let svg = String::from_utf8(out).unwrap();

        assert_eq!(count_svg_rows(&svg), 2, "should export 2 rows (1:3)");
        assert_eq!(count_svg_cols(&svg), 12, "should export all 12 columns");
        assert!(svg.contains("seq1"), "should contain seq1 header");
        assert!(svg.contains("seq2"), "should contain seq2 header");
        assert!(!svg.contains("seq0"), "should not contain seq0");
        assert!(!svg.contains("seq3"), "should not contain seq3");
    }

    #[test]
    fn region_col_subset_exports_only_specified_cols() {
        let aln = test_alignment();
        let mut opts = ExportOpts::default();
        opts.region = Region {
            rows: 0..aln.num_seq(),
            cols: 3..9,  // 6 columns
        };

        let layout = compute_layout(&aln, &opts);
        let mut out = Vec::new();
        export_svg(&aln, &opts, &layout, &mut out).unwrap();
        let svg = String::from_utf8(out).unwrap();

        assert_eq!(count_svg_rows(&svg), 4, "should export all 4 sequences");
        assert_eq!(count_svg_cols(&svg), 6, "should export 6 columns (3:9)");
    }

    #[test]
    fn region_row_and_col_subset_exports_specified_rect() {
        let aln = test_alignment();
        let mut opts = ExportOpts::default();
        opts.region = Region {
            rows: 1..3,  // 2 rows
            cols: 2..6,  // 4 columns
        };

        let layout = compute_layout(&aln, &opts);
        let mut out = Vec::new();
        export_svg(&aln, &opts, &layout, &mut out).unwrap();
        let svg = String::from_utf8(out).unwrap();

        assert_eq!(count_svg_rows(&svg), 2, "should export 2 rows (1:3)");
        assert_eq!(count_svg_cols(&svg), 4, "should export 4 columns (2:6)");
        assert!(svg.contains("seq1"), "should contain seq1");
        assert!(svg.contains("seq2"), "should contain seq2");
    }
}
