mod app;
mod ui;
mod alignment;
mod vec_f64_aux;

use log::{info,debug};

use std::{
    io::{stdout, Result},
};

use clap::{arg, command, Parser, };

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Rect, Terminal},
    TerminalOptions, Viewport,
};

use crate::app::App;
use crate::ui::{
    {UI, ZoomLevel},
    render::render_ui,
};

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

    /// Start with labels pane hidden
    #[arg(short='L', long,)]
    hide_labels_pane: bool,

    /// Start with bottom pane hidden
    #[arg(short='B', long,)]
    hide_bottom_pane: bool,

    /// (Currently no effect)
    #[arg(short='D', long)]
    debug: bool,

    /// Disable colour
    #[arg(short='C', long="no-colour")]
    no_colour: bool,

    /// Disable scrollbars (mostly for testing)
    #[arg(long="no-scrollbars")]
    no_scrollbars: bool,

    /// Poll wait time [ms]
    #[clap(long="poll-wait-time", default_value_t = 100)]
    poll_wait_time: u64,

    /// Panic (for testing)
    #[clap(long="panic")]
    panic: bool,

    /// Do not show zoom box (zooming itself is not disabled)
    #[arg(long="no-zoom-box")]
    no_zoombox: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting log");

    let cli = Cli::parse(); 
    if cli.panic { panic!("User-requested panic"); }

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

    let app = App::new(fasta_file)?;
    let mut app_ui = UI::new(&app);
    if cli.no_scrollbars { app_ui.disable_scrollbars(); }
    if cli.no_colour { app_ui.set_monochrome(); }
    if cli.no_zoombox { app_ui.set_zoombox(false); }
    if cli.hide_labels_pane { app_ui.set_label_pane_width(0); }
    if cli.hide_bottom_pane { app_ui.set_bottom_pane_height(0); }

    // main loop
    loop {
        debug!("**** Draw Iteration ****");
        debug!("terminal size: {:?}", terminal.size().unwrap());
        terminal.draw(|f|  render_ui(f, &mut app_ui) )?;
        // handle events
        if event::poll(std::time::Duration::from_millis(cli.poll_wait_time))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                        match key.code {
                            // ----- Motion -----
                            
                            // Down
                            KeyCode::Char('j') => match app_ui.zoom_level() {
                                ZoomLevel::ZoomedIn => app_ui.scroll_one_line_down(),
                                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR=> app_ui.scroll_zoombox_one_line_down(),
                            }
                            KeyCode::Char('J') => app_ui.scroll_one_screen_down(),
                            KeyCode::Char('G') => app_ui.jump_to_bottom(),

                            // Up
                            KeyCode::Char('k') => match app_ui.zoom_level() {
                                ZoomLevel::ZoomedIn => app_ui.scroll_one_line_up(),
                                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => app_ui.scroll_zoombox_one_line_up(),
                            }
                            KeyCode::Char('K') => app_ui.scroll_one_screen_up(),
                            KeyCode::Char('g') => app_ui.jump_to_top(),

                            // Right
                            KeyCode::Char('l') => match app_ui.zoom_level() {
                                ZoomLevel::ZoomedIn => app_ui.scroll_one_col_right(),
                                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => app_ui.scroll_zoombox_one_col_right(),
                            }
                            KeyCode::Char('L') => app_ui.scroll_one_screen_right(),
                            KeyCode::Char('$') => app_ui.jump_to_end(),

                            // Left
                            KeyCode::Char('h') => match app_ui.zoom_level() {
                                ZoomLevel::ZoomedIn => app_ui.scroll_one_col_left(),
                                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => app_ui.scroll_zoombox_one_col_left(),
                            }
                            KeyCode::Char('H') => app_ui.scroll_one_screen_left(),
                            KeyCode::Char('^') => app_ui.jump_to_begin(),

                            // Label Pane width
                            // NOTE: for these methods I'm using a more general approach than for
                            // motion: pass the argument instead of having separate functions for
                            // each increment.
                            KeyCode::Char('>') => app_ui.widen_label_pane(1),
                            KeyCode::Char('<') => app_ui.reduce_label_pane(1),

                            // Zoom
                            KeyCode::Char('z') => app_ui.cycle_zoom(),
                            // Since there are 3 zoom levels, cycling twice amounts to cycling
                            // backwards.
                            KeyCode::Char('Z') => {
                                app_ui.cycle_zoom();
                                app_ui.cycle_zoom();
                            }

                            // Mark consensus positions that are retained in the zoom box
                            KeyCode::Char('r') => app_ui.toggle_hl_retained_cols(),
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
