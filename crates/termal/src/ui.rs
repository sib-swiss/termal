// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier
mod aln_widget;
mod barchart;
pub mod color_map;
mod color_scheme;
pub mod key_handling;
mod msg_theme;
pub mod render;
mod style;
mod zoombox;

use std::{
    cmp::{max, min},
    fmt,
};

use bitflags::bitflags;

use ratatui::layout::Size;
use ratatui::style::{Color, Style};

use self::{
    color_map::colormap_gecos,
    color_scheme::{ColorScheme, Theme},
};

use crate::{
    app::{App, JumpTarget},
    seq_match::SequenceSearchTarget,
};

const V_SCROLLBAR_WIDTH: u16 = 1;
const MIN_COLS_SHOWN: u16 = 1;
const BORDER_WIDTH: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZoomLevel {
    ZoomedIn,
    ZoomedOut,
    ZoomedOutAR,
}

// FIXME: this is obsolete, was never documented, and takes up space. Discard it.
#[derive(Debug)]
enum BottomPanePosition {
    Adjacent,
    ScreenBottom,
}

#[derive(Clone, Copy, PartialEq)]
enum VideoMode {
    Direct,
    Inverse,
}

#[derive(Clone, Debug, PartialEq)]
enum InputMode {
    Normal,
    Help,
    PendingCount {
        count: usize,
    },
    LabelSearch {
        pattern: String,
    },
    Search {
        pattern: String,
        target: SequenceSearchTarget,
    },
    SetReference {
        ref_spec: String,
    },
    PaneCmdPrefix,
    DiffCmdPrefix,
    SearchCmdPrefix,
    // ExCommand { buffer: String },
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct HelpState {
    top_line: u16,
    leftmost_col: u16,
}

impl fmt::Display for VideoMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VideoMode::Direct => "Dir",
            VideoMode::Inverse => "Inv",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy)]
pub enum DiffMode {
    Original,   // sequence residue/gap
    DiffWRTRef, // residue IFF != ref, else gap
}

// A bit field that denotes if the alignment is too wide (with respect to the sequence panel), too
// tall, both, or neither.

bitflags! {
    #[derive(PartialEq)]
    pub struct AlnWRTSeqPane: u8 {
        const Fits           = 0b00;
        const TooTall        = 0b01;
        const TooWide        = 0b10;
        const TooTallAndWide = 0b11;
    }
}

pub struct UI<'a> {
    app: &'a mut App,
    color_schemes: Vec<ColorScheme>,
    current_color_scheme_index: usize,
    zoom_level: ZoomLevel,
    show_zoombox: bool,
    //zoombox_color: Style,
    show_zb_guides: bool,
    show_scrollbars: bool,
    highlight_retained_cols: bool,
    top_line: u16,
    leftmost_col: u16,
    left_pane_width: u16,
    previous_left_pane_width: u16, // To restore width after hiding pane
    bottom_pane_height: u16,
    previous_bottom_pane_height: u16,
    bottom_pane_position: BottomPanePosition,
    // These cannot be known when the structure is initialized, so they are Options -- but it is
    // possible that they need not be stored at all, as they can in principle be computed when the
    // layout is known.
    aln_pane_size: Option<Size>,
    frame_size: Option<Size>, // whole app
    full_screen: bool,
    video_mode: VideoMode,
    input_mode: InputMode,
    help_state: HelpState,
    diff_mode: DiffMode,
}

impl<'a> UI<'a> {
    pub fn new(app: &'a mut App) -> Self {
        let macromolecule_type = app.alignment.macromolecule_type();
        app.info_msg("Press '?' for help");
        UI {
            app,
            color_schemes: vec![
                ColorScheme::color_scheme_dark(macromolecule_type),
                ColorScheme::color_scheme_light(macromolecule_type),
                ColorScheme::color_scheme_monochrome(),
            ],
            current_color_scheme_index: 0,
            zoom_level: ZoomLevel::ZoomedIn,
            show_zoombox: true,
            show_zb_guides: true,
            show_scrollbars: true,
            highlight_retained_cols: false,
            top_line: 0,
            leftmost_col: 0,
            left_pane_width: 18, // Reasonable default, I'd say...
            previous_left_pane_width: 0,
            bottom_pane_height: 5,
            previous_bottom_pane_height: 0,
            bottom_pane_position: BottomPanePosition::Adjacent,
            aln_pane_size: None,
            frame_size: None,
            full_screen: false,
            video_mode: VideoMode::Inverse,
            input_mode: InputMode::Normal,
            help_state: HelpState::default(),
            diff_mode: DiffMode::Original,
        }
    }

