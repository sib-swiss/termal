// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier
use ratatui::{
    prelude::{Constraint, Direction, Layout, Line, Margin, Rect, Span, Style, Text},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use log::debug;

use crate::{
    ui::{
        barchart::{value_to_hbar, values_barchart},
        color_scheme::Theme,
        AlnWRTSeqPane, BottomPanePosition, VideoMode, 
    },
    vec_f64_aux::{normalize, ones_complement, product},
    ZoomLevel, UI,
};

/*****************************************************************
 * Panel Texts
 *
 * for all zoom levels
*****************************************************************/

fn retained_col_ndx(ui: &UI) -> Vec<usize> {
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            panic!("should not be called in zoomed-in mode")
        }
        ZoomLevel::ZoomedOut => every_nth(ui.app.aln_len() as usize, ui.max_nb_col_shown().into()),
        ZoomLevel::ZoomedOutAR => {
            let ratio = ui.common_ratio();
            // This call to round() is ok as it is not an indx into an array.
            let num_retained_cols: usize = (ui.app.aln_len() as f64 * ratio).round() as usize;
            every_nth(ui.app.aln_len() as usize, num_retained_cols)
        }
    }
}

fn retained_seq_ndx(ui: &UI) -> Vec<usize> {
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            panic!("should not be called in zoomed-in mode")
        }
        ZoomLevel::ZoomedOut => every_nth(ui.app.num_seq() as usize, ui.max_nb_seq_shown().into()),
        ZoomLevel::ZoomedOutAR => {
            let ratio = ui.common_ratio();
            debug!(
                "h-ratio: {}, v-ratio: {} -> Ratio: {ratio}",
                ui.h_ratio(),
                ui.v_ratio()
            );
            // This call to round() is ok as it is not an indx into an array.
            let num_retained_seqs: usize = (ui.app.num_seq() as f64 * ratio).round() as usize;
            debug!(
                "Num retained seqs: {} * {} = {} (total: {})",
                ui.app.num_seq(),
                ratio,
                num_retained_seqs,
                ui.app.num_seq()
            );
            every_nth(ui.app.num_seq() as usize, num_retained_seqs)
        }
    }
}

fn compute_label_numbers<'a>(ui: &UI) -> Vec<Line<'a>> {
    let num_cols = ui.app.num_seq().ilog10() as usize + 1;
    let numbers = ui
        .app
        .ordering
        .iter()
        .map(|n| Line::from(format!("{:1$}!", n + 1, num_cols))) // n+1 -> 1-based (for humans...)
        .collect();
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => numbers,
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
            let mut result: Vec<Line> = Vec::new();
            for i in retained_seq_ndx(ui) {
                result.push(numbers[i].clone());
            }
            result
        }
    }
}

fn compute_seq_metrics<'a>(ui: &UI) -> Vec<Line<'a>> {
    let order_values = ui.app.order_values();
    let numbers = ui
        .app
        .ordering
        .iter()
        .map(|id| Line::from(value_to_hbar(order_values[*id]).to_string()))
        .collect();
    match ui.zoom_level {
        ZoomLevel::ZoomedIn => numbers,
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
            let mut result: Vec<Line> = Vec::new();
            for i in retained_seq_ndx(ui) {
                result.push(numbers[i].clone());
            }
            result
        }
    }
}

fn zoom_in_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    ui.app
        .ordering
        .iter()
        .map(|i| {
            Line::from(Span::raw(
                // TODO: this clne should be avoidable, since Span takes a Cow. This would of
                // course entail some lifetime wrangling.
                ui.app.alignment.headers[*i].clone(),
            ))
        })
        .collect()
}

fn zoom_out_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let mut ztext: Vec<Line> = Vec::new();

    for i in retained_seq_ndx(ui) {
        ztext.push(Line::from(
            ui.app.alignment.headers[ui.app.ordering[i]].clone(),
        ));
    }

    ztext
}

fn get_label_num_style(theme: Theme, color: Color) -> Style {
    let mut style = Style::default();

    match theme {
        Theme::Dark | Theme::Light => {
            style = style.fg(color);
        }
        Theme::Monochrome => {
            style = style.fg(Color::Reset).bg(Color::Reset);
        }
    }

    style
}

