mod svg;

//use termal_alignment::{Alignment, Region};
// NOTE: Regions will be implemented later
use termal_alignment::Alignment;

pub use svg::export_svg;

#[derive(Clone, Debug)]
pub struct ExportOpts {
    pub cell_w: f32,
    pub cell_h: f32,
    pub margin_x: f32,
    pub margin_y: f32,
}

impl Default for ExportOpts {
    fn default() -> Self {
        Self {
            cell_w: 12.0,
            cell_h: 14.0,
            margin_x: 10.0,
            margin_y: 10.0,
        }
    }
}
