// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier
pub mod color_map;
mod color_scheme;
mod barchart;
pub mod key_handling;
pub mod render;

use std::cmp::min; 

use log::debug;

use bitflags::bitflags;

use ratatui::layout::Size;
use ratatui::style::Color;

use crate::{
    ui::color_map::{
        MONOCHROME_INDEX, builtin_colormaps,
        ColorMap,
    },
    ui::color_scheme::{color_scheme_colored, ColorScheme},
    App,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZoomLevel {
    ZoomedIn,
    ZoomedOut,
    ZoomedOutAR,
}

#[derive(Debug)]
enum BottomPanePosition {
    Adjacent,
    ScreenBottom,
}

enum Theme {
    Light,
    Dark,
}
// A bit field that denotes if the alignment is too wide (with respect to the sequence panel), too
// tall, both, or neither.

bitflags! {
    #[derive(PartialEq)]
    pub struct AlnWRTSeqPane: u8 {
        const Fits           = 0b00;
        const TooTall        = 0b01;
        const TooWide        = 0b10;
        const TooTallAndWide = 0b11;
    }
}

pub struct UI<'a> {
    app: &'a mut App,
    color_scheme: ColorScheme, 
    zoom_level: ZoomLevel,
    show_zoombox: bool,
    //zoombox_color: Style,
    show_zb_guides: bool,
    show_scrollbars: bool,
    highlight_retained_cols: bool,
    top_line: u16,
    leftmost_col: u16,
    label_pane_width: u16,
    previous_label_pane_width: u16, // To restore width after hiding pane
    bottom_pane_height: u16,
    previous_bottom_pane_height: u16,
    bottom_pane_position: BottomPanePosition,
    // These cannot be known when the structure is initialized, so they are Options -- but it is
    // possible that they need not be stored at all, as they can in principle be computed when the
    // layout is known.
    aln_pane_size: Option<Size>,
    frame_size: Option<Size>, // whole app
    show_help: bool,
    full_screen: bool,
    message: String, // Simple, 1-line message (possibly just "", no need for Option IMHO)
    inverse: bool,   // invert bg/fg
    theme: Theme,
    colormaps: Vec<ColorMap>,
}

impl<'a> UI<'a> {
    pub fn new(app: &'a mut App) -> Self {
        let macromolecule_type = app.alignment.macromolecule_type();
        UI {
            app,
            color_scheme: color_scheme_colored(macromolecule_type),
            zoom_level: ZoomLevel::ZoomedIn,
            show_zoombox: true,
            show_zb_guides: true,
            show_scrollbars: true,
            highlight_retained_cols: false,
            top_line: 0,
            leftmost_col: 0,
            label_pane_width: 18, // Reasonable default, I'd say...
            previous_label_pane_width: 0,
            bottom_pane_height: 5,
            previous_bottom_pane_height: 0,
            bottom_pane_position: BottomPanePosition::Adjacent,
            aln_pane_size: None,
            frame_size: None,
            show_help: false,
            full_screen: false,
            message: " Press '?' for help ".into(),
            inverse: true,
            theme: Theme::Dark,
            colormaps: builtin_colormaps(),
        }
    }

    // ****************************************************************
    /*
     * Dimensions
     *
     * The layout determines the maximal number of sequences and columns shown; this in turn
     * affects the maximal top line and leftmost column, etc.
     * */

    fn max_nb_seq_shown(&self) -> u16 {
        let height = self.aln_pane_size.unwrap().height;
        if height >= 2 {
            // border, should later be a constant or a field of UI
            height - 2
        } else {
            // Set to null (prevents display) if not enough room
            // NOTE: this causes v_ratio() to return 0, which in turn causes the number of retained
            // sequences to be 0, causing render::every_nth() to crash. Maybe the minimum should be
            // 2, not 0. TODO: prepare more tests (esp. w/ small sets), change to 2, and check.
            // Then do the same for max_nb_col_shown().
            0
        }
    }

    fn max_nb_col_shown(&self) -> u16 {
        let width = self.aln_pane_size.unwrap().width;
        if width >= 2 {
            width - 2
        } else {
            0
        }
    }

    // Resizing (as when the user resizes the terminal window where Termal runs) affects
    // max_top_line and max_leftmost_col (because the number of available lines (resp. columns)
    // will generally change), so top_line and leftmost_col may now exceed them. This function,
    // which should be called after the layout is solved but before the widgets are drawn, makes
    // sure that l_max corresponds to the size of the alignment panel, so that l does not exceed
    // l_max, etc.

