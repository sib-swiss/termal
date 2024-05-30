use std::collections::{HashMap, HashSet};

use ratatui::{
    Frame,
    prelude::{Color, Constraint, Direction, Layout, Line, Span},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

use crate::App;

pub struct UI {
    colour_map: HashMap<char, Color>, 
}

impl UI {
    pub fn new() -> Self {
        let colour_map = color_scheme_lesk();
        UI {
            colour_map,
        }
    }
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

// Draw UI

pub fn ui(f: &mut Frame, app: &mut App, app_ui: &mut UI) {
    let area = f.size();
    let nskip = app.leftmost_col.into();
    let ntake = (f.size().width - 2).into();
    let nseqskip: usize = app.top_line.into();
    let nseqtake: usize = f.size().height.into(); // whole frame's height, should take the sequence
                                                  // area; also should be named just
                                                  // 'seq_frame_height, or something.
    let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Fill(1), Constraint::Length(3)])
        .split(area);

    let title = format!(" {} - {}s x {}c ", app.filename, app.num_seq(), app.aln_len());
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let mut text: Vec<Line> = Vec::new();
    for seq in app.alignment.sequences.iter()
        .skip(nseqskip).take(nseqtake) {
        let spans: Vec<Span> = seq.chars()
            .skip(nskip).take(ntake)
            .map(|c| Span::styled(c.to_string(),
                *app_ui.colour_map.get(&c).unwrap()))
            .collect();
        let line: Line = Line::from(spans);
        text.push(line);
    }

    let ztitle = format!(" {} - {}s x {}c - fully zoomed out ", app.filename, app.num_seq(), app.aln_len());
    let zoom_block = Block::default().title(ztitle).borders(Borders::ALL);
    let mut ztext: Vec<Line> = Vec::new();
    let retained_seqs_ndx: Vec<usize> = every_nth(f.size().height.into(), nseqtake);
    let retained_cols_ndx: Vec<usize> = every_nth(f.size().width.into(), ntake);
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

    let seq_para = Paragraph::new(ztext)
    //let seq_para = Paragraph::new(text)
        .white()
        .block(aln_block);
    let msg_block = Block::default().borders(Borders::ALL);
    let msg_para = Paragraph::new(format!("{:?} nskip: {}, ntake: {}", layout[0].as_size(), nskip, ntake))
        .white()
        .block(msg_block);

    f.render_widget(seq_para, layout[0]);
    f.render_widget(msg_para, layout[1]);

    app.set_seq_para_height(layout[0].as_size().height - 2); // -2: borders
    app.set_seq_para_width(layout[0].as_size().width - 2);
}

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