fn get_residue_style(video_mode: VideoMode, theme: Theme, color: Color) -> Style {
    let mut style = Style::default();

    match theme {
        Theme::Dark | Theme::Light => {
            style = style.fg(color);
        }
        Theme::Monochrome => {
            style = style.fg(color).bg(Color::Black);
        }
    }

    match video_mode {
        VideoMode::Inverse  => {
            style = style.add_modifier(Modifier::REVERSED);
            if Theme::Light == theme {
                style = style.bg(Color::Black);
            }
        },
        _ => { },
    }

    style
}

fn zoom_in_seq_text<'a>(ui: &'a UI) -> Vec<Line<'a>> {
    let top_i = ui.top_line as usize;
    let bot_i = (ui.top_line + ui.max_nb_seq_shown()) as usize;
    let lft_j = ui.leftmost_col as usize;
    let rgt_j = (ui.leftmost_col + ui.max_nb_col_shown()) as usize;

    let mut text: Vec<Line> = Vec::new();
    // TODO: would it be possible to add a method to UI that returns a ref to the current colormap?
    // Or, failing that, to ask UI itself for the color to apply to a given char? If so, also apply
    // to zoom_out_lbl_text() and zoom_out_ar_seq_text().
    let colormap = ui.color_scheme().current_residue_colormap();
    let ordering = &ui.app.ordering;

    for i in top_i..bot_i {
        if i >= ui.app.num_seq().into() {
            break;
        } // if there is extra vertical space
        let mut spans: Vec<Span> = Vec::new();
        for j in lft_j..rgt_j {
            if j >= ui.app.aln_len().into() {
                break;
            } // ", horizontal
            let cur_seq_ref = &ui.app.alignment.sequences[ordering[i]];
            // TODO: is the conversion to bytes done at _each_ iteration?
            let cur_char = (*cur_seq_ref).as_bytes()[j] as char;
            let style = get_residue_style(ui.video_mode,
                ui.theme(), colormap.get(cur_char));
            spans.push(Span::styled(cur_char.to_string(), style));
        }
        text.push(Line::from(spans));
    }

    text
}

fn zoom_out_seq_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let colormap = ui.color_scheme().current_residue_colormap();
    let ordering = &ui.app.ordering;

    let mut ztext: Vec<Line> = Vec::new();
    for i in retained_seq_ndx(ui) {
        let seq: &String = &ui.app.alignment.sequences[ordering[i]];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in retained_col_ndx(ui) {
            let cur_char: char = seq_chars[j];
            let style = get_residue_style(ui.video_mode,
                ui.theme(), colormap.get(cur_char));
            let span = Span::styled(cur_char.to_string(), style);
            spans.push(span);
        }
        ztext.push(Line::from(spans));
    }

    ztext
}

fn zoom_out_ar_seq_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let colormap = ui.color_scheme().current_residue_colormap();
    let ordering = &ui.app.ordering;
    let mut ztext: Vec<Line> = Vec::new();
    for i in retained_seq_ndx(ui) {
        let seq: &String = &ui.app.alignment.sequences[ordering[i]];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in retained_col_ndx(ui) {
            let cur_char: char = seq_chars[j];
            let style = get_residue_style(ui.video_mode,
                ui.theme(), colormap.get(cur_char));
            let span = Span::styled(cur_char.to_string(), style);
            spans.push(span);
        }
        ztext.push(Line::from(spans));
    }

    ztext
}

// Auxiliary fn for mark_zoombox() - _could_ use an internal fn or a closure, but that would make
// the function too long for my taste.
//
fn mark_zoombox_general_case(
    seq_para: &mut [Line],
    zb_top: usize,
    zb_bottom: usize,
    zb_left: usize,
    zb_right: usize,
    zb_style: Style,
) {
    let mut l: &mut Line = &mut seq_para[zb_top];
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::styled("─", zb_style));
    }
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("┌", zb_style));
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::styled("┐", zb_style));

    // NOTE: Clippy suggests using an iterator here, but if I want, say, residues 600-680, then
    // there are going to be 600 useless iterations. I imagine indexing is faster, though
    // admittedly I did not benchmark it... except with my eye-o-meter, which indeed did not detect
    // any difference on a 11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz machine running WSL2, and
    // a 144-column by 33-lines terminal.

    // mine
    /*
    for s in zb_top+1 .. zb_bottom {
        l = &mut seq_para[s];
        let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("│"));
        let _ = std::mem::replace(&mut l.spans[zb_right-1], Span::raw("│"));
    }
    */

    // Clippy
    // /*
    for l in seq_para.iter_mut().take(zb_bottom).skip(zb_top + 1) {
        // let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("│", zb_style));
        let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("│", zb_style));
        let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::styled("│", zb_style));
    }
    //*/
    l = &mut seq_para[zb_bottom - 1];
    //FIXME: it should not be necessary to iterate _twice_ from zb_left+1 to zb_right
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::styled("─", zb_style));
    }
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("└", zb_style));
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::styled("┘", zb_style));
}

