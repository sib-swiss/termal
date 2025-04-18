// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier
mod alignment;
mod app;
mod ui;
mod vec_f64_aux;

use log::{debug, info};

use std::io::{stdout, Result};
//use std::process::exit;

use clap::{arg, command, Parser};

use crossterm::{
    event::{self, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Rect, Terminal},
    TerminalOptions, Viewport,
};

use crate::app::App;
use crate::ui::{
    color_map::colormap_gecos,
    key_handling::handle_key_press,
    render::render_ui,
    {ZoomLevel, UI},
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None) ]
struct Cli {
    /// Alignment file
    aln_fname: Option<String>,

    /// Show key bindings and exit successfully
    #[arg(short = 'b', long = "show-bindings")]
    show_bindings: bool,

    /// Info mode (no TUI)
    #[arg(short, long)]
    info: bool,

    /// Gecos color map
    #[arg(short, long = "color-map")]
    color_map: Option<String>,

    /// Fixed terminal width (mostly used for testing/debugging)
    #[arg(short, long, requires = "height")]
    width: Option<u16>,

    /// Fixed terminal height ("tall" -- -h is already used)
    #[arg(short = 't', long, requires = "width")]
    height: Option<u16>,

    /// Start with labels pane hidden
    #[arg(short = 'L', long)]
    hide_labels_pane: bool,

    /// Start with bottom pane hidden
    #[arg(short = 'B', long)]
    hide_bottom_pane: bool,

    /// (Currently no effect)
    #[arg(short = 'D', long)]
    debug: bool,

    /// Disable color
    #[arg(short = 'C', long = "no-color")]
    no_color: bool,

    /// Disable scrollbars (mostly for testing)
    #[arg(long = "no-scrollbars")]
    no_scrollbars: bool,

    /// Poll wait time [ms]
    #[clap(long = "poll-wait-time", default_value_t = 100)]
    poll_wait_time: u64,

    /// Panic (for testing)
    #[clap(long = "panic")]
    panic: bool,

    /// Do not show zoom box (zooming itself is not disabled)
    #[arg(long = "no-zoom-box")]
    no_zoombox: bool,

    /// Do not show zoom box guides (only useful if zoom box not shown)
    #[arg(long = "no-zb-guides")]
    no_zb_guides: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting log");

    let cli = Cli::parse();
    if cli.panic {
        panic!("User-requested panic");
    }

    if cli.show_bindings {
        println!("{}", include_str!("ui/bindings.md"));
        return Ok(());
    }

    if let Some(fasta_file) = &cli.aln_fname {
        let mut app = App::new(fasta_file)?;

            if cli.info {
                info!("Running in debug mode.");
                app.output_info();
                return Ok(());
            }

            stdout().execute(EnterAlternateScreen)?;
            enable_raw_mode()?;

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

            let mut app_ui = UI::new(&mut app);
            if cli.no_scrollbars {
                app_ui.disable_scrollbars();
            }
            if cli.no_color {
                app_ui.set_monochrome();
            }
            if cli.no_zoombox {
                app_ui.set_zoombox(false);
            }
            if cli.no_zb_guides {
                app_ui.set_zoombox_guides(false);
            }
            if cli.hide_labels_pane {
                app_ui.set_label_pane_width(0);
            }
            if cli.hide_bottom_pane {
                app_ui.set_bottom_pane_height(0);
            }

            if let Some(path) = cli.color_map {
                let _cmap = colormap_gecos(path.into());
                // TODO: add to colormaps (and remove the underscore)
            }

            // main loop
            loop {
                debug!("\n**** Draw Iteration ****");
                debug!("terminal size: {:?}", terminal.size().unwrap());
                terminal.draw(|f| render_ui(f, &mut app_ui))?;
                // handle events
                if event::poll(std::time::Duration::from_millis(cli.poll_wait_time))? {
                    if let event::Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            // handle_key_press() returns true IFF user quits
                            let done = handle_key_press(&mut app_ui, key);
                            if done {
                                break;
                            }
                        }
                    }
                }
            }

            stdout().execute(LeaveAlternateScreen)?;
            disable_raw_mode()?;

            Ok(())
        } else {
            panic!("Expected filename argument");
        }
    }
