// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use ratatui::prelude::Color;

use crate::{
    alignment::SeqType,
    ui::{
        color_map::{
            ColorMap,
            builtin_polychrome_colormaps,
            monochrome_colormap,
        },
        color_scheme::SeqType::Protein,
    }
};

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

// JalView Nucleotide Colors

pub const JALVIEW_NUCLEOTIDE_A: Color = Color::from_u32(0x0064F73F);
pub const JALVIEW_NUCLEOTIDE_C: Color = Color::from_u32(0x00FFB340);
pub const JALVIEW_NUCLEOTIDE_G: Color = Color::from_u32(0x00EB413C);
pub const JALVIEW_NUCLEOTIDE_T: Color = Color::from_u32(0x003C88EE);
pub const JALVIEW_NUCLEOTIDE_U: Color = Color::from_u32(0x003C88EE);
pub const JALVIEW_NUCLEOTIDE_I: Color = Color::from_u32(0x00ffffff);
pub const JALVIEW_NUCLEOTIDE_X: Color = Color::from_u32(0x004f6f6f);
pub const JALVIEW_NUCLEOTIDE_R: Color = Color::from_u32(0x00CD5C5C);
pub const JALVIEW_NUCLEOTIDE_Y: Color = Color::from_u32(0x00008000);
pub const JALVIEW_NUCLEOTIDE_W: Color = Color::from_u32(0x004682B4);
pub const JALVIEW_NUCLEOTIDE_S: Color = Color::from_u32(0x00FF8C00);
pub const JALVIEW_NUCLEOTIDE_M: Color = Color::from_u32(0x009ACD32);
pub const JALVIEW_NUCLEOTIDE_K: Color = Color::from_u32(0x009932CC);
pub const JALVIEW_NUCLEOTIDE_B: Color = Color::from_u32(0x008b4513);
pub const JALVIEW_NUCLEOTIDE_H: Color = Color::from_u32(0x00808080);
pub const JALVIEW_NUCLEOTIDE_D: Color = Color::from_u32(0x00483D8B);
pub const JALVIEW_NUCLEOTIDE_V: Color = Color::from_u32(0x00b8860b);
pub const JALVIEW_NUCLEOTIDE_N: Color = Color::from_u32(0x002f4f4f);

pub struct ColorScheme {
    label_num_color: Color,
    seq_metric_color: Color,

    // Different color schemes may have different available color maps.
    residue_colormaps: Vec<ColorMap>,
    // Index into Vec of &Colormaps
    residue_colormap_index: usize,

    zoombox_color: Color,

    position_color: Color,
    conservation_color: Color,
}

impl ColorScheme {
    // TODO: the Vec of colormaps should depend on the macromolecule, i.e. only protein maps for aa,
    // and only nt maps for nt.
    pub fn color_scheme_dark(macromolecule_type: SeqType) -> Self {
        // These are indices into the Vec of built-in color maps, see color_maps.rs
        let index = if macromolecule_type == Protein { 1 } else { 0 };
        ColorScheme {
            label_num_color: Color::LightGreen,
            seq_metric_color: Color::LightBlue,
            residue_colormaps: builtin_polychrome_colormaps(),
            residue_colormap_index: index,
            zoombox_color: Color::Cyan,
            position_color: Color::White,
            conservation_color: SALMON,
        }
    }

    pub fn color_scheme_light(macromolecule_type: SeqType) -> Self {
        // These are indices into the Vec of built-in color maps, see color_maps.rs
        let index = if macromolecule_type == Protein { 1 } else { 0 };
        ColorScheme {
            label_num_color: Color::from_u32(0x00008000), 
            seq_metric_color: Color::Rgb(25, 127, 229),
            residue_colormaps: builtin_polychrome_colormaps(),
            residue_colormap_index: index,
            zoombox_color: Color::Cyan,
            position_color: Color::White,
            conservation_color: SALMON,
        }
    }

    pub fn color_scheme_monochrome() -> Self {
        ColorScheme {
            label_num_color: Color::White,
            seq_metric_color: Color::White,
            residue_colormaps: monochrome_colormap(), // Vec<ColorMap>
            residue_colormap_index: 0,
            zoombox_color: Color::White,
            position_color: Color::White,
            conservation_color: Color::White,
        }
    }

    pub fn current_residue_colormap(&self) -> &ColorMap {
        &(self.residue_colormaps[self.residue_colormap_index])
    }

    pub fn cycle_colormaps(&mut self) {
        let size = self.residue_colormaps.len();
        self.residue_colormap_index += 1;
        self.residue_colormap_index %= size;
    }
}
