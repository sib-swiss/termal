// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use std::{
    fmt,
    fs::File,
    io::{stdout, BufRead, BufReader},
    time::{Duration, Instant},
};

use log::info;

use termal_alignment::seq::fasta::read_fasta_file;
use termal_alignment::seq::stockholm::read_stockholm_file;
use termal_alignment::Alignment;

use crate::app::App;
use crate::ui::{key_handling::handle_key_press, render::render_ui, UI};

use clap::{Parser, ValueEnum};

use crossterm::{
    event::{self, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::{CrosstermBackend, Rect, Terminal},
    TerminalOptions, Viewport,
};

use crate::errors::TermalError;

#[derive(Debug, Parser)]
#[command(name = "termal", version = env!("CARGO_PKG_VERSION"))]
#[command(arg_required_else_help = true)]
//#[command(version, about, long_about = None) ]
struct Cli {
    /// Alignment file
    aln_fname: Option<String>,

    /// Show key bindings and exit successfully
    #[arg(short = 'b', long = "show-bindings")]
    show_bindings: bool,

    /// Info mode (no TUI)
    #[arg(short, long)]
    info: bool,

    /// Sequence file format
    #[arg(short, long = "format", default_value_t = SeqFileFormat::FastA,
        help = "Sequence file format [fasta|stockholm] (or just f|s); default: fasta",
        hide_default_value = true,
        hide_possible_values = true,
    )]
    format: SeqFileFormat,

    /// User-supplied order (filename)
    #[arg(short = 'o', long)]
    user_order: Option<String>,

    /// Gecos color map
    #[arg(short, long = "color-map")]
    color_map: Option<String>,

    /// Fixed terminal width (mostly used for testing/debugging)
    #[arg(short, long, requires = "height")]
    width: Option<u16>,

    /// Fixed terminal height ("tall" -- -h is already used)
    #[arg(short = 't', long, requires = "width")]
    height: Option<u16>,

    /// Dry run - show parameters and quit
    #[arg(short = 'n', long)]
    dry_run: bool,

    // Rare options (long form only)
    /// Start with labels pane hidden
    #[arg(long, hide_short_help = true)]
    hide_labels_pane: bool,

    /// Start with bottom pane hidden
    #[arg(long, hide_short_help = true)]
    hide_bottom_pane: bool,

    /// (Currently no effect)
    #[arg(long, hide_short_help = true)]
    debug: bool,

    /// Switch to monochrome
    #[arg(long = "no-color", hide_short_help = true)]
    no_color: bool,

    /// Disable scrollbars (mostly for testing)
    #[arg(long = "no-scrollbars", hide_short_help = true)]
    no_scrollbars: bool,

    /// Poll wait time [ms]
    #[arg(long = "poll-wait-time", default_value_t = 50, hide_short_help = true)]
    poll_wait_time: u64,

    /// Panic (for testing)
    #[arg(long = "panic", hide_short_help = true)]
    panic: bool,

    // TODO: the ZB can be disabled at runtime (or at least it should)
    /// Do not show zoom box (zooming itself is not disabled)
    #[arg(long = "no-zoom-box", hide_short_help = true)]
    no_zoombox: bool,

    // TODO: this is only ever used when the bottom pane is at the bottom of the terminal, which is
    // practically never.
    //
    /// Do not show zoom box guides (only useful if zoom box not shown)
    #[arg(long = "no-zb-guides", hide_short_help = true)]
    no_zb_guides: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum SeqFileFormat {
    #[clap(name = "fasta")]
    #[clap(alias = "f")]
    FastA,
    #[clap(name = "stockholm")]
    #[clap(alias = "s")]
    Stockholm,
}

impl fmt::Display for SeqFileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SeqFileFormat::FastA => "fasta",
            SeqFileFormat::Stockholm => "stockholm",
        };
        write!(f, "{}", s)
    }
}

