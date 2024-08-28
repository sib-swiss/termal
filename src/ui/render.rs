use std::cmp::min;

use ratatui::{
    prelude::{Constraint, Direction, Layout, Line, Margin, Rect, Span, Text},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use log::debug;

use crate::{
    ui::{conservation::values_barchart, AlnWRTSeqPane},
    vec_f64_aux::{normalize, ones_complement, product},
    ZoomLevel, UI,
};

/*****************************************************************
 * Panel Texts
 *
 * for all zoom levels
*****************************************************************/

fn zoom_in_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    ui.app
        .alignment
        .headers
        .iter()
        .map(|h| Line::from(h.clone()))
        .collect()
}

fn zoom_out_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let mut ztext: Vec<Line> = Vec::new();
    let num_seq: usize = ui.app.num_seq() as usize;
    let retained_seqs_ndx: Vec<usize> = every_nth(num_seq, ui.seq_para_height().into());
    for i in &retained_seqs_ndx {
        ztext.push(Line::from(ui.app.alignment.headers[*i].clone()));
    }

    ztext
}

fn zoom_out_AR_lbl_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let ratio = ui.h_ratio().min(ui.v_ratio());
    let mut ztext: Vec<Line> = Vec::new();
    let num_seq: usize = (ui.app.num_seq() as f64 * ratio).round() as usize;
    let retained_seqs_ndx: Vec<usize> = every_nth(num_seq, ui.seq_para_height().into());
    for i in &retained_seqs_ndx {
        ztext.push(Line::from(ui.app.alignment.headers[*i].clone()));
    }

    ztext
}

fn zoom_in_seq_text<'a>(ui: &'a UI) -> Vec<Line<'a>> {
    let top_i = ui.top_line as usize;
    let bot_i = (ui.top_line + ui.seq_para_height()) as usize;
    let lft_j = ui.leftmost_col as usize;
    let rgt_j = (ui.leftmost_col + ui.seq_para_width()) as usize;

    let mut text: Vec<Line> = Vec::new();

    for i in top_i..bot_i {
        if i >= ui.app.num_seq().into() {
            break;
        } // if there is extra vertical space
        let mut spans: Vec<Span> = Vec::new();
        for j in lft_j..rgt_j {
            if j >= ui.app.aln_len().into() {
                break;
            } // ", horizontal
            let cur_seq_ref = &ui.app.alignment.sequences[i];
            let cur_char = (*cur_seq_ref).as_bytes()[j] as char;
            spans.push(Span::styled(
                cur_char.to_string(),
                *ui.colour_map.get(&cur_char).unwrap(),
            ));
        }
        text.push(Line::from(spans));
    }

    text
}

fn zoom_out_seq_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let num_seq: usize = ui.app.num_seq() as usize;
    let aln_len: usize = ui.app.aln_len() as usize;
    let seq_area_width: usize = ui.seq_para_width().into(); 
    let seq_area_height: usize = ui.seq_para_height().into(); // "
    let mut ztext: Vec<Line> = Vec::new();
    debug!("ZO: num seq: {}, num cols: {}\n", num_seq, aln_len);
    let retained_seqs_ndx: Vec<usize> = every_nth(num_seq, seq_area_height);
    let retained_cols_ndx: Vec<usize> = every_nth(aln_len, seq_area_width);
    for i in &retained_seqs_ndx {
        let seq: &String = &ui.app.alignment.sequences[*i];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in &retained_cols_ndx {
            let c: char = seq_chars[*j];
            let span = Span::styled(c.to_string(), *ui.colour_map.get(&c).unwrap());
            spans.push(span);
        }
        ztext.push(Line::from(spans));
    }

    ztext
}

