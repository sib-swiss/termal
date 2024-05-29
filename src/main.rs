mod app;
mod ui;
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
    prelude::{CrosstermBackend, Terminal},
};

use crate::app::App;
use crate::ui::{UI, ui};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let fasta_file: &str = &args.get(1).expect("Expecting 1 arg");

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new(fasta_file);
    let mut app_ui = UI::new();

    // main loop
    loop {
        terminal.draw(|f| ui(f, &mut app, &mut app_ui))?;
        // handle events
        if event::poll(std::time::Duration::from_millis(16))? {
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