// pub fn read_fasta_file<P: AsRef<Path>>(path: P) -> Result<SeqFile, std::io::Error> {
fn read_user_ordering(fname: &str) -> Result<Vec<String>, std::io::Error> {
    let uord_file = File::open(fname)?;
    let reader = BufReader::new(uord_file);
    reader.lines().collect()
}

fn show_params(cli: &Cli, ui: &UI) {
    println!("Alignment file: {}", cli.aln_fname.as_ref().unwrap());
    println!("Alignment file format: {}", cli.format);
    if let Some(map_fname) = &cli.color_map {
        println!("User color map file: {}", map_fname);
        println!(
            "User color map: {}",
            ui.color_scheme().current_residue_colormap().map()
        );
    }
}

pub fn run() -> Result<(), TermalError> {
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

    if let Some(seq_filename) = &cli.aln_fname {
        let seq_file = match cli.format {
            SeqFileFormat::FastA => read_fasta_file(seq_filename)?,
            SeqFileFormat::Stockholm => read_stockholm_file(seq_filename)?,
        };
        let alignment = Alignment::from_file(seq_file);
        let mut ordering_err_msg: Option<String> = None;
        let mut user_ordering = match cli.user_order {
            Some(ref fname) => {
                // TODO: should be called from_path()
                let get_ord_vec = read_user_ordering(fname);
                match get_ord_vec {
                    Ok(ord_vec) => Some(ord_vec),
                    Err(_) => {
                        ordering_err_msg = Some(format!("Error reading ordering file {}", fname));
                        None // => App ignores bad user ordering
                    }
                }
            }
            None => None,
        };
        // Check for discrepancies beween the user-specied ordering and alignment headers. The two
        // sets should be identical.
        if let Some(ref ord_vec) = user_ordering {
            let mut uo_clone = ord_vec.clone();
            let mut ah_clone = alignment.headers.clone();
            uo_clone.sort();
            ah_clone.sort();
            if uo_clone != ah_clone {
                ordering_err_msg = Some(String::from("Discrepancies in ordering vs alignment"));
                // App must ignore bad user ordering
                user_ordering = None;
            }
        };
        let mut app = App::new(seq_filename, alignment, user_ordering);
        if let Some(msg) = ordering_err_msg {
            app.error_msg(msg);
        }

        if cli.info {
            info!("Running in debug mode.");
            app.output_info();
            return Ok(());
        }

        let mut app_ui = UI::new(&mut app);

        if let Some(path) = &cli.color_map {
            app_ui.add_user_colormap(path);
            app_ui.select_first_colormap();
        }

        if cli.dry_run {
            show_params(&cli, &app_ui);
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
            app_ui.set_left_pane_width(0);
        }
        if cli.hide_bottom_pane {
            app_ui.set_bottom_pane_height(0);
        }

        let poll_wait = Duration::from_millis(cli.poll_wait_time);
        let frame_interval = Duration::from_millis(50); // FIXME: constant or option
        let mut last_draw: Instant;

        terminal.draw(|f| render_ui(f, &mut app_ui))?;
        last_draw = Instant::now();

        // main loop
        loop {
            // Wait for an event (or timeout)
            // TODO: redraw only if 'dirty', i.e. visuals have changes (most keys, but not e.g.
            // when scrolling past a boundary (=> no change). Have handle_key_press() return (done,
            // dirty) (i.e. a tuple of booleans).
            //let mut dirty = true;
            if event::poll(poll_wait)? {
                match event::read()? {
                    event::Event::Key(key) if key.kind == KeyEventKind::Press => {
                        let done = handle_key_press(&mut app_ui, key);
                        if done {
                            break;
                        }

                        // Only draw if enough time has elapsed
                        if last_draw.elapsed() >= frame_interval {
                            terminal.draw(|f| render_ui(f, &mut app_ui))?;
                            last_draw = Instant::now();
                        }
                    }
                    event::Event::Resize(_, _) => {
                        terminal.draw(|f| render_ui(f, &mut app_ui))?;
                        last_draw = Instant::now();
                    }
                    _ => {}
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