    pub fn adjust_seq_pane_position(&mut self) {
        if self.leftmost_col > self.max_leftmost_col() {
            self.leftmost_col = self.max_leftmost_col();
        }
        if self.top_line > self.max_top_line() {
            self.top_line = self.max_top_line();
        }
    }

    /* The following are only called internally. */

    fn max_top_line(&self) -> u16 {
        if self.app.num_seq() >= self.max_nb_seq_shown() {
            self.app.num_seq() - self.max_nb_seq_shown()
        } else {
            0
        }
    }

    fn max_leftmost_col(&self) -> u16 {
        if self.app.aln_len() >= self.max_nb_col_shown() {
            self.app.aln_len() - self.max_nb_col_shown()
        } else {
            0
        }
    }

    // Side panel dimensions

    pub fn set_label_pane_width(&mut self, width: u16) {
        self.label_pane_width = width;
    }

    // Also stores previous width
    pub fn hide_label_pane(&mut self) {
        self.previous_label_pane_width = self.label_pane_width;
        self.label_pane_width = 0;
    }

    pub fn show_label_pane(&mut self) {
        self.label_pane_width = self.previous_label_pane_width;
    }

    pub fn widen_label_pane(&mut self, amount: u16) {
        // TODO: heed the border width (not sure if we'll keep them)
        self.label_pane_width = if self.label_pane_width + amount < self.frame_size.unwrap().width {
            self.label_pane_width + amount
        } else {
            self.frame_size.unwrap().width
        }
    }

    pub fn reduce_label_pane(&mut self, amount: u16) {
        // TODO: heed the border width (not sure if we'll keep them)
        self.label_pane_width = if self.label_pane_width > amount {
            self.label_pane_width - amount
        } else {
            0
        }
    }

    // Bottom pane dimensions

    pub fn set_bottom_pane_height(&mut self, height: u16) {
        self.bottom_pane_height = height;
    }

    pub fn hide_bottom_pane(&mut self) {
        self.previous_bottom_pane_height = self.bottom_pane_height;
        self.bottom_pane_height = 0;
    }

    pub fn show_bottom_pane(&mut self) {
        self.bottom_pane_height = 5;
    }

    // ****************************************************************
    // Zooming

    // Determines if the alignment fits on the screen or is too wide or tall (it can be both).
    // TODO: might be an inner function of cycle_zoom, as it is not used anywhere else.
    fn aln_wrt_seq_pane(&self) -> AlnWRTSeqPane {
        let mut rel = AlnWRTSeqPane::Fits;
        if self.app.aln_len() > self.max_nb_col_shown() {
            rel |= AlnWRTSeqPane::TooWide;
        }
        if self.app.num_seq() > self.max_nb_seq_shown() {
            rel |= AlnWRTSeqPane::TooTall;
        }

        rel
    }

    // TODO: is this accessor needed?
    pub fn zoom_level(&self) -> ZoomLevel {
        self.zoom_level
    }

    pub fn cycle_zoom(&mut self) {
        self.zoom_level = match self.zoom_level {
            ZoomLevel::ZoomedIn => {
                // ZoomedOut, unless alignment fits
                if self.aln_wrt_seq_pane() == AlnWRTSeqPane::Fits {
                    ZoomLevel::ZoomedIn
                } else {
                    ZoomLevel::ZoomedOut
                }
            }
            ZoomLevel::ZoomedOut => ZoomLevel::ZoomedOutAR,
            ZoomLevel::ZoomedOutAR => ZoomLevel::ZoomedIn,
        }
    }

    pub fn h_ratio(&self) -> f64 {
        self.max_nb_col_shown() as f64 / self.app.aln_len() as f64
    }

    pub fn v_ratio(&self) -> f64 {
        self.max_nb_seq_shown() as f64 / self.app.num_seq() as f64
    }

