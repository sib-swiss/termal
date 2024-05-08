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
    prelude::{Color, CrosstermBackend, Stylize,
        Line, Span, Style, Terminal, Text},
    widgets::{Block,Borders,Paragraph},
};

use crate::app::App;

fn make_span(c: &str) -> Span {
    Span::styled(c, Style::default().fg(Color::Red))
}

// Draw UI
fn ui(f: &mut Frame, app: &App) {
    let area = f.size();
    let title = format!(" {} ", app.filename.as_str());
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let line_aln: Vec<Line> = app.alignment.sequences
        .iter()
        .map(|l| Line::from(String::as_str(l)))
        .collect();
    let text = Text::from(line_aln);
    let para = Paragraph::new(text)
        .white()
        .block(aln_block)
        .scroll((app.top_line, app.leftmost_col));
    f.render_widget(
        para,
        area,
    );
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
        terminal.draw(|f| ui(f, &app))?;
        // handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('j') => { app.top_line += 1; },
                        // KeyCode::Char('k') => { app.top_line -= 1; },
                        KeyCode::Char('k') => app.scroll_one_line_up(),
                        KeyCode::Char('l') => { app.leftmost_col += 1; },
                        KeyCode::Char('h') => app.scroll_one_col_left(),
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