fn zoom_out_AR_seq_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let ratio = ui.h_ratio().min(ui.v_ratio());
    let num_retained_seq: usize = (ui.app.num_seq() as f64 * ratio).round() as usize;
    let num_retained_cols: usize = (ui.app.aln_len() as f64 * ratio).round() as usize;

    let mut ztext: Vec<Line> = Vec::new();
    let retained_seqs_ndx: Vec<usize> = every_nth(num_retained_seq, ui.seq_para_height().into());
    let retained_cols_ndx: Vec<usize> = every_nth(num_retained_cols, ui.seq_para_width().into());
    for i in &retained_seqs_ndx {
        let seq: &String = &ui.app.alignment.sequences[*i];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in &retained_cols_ndx {
            let c: char = seq_chars[*j];
            let span = Span::styled(c.to_string(), *ui.colour_map.get(&c).unwrap());
            spans.push(span);
        }
        ztext.push(Line::from(spans));
    }

    ztext
}

// Draws the zoombox (just overwrites the sequence area with box-drawing characters).
//
fn mark_zoombox(seq_para: &mut [Line], ui: &UI) {
    let zb_top: usize = ((ui.top_line as f64) * ui.v_ratio()).round() as usize;
    let mut zb_bottom: usize =
        (((ui.top_line + ui.seq_para_height()) as f64) * ui.v_ratio()).round() as usize;
    // If h_a < h_p
    if zb_bottom > ui.app.num_seq() as usize {
        zb_bottom = ui.app.num_seq() as usize;
    }

    let zb_left: usize = ((ui.leftmost_col as f64) * ui.h_ratio()).round() as usize;
    let mut zb_right: usize =
        (((ui.leftmost_col + ui.seq_para_width()) as f64) * ui.h_ratio()).round() as usize;
    // If w_a < w_p
    if zb_right > ui.app.aln_len() as usize {
        zb_right = ui.app.aln_len() as usize;
    }
    // debug!("w_a: {}, w_p: {}, r_h: {}", ui.app.aln_len(), ui.seq_para_width(), ui.h_ratio());
    ui.assert_invariants();

    let mut l: &mut Line = &mut seq_para[zb_top];
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("┌"));
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("┐"));

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
        let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("│"));
        let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("│"));
    }
    //*/
    l = &mut seq_para[zb_bottom - 1];
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("└"));
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("┘"));
}

// Draws the zoombox, but preserving aspect ratio
//
fn mark_zoombox_AR(seq_para: &mut [Line], ui: &UI) {
    let ratio = ui.h_ratio().min(ui.v_ratio());
    debug!("[MZAR] ratio: {}", ratio);

    /* IN AR mode, the height of the alignment paragraph is the smallest of (i) the number of
     * retained sequences (which are in seq_para), and (ii) the alignment panel's height. */
    let aln_para_height = min(seq_para.len() as u16, ui.seq_para_height());
    debug!("[MZAR] aln para height: {}", aln_para_height);
    let zb_top: usize = ((ui.top_line as f64) * ratio).round() as usize;
    let mut zb_bottom: usize =
        (((ui.top_line + ui.seq_para_height()) as f64) * ratio).round() as usize;
    // If h_a < h_p
    if zb_bottom > aln_para_height as usize {
        zb_bottom = aln_para_height as usize;
    }
    debug!("[MZAR] zb_top: {}, zb_bottom: {}", zb_top, zb_bottom);

    /* IN AR mode, the width of the alignment paragraph is the smallest of (i) the number of
     * retained columns, and (ii) the alignment panel's width. */
    let aln_para_width = min((seq_para[0]).width() as u16, ui.seq_para_width());
    let zb_left: usize = ((ui.leftmost_col as f64) * ratio).round() as usize;
    let mut zb_right: usize =
        (((ui.leftmost_col + ui.seq_para_width()) as f64) * ratio).round() as usize;
    // If w_a < w_p
    if zb_right > aln_para_width as usize {
        zb_right = aln_para_width as usize;
    }
    // debug!("w_a: {}, w_p: {}, r_h: {}", ui.app.aln_len(), ui.seq_para_width(), ratio);
    ui.assert_invariants();

    let mut l: &mut Line = &mut seq_para[zb_top];
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("┌"));
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("┐"));

    for l in seq_para.iter_mut().take(zb_bottom).skip(zb_top + 1) {
        let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("│"));
        let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("│"));
    }

    debug!("[MZAR] seq_para length: {} index: {}", seq_para.len(), zb_bottom-1);
    l = &mut seq_para[zb_bottom - 1];
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("└"));
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("┘"));
}

