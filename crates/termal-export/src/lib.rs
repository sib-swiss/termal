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
    pub margin_x: f32,
    pub margin_y: f32,
    pub cell_frames: bool,
}

// NOTE: this is mainly intended for tests, in which Clap may not be available.

impl Default for ExportOpts {
    fn default() -> Self {
        Self {
            cell_width: 11.0,
            cell_height: 12.0,
            residue_font_size: 14,
            ascent_corr: 12.0,
            margin_x: 10.0,
            margin_y: 10.0,
            cell_frames: false,
        }
    }
}
