// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use ratatui::{
    prelude::{Buffer, Position, Rect},
    style::{Modifier, Style},
    widgets::Widget,
};

use termal_alignment::alignment::RefSpec;

use crate::{app::App, ui::{DiffMode, zoombox::draw_zoombox_border}};

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

// TODO: is it necessary that these fields (and the struct itself) be pub?
pub struct SeqPane<'a> {
    pub app: &'a App,
    pub sequences: &'a [String],
    pub ordering: &'a [usize],
    pub ref_spec: RefSpec,
    pub diff_mode: DiffMode,
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
                let diffed_char: u8;
                match self.ref_spec {
                    RefSpec::Consensus => {
                        let reference = &self.app.alignment.consensus.as_bytes();
                        let ref_char = reference[j];
                        diffed_char = apply_diff_mode(raw_char, ref_char, self.diff_mode);
                    }
                    RefSpec::Rank(rk) => {
                        let ref_screenline = self.ordering[rk];
                        let reference = self.sequences[ref_screenline].as_bytes();
                        let ref_char = reference[j];
                        if i == ref_screenline { // Keep the reference untouched
                            diffed_char = raw_char;
                        } else {
                            diffed_char = apply_diff_mode(raw_char, ref_char, self.diff_mode);
                        }
                    }
                }

                buf.cell_mut(Position::from((area.x + c as u16, area.y + r as u16)))
                    .expect("Wrong position")
                    .set_char(diffed_char as char)
                    .set_style(style);
            }
        }
    }
}

// TODO: is it necessary that these fields (and the struct itself) be pub?
pub struct SeqPaneZoomedOut<'a> {
    pub app: &'a App,
    pub sequences: &'a [String],    // alignment.sequences
    pub ordering: &'a [usize],      // ordering map
    pub reference: &'a [u8],
    pub diff_mode: DiffMode,
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
                let ref_char = self.reference[j as usize];
                let diffed_char: u8 = apply_diff_mode(raw_char, ref_char, self.diff_mode);

                buf.cell_mut(Position::from((area.x + c as u16, area.y + r as u16)))
                    .expect("Wrong position")
                    .set_char(diffed_char as char)
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

fn apply_diff_mode(raw_c: u8, ref_c: u8, diff_mode: DiffMode) -> u8 {
    match diff_mode {
        DiffMode::Original => {
            raw_c
        }
        DiffMode::DiffWRTRef => {
            if raw_c.to_ascii_uppercase() != ref_c.to_ascii_uppercase() {
                raw_c
            } else {
                b'-'
            }
        }
    }
}

#[cfg(test)]
mod tests {

use crate::ui::{DiffMode, aln_widget::apply_diff_mode};

    #[test]
    fn test_apply_diff_mode() {

        let diff_mode = DiffMode::Original;
        let raw_char = b'A';
        let ref_char = b'A';
        let obt = apply_diff_mode(raw_char, ref_char, diff_mode);
        assert_eq!(obt, b'A');

        let diff_mode = DiffMode::DiffWRTRef;
        let raw_char = b'A';
        let ref_char = b'A';
        let obt = apply_diff_mode(raw_char, ref_char, diff_mode);
        assert_eq!(obt, b'-');

        let diff_mode = DiffMode::DiffWRTRef;
        let raw_char = b'C';
        let ref_char = b'A';
        let obt = apply_diff_mode(raw_char, ref_char, diff_mode);
        assert_eq!(obt, b'C');

    }

}
