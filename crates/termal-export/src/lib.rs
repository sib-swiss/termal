mod svg;

//use termal_alignment::{Alignment, Region};
// NOTE: Regions will be implemented later
use termal_alignment::{
    Alignment,
    rgb::ResidueColorMap,
};

pub use svg::export_svg;

const GUTTER_WIDTH: f32 = 20.0; // h space between headers and sequences

#[derive(Clone, Debug)]
pub struct ExportOpts {
    pub cell_width: f32,
    pub cell_height: f32,
    pub residue_font_size: u32,
    pub ascent_corr: f32,
    pub char_width: f32,
    pub colormap: ResidueColorMap,
    pub margin_x: f32,
    pub margin_y: f32,
    pub cell_frames: bool,
}

#[derive(Clone, Debug)]
pub struct Layout {
    pub grid_width: f32,
    pub grid_height: f32,
    pub hdr_txt_width: f32,
}

pub fn compute_layout(aln: &Alignment, opts: &ExportOpts) -> Layout {
    let max_hdr_len = aln.headers.iter().map(|h| h.len()).max().unwrap_or(0);
    // + 1: add 1 char's width of space between headers and sequences.
    let hdr_txt_width = max_hdr_len as f32 * opts.char_width + GUTTER_WIDTH;
    
    Layout {
        grid_width: hdr_txt_width + aln.aln_len() as f32 * opts.cell_width,
        grid_height: aln.num_seq() as f32 * opts.cell_height,
        hdr_txt_width: hdr_txt_width,
    }
}

// NOTE: this is mainly intended for tests, in which Clap may not be available.

impl Default for ExportOpts {
    fn default() -> Self {
        let colormap: ResidueColorMap = ResidueColorMap::aa_lesk();
        Self {
            cell_width: 11.0,
            cell_height: 12.0,
            residue_font_size: 14,
            ascent_corr: 12.0,
            char_width: 8.0,
            colormap: colormap,
            margin_x: 10.0,
            margin_y: 10.0,
            cell_frames: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{compute_layout, ExportOpts};
    use termal_alignment::Alignment;

    #[test]
    fn compute_layout_uses_longest_header_and_alignment_size() {
        let aln = Alignment::from_vecs(
            vec!["a".to_string(), "long_hdr".to_string()],
            vec!["ACG".to_string(), "TTT".to_string()],
        );
        let opts = ExportOpts::default();

        let layout = compute_layout(&aln, &opts);

        assert_eq!(layout.hdr_txt_width, 72.0);
        assert_eq!(layout.grid_width, 105.0);
        assert_eq!(layout.grid_height, 24.0);
    }
}
