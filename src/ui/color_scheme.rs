use std::collections::HashMap;

use ratatui::prelude::Color;

use crate::ui::color_map::ColorMap;

// In-house colors
pub const ORANGE: Color = Color::Rgb(255, 165, 0);
pub const SALMON: Color = Color::Rgb(250, 128, 114);

// ClustalX colors (source:
// https://www.cgl.ucsf.edu/chimera/1.2065/docs/ContributedSoftware/multalignviewer/colprot.par)
pub const CLUSTALX_RED: Color = Color::Rgb(229, 51, 25);
pub const CLUSTALX_BLUE: Color = Color::Rgb(25, 127, 229);
pub const CLUSTALX_GREEN: Color = Color::Rgb(25, 204, 25);
pub const CLUSTALX_CYAN: Color = Color::Rgb(25, 178, 178);
pub const CLUSTALX_PINK: Color = Color::Rgb(229, 127, 127);
pub const CLUSTALX_MAGENTA: Color = Color::Rgb(204, 76, 204);
pub const CLUSTALX_YELLOW: Color = Color::Rgb(204, 204, 0);
pub const CLUSTALX_ORANGE: Color = Color::Rgb(229, 153, 76);

pub struct ColorScheme {
    pub label_num_color: Color,
    pub seq_metric_color: Color,

    // Index into Vec of &Colormaps
    pub colormap_index: usize,

    pub zoombox_color: Color,

    pub position_color: Color,
    pub conservation_color: Color,
    pub consensus_default_color: Color,
}

pub fn color_scheme_monochrome() -> ColorScheme {
    ColorScheme {
        label_num_color: Color::White,
        colormap_index: 0,
        zoombox_color: Color::White,
        seq_metric_color: Color::White,
        position_color: Color::White,
        conservation_color: Color::White,
        consensus_default_color: Color::White,
    }
}

pub fn color_scheme_colored() -> ColorScheme {
    ColorScheme {
        label_num_color: Color::LightGreen,
        colormap_index: 1,
        seq_metric_color: Color::LightBlue,
        zoombox_color: Color::Cyan,
        position_color: Color::White,
        conservation_color: SALMON,
        consensus_default_color: Color::White,
    }
}
