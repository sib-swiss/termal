use std::collections::{HashMap, HashSet};

use ratatui::{
    Frame,
    prelude::{Color, Constraint, Direction, Layout, Line, Rect, Span},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

use crate::App;

enum ZoomLevel {
    ZOOMED_IN,
    ZOOMED_OUT,
    ZOOMED_OUT_AR,
}

// TODO: do we really need a separate UI struct, or could this just go into the App?
//
pub struct UI {
    colour_map: HashMap<char, Color>, 
    zoom_level: ZoomLevel,
    show_debug_pane: bool,
}

impl UI {
    pub fn new() -> Self {
        let colour_map = color_scheme_lesk();
        let zoom_level = ZoomLevel::ZOOMED_IN;
        let show_debug_pane = false;
        UI {
            colour_map,
            zoom_level,
            show_debug_pane,
        }
    }

    pub fn cycle_zoom(&mut self) {
        self.zoom_level = match self.zoom_level {
            ZoomLevel::ZOOMED_IN => ZoomLevel::ZOOMED_OUT,
            ZoomLevel::ZOOMED_OUT => ZoomLevel::ZOOMED_IN,
            ZoomLevel::ZOOMED_OUT_AR => ZoomLevel::ZOOMED_IN,
            // TODO: OUT -> OUT_AR
        }
    }

    pub fn set_debug(&mut self, state: bool) {
        self.show_debug_pane = state;
    }

    pub fn set_monochrome(&mut self) {
        self.colour_map = color_scheme_monochrome();
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

fn zoom_in_seq_text<'a>(area: Rect, app: &'a App, app_ui: &'a UI) -> Vec<Line<'a>> {
    let nskip: usize = app.leftmost_col.into();
    let ntake: usize = (area.width - 2).into();
    let nseqskip: usize = app.top_line.into();
    let nseqtake: usize = area.height.into(); 
    let mut text: Vec<Line> = Vec::new();
    for seq in app.alignment.sequences.iter()
        .skip(nseqskip).take(nseqtake) {
        let spans: Vec<Span> = seq.chars()
            .skip(nskip).take(ntake)
            .map(|c| Span::styled(c.to_string(), *app_ui.colour_map.get(&c).unwrap()))
            .collect();
        let line: Line = Line::from(spans);
        text.push(line);
    }

    text
}

fn zoom_out_seq_text<'a>(area: Rect, app: &'a App, app_ui: &UI) -> Vec<Line<'a>> {
    let num_seq: usize = app.num_seq() as usize;
    let aln_len: usize = app.aln_len() as usize;
    let seq_area_width: usize = (area.width - 2).into();  // -2 <- panel border
    let seq_area_height: usize = (area.height - 2).into(); // "
    let mut ztext: Vec<Line> = Vec::new();
    let retained_seqs_ndx: Vec<usize> = every_nth(num_seq, seq_area_height);
    let retained_cols_ndx: Vec<usize> = every_nth(aln_len, seq_area_width);
    for i in &retained_seqs_ndx {
        let seq: &String = &app.alignment.sequences[*i];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in &retained_cols_ndx {
            // NOTE: I don't want to iterate through all chars in seq intil I find the j-th: this
            // is going to be much too slow. 
            let c: char = seq_chars[*j];
            let span = Span::styled(c.to_string(),
                                    *app_ui.colour_map.get(&c).unwrap());
            spans.push(span);
        }
        ztext.push(Line::from(spans));
    }

    ztext
}

fn make_layout(show_debug_pane: bool) -> Layout {
    /*
     *  One approach is to add the debug pane only if requested; another is to set its height to 3
     * IFF requested.
     */
    let mut constraints: Vec<Constraint> = vec![Constraint::Fill(1)];
    if show_debug_pane {
        constraints.push(Constraint::Length(3));
    }
    let layout = Layout::new(
            Direction::Vertical,
            constraints);

    layout
}

// Draw UI

pub fn ui(f: &mut Frame, app: &mut App, app_ui: &mut UI) {
    let layout_panes = make_layout(app_ui.show_debug_pane)
        .split(f.size());

    let mut text: Vec<Line> = Vec::new();
    let title: String;
    match app_ui.zoom_level {
        ZoomLevel::ZOOMED_IN => {
            title = format!(" {} - {}s x {}c ", app.filename, app.num_seq(), app.aln_len());
            text = zoom_in_seq_text(f.size(), app, app_ui);
        }
        ZoomLevel::ZOOMED_OUT => {
            title = format!(" {} - {}s x {}c - fully zoomed out ", app.filename, app.num_seq(), app.aln_len());
            text = zoom_out_seq_text(f.size(), app, app_ui);
        }
        ZoomLevel::ZOOMED_OUT_AR => todo!()
    }

    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let seq_para = Paragraph::new(text)
        .white()
        .block(aln_block);

    // let port_box = 
    f.render_widget(seq_para, layout_panes[0]);

    app.set_seq_para_height(layout_panes[0].as_size().height - 2); // -2: borders
    app.set_seq_para_width(layout_panes[0].as_size().width - 2);

    if app_ui.show_debug_pane {
        let msg_block = Block::default().borders(Borders::ALL);
        let msg_para = Paragraph::new(format!("{:?}", layout_panes[0].as_size()))
            .white()
            .block(msg_block);
        f.render_widget(msg_para, layout_panes[1]);
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
