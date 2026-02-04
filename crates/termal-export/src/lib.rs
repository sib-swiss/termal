mod svg;

//use termal_alignment::{Alignment, Region};
// NOTE: Regions will be implemented later
use termal_alignment::Alignment;

pub use svg::export_svg;

#[derive(Clone, Debug)]
pub struct ExportOpts {
    pub cell_width: f32,
    pub cell_height: f32,
    pub residue_font_size: u32,
    pub ascent_corr: f32,
    pub char_width: f32,
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
    let hdr_txt_width = (max_hdr_len + 1) as f32 * opts.char_width;
    
    Layout {
        grid_width: hdr_txt_width + aln.aln_len() as f32 * opts.cell_width,
        grid_height: aln.num_seq() as f32 * opts.cell_height,
        hdr_txt_width: hdr_txt_width,
    }
}

// NOTE: this is mainly intended for tests, in which Clap may not be available.

impl Default for ExportOpts {
    fn default() -> Self {
        Self {
            cell_width: 11.0,
            cell_height: 12.0,
            residue_font_size: 14,
            ascent_corr: 12.0,
            char_width: 8.0,
            margin_x: 10.0,
            margin_y: 10.0,
            cell_frames: false,
        }
    }
}
