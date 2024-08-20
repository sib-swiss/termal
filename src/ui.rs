mod conservation;
mod color_scheme;

use std::collections::HashMap;

use bitflags::bitflags;

use log::{info,debug};

use ratatui::{
    Frame,
    prelude::{Color, Constraint, Direction, Layout, Line, Margin, Rect, Span, Text},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState},
};

use crate::{
    App,
    ui::conservation::values_barchart,
    ui::color_scheme::{
        color_scheme_lesk,
        color_scheme_monochrome,
    },
    vec_f64_aux::{
        normalize,
        ones_complement,
        product,
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
    // TODO: see about shortcuts in constructors
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

/*****************************************************************
 * Panel Texts
 *
 * for all zoom levels
*****************************************************************/

fn zoom_in_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    ui.app.alignment.headers.iter()
        .map(|h| Line::from(h.clone())).collect()
}

fn zoom_out_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let mut ztext: Vec<Line> = Vec::new();
    let num_seq: usize = ui.app.num_seq() as usize;
    let retained_seqs_ndx: Vec<usize> = every_nth(num_seq, ui.seq_para_height.into());
    for i in &retained_seqs_ndx {
        ztext.push(Line::from(ui.app.alignment.headers[*i].clone()));
    }

    ztext
}

fn zoom_in_seq_text<'a>(ui: &'a UI) -> Vec<Line<'a>> {
    let nskip: usize = ui.leftmost_col.into();
    let ntake: usize = ui.seq_para_width.into();
    let nseqskip: usize = ui.top_line.into();
    let nseqtake: usize = ui.seq_para_height.into(); 

    let top_i = ui.top_line as usize;
    let bot_i = (ui.top_line+ui.seq_para_height) as usize;
    let lft_j = ui.leftmost_col as usize; 
    let rgt_j = (ui.leftmost_col+ui.seq_para_width) as usize;

    let mut text: Vec<Line> = Vec::new();

    for i in top_i .. bot_i {
        if i >= ui.app.num_seq().into() { break; } // if there is extra vertical space
        let mut spans: Vec<Span> = Vec::new();
        for j in lft_j .. rgt_j {
            if j >= ui.app.aln_len().into() { break; } // ", horizontal
            let cur_seq_ref = &ui.app.alignment.sequences[i];
            let cur_char = (*cur_seq_ref).as_bytes()[j] as char;
            spans.push(Span::styled(cur_char.to_string(), *ui.colour_map.get(&cur_char).unwrap()));
        }
        text.push(Line::from(spans));
    }

    text
}

fn zoom_out_seq_text<'a>(area: Rect, ui: &UI) -> Vec<Line<'a>> {
    let num_seq: usize = ui.app.num_seq() as usize;
    let aln_len: usize = ui.app.aln_len() as usize;
    // TODO: use UI members - seq_para_{width,height}
    let seq_area_width: usize = (area.width - 2).into();  // -2 <- panel border
    let seq_area_height: usize = (area.height - 2).into(); // "
    let mut ztext: Vec<Line> = Vec::new();
    let retained_seqs_ndx: Vec<usize> = every_nth(num_seq, seq_area_height);
    let retained_cols_ndx: Vec<usize> = every_nth(aln_len, seq_area_width);
    for i in &retained_seqs_ndx {
        let seq: &String = &ui.app.alignment.sequences[*i];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in &retained_cols_ndx {
            let c: char = seq_chars[*j];
            let span = Span::styled(c.to_string(),
                                    *ui.colour_map.get(&c).unwrap());
            spans.push(span);
        }
        ztext.push(Line::from(spans));
    }

    ztext
}

