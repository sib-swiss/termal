mod app;
mod ui;
mod alignment;


use std::{
    env,
    fs::File,
    io::{stdout, Result, Write},
};

use clap::{arg, command, Parser, };

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen, SetSize,
    },
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Rect, Terminal},
    TerminalOptions, Viewport,
};

use crate::app::App;
use crate::ui::{UI, ui};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Alignment file
    aln_fname: String,

    /// Fixed terminal width (mostly used for testing/debugging)
    #[arg(short, long, requires="height")]
    width: Option<u16>,

    /// Fixed terminal height ("tall" -- -h is already used)
    #[arg(short='t', long, requires="width")]
    height: Option<u16>,

    /// Show debug panel
    #[arg(short='D', long)]
    debug: bool,

    /// Disable colour
    #[arg(short='C')]
    no_colour: bool,

    /// Disable viewport
    #[arg(long="no-viewport")]
    no_viewport: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse(); 
    let fasta_file: &str = &cli.aln_fname;

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    //let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let backend = CrosstermBackend::new(stdout());
    let viewport: Viewport;
    // Fix viewport dimensions IFF supplied (mainly for tests)
    //
    if let Some(width) = cli.width {
        // height must be defined too (see 'requires' in struct Cli above)
        let height = cli.height.unwrap();
        viewport = Viewport::Fixed(Rect::new(0, 0, width, height));
    } else {
        viewport = Viewport::Fullscreen;
    }
    let mut terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;
    terminal.clear()?;

    let mut app = App::new(fasta_file)?;
    let mut app_ui = UI::new();
    app_ui.set_debug(cli.debug);
    if cli.no_colour { app_ui.set_monochrome(); }
    if cli.no_viewport { app_ui.set_viewport(false); }

    // main loop
    loop {
        terminal.draw(|f| ui(f, &mut app, &mut app_ui))?;
        // handle events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        // Motion
                        KeyCode::Char('j') => app.scroll_one_line_down(),
                        KeyCode::Char('G') => app.jump_to_bottom(),

                        KeyCode::Char('k') => app.scroll_one_line_up(),
                        KeyCode::Char('g') => app.jump_to_top(),

                        KeyCode::Char('l') => app.scroll_one_col_right(),
                        KeyCode::Char('$') => app.jump_to_end(),

                        KeyCode::Char('h') => app.scroll_one_col_left(),
                        KeyCode::Char('^') => app.jump_to_begin(),

                        // Zoom
                        KeyCode::Char('z') => app_ui.cycle_zoom(),

                        // Exit
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