// Auxiliary fn for mark_zoombox() - see remarks on previous fn.

fn mark_zoombox_zero_height(
    seq_para: &mut [Line],
    zb_top: usize, // zb_bottom == zb_top
    zb_left: usize,
    zb_right: usize,
    zb_style: Style,
) {
    let l: &mut Line = &mut seq_para[zb_top];
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("╾", zb_style));
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::styled("─", zb_style));
    }
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::styled("╼", zb_style));
}

// Auxiliary fn for mark_zoombox() - see remarks on previous fn.

fn mark_zoombox_zero_width(
    seq_para: &mut [Line],
    zb_top: usize,
    zb_bottom: usize,
    zb_left: usize, // zb_right == zb_left
    zb_style: Style,
) {
    let mut l: &mut Line = &mut seq_para[zb_top];
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("╿", zb_style));

    for l in seq_para.iter_mut().take(zb_bottom).skip(zb_top + 1) {
        let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("│", zb_style));
    }

    l = &mut seq_para[zb_bottom - 1];
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("╽", zb_style));
}

// Auxiliary fn for mark_zoombox() - see remarks on previous fn.
//
fn mark_zoombox_point(
    seq_para: &mut [Line],
    zb_top: usize,
    zb_left: usize, // zb_bottom == zb_top, zb_right == zb_left
    zb_style: Style,
) {
    let l: &mut Line = &mut seq_para[zb_top];
    debug!("mark_zoombox_point(): zb_left = {zb_left}");
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::styled("▯", zb_style));
}

// Draws the zoombox (just overwrites the sequence area with box-drawing characters).
//
fn mark_zoombox(seq_para: &mut [Line], ui: &UI) {
    // I want zb_top to be immutable, but I may need to change it just after intialization
    let zb_top = ui.zoombox_top();
    let zb_bottom = ui.zoombox_bottom(seq_para.len());
    let zb_left = ui.zoombox_left();
    let zb_right = ui.zoombox_right(seq_para[0].spans.len());
    /*
    let mut zb_right: usize =
        (((ui.leftmost_col + ui.max_nb_col_shown()) as f64) * ui.h_ratio()).round() as usize;
    // If w_a < w_p
    if zb_right > ui.app.aln_len() as usize {
        zb_right = ui.app.aln_len() as usize;
    }
    ui.assert_invariants();
    */

    let zb_style = Style::new().fg(ui.color_scheme().zoombox_color);

    if zb_bottom - zb_top < 2 {
        if zb_right - zb_left < 2 {
            // Zoom box is on a single line & column
            mark_zoombox_point(seq_para, zb_top, zb_left, zb_style);
        } else {
            // Zoom box has a height of 1 line
            mark_zoombox_zero_height(seq_para, zb_top, zb_left, zb_right, zb_style);
        }
    } else if zb_right - zb_left < 2 {
        // Zoom box has a width of 1 column
        mark_zoombox_zero_width(seq_para, zb_top, zb_bottom, zb_left, zb_style);
    } else {
        // General case: height and width both > 1
        mark_zoombox_general_case(seq_para, zb_top, zb_bottom, zb_left, zb_right, zb_style);
    }
}

// Draws guides from the scale to the zoom box (hence, only meaningful in one of the zoomed-out
// modes, and only if there are empty lines). TODO: to avoid having to specify a lifetime, try
// passing relevant info (i.e., seq_para's length, zoombox's left and right cols, etc.)
//
fn draw_zoombox_guides<'a>(aln_bottom: usize, aln_len: usize, ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let mut guides: Vec<Line> = Vec::new();
    let zb_left = ui.zoombox_left();
    let zb_right = ui.zoombox_right(aln_len);

    // position of left guide
    let left_guide_pos = |j: usize| {
        let h = ui.max_nb_seq_shown() as f64;
        let slope = zb_left as f64 / (aln_bottom as f64 - h);
        (slope * j as f64 - slope * h).round() as usize
    };

    // position of right guide
    let right_guide_pos = |j: usize| {
        // -1: align the right guide to the last col of the alignment.
        let right_zb_pos = (zb_right - 1) as f64;
        let slope = ((ui.max_nb_col_shown() - 1) as f64 - right_zb_pos)
            / (ui.max_nb_seq_shown() as usize - aln_bottom) as f64;
        let y_int = right_zb_pos - aln_bottom as f64 * slope;
        (slope * j as f64 + y_int).round() as usize
    };

    for j in aln_bottom + 1..ui.max_nb_seq_shown() as usize {
        let mut line = String::new();
        let left_guide_col = left_guide_pos(j);
        let right_guide_col = right_guide_pos(j);
        for i in 0..ui.max_nb_col_shown() as usize {
            if i == left_guide_col {
                line.push('.');
            } else if i == right_guide_col {
                line.push('.');
            } else {
                line.push(' ');
            }
        }
        guides.push(Line::from(line));
    }

    guides
}

