// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier
use std::{collections::HashMap, fmt, fs::File, io::BufReader};

use hex_color::HexColor;
// NOTE: ideally, the color maps should not depend on crates, since one might conceivably switch to
// a different library.
use ratatui::prelude::Color;

use serde_json::Value::Object;

use crate::errors::TermalError;
use crate::ui::color_scheme::{
    CLUSTALX_BLUE, CLUSTALX_CYAN, CLUSTALX_GREEN, CLUSTALX_MAGENTA, CLUSTALX_ORANGE, CLUSTALX_PINK,
    CLUSTALX_RED, CLUSTALX_YELLOW, JALVIEW_NUCLEOTIDE_A, JALVIEW_NUCLEOTIDE_B,
    JALVIEW_NUCLEOTIDE_C, JALVIEW_NUCLEOTIDE_D, JALVIEW_NUCLEOTIDE_G, JALVIEW_NUCLEOTIDE_H,
    JALVIEW_NUCLEOTIDE_I, JALVIEW_NUCLEOTIDE_K, JALVIEW_NUCLEOTIDE_M, JALVIEW_NUCLEOTIDE_N,
    JALVIEW_NUCLEOTIDE_R, JALVIEW_NUCLEOTIDE_S, JALVIEW_NUCLEOTIDE_T, JALVIEW_NUCLEOTIDE_U,
    JALVIEW_NUCLEOTIDE_V, JALVIEW_NUCLEOTIDE_W, JALVIEW_NUCLEOTIDE_X, JALVIEW_NUCLEOTIDE_Y, ORANGE,
};

use termal_alignment::rgb::{
    GAP_COLOR,
    RGB_WHITE,
    ResidueColorMap,
    Rgb,
};

#[derive(Clone)]
pub struct ColorMap {
    pub name: String,
    map: ResidueColorMap,
}

impl ColorMap {

    pub fn get(&self, residue: char) -> Color {
        let rgb = self.map.rgb(residue as u8);
        Color::Rgb(rgb.r, rgb.g, rgb.b)
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
            let color = Rgb { r: hex_color.r, g: hex_color.g, b: hex_color.b };
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
