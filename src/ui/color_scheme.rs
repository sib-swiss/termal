use std::collections::HashMap;

use ratatui::prelude::Color;

// NOTE: if it turns out that these hash maps are not efficient (didn't benchmark yet), we might
// want to look at perfect hash functions - see e.g https://crates.io/crates/phf

pub const ORANGE: Color = Color::Rgb(255, 165, 0);
pub const SALMON: Color = Color::Rgb(250, 128, 114);

// It's prolly easier to have a no-op colorscheme than to decide at every iteration if we do a
// lookup or not.

pub fn color_scheme_monochrome() -> HashMap<char, Color> {
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
        ('-', Color::White),
    ])
}

pub fn color_scheme_lesk() -> HashMap<char, Color> {
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
        ('-', Color::Gray),
    ])
}