// Draws the zoombox, but preserving aspect ratio
//
//// TODO: this fn is now prolly identical with mark_zoombox()... keep only 1.
//
/*
fn mark_zoombox_ar(seq_para: &mut [Line], ui: &UI) {
    let zb_top = ui.zoombox_top(seq_para.len());
    let zb_bottom = ui.zoombox_bottom(seq_para.len());

    let zb_left = ui.zoombox_left();
    let zb_right = ui.zoombox_right(seq_para[0].spans.len());
    /*
    let mut zb_right: usize =
        (((ui.leftmost_col + ui.max_nb_col_shown()) as f64) * ratio).round() as usize;
    // If w_a < w_p
    if zb_right > aln_para_width as usize {
        zb_right = aln_para_width as usize;
    }
    */
    ui.assert_invariants();

    if zb_bottom - zb_top < 2 {
        if zb_right - zb_left < 2 {
            // Zoom box is on a single line & column
            mark_zoombox_point(seq_para, zb_top, zb_left);
        } else {
            // Zoom box has a height of 1 line
            mark_zoombox_zero_height(seq_para, zb_top, zb_left, zb_right);
        }
    } else if zb_right - zb_left < 2 {
        // Zoom box has a width of 1 column
        mark_zoombox_zero_width(seq_para, zb_top, zb_bottom, zb_left);
    } else {
        // General case: height and width both > 1
        mark_zoombox_general_case(seq_para, zb_top, zb_bottom, zb_left, zb_right);
    }
}
*/

/****************************************************************
* Layout
****************************************************************/

struct Panes {
    // Top-left (labels) pane
    lbl_num: Rect,
    labels: Rect,
    seq_metrics: Rect,

    // Alignment pane
    sequence: Rect,

    corner: Rect,

    // Bottom pane: position, consensus, etc.
    bottom: Rect,

    dialog: Rect,
}

// Height for Max constraint below (used in Adjacent bottom panel mode). In Zoomed In and ZoomedOut
// modes, the height of the sequence panel should not exceed the number of sequences in the
// alignment, in ZoomedOutAR mode it should not exceed the number of sequences shown while still
// preserving the aspect ratio. Now this itself depends on the screen's dimensions, so we need to
// do a first pass through Layout in order to determine this.
fn max_num_seq(f: &Frame, ui: &UI) -> u16 {
    match ui.zoom_level {
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedIn => ui.app.num_seq(),
        ZoomLevel::ZoomedOutAR => {
            let v_constraints = vec![Constraint::Fill(1), Constraint::Max(ui.bottom_pane_height)];
            let top_chunk = Layout::new(Direction::Vertical, v_constraints).split(f.area())[0];

            let aln_pane = Layout::new(
                Direction::Horizontal,
                vec![Constraint::Max(ui.label_pane_width), Constraint::Fill(1)],
            )
            .split(top_chunk)[1];

            //debug!("1st-pass seq area: {:?}", aln_pane);
            let v_ratio = (aln_pane.height - 2) as f64 / ui.app.num_seq() as f64;
            //debug!("1st-pass v-ratio: {}", v_ratio);
            // This is WRONG - need to discount left panes' width
            let h_ratio = (aln_pane.width - 2) as f64 / ui.app.aln_len() as f64;
            //debug!("1st-pass h-ratio: {}", h_ratio);
            let ratio = h_ratio.min(v_ratio);
            //debug!("1st-pass ratio: {}", ratio);
            debug!(
                "max #seq: {}",
                (ui.app.num_seq() as f64 * ratio).round() as u16
            );
            let max_num_seq = (ui.app.num_seq() as f64 * ratio).round() as u16;

            max_num_seq
        }
    }
}

