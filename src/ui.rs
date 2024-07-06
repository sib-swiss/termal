mod conservation;

use std::collections::HashMap;
use bitflags::bitflags;

use log::{info,debug};

use ratatui::{
    Frame,
    prelude::{Color, Constraint, Direction, Layout, Line, Rect, Span},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    App,
    ui::conservation::values_barchart,
    vec_f64_aux::{
        normalize,
        ones_complement,
        product,
    },
};

#[derive(Clone,Copy)]
pub enum ZoomLevel {
    ZoomedIn,
    ZoomedOut,
    ZoomedOutAR,
}

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
    show_debug_pane: bool,
    show_zoombox: bool,
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
        let colour_map = color_scheme_lesk();
        let zoom_level = ZoomLevel::ZoomedIn;
        let show_debug_pane = false;
        let show_zoombox = true;
        let top_line = 0;
        let leftmost_col = 0;
        let seq_para_width = 0;
        let seq_para_height = 0;
        let label_pane_width = None;
        let bottom_pane_height = None;
        let frame_width = None;
        let frame_height = None;
        UI {
            app,
            colour_map,
            zoom_level,
            show_debug_pane,
            show_zoombox,
            top_line,
            leftmost_col,
            seq_para_width,
            seq_para_height,
            label_pane_width,
            bottom_pane_height,
            frame_width,
            frame_height,
        }
    }

    // Handling Resizes

    // Resizing affects max_top_line and max_leftmost_col, so top_line and leftmost_col may now
    // exceed them. This function, which should be called after the layout is solved but before the
    // widgets are drawn, makes sure that l does not exceed l_max, etc.

    pub fn adjust_seq_pane_position(&mut self) {
        if self.leftmost_col > self.max_leftmost_col() { self.leftmost_col = self.max_leftmost_col(); }
        if self.top_line > self.max_top_line() { self.top_line = self.max_top_line(); }
    }

    // Zooming

    pub fn aln_wrt_seq_pane(&self) -> AlnWRTSeqPane {
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
    //
    // Setting size (must be done after layout is solved) - this is layout-agnostic, i.e. the
    // height is th aactual number of lines displayable in the sequence widget, after taking into
    // account its size, the presence of borders, etc.

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

// It's prolly easier to have a no-op colorscheme than to decide at every iteration if we do a
// lookup or not.


fn color_scheme_monochrome() -> HashMap<char, Color> {
    let map = HashMap::from([
        ('G', Color::White),
        ('A', Color::White),
        ('S', Color::White),
        ('T', Color::White),
        ('C', Color::White),
        ('V', Color::White),
        ('I', Color::White),
        ('L', Color::White),
        ('P', Color::White),
        ('F', Color::White),
        ('Y', Color::White),
        ('M', Color::White),
        ('W', Color::White),
        ('N', Color::White),
        ('Q', Color::White),
        ('H', Color::White),
        ('D', Color::White),
        ('E', Color::White),
        ('K', Color::White),
        ('R', Color::White),
        ('-', Color::White),
        ]);
    map
}

fn color_scheme_lesk() -> HashMap<char, Color> {
    let orange = Color::Rgb(255, 165, 0);
    let map = HashMap::from([
        ('G', orange),
        ('A', orange),
        ('S', orange),
        ('T', orange),
        ('C', Color::Green),
        ('V', Color::Green),
        ('I', Color::Green),
        ('L', Color::Green),
        ('P', Color::Green),
        ('F', Color::Green),
        ('Y', Color::Green),
        ('M', Color::Green),
        ('W', Color::Green),
        ('N', Color::Magenta),
        ('Q', Color::Magenta),
        ('H', Color::Magenta),
        ('D', Color::Red),
        ('E', Color::Red),
        ('K', Color::Blue),
        ('R', Color::Blue),
        ('-', Color::Gray),
        ]);
    map
}

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

fn mark_zoombox(seq_para: &mut Vec<Line>, area: Rect, ui: &mut UI) {

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


// Draw UI

pub fn ui(f: &mut Frame, ui: &mut UI) {
    let layout_panes = make_layout(f, ui);

    debug!("seq pane size: {:?}", layout_panes.sequence.as_size());
    ui.set_seq_para_height(layout_panes.sequence.as_size().height); // the f() takes care of
                                                                    // borders!
    ui.set_seq_para_width(layout_panes.sequence.as_size().width);
    ui.adjust_seq_pane_position();
    ui.frame_width = Some(f.size().width);
    ui.frame_height = Some(f.size().height);

    ui.assert_invariants();

    let labels;
    let mut sequences;
    let title: String;
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            title = format!(" {} - {}s x {}c ", ui.app.filename, ui.app.num_seq(), ui.app.aln_len());
            labels = zoom_in_lbl_text(ui);
            sequences = zoom_in_seq_text(ui);
        }
        ZoomLevel::ZoomedOut => {
            title = format!(" {} - {}s x {}c - fully zoomed out ", ui.app.filename, ui.app.num_seq(), ui.app.aln_len());
            labels = zoom_out_lbl_text(ui);
            sequences = zoom_out_seq_text(f.size(), ui);
            if ui.show_zoombox { mark_zoombox(&mut sequences, f.size(), ui); }
        }
        ZoomLevel::ZoomedOutAR => todo!()
    }
    debug!("showing {} sequences", sequences.len());

    let lbl_block = Block::default().borders(Borders::ALL);
    let top_lbl_line = match ui.zoom_level() {
        ZoomLevel::ZoomedIn => ui.top_line,
        ZoomLevel::ZoomedOut => 0,
        ZoomLevel::ZoomedOutAR => todo!(),
    };
    let lbl_para = Paragraph::new(labels)
        .white()
        .scroll((top_lbl_line, 0))
        .block(lbl_block);
    f.render_widget(lbl_para, layout_panes.labels);

    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let seq_para = Paragraph::new(sequences)
        .white()
        .block(aln_block);
    f.render_widget(seq_para, layout_panes.sequence);

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
    f.render_widget(btm_para, layout_panes.bottom);
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
