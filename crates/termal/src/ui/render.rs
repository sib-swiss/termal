// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier
use ratatui::{
    prelude::{Constraint, Direction, Layout, Line, Margin, Rect, Span, Style, Text},
    style::{Color, Modifier, Stylize},
    widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use termal_alignment::alignment::RefSpec::{self, Rank};

use super::{
    aln_widget::{SeqPane, SeqPaneZoomedOut},
    barchart::{value_to_hbar, values_barchart},
    color_scheme::Theme,
    msg_theme::style_for,
    style::{build_style_lut, get_residue_style},
    AlnWRTSeqPane, BottomPanePosition, InputMode, VideoMode, ZoomLevel, BORDER_WIDTH,
    MIN_COLS_SHOWN, UI, V_SCROLLBAR_WIDTH,
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
            // This call to round() is ok as it is not an indx into an array.
            let num_retained_seqs: usize = (ui.app.num_seq() as f64 * ratio).round() as usize;
            every_nth(ui.app.num_seq() as usize, num_retained_seqs)
        }
    }
}

fn compute_label_numbers<'a>(ui: &UI) -> Vec<Line<'a>> {
    let num_cols = ui.seq_num_max_len() as usize;
    let numbers = ui
        .app
        .ordering
        .iter()
        .map(|n| Line::from(format!("{:1$}", n + 1, num_cols))) // n+1 -> 1-based (for humans...)
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
        .enumerate()
        .map(|(scrln, i)| {
            let hl_style = if ui.app.screenline_is_current_hdr_match(scrln) {
                Style::default()
                    .add_modifier(Modifier::REVERSED)
                    .add_modifier(Modifier::BOLD)
            } else if ui.app.screenline_is_hdr_match(scrln) {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            let span = Span::styled(ui.app.alignment.headers[*i].clone(), hl_style);
            Line::from(span)
        })
        .collect()
}

fn zoom_out_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let mut ztext: Vec<Line> = Vec::new();

    for i in retained_seq_ndx(ui) {
        let hl_style = if ui.app.screenline_is_current_hdr_match(i) {
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .add_modifier(Modifier::BOLD)
        } else if ui.app.screenline_is_hdr_match(i) {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };
        let rank = ui.app.ordering[i];
        ztext.push(Line::from(Span::styled(
            ui.app.alignment.headers[rank].clone(),
            hl_style,
        )));
    }

    ztext
}

// FIXME Shouldn't this be a part of UI rather than Render? We could dispense with passing the
// theme.
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
                vec![Constraint::Max(ui.left_pane_width), Constraint::Fill(1)],
            )
            .split(top_chunk)[1];

            let v_ratio = (aln_pane.height - 2) as f64 / ui.app.num_seq() as f64;
            // This is WRONG - need to discount left panes' width
            let h_ratio = (aln_pane.width - 2) as f64 / ui.app.aln_len() as f64;
            let ratio = h_ratio.min(v_ratio);

            (ui.app.num_seq() as f64 * ratio).round() as u16
        }
    }
}

fn delineate_help_pane(frame_area: Rect) -> Rect {
    // Help benefits from a larger viewport than the default modal size, especially on smaller
    // terminals. Keep a small margin around it instead of reserving a full 10% on each side.
    let dialog_v_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(frame_area);
    let dialog_h_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(4),
        ])
        .split(dialog_v_layout[1]);

    dialog_h_layout[1]
}

fn make_layout(f: &Frame, ui: &UI) -> Panes {
    // TODO: refactor into several fns; perhaps in a separate module
    let mns = max_num_seq(f, ui);

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

    let min_seq_pane_width = V_SCROLLBAR_WIDTH + MIN_COLS_SHOWN + BORDER_WIDTH;
    let upper_panes = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Max(ui.left_pane_width),
            Constraint::Min(min_seq_pane_width),
        ],
    )
    .split(v_panes[0]);
    let lbl_num_pane_num_cols = ui.seq_num_pane_width();
    let lbl_pane = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Length(lbl_num_pane_num_cols),
            Constraint::Fill(1),
            Constraint::Length(3),
        ],
    )
    .split(upper_panes[0]);
    let lower_panes = Layout::new(
        Direction::Horizontal,
        vec![
            Constraint::Max(ui.left_pane_width),
            Constraint::Fill(min_seq_pane_width),
        ],
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

fn compute_title(ui: &UI) -> String {
    ui.common_ratio();
    let title = format!(
        " {} | {}/{}s x {}/{}c | {} {}",
        ui.app.filename,
        ui.max_nb_seq_shown(),
        ui.app.num_seq(),
        ui.max_nb_col_shown(),
        ui.app.aln_len(),
        ui.color_scheme(),
        ui.video_mode,
    );
    format!(
        "{} | {} ",
        title,
        match ui.zoom_level {
            ZoomLevel::ZoomedIn => "Zoomed in",
            ZoomLevel::ZoomedOut => "Zoomed out ",
            ZoomLevel::ZoomedOutAR => "Z. out (Aspect)",
        }
    )
}

fn compute_labels_pane_text<'a>(ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let labels: Vec<Line> = match ui.zoom_level {
        ZoomLevel::ZoomedIn => zoom_in_lbl_text(ui),
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => zoom_out_lbl_text(ui),
    };

    labels
}