fn delineate_help_pane(frame_area: Rect) -> Rect {
    // We take all the screen except the top, bottom, left and right 10%. This means dividing the
    // screen in three vertically, taking the middle 80%, and then dividing that in three and
    // taking its middle 80%.
    // NOTE here I'm using the builder style, might want to apply it elsewhere, if only for
    // consistency.
    let dialog_v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(frame_area);
    let dialog_h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(dialog_v_layout[1]);

    dialog_h_layout[1]
}

fn make_layout(f: &Frame, ui: &UI) -> Panes {
    // TODO: refactor into several fns; perhaps in a separate module
    let mns = max_num_seq(f, ui);
    debug!("max num seq: {}", mns);
    let constraints: Vec<Constraint> = match ui.bottom_pane_position {
        BottomPanePosition::Adjacent => vec![
            Constraint::Max(mns + 2), // + 2 <- borders
            // Constraint::Max(ui.app.num_seq()),
            Constraint::Max(ui.bottom_pane_height),
        ],
        BottomPanePosition::ScreenBottom => {
            vec![Constraint::Fill(1), Constraint::Max(ui.bottom_pane_height)]
        }
    };
    let v_panes = Layout::new(Direction::Vertical, constraints).split(f.area());

    let upper_panes = Layout::new(
        Direction::Horizontal,
        vec![Constraint::Max(ui.label_pane_width), Constraint::Fill(1)],
    )
    .split(v_panes[0]);
    // number of columns for the label number pane :-)
    // which is 1 + the log_10 of the number of sequences (rounded down), plus room for the left
    // border.
    let lbl_num_pane_num_cols = ui.app.num_seq().ilog10() + 2;
    let lbl_pane = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Length(lbl_num_pane_num_cols.try_into().unwrap()),
            Constraint::Fill(1),
            Constraint::Length(3),
        ],
    )
    .split(upper_panes[0]);
    let lower_panes = Layout::new(
        Direction::Horizontal,
        vec![Constraint::Max(ui.label_pane_width), Constraint::Fill(1)],
    )
    .split(v_panes[1]);

    // The dialog is only used in help mode, but we compute its position now all the same.
    let help_dialog_pane = delineate_help_pane(f.area());

    Panes {
        lbl_num: lbl_pane[0],
        labels: lbl_pane[1],
        seq_metrics: lbl_pane[2],
        sequence: upper_panes[1],
        corner: lower_panes[0],
        bottom: lower_panes[1],
        dialog: help_dialog_pane,
    }
}

// Ticks and tick marks (e.g. for bottom pane)

fn tick_marks(aln_length: usize, primary: Option<char>, secondary: Option<char>) -> String {
    let mut ticks = String::with_capacity(aln_length);
    ticks += "    :    ";
    for i in 10..aln_length {
        ticks.push(if i % 10 == 0 {
            primary.unwrap_or('|')
        } else if i % 5 == 0 {
            secondary.unwrap_or(' ')
        } else {
            ' '
        });
    }

    ticks
}

fn tick_position(aln_length: usize) -> String {
    let mut intervals: Vec<String> = vec![String::from("1       10")];
    let mut tens = 20;
    while tens < aln_length {
        let int = format!("{:>10}", tens);
        tens += 10;
        intervals.push(int);
    }
    intervals.join("")
}

/****************************************************************
// Draw UI
****************************************************************/

fn compute_title(ui: &UI, aln_para: &[Line]) -> String {
    ui.common_ratio();
    let title = format!(
        " {} - {}/{}s x {}/{}c {}",
        ui.app.filename,
        aln_para.len(),
        ui.app.num_seq(),
        aln_para[0].spans.len(),
        ui.app.aln_len(),
        ui.theme(),
    );
    format!(
        "{} - {}",
        title,
        match ui.zoom_level {
            ZoomLevel::ZoomedIn => "",
            ZoomLevel::ZoomedOut => "fully zoomed out ",
            ZoomLevel::ZoomedOutAR => "fully zoomed out, preserving aspect ratio ",
        }
    )
}

fn compute_aln_pane_text<'a>(ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let mut sequences: Vec<Line>;

    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            sequences = zoom_in_seq_text(ui);
        }
        ZoomLevel::ZoomedOut => {
            sequences = zoom_out_seq_text(ui);
            if ui.show_zoombox {
                mark_zoombox(&mut sequences, ui);
            }
        }
        ZoomLevel::ZoomedOutAR => {
            sequences = zoom_out_ar_seq_text(ui);
            if ui.show_zoombox {
                mark_zoombox(&mut sequences, ui);
            }
        }
    }

    debug!(
        "compute_aln_pane_text(): {}s x {}c",
        sequences.len(),
        sequences[0].spans.len()
    );
    sequences
}

