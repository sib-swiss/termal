mod conservation;
mod color_scheme;
pub mod render;

use std::collections::HashMap;

use bitflags::bitflags;

use log::{debug};

use ratatui::{
    prelude::Color,
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

// A bit field that denotes if the alignment is too wide (with respect to the sequence panel), to
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

// TODO see about keeping members private, especially those that should not be set without a bundary
// check (such as top_line).

pub struct UI<'a> {
    app: &'a App,
    colour_map: HashMap<char, Color>, 
    zoom_level: ZoomLevel,
    show_debug_pane: bool,
    show_zoombox: bool,
    show_scrollbars: bool,
    top_line: u16,
    leftmost_col: u16,
    seq_para_width: u16,
    seq_para_height: u16,
    // No meaningful value can be given to signal that `label_pane_width` still needs to be
    // initalised: 0 is a valid width (when hiding the pane), and -1 (or any other negative number)
    // is invalid due to the unsigned type. I could (i) use a signed integer (but all negative
    // values except -1 would be wasted) or (ii) use an Option. Let's try that.
    label_pane_width: Option<u16>,
    bottom_pane_height: Option<u16>,
    // Whole app
    frame_width: Option<u16>,
    frame_height: Option<u16>,
}

impl<'a> UI<'a> {
    pub fn new(app: &'a App) -> Self {
        UI {
            app,
            colour_map: color_scheme_lesk(),
            zoom_level: ZoomLevel::ZoomedIn,
            show_debug_pane: false,
            show_zoombox: true,
            show_scrollbars: true,
            top_line: 0,
            leftmost_col: 0,
            seq_para_width: 0,
            seq_para_height: 0,
            label_pane_width: None,
            bottom_pane_height: None,
            frame_width: None,
            frame_height: None,
        }
    }

    // Handling Resizes

    // Resizing (as when the user resizes the terminal window where Termal runs) affects
    // max_top_line and max_leftmost_col (because the number of available lines (resp. columns)
    // will generally change), so top_line and leftmost_col may now exceed them. This function,
    // which should be called after the layout is solved but before the widgets are drawn, makes
    // sure that l does not exceed l_max, etc.

    pub fn adjust_seq_pane_position(&mut self) {
        if self.leftmost_col > self.max_leftmost_col() { self.leftmost_col = self.max_leftmost_col(); }
        if self.top_line > self.max_top_line() { self.top_line = self.max_top_line(); }
    }

    // Zooming

    // This functions determines if the alignment fits on the screen or is too wide or tall (it can
    // be both).
    // TODO: might be an inner function of cycle_zoom, as it is not used anywhere else.
    fn aln_wrt_seq_pane(&self) -> AlnWRTSeqPane {
        let mut rel = AlnWRTSeqPane::Fits;
        if self.app.aln_len() > self.seq_para_width {
            rel |= AlnWRTSeqPane::TooWide;
        }
        if self.app.num_seq() > self.seq_para_height {
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
        (self.seq_para_width as f64 / self.app.aln_len() as f64) as f64
    }

    pub fn v_ratio(&self) -> f64 {
        (self.seq_para_height as f64 / self.app.num_seq() as f64) as f64
    }

    pub fn set_debug(&mut self, state: bool) {
        self.show_debug_pane = state;
    }

    pub fn set_monochrome(&mut self) {
        self.colour_map = color_scheme_monochrome();
    }

    pub fn set_zoombox(&mut self, state: bool) { self.show_zoombox = state; }

    pub fn disable_scrollbars(&mut self) { self.show_scrollbars = false; }

    // Update size (must be done after layout is solved) - this is layout-agnostic, i.e. the
    // height is the actual number of lines displayable in the sequence widget, after taking into
    // account its size, the presence of borders, etc.
    // TODO: arguably, functions could just look up the sizes of chunks after layout is done, so
    // perhaps this function isn't necessary.

    pub fn set_seq_para_height(&mut self, height: u16) {
        self.seq_para_height = if height >= 2 {  // border, should later be a constant or a field of UI
             height - 2
        } else {
             0
        }
    }

    pub fn set_seq_para_width(&mut self, width: u16) {
        self.seq_para_width = if width >= 2 {
            width - 2
        } else {
            0
        }
    }

    // Bounds 
    
    fn max_top_line(&self) -> u16 {
        if self.app.num_seq() >= self.seq_para_height {
            self.app.num_seq() - self.seq_para_height
        } else {
            0
        }
    }

    fn max_leftmost_col(&self) -> u16 {
        if self.app.aln_len() >= self.seq_para_width {
            self.app.aln_len() - self.seq_para_width
        } else {
            0
        }
    }


    // Side panel dimensions
    
    pub fn set_label_pane_width(&mut self, width: u16) {
        self.label_pane_width = Some(width);
    }

    pub fn set_bottom_pane_height(&mut self, height: u16) {
        self.bottom_pane_height = Some(height);
    }

    pub fn widen_label_pane(&mut self, amount: u16) {
        // TODO: heed the border width (not sure if we'll keep them)
        self.label_pane_width = if self.label_pane_width.unwrap() + amount < self.frame_width.unwrap() {
            Some(self.label_pane_width.unwrap() + amount)
        } else {
            Some(self.frame_width.unwrap())
        }
    }

    pub fn reduce_label_pane(&mut self, amount: u16) {
        // TODO: heed the border width (not sure if we'll keep them)
        self.label_pane_width = if self.label_pane_width.unwrap() > amount {
            Some(self.label_pane_width.unwrap() - amount)
        } else {
            Some(0)
        }
    }

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
       if self.top_line > self.seq_para_height  {
           self.top_line -= self.seq_para_height;
       } else {
           self.top_line = 0;
       }
    }

    pub fn scroll_one_screen_left(&mut self) {
       if self.leftmost_col > self.seq_para_width  {
           self.leftmost_col -= self.seq_para_width;
       } else {
           self.leftmost_col = 0;
       }
    }

    pub fn scroll_one_screen_down(&mut self) {
       if self.top_line + self.seq_para_height < self.max_top_line() {
           self.top_line += self.seq_para_height;
       } else {
           self.top_line = self.max_top_line();
       }
    }

    pub fn scroll_one_screen_right(&mut self) {
       if self.leftmost_col + self.seq_para_width < self.max_leftmost_col() {
           self.leftmost_col += self.seq_para_width;
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
        debug!("w_a: {}, w_p: {}", self.app.aln_len(), self.seq_para_width);
        debug!("h_a: {}, h_p: {}", self.app.num_seq(), self.seq_para_height);
        if self.seq_para_width > self.app.aln_len() {
            assert!(self.max_leftmost_col() == 0);
        } else {
            assert!(self.max_leftmost_col() + self.seq_para_width == self.app.aln_len(),
                "l_max: {} + w_p: {} == w_a: {} failed",
                self.max_leftmost_col(), self.seq_para_width, self.app.aln_len()
            );
        }
        assert!(self.leftmost_col <= self.max_leftmost_col(), 
            "l: {}<= l_max: {}", self.leftmost_col, self.max_leftmost_col())
    }
}
