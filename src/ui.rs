use std::collections::HashMap;

use log::{info,debug};

use ratatui::{
    Frame,
    prelude::{Color, Constraint, Direction, Layout, Line, Rect, Span},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

use crate::App;

#[derive(Clone,Copy)]
pub enum ZoomLevel {
    ZoomedIn,
    ZoomedOut,
    ZoomedOutAR,
}

// TODO: do we really need a separate UI struct, or could this just go into the App?
//
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

    pub fn zoom_level(&self) -> ZoomLevel { self.zoom_level }

    pub fn cycle_zoom(&mut self) {
        self.zoom_level = match self.zoom_level {
            ZoomLevel::ZoomedIn => ZoomLevel::ZoomedOut,
            ZoomLevel::ZoomedOut => ZoomLevel::ZoomedIn,
            ZoomLevel::ZoomedOutAR => ZoomLevel::ZoomedIn,
            // TODO: OUT -> OUT_AR
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

    pub fn set_seq_para_height(&mut self, height: u16) { self.seq_para_height = height; }

    pub fn set_seq_para_width(&mut self, width: u16) { self.seq_para_width = width; }

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
        self.label_pane_width = if self.label_pane_width.unwrap() + amount < self.seq_para_width {
            Some(self.label_pane_width.unwrap() + amount)
        } else {
            Some(self.seq_para_width)
        }
    }

    pub fn reduce_label_pane(&mut self, amount: u16) {
        // TODO: heed the border width (not sure if we'll keep them)
        self.label_pane_width = if self.label_pane_width.unwrap() - amount > 0 {
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
    let nseqtake: usize = ui.seq_para_width.into(); // FIXME should be HEIGHT!!!
    let mut text: Vec<Line> = Vec::new();
    // TODO: we probably don't need to skip() and then take(): why not just access elements
    // directly, as is done in mark_zoombox() ? See also zoom_out_seq_text().
    for seq in ui.app.alignment.sequences.iter()
        .skip(nseqskip).take(nseqtake) {
        let spans: Vec<Span> = seq.chars()
            .skip(nskip).take(ntake)
            .map(|c| Span::styled(c.to_string(), *ui.colour_map.get(&c).unwrap()))
            .collect();
        let line: Line = Line::from(spans);
        text.push(line);
    }

    text
}

fn zoom_out_seq_text<'a>(area: Rect, ui: &UI) -> Vec<Line<'a>> {
    let num_seq: usize = ui.app.num_seq() as usize;
    let aln_len: usize = ui.app.aln_len() as usize;
    // TODO: use UI members
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
            // NOTE: I don't want to iterate through all chars in seq until I find the j-th: this
            // is going to be much too slow. 
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
    let vb_bottom: usize = (((ui.top_line + ui.seq_para_height) as f64) * ui.v_ratio()).round() as usize;
    let vb_left:   usize = ((ui.leftmost_col as f64) * ui.h_ratio()).round() as usize;
    let vb_right:  usize = (((ui.leftmost_col + ui.seq_para_width) as f64) * ui.h_ratio()).round() as usize;

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

// Draw UI

pub fn ui(f: &mut Frame, ui: &mut UI) {
    let layout_panes = make_layout(f, ui);

    debug!("seq pane size: {:?}", layout_panes.sequence.as_size());
    ui.set_seq_para_height(layout_panes.sequence.as_size().height - 2); // -2: borders
    ui.set_seq_para_width(layout_panes.sequence.as_size().width - 2);
    ui.adjust_seq_pane_position();

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

    if ui.show_debug_pane {
        let msg_block = Block::default().borders(Borders::ALL);
        let msg_para = Paragraph::new(format!("{:?}", layout_panes.sequence.as_size()))
            .white()
            .block(msg_block);
        f.render_widget(msg_para, layout_panes.bottom);
    }
}

/* Computes n indexes out of l. The indexes are as evenly spaced as possible, and always include
 * the first (0) and last (l-1) indexes. */

pub fn every_nth(l: usize, n: usize) -> Vec<usize> {
    let step: f32 = (l-1) as f32 / (n-1) as f32;
    let r: Vec<usize> = (0..n).map(|e| ((e as f32) * step).round() as usize).collect();
    r
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
}
