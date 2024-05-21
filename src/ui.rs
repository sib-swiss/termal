use ratatui::{
    Frame,
    prelude::{Buffer, Color, Constraint, Direction, Layout, Line, Rect, Span, Text},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

use crate::App;

pub struct UI {
    // TODO: maybe use a Hash of Colors indexed by (x,y) tuples?
    res_colours: Vec<Vec<Color>>,
}

impl UI {
    pub fn new(app: &App) -> Self {
        let mut res_colours = Vec::new();
        for line in app.alignment.sequences.iter() {
            let line_res_colours: Vec<Color> = line.chars().map(aa_color_scheme_lesk).collect();
            res_colours.push(line_res_colours);
        }
        UI { res_colours }
    }
}

fn aa_color_scheme_lesk(aa: char) -> Color {
    match aa {
        'G' | 'A' | 'S' | 'T' => Color::Rgb(255, 165, 0),
        'C' | 'V' | 'I' | 'L' | 'P' | 'F' | 'Y' | 'M' | 'W' => Color::Green,
        'N' | 'Q' | 'H' => Color::Magenta,
        'D' | 'E' => Color::Red,
        'K' | 'R' => Color::Blue,
        '-' => Color::Gray,
        _   => Color::Gray,
    }
}

// Draw UI
pub fn ui(f: &mut Frame, app: &mut App, app_ui: &mut UI) {
    let mut nskip: usize = 0;
    let mut ntake: usize = 0;
    let area = f.size();
    let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Fill(1), Constraint::Length(3)])
        .split(area);
    let title = format!("(s{}, c{}) - (s{}, c{}) ",
        app.top_line, app.leftmost_col, 
        app.top_line + f.size().height, app.leftmost_col + f.size().width);
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let nseqskip: usize = app.top_line.into();
    let nseqtake: usize = f.size().height.into(); // whole frame's height, should take the sequence area
                                                  // instead
    let text: Vec<Line> = app.alignment.sequences.iter()
        .skip(nseqskip).take(nseqtake)
        .map(|l| {
        nskip = app.leftmost_col.into();
        ntake = (f.size().width - 2).into();
        let lref: Vec<Span> = l.chars()
            .skip(nskip).take(ntake)
            .map(|c| Span::styled(c.to_string(), aa_color_scheme_lesk(c))).collect();
        Line::from(lref)
    }).collect();

    let mut ztext: Vec<Line> = Vec::new();
    let lzip = app.alignment.sequences.iter().zip(
        app_ui.res_colours.iter());
    for (seq, seq_colours) in lzip {
        let spans: Vec<Span> = seq.chars().zip(seq_colours.iter())
            .map(|(chr, col)| Span::styled(chr.to_string(), *col))
            .collect();
        ztext.push(Line::from(spans));
    }

    // TODO: rewrite above loop so that colour is _looked up_ in UI.res_colours instead of
    // computed.
    let seq_para = Paragraph::new(ztext)
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