fn compute_labels_pane_text<'a>(ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let labels: Vec<Line> = match ui.zoom_level {
        ZoomLevel::ZoomedIn => zoom_in_lbl_text(ui),
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => zoom_out_lbl_text(ui),
    };

    labels
}

fn render_label_nums_pane(f: &mut Frame, num_chunk: Rect, ui: &UI) {
    let style = get_label_num_style(ui.theme(), ui.get_label_num_color());
    let lbl_nums = Text::from(compute_label_numbers(ui)).style(style);
    let lbl_num_block = Block::default().borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);
    let top_lbl_line = match ui.zoom_level() {
        ZoomLevel::ZoomedIn => ui.top_line,
        ZoomLevel::ZoomedOut => 0,
        ZoomLevel::ZoomedOutAR => 0,
    };
    let lbl_num_para = Paragraph::new(lbl_nums)
        .scroll((top_lbl_line, 0))
        .block(lbl_num_block);
    f.render_widget(lbl_num_para, num_chunk);
}

fn render_labels_pane(f: &mut Frame, seq_chunk: Rect, ui: &UI) {
    /* Labels pane */
    let labels = compute_labels_pane_text(ui);
    let lbl_block = Block::default().borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);
    let top_lbl_line = match ui.zoom_level() {
        ZoomLevel::ZoomedIn => ui.top_line,
        ZoomLevel::ZoomedOut => 0,
        ZoomLevel::ZoomedOutAR => 0,
    };
    let lbl_para = Paragraph::new(labels)
        .scroll((top_lbl_line, 0))
        .block(lbl_block);
    f.render_widget(lbl_para, seq_chunk);
}

fn render_seq_metrics_pane(f: &mut Frame, num_chunk: Rect, ui: &UI) {
    let seq_metrics = Text::from(compute_seq_metrics(ui)).style(ui.get_seq_metric_color());
    let seq_metrics_block =
        Block::default().borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM);
    let top_lbl_line = match ui.zoom_level() {
        ZoomLevel::ZoomedIn => ui.top_line,
        ZoomLevel::ZoomedOut => 0,
        ZoomLevel::ZoomedOutAR => 0,
    };
    let seq_metrics_para = Paragraph::new(seq_metrics)
        .scroll((top_lbl_line, 0))
        .block(seq_metrics_block);
    f.render_widget(seq_metrics_para, num_chunk);
}

