mod app;
mod alignment;

use std::io::{stdout, Result};
use std::env;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    Frame,
    prelude::{Color, Constraint, CrosstermBackend, Direction,
        Layout, Line, Span, Style, Stylize, Terminal, Text},
    widgets::{Block,Borders,Paragraph},
};

use crate::app::App;

fn make_span(c: &str) -> Span {
    Span::styled(c, Style::default().fg(Color::Red))
}

// Draw UI
fn ui(f: &mut Frame, app: &mut App) {
    let area = f.size();
    let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Fill(1), Constraint::Length(3)])
        .split(area);
    let title = format!(" {} ({} sequences of {} residues) ",
        app.filename.as_str(), app.num_seq(), app.aln_len());
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let line_aln: Vec<Line> = app.alignment.sequences
        .iter()
        .map(|l| Line::from(String::as_str(l)))
        .collect();
    let text = Text::from(line_aln);
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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let fasta_file: &str = &args.get(1).expect("Expecting 1 arg");

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new(fasta_file);

    // main loop
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => app.scroll_one_line_down(),
                        KeyCode::Char('G') => app.jump_to_bottom(),

                        KeyCode::Char('k') => app.scroll_one_line_up(),
                        KeyCode::Char('g') => app.jump_to_top(),

                        KeyCode::Char('l') => app.scroll_one_col_right(),
                        KeyCode::Char('$') => app.jump_to_end(),

                        KeyCode::Char('h') => app.scroll_one_col_left(),
                        KeyCode::Char('^') => app.jump_to_begin(),

                        KeyCode::Char('q') => break,
                        KeyCode::Char('Q') => break,
                        _ => {}
                    }
                }
            }
        }
    }
    
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
