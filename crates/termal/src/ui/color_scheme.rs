// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use std::fmt;

use ratatui::prelude::Color;

use termal_alignment::alignment::SeqType;

use crate::ui::{
    color_map::{builtin_polychrome_colormaps, monochrome_colormap, ColorMap},
    color_scheme::SeqType::Protein,
};

// In-house colors
// FIXME: there is also a Salmon colour in termal_alignment::rgb, but IIRC it not used for any
// residue.
pub const SALMON: Color = Color::Rgb(250, 128, 114);

// ClustalX colors (source:
// https://www.cgl.ucsf.edu/chimera/1.2065/docs/ContributedSoftware/multalignviewer/colprot.par)

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    Monochrome,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Monochrome => "Mono",
        };
        write!(f, "{}", s)
    }
}

// TODO: make these private and use getters.
pub struct ColorScheme {
    pub theme: Theme,
    pub label_num_color: Color,
    pub seq_metric_color: Color,
    // Different color schemes may have different available color maps.
    pub residue_colormaps: Vec<ColorMap>,
    // Index into Vec of &Colormaps
    pub residue_colormap_index: usize,
    pub zoombox_color: Color,
    pub conservation_color: Color,
}

impl ColorScheme {
    // TODO: the Vec of colormaps should depend on the macromolecule, i.e. only protein maps for aa,
    // and only nt maps for nt.
    pub fn color_scheme_dark(macromolecule_type: SeqType) -> Self {
        // These are indices into the Vec of built-in color maps, see color_maps.rs
        let index = if macromolecule_type == Protein { 1 } else { 0 };
        ColorScheme {
            theme: Theme::Dark,
            label_num_color: Color::LightGreen,
            seq_metric_color: Color::LightBlue,
            residue_colormaps: builtin_polychrome_colormaps(),
            residue_colormap_index: index,
            zoombox_color: Color::Cyan,
            conservation_color: SALMON,
        }
    }

    pub fn color_scheme_light(macromolecule_type: SeqType) -> Self {
        // These are indices into the Vec of built-in color maps, see color_maps.rs
        let index = if macromolecule_type == Protein { 1 } else { 0 };
        ColorScheme {
            theme: Theme::Light,
            label_num_color: Color::from_u32(0x00008000),
            seq_metric_color: Color::Rgb(25, 127, 229),
            residue_colormaps: builtin_polychrome_colormaps(),
            residue_colormap_index: index,
            zoombox_color: Color::Cyan,
            conservation_color: SALMON,
        }
    }

    pub fn color_scheme_monochrome() -> Self {
        ColorScheme {
            theme: Theme::Monochrome,
            label_num_color: Color::White,
            seq_metric_color: Color::White,
            residue_colormaps: monochrome_colormap(), // Vec<ColorMap>
            residue_colormap_index: 0,
            zoombox_color: Color::White,
            conservation_color: Color::White,
        }
    }

    // Inserts a colormap and returns the index at which the map was inserted (namely, the last
    // valid index in the vec).
    pub fn add_colormap(&mut self, cmap: ColorMap) {
        self.residue_colormaps.insert(0, cmap);
    }

    pub fn select_first_colormap(&mut self) {
        self.residue_colormap_index = 0;
    }

    pub fn current_residue_colormap(&self) -> &ColorMap {
        &(self.residue_colormaps[self.residue_colormap_index])
    }

    pub fn next_colormap(&mut self) {
        let size = self.residue_colormaps.len();
        self.residue_colormap_index += 1;
        self.residue_colormap_index %= size;
    }

    pub fn prev_colormap(&mut self) {
        let size = self.residue_colormaps.len();
        // Add size -> no need to check for < 0
        self.residue_colormap_index += size - 1;
        self.residue_colormap_index %= size;
    }
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.theme, self.current_residue_colormap())
    }
}
