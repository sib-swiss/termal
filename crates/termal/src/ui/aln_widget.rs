// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use ratatui::{
    prelude::{Buffer, Position, Rect},
    style::{Modifier, Style},
    widgets::Widget,
};

use crate::{app::App, ui::zoombox::draw_zoombox_border};

fn highlight_match_style(mut style: Style, is_match: bool, is_current_match: bool) -> Style {
    if is_match {
        if style.add_modifier.contains(Modifier::REVERSED) {
            style = style.remove_modifier(Modifier::REVERSED);
        } else {
            style = style.add_modifier(Modifier::REVERSED);
        }
        style = style.add_modifier(Modifier::UNDERLINED);
    }

    if is_current_match {
        style = style.add_modifier(Modifier::BOLD);
    }

    style
}

pub struct SeqPane<'a> {
    pub app: &'a App,
    pub sequences: &'a [String],
    pub ordering: &'a [usize],
    pub top_i: usize,
    pub left_j: usize,
    pub style_lut: &'a [Style],
    // TODO: not sure this is required - if not, also remove from other SeqPane* structs
    pub base_style: Style, // optional, for clearing/background
}

impl<'a> Widget for SeqPane<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = area.height as usize;
        let cols = area.width as usize;

        // Clear the pane so “extra space” doesn’t show stale cells.
        for y in 0..rows {
            for x in 0..cols {
                buf.cell_mut(Position::from((area.x + x as u16, area.y + y as u16)))
                    .expect("Wrong position")
                    .set_char(' ')
                    .set_style(self.base_style);
            }
        }

        for r in 0..rows {
            let i = self.top_i + r;
            if i >= self.ordering.len() {
                break;
            }
            let seq = self.sequences[self.ordering[i]].as_bytes();

            for c in 0..cols {
                let j = self.left_j + c;
                if j >= seq.len() {
                    break;
                }
                let raw_char = seq[j];
                let style = highlight_match_style(
                    self.style_lut[raw_char as usize],
                    self.app.cell_is_seq_match(i, j),
                    self.app.cell_is_current_seq_match(i, j),
                );
                let diffed_char = apply_diff_mode(raw_char, ref_char, diff_mode);

                buf.cell_mut(Position::from((area.x + c as u16, area.y + r as u16)))
                    .expect("Wrong position")
                    .set_char(raw_char as char)
                    .set_style(style);
            }
        }
    }
}

pub struct SeqPaneZoomedOut<'a> {
    pub app: &'a App,
    pub sequences: &'a [String],    // alignment.sequences
    pub ordering: &'a [usize],      // ordering map
    pub retained_rows: &'a [usize], // indices into "logical rows"
    pub retained_cols: &'a [usize], // indices into alignment columns
    pub style_lut: &'a [Style],     // style per byte (0..=255)
    pub base_style: Style,          // for clearing/background
    pub show_zoombox: bool,
    pub zb_top: usize,
    pub zb_bottom: usize,
    pub zb_left: usize,
    pub zb_right: usize,
    pub zb_style: Style,
}

impl<'a> Widget for SeqPaneZoomedOut<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let rows = area.height as usize;
        let cols = area.width as usize;

        // Clear pane (see ZoomedIn mode)
        for y in 0..rows {
            for x in 0..cols {
                buf.cell_mut(Position::from((area.x + x as u16, area.y + y as u16)))
                    .expect("Wrong position")
                    .set_char(' ')
                    .set_style(self.base_style);
            }
        }

        // Render sampled rows/cols
        let max_r = rows.min(self.retained_rows.len());
        let max_c = cols.min(self.retained_cols.len());

        for r in 0..max_r {
            let i = self.retained_rows[r];
            // should never happen
            if i >= self.ordering.len() {
                panic!();
            }

            let seq_bytes = self.sequences[self.ordering[i]].as_bytes();

            for c in 0..max_c {
                let j = self.retained_cols[c];
                // should never happen
                if j >= seq_bytes.len() {
                    panic!();
                }

                let raw_char = seq_bytes[j];
                let style = highlight_match_style(
                    self.style_lut[raw_char as usize],
                    self.app.cell_is_seq_match(i, j),
                    self.app.cell_is_current_seq_match(i, j),
                );
                let diffed_char = apply_diff_mode(raw_char, ref_char, diff_mode);

                buf.cell_mut(Position::from((area.x + c as u16, area.y + r as u16)))
                    .expect("Wrong position")
                    .set_char(raw_char as char)
                    .set_style(style);
            }
        }

        if self.show_zoombox {
            draw_zoombox_border(
                buf,
                area,
                self.zb_top,
                self.zb_bottom,
                self.zb_left,
                self.zb_right,
                self.zb_style,
            );
        }
    }
}