/****************************************************************
* Layout
****************************************************************/

struct Panes {
    sequence: Rect,
    labels: Rect,
    bottom: Rect,
    corner: Rect,
}

fn make_layout(f: &Frame, ui: &UI) -> Panes {
    let constraints: Vec<Constraint> =
        vec![Constraint::Fill(1), Constraint::Max(ui.bottom_pane_height)];
    let v_panes = Layout::new(Direction::Vertical, constraints).split(f.size());
    let upper_panes = Layout::new(
        Direction::Horizontal,
        vec![Constraint::Max(ui.label_pane_width), Constraint::Fill(1)],
    )
    .split(v_panes[0]);
    let lower_panes = Layout::new(
        Direction::Horizontal,
        vec![Constraint::Max(ui.label_pane_width), Constraint::Fill(1)],
    )
    .split(v_panes[1]);

    Panes {
        labels: upper_panes[0],
        sequence: upper_panes[1],
        corner: lower_panes[0],
        bottom: lower_panes[1],
    }
}

// Ticks and tick marks for bottom pane

fn tick_marks(aln_length: usize) -> String {
    let mut ticks = String::with_capacity(aln_length);
    for i in 0..aln_length {
        ticks.push(if i % 10 == 0 { '|' } else { ' ' });
    }

    ticks
}

fn tick_position(aln_length: usize) -> String {
    let mut intervals: Vec<String> = vec![String::from("0")];
    let mut tens = 10;
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
    let title: String = match ui.zoom_level {
        ZoomLevel::ZoomedIn => {
            format!(
                " {} - {}s x {}c ",
                ui.app.filename,
                ui.app.num_seq(),
                ui.app.aln_len()
            )
        }
        ZoomLevel::ZoomedOut => {
            format!(
                " {} - {}s x {}c - fully zoomed out ",
                ui.app.filename,
                ui.app.num_seq(),
                ui.app.aln_len()
            )
        }
        ZoomLevel::ZoomedOutAR => {
            format!(
                " {} - {}s x {}c - fully zoomed out, preserving aspect ratio ",
                ui.app.filename,
                ui.app.num_seq(),
                ui.app.aln_len()
            )
        }
    };

    title
}

fn compute_aln_pane_text<'a>(frame_size: Rect, ui: &'a UI<'a>) -> Vec<Line<'a>> {
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
            sequences = zoom_out_AR_seq_text(ui);
            if ui.show_zoombox {
                mark_zoombox_AR(&mut sequences, ui);
            }
        }
    }

    sequences
}

fn compute_labels_pane_text<'a>(ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let labels: Vec<Line> = match ui.zoom_level {
        ZoomLevel::ZoomedIn => zoom_in_lbl_text(ui),
        ZoomLevel::ZoomedOut => zoom_out_lbl_text(ui),
        ZoomLevel::ZoomedOutAR => zoom_out_AR_lbl_text(ui),
    };

    labels
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
        .white()
        .scroll((top_lbl_line, 0))
        .block(lbl_block);
    f.render_widget(lbl_para, seq_chunk);
}