fn render_label_nums_pane(f: &mut Frame, num_chunk: Rect, ui: &UI) {
    log::debug!("Entering render_label_nums_pane()");
    let base_style = get_label_num_style(ui.theme(), ui.get_label_num_color());
    let mut lbl_nums = Text::from(compute_label_numbers(ui)).style(base_style);
    // log::debug!("lbl_nums: {:#?}", lbl_nums);

    // If the reference is an alignment sequence (as opposed to the consnsus), highlight it.

    if let Rank(ref_rk) = ui.app.alignment.get_ref_spec() {
        log::debug!("Ref rank: {}", ref_rk);
        let ref_screenline = ui.app.rank_to_screenline(ref_rk);
        log::debug!("Ref scln: {}", ref_screenline);
        let highlight_style = Style::default().add_modifier(Modifier::REVERSED);
        match ui.zoom_level {
            ZoomLevel::ZoomedIn => {
                if let Some(ref_num) = lbl_nums.lines.get_mut(ref_screenline) {
                    ref_num.style = highlight_style;
                }
            }
            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                let try_pos = retained_seq_ndx(ui)
                    .iter()
                    .position(|scln| ui.app.screenline_to_rank(*scln) == ref_rk);
                if let Some(pos) = try_pos {
                    log::debug!("pos: {:#?}", pos);
                    if let Some(ref_num) = lbl_nums.lines.get_mut(pos) {
                        ref_num.style = highlight_style;
                    }
                }
            }
        }
    }

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
    let seq_metrics = Text::from(compute_seq_metrics(ui)).style(ui.get_seq_metric_style());
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
    //let mut seq = compute_aln_pane_text(ui);
    let title = compute_title(ui);
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let inner_aln_block = aln_block.inner(aln_chunk);

    f.render_widget(aln_block, aln_chunk);

    let style_lut = build_style_lut(ui);

    match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            let pane = SeqPane {
                app: ui.app,
                sequences: &ui.app.alignment.sequences,
                ordering: &ui.app.ordering,
                ref_spec: ui.app.alignment.get_ref_spec(),
                diff_mode: ui.diff_mode,
                top_i: ui.top_line as usize,
                left_j: ui.leftmost_col as usize,
                style_lut: &style_lut,
                base_style: Style::default(),
            };
            f.render_widget(pane, inner_aln_block);
        }
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
            let zoombox_color = ui.get_zoombox_color();
            let pane = SeqPaneZoomedOut {
                app: ui.app,
                sequences: &ui.app.alignment.sequences,
                ordering: &ui.app.ordering,
                ref_spec: ui.app.alignment.get_ref_spec(),
                diff_mode: ui.diff_mode,
                retained_rows: &retained_seq_ndx(ui),
                retained_cols: &retained_col_ndx(ui),
                style_lut: &style_lut,
                base_style: Style::default(),
                show_zoombox: ui.show_zoombox,
                zb_top: ui.zoombox_top(),
                zb_bottom: ui.zoombox_bottom(retained_seq_ndx(ui).len()),
                zb_left: ui.zoombox_left(),
                zb_right: ui.zoombox_right(retained_col_ndx(ui).len()),
                zb_style: Style::new().fg(zoombox_color),
            };
            f.render_widget(pane, inner_aln_block);
        }
    }

    // let seq_para = Paragraph::new(seq).block(aln_block);
    // f.render_widget(seq_para, aln_chunk);

    if ui.zoom_level == ZoomLevel::ZoomedIn && ui.show_scrollbars {
        let zoombox_color = ui.get_zoombox_color();
        // vertical scrollbar
        if (AlnWRTSeqPane::TooTall == (ui.aln_wrt_seq_pane() & AlnWRTSeqPane::TooTall))
            && ui.max_nb_seq_shown() > 2
        {
            let mut v_scrollbar_state = ScrollbarState::default()
                .content_length((ui.app.num_seq() - ui.max_nb_seq_shown()).into())
                .viewport_content_length((ui.max_nb_seq_shown() - 2).into())
                .position(ui.top_line.into());
            let v_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_style(zoombox_color)
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
                .thumb_style(zoombox_color)
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
    // number, label and sequence metric) could be done within a single function (render_left_pane).
    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Fill(1)],
    )
    .split(corner_chunk);

    let seq_metric_chunk = layout[0];
    let cons_chunk = layout[1];
    let seq_metric_block = Block::default().borders(Borders::LEFT);
    let cons_block = Block::default().borders(Borders::LEFT | Borders::BOTTOM);

    let seq_metric_text_style = ui.get_seq_metric_style().add_modifier(Modifier::BOLD);
    let seq_metric_para = Paragraph::new(Text::styled(
        format!("{} {}", ui.app.get_seq_metric(), ui.app.get_seq_ordering()),
        seq_metric_text_style,
    ))
    .block(seq_metric_block)
    .right_aligned();
    f.render_widget(seq_metric_para, seq_metric_chunk);

    let ref_string = match ui.app.alignment.get_ref_spec() {
        RefSpec::Consensus => "Ref: consensus".into(),
        RefSpec::Rank(rk) => format!("Ref: #{}", rk + 1), // + 1 <- user is 1-based
    };

    // TODO: Should be renamed col-mnetric color, unless metrics get their own colors
    let conservation_color = match ui.color_scheme().theme {
        Theme::Dark | Theme::Light => ui.color_scheme().conservation_color,
        Theme::Monochrome => Color::Reset,
    };

    let cons_text = Text::from(vec![
        Line::from("Position"),
        Line::from(ref_string),
        Line::from(format!("Metric: {}", ui.app.current_col_metric())).style(conservation_color),
    ]);
    let cons_para = Paragraph::new(cons_text).block(cons_block);
    f.render_widget(cons_para, cons_chunk);
}