    // ZoomLevel::ZoomedOutAR mode uses a _single_ ratio, which is usually the minimum of the
    // vertical and horizontal ratios, but it _can_ use the mmaximum if the resulting alignment
    // still fits.
    pub fn common_ratio(&self) -> f64 {
        let min_ratio = self.h_ratio().min(self.v_ratio());
        let max_ratio = self.h_ratio().max(self.v_ratio());
        let min_r_cols = (self.app.aln_len() as f64 * min_ratio).floor() as u16;
        let min_r_seqs = (self.app.num_seq() as f64 * min_ratio).floor() as u16;
        let max_r_cols = (self.app.aln_len() as f64 * max_ratio).floor() as u16;
        let max_r_seqs = (self.app.num_seq() as f64 * max_ratio).floor() as u16;

        debug!("  ***");
        debug!(
            "  max shown cols: {}, max shown seqs: {}",
            self.max_nb_col_shown(),
            self.max_nb_seq_shown()
        );
        debug!(
            "  h_r: {:.2}, v_r: {:.2}, min_r: {:.2}, max_r: {:.2}",
            self.h_ratio(),
            self.v_ratio(),
            min_ratio,
            max_ratio,
        );
        debug!(
            "  min ratio ({:.2}): {} seqs x {} cols",
            min_ratio, min_r_seqs, min_r_cols
        );
        debug!(
            "  max ratio ({:.2}): {} seqs x {} cols",
            max_ratio, max_r_seqs, max_r_cols
        );

        if max_r_cols == self.max_nb_col_shown() && max_r_seqs == self.max_nb_seq_shown() {
            max_ratio
        } else {
            min_ratio
        }
    }

    pub fn set_zoombox(&mut self, state: bool) {
        self.show_zoombox = state;
    }

    pub fn toggle_zoombox(&mut self) {
        self.show_zoombox = !self.show_zoombox;
    }

