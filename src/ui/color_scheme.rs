// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use ratatui::prelude::Color;

use crate::{
    alignment::SeqType,
    ui::{
        color_scheme::SeqType::Protein
    },
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
    pub label_num_color: Color,
    pub seq_metric_color: Color,

    // Index into Vec of &Colormaps
    pub colormap_index: usize,

    pub zoombox_color: Color,

    pub position_color: Color,
    pub conservation_color: Color,
    #[allow(dead_code)]
    pub consensus_default_color: Color,
}

#[allow(dead_code)]
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

pub fn color_scheme_colored(macromolecule_type: SeqType) -> ColorScheme {
    // These are indices into the Vec of built-in color maps, see color_maps.rs
    let index = if macromolecule_type == Protein {
        1
    } else {
        0
    };
    ColorScheme {
        label_num_color: Color::LightGreen,
        colormap_index: index,
        seq_metric_color: Color::LightBlue,
        zoombox_color: Color::Cyan,
        position_color: Color::White,
        conservation_color: SALMON,
        consensus_default_color: Color::White,
    }
}