fn mark_consensus_zb_pos(consensus: &mut [Span], ui: &UI) {
    let retained_pos = &retained_col_ndx(ui);
    let highlight = match ui.video_mode {
        VideoMode::Inverse => Style::new().remove_modifier(Modifier::REVERSED),
        VideoMode::Direct => Style::new().reversed(),
    };
    for pos in retained_pos {
        let retained_span = consensus[*pos].clone().patch_style(highlight);
        let _ = std::mem::replace(&mut consensus[*pos], retained_span);
    }
}

fn render_bottom_pane(f: &mut Frame, bottom_chunk: Rect, ui: &UI) {
    let colormap = ui.color_scheme().current_residue_colormap();
    let btm_block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);

    let mut colored_consensus: Vec<Span> = ui
        .app
        .alignment
        .reference()
        .chars()
        .map(|c| {
            Span::styled(
                c.to_string(),
                get_residue_style(ui.video_mode, ui.theme(), colormap.get(c)),
            )
        })
        .collect();

    if ZoomLevel::ZoomedIn != ui.zoom_level && ui.highlight_retained_cols {
        mark_consensus_zb_pos(&mut colored_consensus, ui);
    }

    let pos_color = match ui.zoom_level {
        ZoomLevel::ZoomedIn => Color::Reset,
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => ui.get_zoombox_color(),
    };

    let conservation_color = match ui.color_scheme().theme {
        Theme::Dark | Theme::Light => ui.color_scheme().conservation_color,
        Theme::Monochrome => Color::Reset,
    };

    let col_metric_values = ui.app.current_col_metric_values();

    let btm_text: Vec<Line> = vec![
        Line::from(Span::styled(
            tick_marks(ui.app.aln_len() as usize, None, Some(':')),
            Style::default().fg(pos_color).bg(Color::Reset),
        )),
        Line::from(Span::styled(
            tick_position(ui.app.aln_len() as usize),
            Style::default().fg(pos_color).bg(Color::Reset),
        )),
        Line::from(colored_consensus),
        Line::from(values_barchart(&col_metric_values)).style(conservation_color),
    ];

    let btm_para = Paragraph::new(btm_text)
        .scroll((0, ui.leftmost_col))
        .block(btm_block);
    f.render_widget(btm_para, bottom_chunk);
}