    // ****************************************************************
    /*
     * Dimensions
     *
     * The layout determines the maximal number of sequences and columns shown; this in turn
     * affects the maximal top line and leftmost column, etc.
     * */

    fn max_nb_seq_shown(&self) -> u16 {
        let height = self.aln_pane_size.unwrap().height;
        height.saturating_sub(2) // Borders - TODO: use constants!
    }

    fn max_nb_col_shown(&self) -> u16 {
        let width = self.aln_pane_size.unwrap().width;
        width.saturating_sub(2) // Borders - TODO: use constants!
    }

    // Resizing (as when the user resizes the terminal window where Termal runs) affects
    // max_top_line and max_leftmost_col (because the number of available lines (resp. columns)
    // will generally change), so top_line and leftmost_col may now exceed them. This function,
    // which should be called after the layout is solved but before the widgets are drawn, makes
    // sure that l_max corresponds to the size of the alignment panel, so that l does not exceed
    // l_max, etc.

    pub fn adjust_seq_pane_position(&mut self) {
        if self.leftmost_col > self.max_leftmost_col() {
            self.leftmost_col = self.max_leftmost_col();
        }
        if self.top_line > self.max_top_line() {
            self.top_line = self.max_top_line();
        }
    }

    /****************************************************************/
    // Location within the alignment

    pub fn top_line(&self) -> u16 {
        self.top_line
    }
    pub fn leftmost_col(&self) -> u16 {
        self.leftmost_col
    }
    pub fn help_top_line(&self) -> u16 {
        self.help_state.top_line
    }
    pub fn help_leftmost_col(&self) -> u16 {
        self.help_state.leftmost_col
    }

    pub fn reset_help_scroll(&mut self) {
        self.help_state = HelpState::default();
    }

    pub fn scroll_help_up(&mut self, count: u16) {
        self.help_state.top_line = self.help_state.top_line.saturating_sub(count);
    }

    pub fn scroll_help_down(&mut self, count: u16, max_top_line: u16) {
        self.help_state.top_line =
            min(self.help_state.top_line.saturating_add(count), max_top_line);
    }

    pub fn clamp_help_scroll(&mut self, max_top_line: u16) {
        self.help_state.top_line = min(self.help_state.top_line, max_top_line);
    }

    // FIXME: use saturating arithmetic (also next fn)
    pub fn max_top_line(&self) -> u16 {
        if self.app.num_seq() >= self.max_nb_seq_shown() {
            self.app.num_seq() - self.max_nb_seq_shown()
        } else {
            0
        }
    }

    pub fn max_leftmost_col(&self) -> u16 {
        if self.app.aln_len() >= self.max_nb_col_shown() {
            self.app.aln_len() - self.max_nb_col_shown()
        } else {
            0
        }
    }

    // Side panel dimensions

    pub fn set_left_pane_width(&mut self, width: u16) {
        self.left_pane_width = width;
    }

    // Also stores previous width
    pub fn hide_label_pane(&mut self) {
        self.previous_left_pane_width = self.left_pane_width;
        self.left_pane_width = 0;
    }

    pub fn show_label_pane(&mut self) {
        self.left_pane_width = self.previous_left_pane_width;
    }

    pub fn toggle_label_pane(&mut self) {
        if self.left_pane_width == 0 {
            self.show_label_pane();
        } else {
            self.hide_label_pane();
        }
    }

    // Number of columns needed to write the highest sequence number, e.g. 4 for 1000. This does
    // NOT take into account any borders.
    pub fn seq_num_max_len(&self) -> u16 {
        self.app.num_seq().ilog10() as u16 + 1
    }

    // Width of seq num pane, which is the length of the longest seq num + border width (1).
    // TODO: Express border width as a constant
    pub fn seq_num_pane_width(&self) -> u16 {
        self.seq_num_max_len() + 1
    }

