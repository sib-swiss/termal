use std::collections::HashMap;

use ratatui::prelude::Color;

use crate::ui::color_map_lesk;

pub struct ColorScheme {
    pub residue_color_map: HashMap<char, Color>, 

    pub zoombox_color: Color,
}


pub fn color_scheme_default() -> ColorScheme {
    ColorScheme{
        residue_color_map: color_map_lesk(),
        zoombox_color: Color::Cyan,
    }
}
