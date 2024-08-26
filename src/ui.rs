mod conservation;
mod color_scheme;
pub mod render;

use std::collections::HashMap;

use bitflags::bitflags;

use log::{debug};

use ratatui::{
    layout::Size,
    prelude::{
        Color,
    },
};

use crate::{
    App,
    ui::color_scheme::{
        color_scheme_lesk,
        color_scheme_monochrome,
    },
};

#[derive(Clone,Copy,PartialEq)]
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
    show_scrollbars: bool,
    top_line: u16,
    leftmost_col: u16,
    label_pane_width: u16,
    bottom_pane_height: u16,
    // These cannot be known when the structure is initialized, so they are Options -- but it is
    // possible that they need not be stored at all, as they can in principle be computed when the
    // layout is known.
    aln_pane_size: Option<Size>,
    // Whole app
    // TODO: instead of passing the frame's width and height separately, pass them as a single
    // Option<Size>, just like aln_pane_size.
    frame_width: Option<u16>,
    frame_height: Option<u16>,
}

impl<'a> UI<'a> {

    pub fn new(app: &'a App) -> Self {
        UI {
            app,
            colour_map: color_scheme_lesk(),
            zoom_level: ZoomLevel::ZoomedIn,
            show_zoombox: true,
            show_scrollbars: true,
            top_line: 0,
            leftmost_col: 0,
            label_pane_width: 15,     // Reasonable default, I'd say...
            bottom_pane_height: 5,
            aln_pane_size: None,
            frame_width: None,
            frame_height: None,
        }
    }

    // **************************************************************** 
    /*
     * Dimensions
     *
     * The layout determines the maximal number of sequences and columns shown; this in turn
     * affects the maximal top line and leftmost column, etc.
     * */

    fn seq_para_height(& self) -> u16 {
        let height = self.aln_pane_size.unwrap().height;
        if height >= 2 {  // border, should later be a constant or a field of UI
             height - 2
        } else {
            // Set to null (prevents display) if not enough room
             0
        }
    }

    fn seq_para_width(& self) -> u16 {
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
        if self.leftmost_col > self.max_leftmost_col() { self.leftmost_col = self.max_leftmost_col(); }
        if self.top_line > self.max_top_line() { self.top_line = self.max_top_line(); }
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
        self.label_pane_width = if self.label_pane_width + amount < self.frame_width.unwrap() {
            self.label_pane_width + amount
        } else {
            self.frame_width.unwrap()
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

    pub fn zoom_level(&self) -> ZoomLevel { self.zoom_level }

    pub fn cycle_zoom(&mut self) {
        // Don't zoom out if the whole aln fits on screen
        if self.aln_wrt_seq_pane() == AlnWRTSeqPane::Fits {
            self.zoom_level =  ZoomLevel::ZoomedIn;
        } else {
            self.zoom_level = match self.zoom_level {
                ZoomLevel::ZoomedIn  => ZoomLevel::ZoomedOut,
                ZoomLevel::ZoomedOut => ZoomLevel::ZoomedIn,
                ZoomLevel::ZoomedOutAR => ZoomLevel::ZoomedIn,
                // TODO: OUT -> OUT_AR
            }
        }
    }

    pub fn h_ratio(&self) -> f64 {
        (self.seq_para_width() as f64 / self.app.aln_len() as f64) as f64
    }

    pub fn v_ratio(&self) -> f64 {
        (self.seq_para_height() as f64 / self.app.num_seq() as f64) as f64
    }

    pub fn set_zoombox(&mut self, state: bool) { self.show_zoombox = state; }

    // **************************************************************** 
    // Color scheme

    pub fn set_monochrome(&mut self) {
        self.colour_map = color_scheme_monochrome();
    }

    // **************************************************************** 

    pub fn disable_scrollbars(&mut self) { self.show_scrollbars = false; }



    // Scrolling

    pub fn scroll_one_line_up(&mut self) {
        if self.top_line > 0 { self.top_line -= 1; }
    }

    pub fn scroll_one_col_left(&mut self) {
        if self.leftmost_col > 0 { self.leftmost_col -= 1; }
    }

    pub fn scroll_one_line_down(&mut self) {
      if self.top_line < self.max_top_line() { self.top_line += 1; }
    }

    pub fn scroll_one_col_right(&mut self) {
        if self.leftmost_col < self.max_leftmost_col() { self.leftmost_col += 1; }
    }

    pub fn scroll_one_screen_up(&mut self) {
       if self.top_line > self.seq_para_height()  {
           self.top_line -= self.seq_para_height();
       } else {
           self.top_line = 0;
       }
    }

    pub fn scroll_one_screen_left(&mut self) {
       if self.leftmost_col > self.seq_para_width()  {
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
        if self.top_line > self.max_top_line() { self.top_line = self.max_top_line(); }
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
        if self.leftmost_col > self.max_leftmost_col() { self.leftmost_col = self.max_leftmost_col(); }
    }

    pub fn scroll_zoombox_one_col_left(&mut self) {
        let cols_to_skip = (1.0 / self.h_ratio()).round() as u16;
        if cols_to_skip < self.leftmost_col {
            self.leftmost_col -= cols_to_skip;
        } else {
            self.leftmost_col = 0; 
        }
    }

    pub fn jump_to_top(&mut self) { self.top_line = 0 }

    pub fn jump_to_begin(&mut self) { self.leftmost_col = 0 }

    pub fn jump_to_bottom(&mut self) { self.top_line = self.max_top_line() }

    pub fn jump_to_end(&mut self) { self.leftmost_col = self.max_leftmost_col() }

    // Debugging

    pub fn assert_invariants(&self) {
        debug!("w_a: {}, w_p: {}", self.app.aln_len(), self.seq_para_width());
        debug!("h_a: {}, h_p: {}", self.app.num_seq(), self.seq_para_height());
        if self.seq_para_width() > self.app.aln_len() {
            assert!(self.max_leftmost_col() == 0);
        } else {
            assert!(self.max_leftmost_col() + self.seq_para_width() == self.app.aln_len(),
                "l_max: {} + w_p: {} == w_a: {} failed",
                self.max_leftmost_col(), self.seq_para_width(), self.app.aln_len()
            );
        }
        assert!(self.leftmost_col <= self.max_leftmost_col(), 
            "l: {}<= l_max: {}", self.leftmost_col, self.max_leftmost_col())
    }
}
