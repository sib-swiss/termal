use ratatui::{
    Frame,
    prelude::{Constraint, Direction, Layout, Line, Span, Text},
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
};

use crate::App;

pub struct UI<'a> {
    aln_para: Paragraph<'a>
}

// Draw UI
pub fn ui(f: &mut Frame, app: &mut App) {
    let area = f.size();
    let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Fill(1), Constraint::Length(3)])
        .split(area);
    let title = format!(" {} ({} sequences of {} residues), showing (s{}, c{}) - (s{}, c{}) ",
        app.filename.as_str(), app.num_seq(), app.aln_len(),
        app.top_line, app.leftmost_col, 
        app.top_line + f.size().width, app.leftmost_col + f.size().height);
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let nseqskip: usize = app.top_line.into();
    let nseqtake: usize = f.size().height.into();
    let text: Vec<Line> = app.alignment.sequences.iter()
        .skip(nseqskip).take(nseqtake)
        .map(|l| {
        // TODO: inject the coordinates based on top_line, leftmost_col, panel width, and panel
        // height.
        let nskip: usize = app.leftmost_col.into();
        let ntake: usize = f.size().width.into();
        let lref: Vec<Span> = l.chars()
            .skip(nskip).take(ntake)
            .map(|c| Span::raw(c.to_string())).collect();
        Line::from(lref)
    }).collect();
    let seq_para = Paragraph::new(text)
        .white()
        .block(aln_block)
        .scroll((app.top_line, app.leftmost_col));
    let msg_block = Block::default().borders(Borders::ALL);
    let msg_para = Paragraph::new(format!("{:?}", layout[0].as_size()))
        .white()
        .block(msg_block);
    f.render_widget(seq_para, layout[0]);
    f.render_widget(msg_para, layout[1]);

    app.set_seq_para_height(layout[0].as_size().height - 2); // -2: borders
    app.set_seq_para_width(layout[0].as_size().width - 2);
}