fn render_modeline(f: &mut Frame, last_content_line: u16, ui: &mut UI) {
    // Do not write anything if BOTH prefix and msg are ""
    if ui.app.current_message().prefix.is_empty() && ui.app.current_message().message.is_empty() {
        return;
    }

    let modeline_rect = Rect {
        x: 1,
        y: last_content_line,
        width: f.area().width - (2 * BORDER_WIDTH),
        height: 1,
    };
    // without '└ ', the modeline would start in the very bottom left.
    let corner_span = Span::raw(" ");
    let message_span = Span::styled(
        format!(
            "{}{}",
            &*ui.app.current_message().prefix,
            &*ui.app.current_message().message
        ),
        style_for(&ui.app.current_message().kind),
    );
    let spacer_span = Span::raw(" ");
    let modeline = Line::from(vec![corner_span, message_span, spacer_span]);
    f.render_widget(modeline, modeline_rect);
}

fn render_help_dialog(f: &mut Frame, dialog_chunk: Rect, ui: &mut UI) {
    let dialog_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().add_modifier(Modifier::BOLD));
    let inner_dialog_chunk = dialog_chunk.inner(Margin {
        vertical: 1,
        horizontal: 1,
    });
    let dialog_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(1)])
        .split(inner_dialog_chunk);
    let banner = vec![
        Line::from("j/k or arrows: scroll   g: top   q, Esc, ?: close")
            .style(Style::default().add_modifier(Modifier::REVERSED)),
        Line::from(""),
    ];
    let help_header = Paragraph::new(banner);
    let bindings = include_str!("bindings.md");
    let text = Text::from(bindings);
    let content_height = dialog_layout[1].height;
    let total_lines = text.lines.len() as u16;
    let max_top_line = total_lines.saturating_sub(content_height);
    ui.clamp_help_scroll(max_top_line);
    let dialog_para = Paragraph::new(text)
        .scroll((ui.help_top_line(), ui.help_leftmost_col()))
        .style(Style::new().white().on_black());
    f.render_widget(Clear, dialog_chunk);
    f.render_widget(dialog_block, dialog_chunk);
    f.render_widget(help_header, dialog_layout[0]);
    f.render_widget(dialog_para, dialog_layout[1]);

    if total_lines > content_height && content_height > 0 {
        let mut v_scrollbar_state = ScrollbarState::default()
            .content_length(total_lines.into())
            .viewport_content_length(content_height.into())
            .position(ui.help_top_line().into());
        let v_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None);
        f.render_stateful_widget(
            v_scrollbar,
            Rect {
                x: dialog_chunk.x + dialog_chunk.width.saturating_sub(1),
                y: dialog_layout[1].y,
                width: 1,
                height: dialog_layout[1].height,
            },
            &mut v_scrollbar_state,
        );
    }
}

pub fn render_ui(f: &mut Frame, ui: &mut UI) {
    let layout_panes = make_layout(f, ui);

    /*
     * Many aspects of the UI depend on the alignment pane's dimensions, e.g. whether the whole
     * alignment fits in it, the horizontal and vertical ratios when zooming, the top line and
     * leftmost column, etc.
     */

    ui.aln_pane_size = Some(layout_panes.sequence.as_size());

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
    // ui.width_debug_msg();

    /* Render panes */
    render_label_nums_pane(f, layout_panes.lbl_num, ui);
    render_labels_pane(f, layout_panes.labels, ui);
    render_seq_metrics_pane(f, layout_panes.seq_metrics, ui);
    render_alignment_pane(f, layout_panes.sequence, ui);
    render_corner_pane(f, layout_panes.corner, ui);
    render_bottom_pane(f, layout_panes.bottom, ui);
    render_modeline(
        f,
        layout_panes.lbl_num.height + layout_panes.corner.height - 1,
        ui,
    );

    if ui.input_mode == InputMode::Help {
        render_help_dialog(f, layout_panes.dialog, ui);
        // after the first display of the help dialog, remove the message
        ui.app.clear_msg();
    }
}

/* Computes n indexes out of l. The indexes are as evenly spaced as possible, and always include
 * the first (0) and last (l-1) indexes. If n >= l, then return 0 .. l. */

pub fn every_nth(l: usize, n: usize) -> Vec<usize> {
    if n >= l {
        (0..l).collect()
    } else {
        let step: f32 = (l.saturating_sub(1)) as f32 / (n.saturating_sub(1)) as f32;
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