    pub fn widen_label_pane(&mut self, amount: u16) {
        self.left_pane_width = min(
            self.left_pane_width + amount,
            self.frame_size.unwrap().width - (V_SCROLLBAR_WIDTH + MIN_COLS_SHOWN + BORDER_WIDTH),
        );
    }

    pub fn reduce_label_pane(&mut self, amount: u16) {
        self.left_pane_width = max(
            self.seq_num_pane_width() + self.seq_metric_pane_width(),
            self.left_pane_width.saturating_sub(amount),
        );
    }

    pub fn seq_metric_pane_width(&self) -> u16 {
        // Two chars for the histogram, and one for the border
        3
    }

    // Bottom pane dimensions

    pub fn set_bottom_pane_height(&mut self, height: u16) {
        self.bottom_pane_height = height;
    }

    pub fn hide_bottom_pane(&mut self) {
        self.previous_bottom_pane_height = self.bottom_pane_height;
        self.bottom_pane_height = 0;
    }

    pub fn show_bottom_pane(&mut self) {
        self.bottom_pane_height = 5;
    }

    pub fn toggle_bottom_pane(&mut self) {
        if self.bottom_pane_height == 0 {
            self.show_bottom_pane();
        } else {
            self.hide_bottom_pane();
        }
    }

    // Full screen

    pub fn toggle_fullscreen(&mut self) {
        if self.full_screen {
            self.show_label_pane();
            self.show_bottom_pane();
            self.full_screen = false;
        } else {
            self.hide_label_pane();
            self.hide_bottom_pane();
            self.full_screen = true;
        }
    }

    // ****************************************************************
    // Zooming

    // Determines if the alignment fits on the screen or is too wide or tall (it can be both).
    // TODO: might be an inner function of cycle_zoom, as it is not used anywhere else.
    fn aln_wrt_seq_pane(&self) -> AlnWRTSeqPane {
        let mut rel = AlnWRTSeqPane::Fits;
        if self.app.aln_len() > self.max_nb_col_shown() {
            rel |= AlnWRTSeqPane::TooWide;
        }
        if self.app.num_seq() > self.max_nb_seq_shown() {
            rel |= AlnWRTSeqPane::TooTall;
        }

        rel
    }

    // TODO: is this accessor needed?
    pub fn zoom_level(&self) -> ZoomLevel {
        self.zoom_level
    }

    pub fn cycle_zoom(&mut self) {
        self.zoom_level = match self.zoom_level {
            ZoomLevel::ZoomedIn => {
                // ZoomedOut, unless alignment fits
                if self.aln_wrt_seq_pane() == AlnWRTSeqPane::Fits {
                    ZoomLevel::ZoomedIn
                } else {
                    ZoomLevel::ZoomedOut
                }
            }
            ZoomLevel::ZoomedOut => ZoomLevel::ZoomedOutAR,
            ZoomLevel::ZoomedOutAR => ZoomLevel::ZoomedIn,
        }
    }

    pub fn h_ratio(&self) -> f64 {
        self.max_nb_col_shown() as f64 / self.app.aln_len() as f64
    }

    pub fn v_ratio(&self) -> f64 {
        self.max_nb_seq_shown() as f64 / self.app.num_seq() as f64
    }

    // ZoomLevel::ZoomedOutAR mode uses a _single_ ratio, which is usually the minimum of the
    // vertical and horizontal ratios, but it _can_ use the mmaximum if the resulting alignment
    // still fits.
    pub fn common_ratio(&self) -> f64 {
        let min_ratio = self.h_ratio().min(self.v_ratio());
        let max_ratio = self.h_ratio().max(self.v_ratio());
        let max_r_cols = (self.app.aln_len() as f64 * max_ratio).floor() as u16;
        let max_r_seqs = (self.app.num_seq() as f64 * max_ratio).floor() as u16;

        if max_r_cols == self.max_nb_col_shown() && max_r_seqs == self.max_nb_seq_shown() {
            max_ratio
        } else {
            min_ratio
        }
    }

    pub fn set_zoombox(&mut self, state: bool) {
        self.show_zoombox = state;
    }

    pub fn toggle_zoombox(&mut self) {
        self.show_zoombox = !self.show_zoombox;
    }