// Draws the zoombox (just overwrites the sequence area with box-drawing characters).
//
fn mark_zoombox(seq_para: &mut Vec<Line>, area: Rect, ui: &UI) {

    let vb_top:    usize = ((ui.top_line as f64) * ui.v_ratio()).round() as usize;
    let mut vb_bottom: usize = (((ui.top_line + ui.seq_para_height) as f64) * ui.v_ratio()).round() as usize;
    // If h_a < h_p
    if vb_bottom > ui.app.num_seq() as usize {
        vb_bottom = ui.app.num_seq() as usize;
    }

    let vb_left:   usize = ((ui.leftmost_col as f64) * ui.h_ratio()).round() as usize;
    let mut vb_right:  usize = (((ui.leftmost_col + ui.seq_para_width) as f64) * ui.h_ratio()).round() as usize;
    // If w_a < w_p
    if vb_right > ui.app.aln_len() as usize {
        vb_right = ui.app.aln_len() as usize;
    }
    debug!("w_a: {}, w_p: {}, r_h: {}", ui.app.aln_len(), ui.seq_para_width, ui.h_ratio());
    ui.assert_invariants();

    let mut l: &mut Line = &mut seq_para[vb_top];
    for c in vb_left+1 .. vb_right {
        let _ = std::mem::replace(&mut (*l).spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut (*l).spans[vb_left], Span::raw("┌"));
    let _ = std::mem::replace(&mut (*l).spans[vb_right-1], Span::raw("┐"));
    for s in vb_top+1 .. vb_bottom {
        l = &mut seq_para[s];
        let _ = std::mem::replace(&mut (*l).spans[vb_left], Span::raw("│"));
        let _ = std::mem::replace(&mut (*l).spans[vb_right-1], Span::raw("│"));
    }
    l = &mut seq_para[vb_bottom-1];
    for c in vb_left+1 .. vb_right {
        let _ = std::mem::replace(&mut (*l).spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut (*l).spans[vb_left], Span::raw("└"));
    let _ = std::mem::replace(&mut (*l).spans[vb_right-1], Span::raw("┘"));
}

/****************************************************************
* Layout
****************************************************************/

struct Panes {
    sequence: Rect,
    labels: Rect,
    bottom: Rect,
    corner: Rect,
}

fn make_layout(f: &Frame, ui: &UI) -> Panes {
    let constraints: Vec<Constraint> = vec![
        Constraint::Fill(1),
        Constraint::Max(ui.bottom_pane_height.unwrap()),
    ];
    let v_panes = Layout::new(
            Direction::Vertical,
            constraints)
        .split(f.size());
    let upper_panes = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Max(ui.label_pane_width.unwrap()), Constraint::Fill(1)])
        .split(v_panes[0]);
    let lower_panes = Layout::new(
            Direction::Horizontal,
            vec![Constraint::Max(ui.label_pane_width.unwrap()), Constraint::Fill(1)])
        .split(v_panes[1]);

   Panes {
       labels: upper_panes[0],
       sequence: upper_panes[1],
       corner: lower_panes[0],
       bottom: lower_panes[1],
   }
}

// Ticks and tick marks for bottom pane

fn tick_marks(aln_length: usize) -> String {
    let mut ticks = String::with_capacity(aln_length);
    for i in 0 .. aln_length {
        ticks.push(
            if i % 10 == 0 { '|' } else { ' ' }
            );
    }

    ticks
}

fn tick_position(aln_length: usize) -> String {
    let mut intervals: Vec<String> = vec![String::from("0")];
    let mut tens = 10;
    while tens < aln_length {
        let int = format!("{:>10}", tens);
        tens += 10;
        intervals.push(int);
    }
    intervals.join("")
}

/****************************************************************
// Draw UI
****************************************************************/

fn compute_title(ui: &UI) -> String {
    let title: String;
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            title = format!(" {} - {}s x {}c ", ui.app.filename, ui.app.num_seq(), ui.app.aln_len());
        }
        ZoomLevel::ZoomedOut => {
            title = format!(" {} - {}s x {}c - fully zoomed out ", ui.app.filename, ui.app.num_seq(), ui.app.aln_len());
        }
        ZoomLevel::ZoomedOutAR => todo!()
    }

    title
}

fn compute_sequence_pane_text<'a>(frame_size: Rect, ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let mut sequences: Vec<Line>;

    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            sequences = zoom_in_seq_text(ui);
        }
        ZoomLevel::ZoomedOut => {
            sequences = zoom_out_seq_text(frame_size, ui);
            if ui.show_zoombox { mark_zoombox(&mut sequences, frame_size, ui); }
        }
        ZoomLevel::ZoomedOutAR => todo!()
    }

    sequences
}

fn compute_labels_pane_text<'a>(ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let labels: Vec<Line>;
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            labels = zoom_in_lbl_text(ui);
        }
        ZoomLevel::ZoomedOut => {
            labels = zoom_out_lbl_text(ui);
        }
        ZoomLevel::ZoomedOutAR => todo!()
    }

    labels
}

fn render_labels_pane(f: &mut Frame, seq_chunk: Rect, ui: &UI) {
    /* Labels pane */
    let labels = compute_labels_pane_text(ui);
    let lbl_block = Block::default()
        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);
    let top_lbl_line = match ui.zoom_level() {
        ZoomLevel::ZoomedIn => ui.top_line,
        ZoomLevel::ZoomedOut => 0,
        ZoomLevel::ZoomedOutAR => todo!(),
    };
    let lbl_para = Paragraph::new(labels)
        .white()
        .scroll((top_lbl_line, 0))
        .block(lbl_block);
    f.render_widget(lbl_para, seq_chunk);
}

// TODO: the "sequences" pane should be called "alignment" pane.

