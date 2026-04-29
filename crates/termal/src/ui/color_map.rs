// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier
use std::{fmt, fs::File, io::BufReader};

use hex_color::HexColor;

use ratatui::prelude::Color;

use serde_json::Value::Object;

use crate::errors::TermalError;

use termal_alignment::rgb::{ResidueColorMap, Rgb, GAP_COLOR, RGB_WHITE};

// allows terminal-based (instead of fixed) grey
const GAP_COLOR_TUI: Color = Color::Gray;

#[derive(Clone)]
pub struct ColorMap {
    pub name: String,
    map: ResidueColorMap,
}

impl ColorMap {
    pub fn get(&self, residue: char) -> Color {
        if residue == '-' || residue == '.' {
            return GAP_COLOR_TUI;
        }
        let rgb = self.map.rgb(residue as u8);
        Color::Rgb(rgb.r, rgb.g, rgb.b)
    }

    pub fn map(&self) -> &ResidueColorMap {
        &self.map
    }
}

impl fmt::Display for ColorMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn builtin_polychrome_colormaps() -> Vec<ColorMap> {
    vec![
        color_map_jalview_nt(), // Keep the nucleotide map at index 0 (see
        color_map_clustalx(),
        color_map_lesk(),
    ]
}

pub fn monochrome_colormap() -> Vec<ColorMap> {
    vec![color_map_monochrome()]
}

fn color_map_monochrome() -> ColorMap {
    ColorMap {
        name: "Mono".to_string(),
        map: ResidueColorMap::monochrome(),
    }
}

pub fn color_map_lesk() -> ColorMap {
    ColorMap {
        name: "Lesk".to_string(),
        map: ResidueColorMap::aa_lesk(),
    }
}

pub fn color_map_clustalx() -> ColorMap {
    ColorMap {
        name: "ClustalX".to_string(),
        map: ResidueColorMap::aa_clustalx(),
    }
}

pub fn color_map_jalview_nt() -> ColorMap {
    ColorMap {
        name: "JalView-nt".to_string(),
        map: ResidueColorMap::dna_jalview(),
    }
}

pub fn colormap_gecos(path: &str) -> Result<ColorMap, TermalError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let cm: serde_json::Value = serde_json::from_reader(reader).unwrap();

    //println!("{:#?}", cm["colors"]);
    let mut color_map = ResidueColorMap::with_default(GAP_COLOR, RGB_WHITE);
    let orig_map = &cm["colors"];
    if let Object(map) = orig_map {
        //println!("Found map: {:#?}", map);
        for (k, v) in map {
            let color_str = serde_json::from_value::<String>(v.clone()).unwrap();
            let hex_color = HexColor::parse_rgb(&color_str).unwrap();
            let color = Rgb {
                r: hex_color.r,
                g: hex_color.g,
                b: hex_color.b,
            };
            let residue = k.chars().collect::<Vec<char>>()[0] as u8;
            color_map.set_pair(residue, color);
            //println!("{} -> {}", k.chars().collect::<Vec<char>>()[0], color);
        }
    }

    Ok(ColorMap {
        name: "custom".to_string(),
        map: color_map,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use ratatui::prelude::Color;

    use super::colormap_gecos;
    use termal_alignment::rgb::{Rgb, GAP_COLOR, RGB_WHITE};

    #[test]
    fn gecos_colormap_loads_custom_entries_and_preserves_defaults() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/colormaps/test.json");
        let cmap = colormap_gecos(path.to_str().expect("utf-8 path")).expect("custom colormap");

        assert_eq!(cmap.name, "custom");
        assert_eq!(cmap.map().rgb(b'A'), Rgb::from_hex("#7FFFD4").unwrap());
        assert_eq!(cmap.map().rgb(b'a'), Rgb::from_hex("#7FFFD4").unwrap());
        assert_eq!(cmap.map().rgb(b'Y'), Rgb::from_hex("#d1fee1").unwrap());
        assert_eq!(cmap.map().rgb(b'Z'), RGB_WHITE);
        assert_eq!(cmap.map().rgb(b'-'), GAP_COLOR);
        assert_eq!(cmap.get('-'), Color::Gray);
        assert_eq!(cmap.get('A'), Color::Rgb(127, 255, 212));
    }
}