    // TODO: do we really need seq_para_len? Or can we just use self.app.num_seq?
    pub fn zoombox_top(&self) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => ((self.top_line as f64) * self.v_ratio()).floor() as usize,
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                ((self.top_line as f64) * ratio).floor() as usize
            }
            _ => panic!(
                "zoombox_top() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn zoombox_bottom(&self, seq_para_len: usize) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                let mut zb_bottom: usize = (((self.top_line + self.max_nb_seq_shown()) as f64)
                    * self.v_ratio())
                .round() as usize;
                // If h_a < h_p
                if zb_bottom > self.app.num_seq() as usize {
                    zb_bottom = self.app.num_seq() as usize;
                }
                zb_bottom
            }
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                let aln_para_height = min(seq_para_len as u16, self.max_nb_seq_shown());
                let mut zb_bottom =
                    (((self.top_line + self.max_nb_seq_shown()) as f64) * ratio).round() as usize;
                // If h_a < h_p
                if zb_bottom > aln_para_height as usize {
                    zb_bottom = aln_para_height as usize;
                }
                zb_bottom
            }
            _ => panic!(
                "zoombox_bottom() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn zoombox_left(&self) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => ((self.leftmost_col as f64) * self.h_ratio()).floor() as usize,
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                ((self.leftmost_col as f64) * ratio).floor() as usize
            }
            _ => panic!(
                "zoombox_left() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn zoombox_right(&self, max_nb_col_shown_ar: usize) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                let mut zb_right = (((self.leftmost_col + self.max_nb_col_shown()) as f64)
                    * self.h_ratio())
                .floor() as usize;
                // If w_a < w_p
                if zb_right > self.app.aln_len() as usize {
                    zb_right = self.app.aln_len() as usize;
                }
                zb_right
            }
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                let aln_para_width = min(max_nb_col_shown_ar as u16, self.max_nb_col_shown());
                let mut zb_right = (((self.leftmost_col + self.max_nb_col_shown()) as f64) * ratio)
                    .floor() as usize;
                // If w_a < w_p
                if zb_right > aln_para_width as usize {
                    zb_right = aln_para_width as usize;
                }
                zb_right
            }
            _ => panic!(
                "zoombox_left() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn cycle_bottom_pane_position(&mut self) {
        self.bottom_pane_position = match self.bottom_pane_position {
            BottomPanePosition::Adjacent => BottomPanePosition::ScreenBottom,
            BottomPanePosition::ScreenBottom => BottomPanePosition::Adjacent,
        }
    }

    pub fn set_zoombox_guides(&mut self, state: bool) {
        self.show_zb_guides = state;
    }

    pub fn toggle_hl_retained_cols(&mut self) {
        self.highlight_retained_cols = !self.highlight_retained_cols;
    }

    // ****************************************************************
    // Colors

    pub fn set_monochrome(&mut self) {
        self.color_scheme.colormap_index = MONOCHROME_INDEX;
    }

    #[allow(dead_code)]
    pub fn set_colormap(&mut self, cmap_ndx: usize) {
        self.color_scheme.colormap_index = cmap_ndx;
    }

    pub fn cycle_colormap(&mut self) {
        let nb_colormaps = self.colormaps.len();
        self.color_scheme.colormap_index =
            (self.color_scheme.colormap_index + 1) % nb_colormaps;
    }

    pub fn toggle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn get_label_num_color(&self) -> Color {
        match self.theme {
            Theme::Dark => self.color_scheme.dark_bg_label_num_color,
            Theme::Light => self.color_scheme.light_bg_label_num_color,
        }
    }

    // ****************************************************************
    // Ordering

    pub fn cycle_ordering_criterion(&mut self) {
        // Just delegate to App
        self.app.cycle_ordering_criterion();
    }

    // ****************************************************************

    pub fn disable_scrollbars(&mut self) {
        self.show_scrollbars = false;
    }

    // Scrolling

    pub fn scroll_one_line_up(&mut self) {
        if self.top_line > 0 {
            self.top_line -= 1;
        }
    }

    pub fn scroll_one_col_left(&mut self) {
        if self.leftmost_col > 0 {
            self.leftmost_col -= 1;
        }
    }

    pub fn scroll_one_line_down(&mut self) {
        if self.top_line < self.max_top_line() {
            self.top_line += 1;
        }
    }

    pub fn scroll_one_col_right(&mut self) {
        if self.leftmost_col < self.max_leftmost_col() {
            self.leftmost_col += 1;
        }
    }

    pub fn scroll_one_screen_up(&mut self) {
        if self.top_line > self.max_nb_seq_shown() {
            self.top_line -= self.max_nb_seq_shown();
        } else {
            self.top_line = 0;
        }
    }

    pub fn scroll_one_screen_left(&mut self) {
        if self.leftmost_col > self.max_nb_col_shown() {
            self.leftmost_col -= self.max_nb_col_shown();
        } else {
            self.leftmost_col = 0;
        }
    }

    pub fn scroll_one_screen_down(&mut self) {
        if self.top_line + self.max_nb_seq_shown() < self.max_top_line() {
            self.top_line += self.max_nb_seq_shown();
        } else {
            self.top_line = self.max_top_line();
        }
    }

    pub fn scroll_one_screen_right(&mut self) {
        if self.leftmost_col + self.max_nb_col_shown() < self.max_leftmost_col() {
            self.leftmost_col += self.max_nb_col_shown();
        } else {
            self.leftmost_col = self.max_leftmost_col();
        }
    }

    pub fn scroll_zoombox_one_line_down(&mut self) {
        self.top_line += (1.0 / self.v_ratio()).round() as u16;
        if self.top_line > self.max_top_line() {
            self.top_line = self.max_top_line();
        }
    }

    pub fn scroll_zoombox_one_line_up(&mut self) {
        let lines_to_skip = (1.0 / self.v_ratio()).round() as u16;
        if lines_to_skip < self.top_line {
            self.top_line -= lines_to_skip;
        } else {
            self.top_line = 0;
        }
    }

    pub fn scroll_zoombox_one_col_right(&mut self) {
        self.leftmost_col += (1.0 / self.h_ratio()).round() as u16;
        if self.leftmost_col > self.max_leftmost_col() {
            self.leftmost_col = self.max_leftmost_col();
        }
    }

    pub fn scroll_zoombox_one_col_left(&mut self) {
        let cols_to_skip = (1.0 / self.h_ratio()).round() as u16;
        if cols_to_skip < self.leftmost_col {
            self.leftmost_col -= cols_to_skip;
        } else {
            self.leftmost_col = 0;
        }
    }

    pub fn jump_to_top(&mut self) {
        self.top_line = 0
    }

    pub fn jump_to_begin(&mut self) {
        self.leftmost_col = 0
    }

    pub fn jump_to_bottom(&mut self) {
        self.top_line = self.max_top_line()
    }

    pub fn jump_to_end(&mut self) {
        self.leftmost_col = self.max_leftmost_col()
    }

    // Debugging

    pub fn assert_invariants(&self) {
        if self.max_nb_col_shown() > self.app.aln_len() {
            assert!(self.max_leftmost_col() == 0);
        } else {
            assert!(
                self.max_leftmost_col() + self.max_nb_col_shown() == self.app.aln_len(),
                "l_max: {} + w_p: {} == w_a: {} failed",
                self.max_leftmost_col(),
                self.max_nb_col_shown(),
                self.app.aln_len()
            );
        }
        assert!(
            self.leftmost_col <= self.max_leftmost_col(),
            "l: {}<= l_max: {}",
            self.leftmost_col,
            self.max_leftmost_col()
        )
    }
}
