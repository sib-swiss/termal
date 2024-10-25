use std::collections::HashMap;

use ratatui::prelude::Color;

use crate::ui::color_map_lesk;

pub const ORANGE: Color = Color::Rgb(255, 165, 0);
pub const SALMON: Color = Color::Rgb(250, 128, 114);
//pub const DARK_GREEN: Color = Color::Rgb(

pub struct ColorScheme {
    pub label_num_color: Color,

    pub residue_color_map: HashMap<char, Color>, 

    pub zoombox_color: Color,

    pub position_color: Color,
    pub conservation_color: Color,
    pub consensus_default_color: Color,
}


pub fn color_scheme_default() -> ColorScheme {
    ColorScheme{
        label_num_color: Color::LightGreen,
        residue_color_map: color_map_lesk(),
        zoombox_color: Color::Cyan,
        position_color: Color::White,
        conservation_color: SALMON,
        consensus_default_color: Color::White,
    }
}