fn render_alignment_pane(f: &mut Frame, aln_chunk: Rect, ui: &UI) {
    let title = compute_title(ui);
    let seq = compute_aln_pane_text(f.size(), ui);
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let seq_para = Paragraph::new(seq).white().block(aln_block);
    f.render_widget(seq_para, aln_chunk);
    //f.render_widget(Paragraph::default(), layout_panes.sequence);
    // debug!("h_z: {}", every_nth(ui.app.num_seq().into(), ui.seq_para_height().into()).len());

    if ui.zoom_level == ZoomLevel::ZoomedIn && ui.show_scrollbars {
        // vertical scrollbar
        if (AlnWRTSeqPane::TooTall == (ui.aln_wrt_seq_pane() & AlnWRTSeqPane::TooTall))
            && ui.seq_para_height() > 2
        {
            let mut v_scrollbar_state = ScrollbarState::default()
                .content_length((ui.app.num_seq() - ui.seq_para_height()).into())
                .viewport_content_length((ui.seq_para_height() - 2).into())
                .position(ui.top_line.into());
            // debug!("v_bar: {:#?}", v_scrollbar_state);
            // debug!("t_max: {}", ui.max_top_line());
            let v_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None);
            f.render_stateful_widget(
                v_scrollbar,
                aln_chunk.inner(&Margin {
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut v_scrollbar_state,
            );
        }

        // horizontal scrollbar
        if (AlnWRTSeqPane::TooWide == (ui.aln_wrt_seq_pane() & AlnWRTSeqPane::TooWide))
            && ui.seq_para_width() > 2
        {
            let mut h_scrollbar_state = ScrollbarState::default()
                .content_length((ui.app.aln_len() - ui.seq_para_width()).into())
                .viewport_content_length((ui.seq_para_width() - 2).into())
                .position(ui.leftmost_col.into());
            let h_scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(None)
                .end_symbol(None);
            f.render_stateful_widget(
                h_scrollbar,
                aln_chunk.inner(&Margin {
                    vertical: 0,
                    horizontal: 1,
                }),
                &mut h_scrollbar_state,
            );
        }
    }
}

fn render_corner_pane(f: &mut Frame, corner_chunk: Rect) {
    let corner_block = Block::default().borders(Borders::LEFT | Borders::BOTTOM);
    let corner_text = Text::from(vec![
        "Consensus".into(),
        "Conservation".into(),
        "".into(),
        "Position".into(),
    ]);
    let corner_para = Paragraph::new(corner_text).block(corner_block);
    f.render_widget(corner_para, corner_chunk);
}

fn render_bottom_pane(f: &mut Frame, bottom_chunk: Rect, ui: &UI) {
    let btm_block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);

    let btm_text: Vec<Line> = vec![
        Line::from(ui.app.alignment.consensus.clone()),
        Line::from(values_barchart(&product(
            &ui.app.alignment.densities,
            &ones_complement(&normalize(&ui.app.alignment.entropies)),
        ))),
        Line::from(tick_marks(ui.app.aln_len() as usize)),
        Line::from(tick_position(ui.app.aln_len() as usize)),
    ];

    let btm_para = Paragraph::new(btm_text)
        .scroll((0, ui.leftmost_col))
        .block(btm_block);
    f.render_widget(btm_para, bottom_chunk);
}

pub fn render_ui(f: &mut Frame, ui: &mut UI) {
    let layout_panes = make_layout(f, ui);

    // debug!("seq pane size (w/ borders): {:?}", layout_panes.sequence.as_size());
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
    ui.frame_size = Some(f.size().as_size());

    ui.assert_invariants();

    /* Render panes */
    render_labels_pane(f, layout_panes.labels, ui);
    render_alignment_pane(f, layout_panes.sequence, ui);
    render_corner_pane(f, layout_panes.corner);
    render_bottom_pane(f, layout_panes.bottom, ui);
}

/* Computes n indexes out of l. The indexes are as evenly spaced as possible, and always include
 * the first (0) and last (l-1) indexes. If n >= l, then return 0 .. l. */

pub fn every_nth(l: usize, n: usize) -> Vec<usize> {
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
    use crate::ui::render::every_nth;

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
}