fn render_alignment_pane(f: &mut Frame, aln_chunk: Rect, ui: &UI) {
    debug!(
        "render_alignment_pane(): max_nb_seq_shown = {}",
        ui.max_nb_seq_shown()
    );
    let mut seq = compute_aln_pane_text(ui);
    debug!("render_alignment_pane(): aln width={}", seq[0].spans.len());
    let title = compute_title(ui, &seq);
    let aln_block = Block::default().title(title).borders(Borders::ALL);

    if ui.show_zb_guides {
        if ui.zoom_level == ZoomLevel::ZoomedIn {
            for _ in seq.len()..ui.max_nb_seq_shown() as usize {
                let mut ticks = tick_marks(ui.app.aln_len() as usize, Some('.'), None);
                ticks.drain(..ui.leftmost_col as usize);
                seq.push(Line::from(ticks));
            }
        } else {
            let mut guides = draw_zoombox_guides(seq.len(), seq[0].spans.len(), ui);
            seq.append(&mut guides);
        }
    }

    let seq_para = Paragraph::new(seq).block(aln_block);
    f.render_widget(seq_para, aln_chunk);

    if ui.zoom_level == ZoomLevel::ZoomedIn && ui.show_scrollbars {
        // vertical scrollbar
        if (AlnWRTSeqPane::TooTall == (ui.aln_wrt_seq_pane() & AlnWRTSeqPane::TooTall))
            && ui.max_nb_seq_shown() > 2
        {
            let mut v_scrollbar_state = ScrollbarState::default()
                .content_length((ui.app.num_seq() - ui.max_nb_seq_shown()).into())
                .viewport_content_length((ui.max_nb_seq_shown() - 2).into())
                .position(ui.top_line.into());
            let v_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_style(ui.color_scheme().zoombox_color)
                .begin_symbol(None)
                .end_symbol(None);
            f.render_stateful_widget(
                v_scrollbar,
                aln_chunk.inner(Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut v_scrollbar_state,
            );
        }

        // horizontal scrollbar
        if (AlnWRTSeqPane::TooWide == (ui.aln_wrt_seq_pane() & AlnWRTSeqPane::TooWide))
            && ui.max_nb_col_shown() > 2
        {
            let mut h_scrollbar_state = ScrollbarState::default()
                .content_length((ui.app.aln_len() - ui.max_nb_col_shown()).into())
                .viewport_content_length((ui.max_nb_col_shown() - 2).into())
                .position(ui.leftmost_col.into());
            let h_scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(None)
                .thumb_style(ui.color_scheme().zoombox_color)
                .thumb_symbol("🬹")
                .end_symbol(None);
            f.render_stateful_widget(
                h_scrollbar,
                aln_chunk.inner(Margin {
                    vertical: 0,
                    horizontal: 1,
                }),
                &mut h_scrollbar_state,
            );
        }
    }
}

fn render_corner_pane(f: &mut Frame, corner_chunk: Rect, ui: &UI) {
    // TODO: This render_* function does its own layout. Perhaps this could be done for other
    // non-top-level layouts, e.g. the layout of the left pane (which has three subpanes, namely
    // number, label and metric) could be done within a single function (render_left_pane).
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Fill(1)],
    )
    .split(corner_chunk);

    let metric_chunk = layout[0];
    let cons_chunk = layout[1];
    let metric_block = Block::default().borders(Borders::LEFT);
    let cons_block = Block::default().borders(Borders::LEFT | Borders::BOTTOM);

    let metric_text_style = Style::new()
        .fg(ui.get_seq_metric_color())
        .add_modifier(Modifier::BOLD);
    let metric_para = Paragraph::new(Text::styled(
        format!(
            "{} {}",
            ui.app.get_metric(),
            ui.app.get_seq_ordering().to_string()
        ),
        metric_text_style,
    ))
    .block(metric_block)
    .right_aligned();
    f.render_widget(metric_para, metric_chunk);

    let cons_text = Text::from(vec![
        "Position".into(),
        "Consensus".into(),
        "Conservation".into(),
    ]);
    let cons_para = Paragraph::new(cons_text).block(cons_block);
    f.render_widget(cons_para, cons_chunk);
}

fn mark_consensus_zb_pos(consensus: &mut [Span], ui: &UI) {
    let retained_pos = &retained_col_ndx(ui);
    let highlight = match ui.video_mode {
        VideoMode::Inverse => Style::new().remove_modifier(Modifier::REVERSED),
        VideoMode::Direct => Style::new().reversed()
    };
    for pos in retained_pos {
        let retained_span = consensus[*pos].clone().patch_style(highlight);
        let _ = std::mem::replace(&mut consensus[*pos], retained_span);
    }
}

fn render_bottom_pane(f: &mut Frame, bottom_chunk: Rect, ui: &UI) {
    let colormap = ui.color_scheme().current_residue_colormap();
    let btm_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .title_bottom(&*ui.message)
        .title_style(Style::new().bold());

    let mut colored_consensus: Vec<Span> = ui
        .app
        .alignment
        .consensus
        .chars()
        .map(|c| {
            Span::styled(
                c.to_string(),
                get_residue_style(ui.video_mode, ui.theme(), colormap.get(c))
            )
        })
        .collect();

    if ZoomLevel::ZoomedIn != ui.zoom_level && ui.highlight_retained_cols {
        mark_consensus_zb_pos(&mut colored_consensus, ui);
    }

    let pos_color = match ui.zoom_level {
        ZoomLevel::ZoomedIn => Color::Reset,
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => ui.color_scheme().zoombox_color,
    };

    let btm_text: Vec<Line> = vec![
        Line::from(Span::styled(
            // TODO: the color should behave like the text in the labels pane, which is
            // automatically white on black or black on white. Possibly this can be done by NOT
            // styling the span. Or else, if styling must be done, then the style should depend on
            // the theme.
            tick_marks(ui.app.aln_len() as usize, None, Some(':')),
            Style::default().fg(pos_color).bg(Color::Reset),
        )),
        Line::from(Span::styled(
            tick_position(ui.app.aln_len() as usize),
            Style::default().fg(pos_color).bg(Color::Reset),
        )),
        Line::from(colored_consensus),
        Line::from(values_barchart(&product(
            &ui.app.alignment.densities,
            &ones_complement(&normalize(&ui.app.alignment.entropies)),
        )))
        .style(ui.color_scheme().conservation_color),
    ];

    let btm_para = Paragraph::new(btm_text)
        .scroll((0, ui.leftmost_col))
        .block(btm_block);
    f.render_widget(btm_para, bottom_chunk);
}

