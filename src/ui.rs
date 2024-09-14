mod color_scheme;
mod conservation;
pub mod render;
pub mod key_handling;

use std::{
    cmp::min,
    collections::HashMap
};

use log::debug;

use bitflags::bitflags;

use ratatui::{layout::Size, prelude::Color};

use crate::{
    ui::color_scheme::{color_scheme_lesk, color_scheme_monochrome},
    App,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZoomLevel {
    ZoomedIn,
    ZoomedOut,
    ZoomedOutAR,
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
    app: &'a App,
    colour_map: HashMap<char, Color>,
    zoom_level: ZoomLevel,
    // What to show
    show_zoombox: bool,
    show_zb_guides: bool,
    show_scrollbars: bool,
    highlight_retained_cols: bool,
    top_line: u16,
    leftmost_col: u16,
    label_pane_width: u16,
    bottom_pane_height: u16,
    // These cannot be known when the structure is initialized, so they are Options -- but it is
    // possible that they need not be stored at all, as they can in principle be computed when the
    // layout is known.
    aln_pane_size: Option<Size>,
    frame_size: Option<Size>, // whole app
}

impl<'a> UI<'a> {
    pub fn new(app: &'a App) -> Self {
        UI {
            app,
            colour_map: color_scheme_lesk(),
            zoom_level: ZoomLevel::ZoomedIn,
            show_zoombox: true,
            show_zb_guides: true,
            show_scrollbars: true,
            highlight_retained_cols: false,
            top_line: 0,
            leftmost_col: 0,
            label_pane_width: 15, // Reasonable default, I'd say...
            bottom_pane_height: 5,
            aln_pane_size: None,
            frame_size: None,
        }
    }

    // ****************************************************************
    /*
     * Dimensions
     *
     * The layout determines the maximal number of sequences and columns shown; this in turn
     * affects the maximal top line and leftmost column, etc.
     * */

    fn seq_para_height(&self) -> u16 {
        let height = self.aln_pane_size.unwrap().height;
        if height >= 2 {
            // border, should later be a constant or a field of UI
            height - 2
        } else {
            // Set to null (prevents display) if not enough room
            0
        }
    }

    fn seq_para_width(&self) -> u16 {
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
        if self.app.num_seq() >= self.seq_para_height() {
            self.app.num_seq() - self.seq_para_height()
        } else {
            0
        }
    }

    fn max_leftmost_col(&self) -> u16 {
        if self.app.aln_len() >= self.seq_para_width() {
            self.app.aln_len() - self.seq_para_width()
        } else {
            0
        }
    }
    //
    // Side panel dimensions

    pub fn set_label_pane_width(&mut self, width: u16) {
        self.label_pane_width = width;
    }

    pub fn set_bottom_pane_height(&mut self, height: u16) {
        self.bottom_pane_height = height;
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

    // ****************************************************************
    // Zooming

    // Determines if the alignment fits on the screen or is too wide or tall (it can be both).
    // TODO: might be an inner function of cycle_zoom, as it is not used anywhere else.
    fn aln_wrt_seq_pane(&self) -> AlnWRTSeqPane {
        let mut rel = AlnWRTSeqPane::Fits;
        if self.app.aln_len() > self.seq_para_width() {
            rel |= AlnWRTSeqPane::TooWide;
        }
        if self.app.num_seq() > self.seq_para_height() {
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
        self.seq_para_width() as f64 / self.app.aln_len() as f64
    }

    pub fn v_ratio(&self) -> f64 {
        self.seq_para_height() as f64 / self.app.num_seq() as f64
    }

    pub fn set_zoombox(&mut self, state: bool) {
        self.show_zoombox = state;
    }

    // TODO: do we really need seq_para_len? Or can we just use self.app.num_seq?
    pub fn zoombox_top(&self, seq_para_len: usize) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                let zb_top = ((self.top_line as f64) * self.v_ratio()).round() as usize;
                // Rounding can push zb_top to seq_para_len, if zoom box has zero height
                if zb_top >= seq_para_len {
                    seq_para_len - 1
                } else {
                    zb_top
                }
            },
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.h_ratio().min(self.v_ratio());
                /* IN AR mode, the height of the alignment paragraph is the smallest of (i) the
                 * number of retained sequences (which are in seq_para), and (ii) the alignment
                 * panel's height. */
                let aln_para_height = min(seq_para_len as u16, self.seq_para_height());
                let zb_top = ((self.top_line as f64) * ratio).round() as usize;
                // Rounding can push zb_top to aln_para_height, if zoom box has zero height
                if zb_top >= aln_para_height.into() {
                    (aln_para_height - 1).into()
                } else {
                    zb_top
                }
            },
            _ => panic!("zoombox_top() should not be called in {:?} mode\n", self.zoom_level),
        }
    }

    pub fn zoombox_bottom(&self, seq_para_len: usize) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                let mut zb_bottom: usize =
                    (((self.top_line + self.seq_para_height()) as f64) * self.v_ratio()).round() as usize;
                // If h_a < h_p
                if zb_bottom > self.app.num_seq() as usize {
                    zb_bottom = self.app.num_seq() as usize;
                }
                zb_bottom
            },
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.h_ratio().min(self.v_ratio());
                let aln_para_height = min(seq_para_len as u16, self.seq_para_height());
                let mut zb_bottom = (((self.top_line + self.seq_para_height()) as f64) * ratio).round() as usize;
                // If h_a < h_p
                if zb_bottom > aln_para_height as usize {
                    zb_bottom = aln_para_height as usize;
                }
                zb_bottom
            },
            _ => panic!("zoombox_bottom() should not be called in {:?} mode\n", self.zoom_level),
        }
    }

    pub fn zoombox_left(&self) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                ((self.leftmost_col as f64) * self.h_ratio()).round() as usize
            },
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.h_ratio().min(self.v_ratio());
                ((self.leftmost_col as f64) * ratio).round() as usize
            },
            _ => panic!("zoombox_left() should not be called in {:?} mode\n", self.zoom_level),
        }
    }

    pub fn zoombox_right(&self, seq_para_width_ar: usize) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                let mut zb_right =
                    (((self.leftmost_col + self.seq_para_width()) as f64) * self.h_ratio()).round() as usize;
                // If w_a < w_p
                if zb_right > self.app.aln_len() as usize {
                    zb_right = self.app.aln_len() as usize;
                }
                zb_right
            },
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.h_ratio().min(self.v_ratio());
                let aln_para_width = min(seq_para_width_ar as u16, self.seq_para_width());
                let mut zb_right =
                    (((self.leftmost_col + self.seq_para_width()) as f64) * ratio).round() as usize;
                // If w_a < w_p
                if zb_right > aln_para_width as usize {
                    zb_right = aln_para_width as usize;
                }
                zb_right
            },
            _ => panic!("zoombox_left() should not be called in {:?} mode\n", self.zoom_level),
        }
    }

    pub fn toggle_hl_retained_cols(&mut self) {
        self.highlight_retained_cols = !self.highlight_retained_cols;
    }

    // ****************************************************************
    // Color scheme

    pub fn set_monochrome(&mut self) {
        self.colour_map = color_scheme_monochrome();
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
        if self.top_line > self.seq_para_height() {
            self.top_line -= self.seq_para_height();
        } else {
            self.top_line = 0;
        }
    }

    pub fn scroll_one_screen_left(&mut self) {
        if self.leftmost_col > self.seq_para_width() {
            self.leftmost_col -= self.seq_para_width();
        } else {
            self.leftmost_col = 0;
        }
    }

    pub fn scroll_one_screen_down(&mut self) {
        if self.top_line + self.seq_para_height() < self.max_top_line() {
            self.top_line += self.seq_para_height();
        } else {
            self.top_line = self.max_top_line();
        }
    }

    pub fn scroll_one_screen_right(&mut self) {
        if self.leftmost_col + self.seq_para_width() < self.max_leftmost_col() {
            self.leftmost_col += self.seq_para_width();
        } else {
            self.leftmost_col = self.max_leftmost_col();
        }
    }

    pub fn scroll_zoombox_one_line_down(&mut self) {
        self.top_line += (1.0 / self.v_ratio()).round() as u16;
        debug!(
            "top_line: {} (max: {})\n",
            self.top_line,
            self.max_top_line()
        );
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
        // debug!("w_a: {}, w_p: {}", self.app.aln_len(), self.seq_para_width());
        // debug!("h_a: {}, h_p: {}", self.app.num_seq(), self.seq_para_height());
        if self.seq_para_width() > self.app.aln_len() {
            assert!(self.max_leftmost_col() == 0);
        } else {
            assert!(
                self.max_leftmost_col() + self.seq_para_width() == self.app.aln_len(),
                "l_max: {} + w_p: {} == w_a: {} failed",
                self.max_leftmost_col(),
                self.seq_para_width(),
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
