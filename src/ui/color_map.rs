use std::{
    collections::HashMap,
    io::BufReader,
    iter::Map,
    fs::File,
};

use ratatui::prelude::Color;
use hex_color::HexColor;

use serde_json::Value;
use serde_json::Value::Object;

use crate::ui::color_scheme::ORANGE;

// NOTE: if it turns out that these hash maps are not efficient (didn't benchmark yet), we might
// want to look at perfect hash functions - see e.g https://crates.io/crates/phf

// NOTE, although these maps do not vary, we cannot dclare them as constants, because they involve
// a function call (namely, to HashMap::from()).

// It's prolly easier to have a no-op colorscheme than to decide at every iteration if we do a
// lookup or not.

pub fn color_map_monochrome() -> HashMap<char, Color> {
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
    ])
}

pub fn color_map_lesk() -> HashMap<char, Color> {
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
    ])
}

pub fn colormap_gecos() -> HashMap<char, Color> {
    let path = "./src/ui/colormaps/gecos_default.json";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let cm: serde_json::Value = serde_json::from_reader(reader).unwrap();

    //println!("{:#?}", cm["colors"]);
    let mut color_map: HashMap<char, Color> = HashMap::new();
    let orig_map =  &cm["colors"];
    if let Object(map) = orig_map {
        //println!("Found map: {:#?}", map);
        for (k, v) in map {
            let color_str = serde_json::from_value::<String>(v.clone()).unwrap();
            let hex_color = HexColor::parse_rgb(&color_str).unwrap();
            let color = Color::Rgb(hex_color.r, hex_color.g, hex_color.b);
            color_map.insert(k.chars().collect::<Vec<char>>()[0], color);
            //println!("{} -> {}", k.chars().collect::<Vec<char>>()[0], color);
        }
        color_map.insert('-', Color::Gray);
    }

    color_map
}