    pub fn zoombox_top(&self) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => ((self.top_line as f64) * self.v_ratio()).floor() as usize,
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                let mut zb_top = ((self.top_line as f64) * ratio).floor() as usize;
                if zb_top >= self.max_nb_seq_shown() as usize {
                    zb_top = (self.max_nb_seq_shown() as usize).saturating_sub(1);
                }
                zb_top
            }
            _ => panic!(
                "zoombox_top() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn zoombox_bottom(&self, seq_para_len: usize) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                let mut zb_bottom: usize = (((self.top_line + self.max_nb_seq_shown()) as f64)
                    * self.v_ratio())
                .round() as usize;
                // If h_a < h_p
                if zb_bottom > self.app.num_seq() as usize {
                    zb_bottom = self.app.num_seq() as usize;
                }
                zb_bottom
            }
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                let aln_para_height = min(seq_para_len as u16, self.max_nb_seq_shown());
                let mut zb_bottom =
                    (((self.top_line + self.max_nb_seq_shown()) as f64) * ratio).round() as usize;
                // If h_a < h_p
                if zb_bottom > aln_para_height as usize {
                    zb_bottom = aln_para_height as usize;
                }
                zb_bottom
            }
            _ => panic!(
                "zoombox_bottom() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn zoombox_left(&self) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => ((self.leftmost_col as f64) * self.h_ratio()).floor() as usize,
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                ((self.leftmost_col as f64) * ratio).floor() as usize
            }
            _ => panic!(
                "zoombox_left() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn zoombox_right(&self, max_nb_col_shown_ar: usize) -> usize {
        match self.zoom_level {
            ZoomLevel::ZoomedOut => {
                let mut zb_right = (((self.leftmost_col + self.max_nb_col_shown()) as f64)
                    * self.h_ratio())
                .floor() as usize;
                // If w_a < w_p
                if zb_right > self.app.aln_len() as usize {
                    zb_right = self.app.aln_len() as usize;
                }
                zb_right
            }
            ZoomLevel::ZoomedOutAR => {
                let ratio = self.common_ratio();
                let aln_para_width = min(max_nb_col_shown_ar as u16, self.max_nb_col_shown());
                let mut zb_right = (((self.leftmost_col + self.max_nb_col_shown()) as f64) * ratio)
                    .floor() as usize;
                // If w_a < w_p
                if zb_right > aln_para_width as usize {
                    zb_right = aln_para_width as usize;
                }
                zb_right
            }
            _ => panic!(
                "zoombox_left() should not be called in {:?} mode\n",
                self.zoom_level
            ),
        }
    }

    pub fn cycle_bottom_pane_position(&mut self) {
        self.bottom_pane_position = match self.bottom_pane_position {
            BottomPanePosition::Adjacent => BottomPanePosition::ScreenBottom,
            BottomPanePosition::ScreenBottom => BottomPanePosition::Adjacent,
        }
    }

    pub fn set_zoombox_guides(&mut self, state: bool) {
        self.show_zb_guides = state;
    }

    pub fn toggle_hl_retained_cols(&mut self) {
        self.highlight_retained_cols = !self.highlight_retained_cols;
    }

    // ****************************************************************
    // Colors and Styles

    pub fn theme(&self) -> Theme {
        self.color_scheme().theme
    }

    pub fn color_scheme(&self) -> &ColorScheme {
        &self.color_schemes[self.current_color_scheme_index]
    }

    fn color_scheme_mut(&mut self) -> &mut ColorScheme {
        &mut self.color_schemes[self.current_color_scheme_index]
    }

    pub fn next_color_scheme(&mut self) {
        self.current_color_scheme_index += 1;
        self.current_color_scheme_index %= self.color_schemes.len();
    }

    pub fn prev_color_scheme(&mut self) {
        let nb_color_schemes = self.color_schemes.len();
        self.current_color_scheme_index += nb_color_schemes - 1;
        self.current_color_scheme_index %= nb_color_schemes;
    }

    pub fn set_monochrome(&mut self) {
        // NOTE: this relies on the convention that the monochrome color scheme is last in the
        // list.
        self.current_color_scheme_index = self.color_schemes.len() - 1;
    }

    pub fn add_user_colormap(&mut self, cmap_fname: &String) {
        let get_cmap = colormap_gecos(cmap_fname);
        match get_cmap {
            Ok(cmap) => {
                // Iterate over color schemes, add cmap unless monochrome
                for cs in &mut self.color_schemes {
                    cs.add_colormap(cmap.clone());
                }
            }
            Err(_) => self
                .app
                .error_msg(format!("Error reading colormap {}.", cmap_fname)),
        }
    }

    pub fn select_first_colormap(&mut self) {
        let cs: &mut ColorScheme = self.color_scheme_mut();
        cs.select_first_colormap();
    }

    pub fn next_colormap(&mut self) {
        let cs: &mut ColorScheme = self.color_scheme_mut();
        cs.next_colormap();
    }

    pub fn prev_colormap(&mut self) {
        let cs: &mut ColorScheme = self.color_scheme_mut();
        cs.prev_colormap();
    }

    pub fn toggle_video_mode(&mut self) {
        self.video_mode = match self.video_mode {
            VideoMode::Direct => VideoMode::Inverse,
            VideoMode::Inverse => VideoMode::Direct,
        }
    }

    pub fn get_label_num_color(&self) -> Color {
        self.color_scheme().label_num_color
    }

    pub fn get_zoombox_color(&self) -> Color {
        match self.color_scheme().theme {
            Theme::Dark | Theme::Light => self.color_scheme().zoombox_color,
            Theme::Monochrome => Color::Reset,
        }
    }

    pub fn get_seq_metric_style(&self) -> Style {
        match self.color_scheme().theme {
            Theme::Dark | Theme::Light => Style::default().fg(self.color_scheme().seq_metric_color),
            // For now, we let monochrome theme use terminal defaults
            Theme::Monochrome => Style::default().fg(Color::Reset).bg(Color::Reset),
        }
    }

    // ****************************************************************
    // Scrolling

    pub fn disable_scrollbars(&mut self) {
        self.show_scrollbars = false;
    }

    // By lines (zoomed in)

    pub fn scroll_one_line_up(&mut self, count: u16) {
        self.top_line = self.top_line.saturating_sub(count);
    }

    pub fn scroll_one_col_left(&mut self, count: u16) {
        self.leftmost_col = self.leftmost_col.saturating_sub(count);
    }

    pub fn scroll_one_line_down(&mut self, count: u16) {
        self.top_line = min(self.top_line.saturating_add(count), self.max_top_line());
    }

    pub fn scroll_one_col_right(&mut self, count: u16) {
        self.leftmost_col = min(
            self.leftmost_col.saturating_add(count),
            self.max_leftmost_col(),
        );
    }

    // By screens

    pub fn scroll_one_screen_up(&mut self, count: u16) {
        self.top_line = self
            .top_line
            .saturating_sub(count.saturating_mul(self.max_nb_seq_shown()));
    }

    pub fn scroll_one_screen_left(&mut self, count: u16) {
        self.leftmost_col = self
            .leftmost_col
            .saturating_sub(count.saturating_mul(self.max_nb_col_shown()));
    }

    pub fn scroll_one_screen_down(&mut self, count: u16) {
        self.top_line = min(
            self.top_line
                .saturating_add(count.saturating_mul(self.max_nb_seq_shown())),
            self.max_top_line(),
        );
    }

    pub fn scroll_one_screen_right(&mut self, count: u16) {
        self.leftmost_col = min(
            self.leftmost_col
                .saturating_add(count.saturating_mul(self.max_nb_col_shown())),
            self.max_leftmost_col(),
        );
    }

    // By lines, zoomed out
    pub fn scroll_zoombox_one_line_up(&mut self, count: u16) {
        self.top_line = self
            .top_line
            .saturating_sub((count as f64 / self.v_ratio()).round() as u16);
    }

    pub fn scroll_zoombox_one_col_left(&mut self, count: u16) {
        self.leftmost_col = self
            .leftmost_col
            .saturating_sub((count as f64 / self.h_ratio()).round() as u16);
    }

    pub fn scroll_zoombox_one_line_down(&mut self, count: u16) {
        self.top_line = min(
            self.top_line
                .saturating_add((count as f64 / self.v_ratio()).round() as u16),
            self.max_top_line(),
        );
    }

    pub fn scroll_zoombox_one_col_right(&mut self, count: u16) {
        self.leftmost_col = min(
            self.leftmost_col
                .saturating_add((count as f64 / self.h_ratio()).round() as u16),
            self.max_leftmost_col(),
        );
    }

    // ********************************************************
    // Jumps

    pub fn jump_to_top(&mut self) {
        self.top_line = 0
    }

    pub fn jump_to_begin(&mut self) {
        self.leftmost_col = 0
    }

    pub fn jump_to_bottom(&mut self) {
        self.top_line = self.max_top_line()
    }

    pub fn jump_to_end(&mut self) {
        self.leftmost_col = self.max_leftmost_col()
    }

    // Jump to (0-based) line.
    pub fn jump_to_line(&mut self, line: u16) {
        self.top_line = min(line, self.max_top_line());
    }

    // Jump to (0-based) line of given rank
    pub fn jump_to_rank(&mut self, rank: u16) {
        let line = self.app.reverse_ordering[rank as usize];
        self.top_line = min(line.try_into().unwrap(), self.max_top_line());
    }

    pub fn jump_to_col(&mut self, col: u16) {
        self.leftmost_col = min(col, self.max_leftmost_col());
    }

    pub fn jump_to_pct_line(&mut self, pct: u16) {
        let clamped_pct = min(100, pct);
        let tgt_line = (clamped_pct as f64 / 100.0 * self.app.num_seq() as f64).round() as u16;
        self.top_line = tgt_line;
    }

    pub fn jump_to_pct_col(&mut self, pct: u16) {
        let clamped_pct = min(100, pct);
        let tgt_col = (clamped_pct as f64 / 100.0 * self.app.aln_len() as f64).round() as u16;
        self.leftmost_col = tgt_col;
    }

    pub fn jump_to_next_match(&mut self, count: i16) {
        self.app.increment_current_match(count as isize);
        let target = self.app.current_match();

        match target {
            Some(JumpTarget::HeaderLine(match_screenline)) => {
                self.jump_to_line(match_screenline as u16);
            }
            Some(JumpTarget::Match(screen_line, match_pos)) => {
                self.jump_to_line(screen_line as u16);
                self.jump_to_col(match_pos.start_col() as u16);
            }
            None => {}
        }
        self.app.display_current_match();
    }

    // Jump to next match, but vertically. This allows the user to stay within the same region,
    // e.g. a conserved domain.

    pub fn jump_to_next_vertical_match(&mut self, count: i16) {
        let mut step_count = count.unsigned_abs();
        let count_sgn: isize = count.signum().into();
        let mut target: Option<JumpTarget> = None;
        // Iteratively jump `count` times to next (resp. previous) match, but discard matches that would require a horizontal jump to display. 
        while step_count > 0 {
            self.app.increment_current_match(count_sgn);
            // Decrement IFF match is suitable
            target = self.app.current_match();
            match target {
                Some(JumpTarget::Match(screen_line, match_pos)) => {
                   if match_pos.end_col() <= (self.leftmost_col + self.max_nb_col_shown()).into() {
                       step_count -= 1;
                   }
                }
                _ => {}
            }
        }

        match target {
            Some(JumpTarget::Match(screen_line, match_pos)) => {
                self.jump_to_line(screen_line as u16);
            }
            _ => {}
        }
        self.app.display_current_match();
    }

    pub fn jump_to_current_match(&mut self) {
        self.jump_to_next_match(0);
    }

    pub fn jump_to_next_hi_col_metric_region(&mut self, count: i16) {
        // FIXME: convert 'as <type>' to 'type::from(...)' whenever possible, as this protects
        // against checkless conversions
        let sought_col = self
            .app
            .next_hi_metric_region_start(usize::from(self.leftmost_col));
        if let Some(col) = sought_col {
            self.leftmost_col = u16::try_from(col).expect("screen col does not fit in u16");
        }
        // None -> noop
    }

    // Debugging

    pub fn assert_invariants(&self) {
        if self.max_nb_col_shown() > self.app.aln_len() {
            assert!(self.max_leftmost_col() == 0);
        } else {
            assert!(
                self.max_leftmost_col() + self.max_nb_col_shown() == self.app.aln_len(),
                "l_max: {} + w_p: {} == w_a: {} failed",
                self.max_leftmost_col(),
                self.max_nb_col_shown(),
                self.app.aln_len()
            );
        }
        assert!(
            self.leftmost_col <= self.max_leftmost_col(),
            "l: {}<= l_max: {}",
            self.leftmost_col,
            self.max_leftmost_col()
        )
    }
}
