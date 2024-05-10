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

fn to_spans(line: &str) -> Vec<Span> {
    let spans = line.chars().map(|c| Span::raw(c.to_string())
                                            .green()
                                            ).collect();

    spans
}

fn line_aln(seqr: Vec<&str>) -> Vec<Line> {
    let line_aln: Vec<Line> = seqr
        .into_iter()
        .map(|l| {
            Line::from(to_spans(l))
        })
        .collect();
    line_aln
}

// Draw UI
pub fn ui(f: &mut Frame, app: &mut App) {
    let area = f.size();
    let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Fill(1), Constraint::Length(3)])
        .split(area);
    let title = format!(" {} ({} sequences of {} residues) ",
        app.filename.as_str(), app.num_seq(), app.aln_len());
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let srefs: Vec<&str> = app.alignment.sequences.iter().map(String::as_ref).collect();
    let seq_lines: Vec<Line> = line_aln(srefs);
    let text = Text::from(seq_lines);
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