fn render_alignment_pane(f: &mut Frame, aln_chunk: Rect, ui: &UI) {
    /* Sequence pane */
    let title = compute_title(ui);
    let seq = compute_sequence_pane_text(f.size(), ui);
    //debug!("showing {} sequences", sequences.len());
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let seq_para = Paragraph::new(seq)
        .white()
        .block(aln_block);
    f.render_widget(seq_para, aln_chunk);
    //f.render_widget(Paragraph::default(), layout_panes.sequence);
    debug!("h_z: {}", every_nth(ui.app.num_seq().into(), ui.seq_para_height.into()).len());

    if ui.zoom_level == ZoomLevel::ZoomedIn
        && ui.show_scrollbars
        && ui.seq_para_height > 2 {

        // vertical scrollbar
        let mut v_scrollbar_state = ScrollbarState::default()
            .content_length((ui.app.num_seq() - ui.seq_para_height ).into())
            .viewport_content_length((ui.seq_para_height - 2).into())
            .position(ui.top_line.into());
        debug!("v_bar: {:#?}", v_scrollbar_state);
        debug!("t_max: {}", ui.max_top_line());
        let v_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);
        f.render_stateful_widget(
            v_scrollbar,
            aln_chunk.inner(&Margin{
                vertical: 1,
                horizontal: 0,
            }),
            &mut v_scrollbar_state);

        // horizontal scrollbar
        let mut h_scrollbar_state = ScrollbarState::default()
            .content_length((ui.app.aln_len() - ui.seq_para_width ).into())
            .viewport_content_length((ui.seq_para_width - 2).into())
            .position(ui.leftmost_col.into());
        let h_scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
            .begin_symbol(None)
            .end_symbol(None);
        f.render_stateful_widget(
            h_scrollbar,
            aln_chunk.inner(&Margin{
                vertical: 0,
                horizontal: 1,
            }),
            &mut h_scrollbar_state);
    }
}

fn render_corner_pane(f: &mut Frame, corner_chunk: Rect) {
    let corner_block = Block::default()
        .borders(Borders::LEFT | Borders::BOTTOM);
    let corner_text = Text::from(vec![
        "Consensus".into(),
        "Conservation".into(),
        "".into(), "Position".into()]);
    let corner_para = Paragraph::new(corner_text)
        .block(corner_block);
    f.render_widget(corner_para, corner_chunk);
}

fn render_bottom_pane(f: &mut Frame, bottom_chunk: Rect, ui: &UI) {
    let btm_block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);
    let mut btm_text: Vec<Line> = Vec::new();
    btm_text.push(Line::from(ui.app.alignment.consensus.clone()));
    btm_text.push(Line::from(
            values_barchart(&product(
                    &ui.app.alignment.densities,
                    &ones_complement(&normalize(&ui.app.alignment.entropies))
                ))));
    btm_text.push(Line::from(tick_marks(ui.app.aln_len() as usize)));
    btm_text.push(Line::from(tick_position(ui.app.aln_len() as usize)));
    let btm_para = Paragraph::new(btm_text)
        .scroll((0, ui.leftmost_col))
        .block(btm_block);
    f.render_widget(btm_para, bottom_chunk);
}

pub fn ui(f: &mut Frame, ui: &mut UI) {
    let layout_panes = make_layout(f, ui);

    debug!("seq pane size (w/ borders): {:?}", layout_panes.sequence.as_size());
    // TODO: do we really need to store this? Or could code within this function just look up the
    // size of the sequence pane? Review the variables in UI and make sure they're all needed;
    // replace those that ar enot by locals in this function.
    ui.set_seq_para_height(layout_panes.sequence.as_size().height); // the f() takes care of
                                                                    // borders!
    ui.set_seq_para_width(layout_panes.sequence.as_size().width);
    // Handle resizing
    ui.adjust_seq_pane_position();
    ui.frame_width = Some(f.size().width);
    ui.frame_height = Some(f.size().height);

    ui.assert_invariants();

    /* Render panes */
    render_labels_pane(f, layout_panes.labels, ui);
    render_alignment_pane(f, layout_panes.sequence, ui);
    render_corner_pane(f, layout_panes.corner);
    render_bottom_pane(f, layout_panes.bottom, ui);
}

/* Computes n indexes out of l. The indexes are as evenly spaced as possible, and always include
 * the first (0) and last (l-1) indexes. If n >= l, then return 0 .. l. */

pub fn every_nth(l: usize, n: usize) -> Vec<usize> {
    if n >= l {
        (0..l).collect()
    } else {
        let step: f32 = (l-1) as f32 / (n-1) as f32;
        let r: Vec<usize> = (0..n).map(|e| ((e as f32) * step).round() as usize).collect();
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::{every_nth};

    #[test]
    fn test_every_nth_1() {
        assert_eq!(vec![0,4,8], every_nth(9,3));
    }

    #[test]
    fn test_every_nth_2() {
        assert_eq!(vec![0,5,9], every_nth(10,3));
    }

    #[test]
    fn test_every_nth_3() {
        assert_eq!(vec![0,1,2,3,4], every_nth(5,5));
    }

    #[test]
    fn test_every_nth_4() {
        assert_eq!(vec![0,1,2,3,4], every_nth(5,10));
    }
}