fn render_help_dialog(f: &mut Frame, dialog_chunk: Rect) {
    let dialog_block = Block::default().borders(Borders::ALL);
    let bindings = include_str!("bindings.md");
    let mut text = Text::from(bindings);
    text.push_line("");
    text.push_line("Press any key to close this dialog.");
    let dialog_para = Paragraph::new(Text::from_iter(text))
        .block(dialog_block)
        .style(Style::new().white().on_black());
    f.render_widget(Clear, dialog_chunk);
    f.render_widget(dialog_para, dialog_chunk);
}

pub fn render_ui(f: &mut Frame, ui: &mut UI) {
    let layout_panes = make_layout(f, ui);

    /*
     * Many aspects of the UI depend on the alignment pane's dimensions, e.g. whether the whole
     * alignment fits in it, the horizontal and vertical ratios when zooming, the top line and
     * leftmost column, etc.
     */
    debug!(
        "render_ui(): aln_pane_size = {:?}",
        layout_panes.sequence.as_size()
    );
    ui.aln_pane_size = Some(layout_panes.sequence.as_size());
    debug!("render_ui(): max_nb_seq_shown = {}", ui.max_nb_seq_shown());
    // Handle resizing
    ui.adjust_seq_pane_position();
    /* NOTE: the docs (https://docs.rs/ratatui/latest/ratatui/struct.Frame.html#method.area) say
     * that ratatui::Frame::size is deprecated and that area() should be used instead, but I get a
     * E0599 if I use area().

    Versions:

     * rustc 1.77.2 (25ef9e3d8 2024-04-09)

     * Ratatui:
       name = "ratatui"
       version = "0.26.2"
       source = "registry+https://github.com/rust-lang/crates.io-index"
       checksum = "a564a852040e82671dc50a37d88f3aa83bbc690dfc6844cfe7a2591620206a80"
    */
    //ui.frame_size = Some(f.area().as_size());
    ui.frame_size = Some(f.area().as_size());

    ui.assert_invariants();

    /* Render panes */
    render_label_nums_pane(f, layout_panes.lbl_num, ui);
    render_labels_pane(f, layout_panes.labels, ui);
    render_seq_metrics_pane(f, layout_panes.seq_metrics, ui);
    render_alignment_pane(f, layout_panes.sequence, ui);
    render_corner_pane(f, layout_panes.corner, ui);
    render_bottom_pane(f, layout_panes.bottom, ui);

    if ui.show_help {
        render_help_dialog(f, layout_panes.dialog);
        // after the first display of the help dialog, remove the message
        ui.message = "".into();
    }
}

/* Computes n indexes out of l. The indexes are as evenly spaced as possible, and always include
 * the first (0) and last (l-1) indexes. If n >= l, then return 0 .. l. */

pub fn every_nth(l: usize, n: usize) -> Vec<usize> {
    //debug!("Computing {} indexes out of {}.", n, l);
    if n >= l {
        (0..l).collect()
    } else {
        let step: f32 = (l - 1) as f32 / (n - 1) as f32;
        let r: Vec<usize> = (0..n)
            .map(|e| ((e as f32) * step).round() as usize)
            .collect();
        r
    }
}

#[cfg(test)]
mod tests {

    use crate::ui::render::{every_nth, tick_marks};

    #[test]
    fn test_every_nth_1() {
        assert_eq!(vec![0, 4, 8], every_nth(9, 3));
    }

    #[test]
    fn test_every_nth_2() {
        assert_eq!(vec![0, 5, 9], every_nth(10, 3));
    }

    #[test]
    fn test_every_nth_3() {
        assert_eq!(vec![0, 1, 2, 3, 4], every_nth(5, 5));
    }

    #[test]
    fn test_every_nth_4() {
        assert_eq!(vec![0, 1, 2, 3, 4], every_nth(5, 10));
    }

    #[test]
    fn test_tick_marks_01() {
        let tm = tick_marks(21, None, None);
        assert_eq!(tm, "    :    |         |");
    }

    #[test]
    fn test_tick_marks_02() {
        let tm = tick_marks(21, Some(':'), Some('.'));
        assert_eq!(tm, "    :    :    .    :");
    }
}
