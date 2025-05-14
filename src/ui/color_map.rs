// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier
use std::{collections::HashMap, fs::File, io::BufReader};

use hex_color::HexColor;
// NOTE: ideally, the color maps should not depend on crates, since one might conceivably switch to
// a different library.
use ratatui::prelude::Color;

use serde_json::Value::Object;

use crate::ui::color_scheme::{
    CLUSTALX_BLUE, CLUSTALX_CYAN, CLUSTALX_GREEN, CLUSTALX_MAGENTA, CLUSTALX_ORANGE, CLUSTALX_PINK,
    CLUSTALX_RED, CLUSTALX_YELLOW, JALVIEW_NUCLEOTIDE_A, JALVIEW_NUCLEOTIDE_B,
    JALVIEW_NUCLEOTIDE_C, JALVIEW_NUCLEOTIDE_D, JALVIEW_NUCLEOTIDE_G, JALVIEW_NUCLEOTIDE_H,
    JALVIEW_NUCLEOTIDE_I, JALVIEW_NUCLEOTIDE_K, JALVIEW_NUCLEOTIDE_M, JALVIEW_NUCLEOTIDE_N,
    JALVIEW_NUCLEOTIDE_R, JALVIEW_NUCLEOTIDE_S, JALVIEW_NUCLEOTIDE_T, JALVIEW_NUCLEOTIDE_U,
    JALVIEW_NUCLEOTIDE_V, JALVIEW_NUCLEOTIDE_W, JALVIEW_NUCLEOTIDE_X, JALVIEW_NUCLEOTIDE_Y, ORANGE,
};

pub struct ColorMap {
    #[allow(dead_code)]
    pub name: String,
    map: HashMap<char, Color>,
}

impl ColorMap {
    pub fn new(name: String, map: HashMap<char, Color>) -> ColorMap {
        ColorMap { name, map }
    }

