// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use ratatui::style::{Color, Modifier, Style};

use super::{
    UI, {Theme, VideoMode},
};

pub fn get_residue_style(video_mode: VideoMode, theme: Theme, color: Color) -> Style {
    let mut style = Style::default();

    match theme {
        Theme::Dark | Theme::Light => {
            style = style.fg(color);
        }
        Theme::Monochrome => {
            style = style.fg(Color::Reset).bg(Color::Reset);
        }
    }

    if video_mode == VideoMode::Inverse {
        style = style.add_modifier(Modifier::REVERSED);
        if Theme::Light == theme {
            style = style.bg(Color::Black);
        }
    }

    style
}

pub fn build_style_lut(ui: &UI) -> [Style; 256] {
    let colormap = ui.color_scheme().current_residue_colormap();
    std::array::from_fn(|b| {
        let ch = b as u8 as char;
        get_residue_style(ui.video_mode, ui.theme(), colormap.get(ch))
    })
}