    pub fn get(&self, residue: char) -> Color {
        if let Some(color) = self.map.get(&residue) {
            *color
        } else {
            Color::White
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, residue: char, color: Color) {
        self.map.insert(residue, color);
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
    vec![
        color_map_monochrome(),
    ]
}

// NOTE: if it turns out that these hash maps are not efficient (didn't benchmark yet), we might
// want to look at perfect hash functions - see e.g https://crates.io/crates/phf

// NOTE, although these maps do not vary, we cannot dclare them as constants, because they involve
// a function call (namely, to HashMap::from()).

// It's prolly easier to have a no-op colorscheme than to decide at every iteration if we do a
// lookup or not.

pub fn color_map_monochrome() -> ColorMap {
    ColorMap::new(
        "Monochrome".into(),
        HashMap::from([
            ('G', Color::White),
            ('A', Color::White),
            ('S', Color::White),
            ('T', Color::White),
            ('C', Color::White),
            ('V', Color::White),
            ('I', Color::White),
            ('L', Color::White),
            ('P', Color::White),
            ('F', Color::White),
            ('Y', Color::White),
            ('M', Color::White),
            ('W', Color::White),
            ('N', Color::White),
            ('Q', Color::White),
            ('H', Color::White),
            ('D', Color::White),
            ('E', Color::White),
            ('K', Color::White),
            ('R', Color::White),
            ('X', Color::White),
            ('g', Color::White),
            ('a', Color::White),
            ('s', Color::White),
            ('t', Color::White),
            ('c', Color::White),
            ('v', Color::White),
            ('i', Color::White),
            ('l', Color::White),
            ('p', Color::White),
            ('f', Color::White),
            ('y', Color::White),
            ('m', Color::White),
            ('w', Color::White),
            ('n', Color::White),
            ('q', Color::White),
            ('h', Color::White),
            ('d', Color::White),
            ('e', Color::White),
            ('k', Color::White),
            ('r', Color::White),
            ('x', Color::White),
            ('-', Color::White),
        ]),
    )
}

pub fn color_map_lesk() -> ColorMap {
    ColorMap::new(
        "Lesk".into(),
        HashMap::from([
            ('G', ORANGE),
            ('A', ORANGE),
            ('S', ORANGE),
            ('T', ORANGE),
            ('C', Color::Green),
            ('V', Color::Green),
            ('I', Color::Green),
            ('L', Color::Green),
            ('P', Color::Green),
            ('F', Color::Green),
            ('Y', Color::Green),
            ('M', Color::Green),
            ('W', Color::Green),
            ('N', Color::Magenta),
            ('Q', Color::Magenta),
            ('H', Color::Magenta),
            ('D', Color::Red),
            ('E', Color::Red),
            ('K', Color::Blue),
            ('R', Color::Blue),
            ('X', Color::White),
            ('g', ORANGE),
            ('a', ORANGE),
            ('s', ORANGE),
            ('t', ORANGE),
            ('c', Color::Green),
            ('v', Color::Green),
            ('i', Color::Green),
            ('l', Color::Green),
            ('p', Color::Green),
            ('f', Color::Green),
            ('y', Color::Green),
            ('m', Color::Green),
            ('w', Color::Green),
            ('n', Color::Magenta),
            ('q', Color::Magenta),
            ('h', Color::Magenta),
            ('d', Color::Red),
            ('e', Color::Red),
            ('k', Color::Blue),
            ('r', Color::Blue),
            ('x', Color::White),
            ('-', Color::Gray),
        ]),
    )
}

pub fn color_map_clustalx() -> ColorMap {
    ColorMap::new(
        "ClustalX".into(),
        HashMap::from([
            ('G', CLUSTALX_ORANGE),
            ('A', CLUSTALX_BLUE),
            ('S', CLUSTALX_GREEN),
            ('T', CLUSTALX_GREEN),
            ('C', CLUSTALX_PINK),
            ('V', CLUSTALX_BLUE),
            ('I', CLUSTALX_BLUE),
            ('L', CLUSTALX_BLUE),
            ('P', CLUSTALX_YELLOW),
            ('F', CLUSTALX_BLUE),
            ('Y', CLUSTALX_CYAN),
            ('M', CLUSTALX_BLUE),
            ('W', CLUSTALX_BLUE),
            ('N', CLUSTALX_GREEN),
            ('Q', CLUSTALX_GREEN),
            ('H', CLUSTALX_CYAN),
            ('D', CLUSTALX_MAGENTA),
            ('E', CLUSTALX_MAGENTA),
            ('K', CLUSTALX_RED),
            ('R', CLUSTALX_RED),
            ('X', Color::White),
            ('g', CLUSTALX_ORANGE),
            ('a', CLUSTALX_BLUE),
            ('s', CLUSTALX_GREEN),
            ('t', CLUSTALX_GREEN),
            ('c', CLUSTALX_PINK),
            ('v', CLUSTALX_BLUE),
            ('i', CLUSTALX_BLUE),
            ('l', CLUSTALX_BLUE),
            ('p', CLUSTALX_YELLOW),
            ('f', CLUSTALX_BLUE),
            ('y', CLUSTALX_CYAN),
            ('m', CLUSTALX_BLUE),
            ('w', CLUSTALX_BLUE),
            ('n', CLUSTALX_GREEN),
            ('q', CLUSTALX_GREEN),
            ('h', CLUSTALX_CYAN),
            ('d', CLUSTALX_MAGENTA),
            ('e', CLUSTALX_MAGENTA),
            ('k', CLUSTALX_RED),
            ('r', CLUSTALX_RED),
            ('x', Color::White),
            ('-', Color::Gray),
        ]),
    )
}

pub fn color_map_jalview_nt() -> ColorMap {
    ColorMap::new(
        "JalView nt".into(),
        HashMap::from([
            ('A', JALVIEW_NUCLEOTIDE_A),
            ('C', JALVIEW_NUCLEOTIDE_C),
            ('G', JALVIEW_NUCLEOTIDE_G),
            ('T', JALVIEW_NUCLEOTIDE_T),
            ('U', JALVIEW_NUCLEOTIDE_U),
            ('I', JALVIEW_NUCLEOTIDE_I),
            ('X', JALVIEW_NUCLEOTIDE_X),
            ('R', JALVIEW_NUCLEOTIDE_R),
            ('Y', JALVIEW_NUCLEOTIDE_Y),
            ('W', JALVIEW_NUCLEOTIDE_W),
            ('S', JALVIEW_NUCLEOTIDE_S),
            ('M', JALVIEW_NUCLEOTIDE_M),
            ('K', JALVIEW_NUCLEOTIDE_K),
            ('B', JALVIEW_NUCLEOTIDE_B),
            ('H', JALVIEW_NUCLEOTIDE_H),
            ('D', JALVIEW_NUCLEOTIDE_D),
            ('V', JALVIEW_NUCLEOTIDE_V),
            ('N', JALVIEW_NUCLEOTIDE_N),
            ('a', JALVIEW_NUCLEOTIDE_A),
            ('c', JALVIEW_NUCLEOTIDE_C),
            ('g', JALVIEW_NUCLEOTIDE_G),
            ('t', JALVIEW_NUCLEOTIDE_T),
            ('u', JALVIEW_NUCLEOTIDE_U),
            ('i', JALVIEW_NUCLEOTIDE_I),
            ('x', JALVIEW_NUCLEOTIDE_X),
            ('r', JALVIEW_NUCLEOTIDE_R),
            ('y', JALVIEW_NUCLEOTIDE_Y),
            ('w', JALVIEW_NUCLEOTIDE_W),
            ('s', JALVIEW_NUCLEOTIDE_S),
            ('m', JALVIEW_NUCLEOTIDE_M),
            ('k', JALVIEW_NUCLEOTIDE_K),
            ('b', JALVIEW_NUCLEOTIDE_B),
            ('h', JALVIEW_NUCLEOTIDE_H),
            ('d', JALVIEW_NUCLEOTIDE_D),
            ('v', JALVIEW_NUCLEOTIDE_V),
            ('n', JALVIEW_NUCLEOTIDE_N),
        ]),
    )
}

pub fn colormap_gecos(path: String) -> ColorMap {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let cm: serde_json::Value = serde_json::from_reader(reader).unwrap();

    //println!("{:#?}", cm["colors"]);
    let mut color_map: HashMap<char, Color> = HashMap::new();
    let orig_map = &cm["colors"];
    if let Object(map) = orig_map {
        //println!("Found map: {:#?}", map);
        for (k, v) in map {
            let color_str = serde_json::from_value::<String>(v.clone()).unwrap();
            let hex_color = HexColor::parse_rgb(&color_str).unwrap();
            let color = Color::Rgb(hex_color.r, hex_color.g, hex_color.b);
            let residue = k.chars().collect::<Vec<char>>()[0];
            let residue_lc = residue.to_ascii_lowercase();
            color_map.insert(residue, color);
            color_map.insert(residue_lc, color);
            //println!("{} -> {}", k.chars().collect::<Vec<char>>()[0], color);
        }
        color_map.insert('-', Color::Gray);
    }

    ColorMap::new("custom".into(), color_map)
}
