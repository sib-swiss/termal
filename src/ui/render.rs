use ratatui::{
    prelude::{Color, Constraint, Direction, Layout, Line, Margin, Rect, Span, Style, Text},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use log::debug;

use crate::{
    ui::{color_scheme::SALMON, conservation::values_barchart, AlnWRTSeqPane},
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
        ZoomLevel::ZoomedOut => every_nth(ui.app.aln_len() as usize, ui.seq_para_width().into()),
        ZoomLevel::ZoomedOutAR => {
            let ratio = ui.h_ratio().min(ui.v_ratio());
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
        ZoomLevel::ZoomedOut => every_nth(ui.app.num_seq() as usize, ui.seq_para_height().into()),
        ZoomLevel::ZoomedOutAR => {
            let ratio = ui.h_ratio().min(ui.v_ratio());
            let num_retained_seqs: usize = (ui.app.num_seq() as f64 * ratio).round() as usize;
            every_nth(ui.app.num_seq() as usize, num_retained_seqs)
        }
    }
}

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

    for i in retained_seq_ndx(ui) {
        ztext.push(Line::from(ui.app.alignment.headers[i].clone()));
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
            // TODO: is the conversion to bytes done at _each_ iteration?
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
    let mut ztext: Vec<Line> = Vec::new();
    for i in retained_seq_ndx(ui) {
        let seq: &String = &ui.app.alignment.sequences[i];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in retained_col_ndx(ui) {
            let c: char = seq_chars[j];
            let span = Span::styled(c.to_string(), *ui.colour_map.get(&c).unwrap());
            spans.push(span);
        }
        ztext.push(Line::from(spans));
    }

    ztext
}

fn zoom_out_ar_seq_text<'a>(ui: &UI) -> Vec<Line<'a>> {
    let mut ztext: Vec<Line> = Vec::new();
    for i in retained_seq_ndx(ui) {
        let seq: &String = &ui.app.alignment.sequences[i];
        let seq_chars: Vec<char> = seq.chars().collect();
        let mut spans: Vec<Span> = Vec::new();
        for j in retained_col_ndx(ui) {
            let c: char = seq_chars[j];
            let span = Span::styled(c.to_string(), *ui.colour_map.get(&c).unwrap());
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
) {
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
    //FIXME: it should not be necessary to iterate _twice_ from zb_left+1 to zb_right
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("└"));
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("┘"));
}

// Auxiliary fn for mark_zoombox() - see remarks on previous fn.

fn mark_zoombox_zero_height(
    seq_para: &mut [Line],
    zb_top: usize, // zb_bottom == zb_top
    zb_left: usize,
    zb_right: usize,
) {
    debug!("zb_top: {}\n", zb_top);
    let l: &mut Line = &mut seq_para[zb_top];
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("╾"));
    for c in zb_left + 1..zb_right {
        let _ = std::mem::replace(&mut l.spans[c], Span::raw("─"));
    }
    let _ = std::mem::replace(&mut l.spans[zb_right - 1], Span::raw("╼"));
}

// Auxiliary fn for mark_zoombox() - see remarks on previous fn.

fn mark_zoombox_zero_width(
    seq_para: &mut [Line],
    zb_top: usize,
    zb_bottom: usize,
    zb_left: usize, // zb_right == zb_left
) {
    let mut l: &mut Line = &mut seq_para[zb_top];
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("╿"));

    for l in seq_para.iter_mut().take(zb_bottom).skip(zb_top + 1) {
        let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("│"));
    }

    l = &mut seq_para[zb_bottom - 1];
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("╽"));
}

// Auxiliary fn for mark_zoombox() - see remarks on previous fn.
//
fn mark_zoombox_point(
    seq_para: &mut [Line],
    zb_top: usize,
    zb_left: usize, // zb_bottom == zb_top, zb_right == zb_left
) {
    let l: &mut Line = &mut seq_para[zb_top];
    let _ = std::mem::replace(&mut l.spans[zb_left], Span::raw("▯"));
}

// Draws the zoombox (just overwrites the sequence area with box-drawing characters).
//
fn mark_zoombox(seq_para: &mut [Line], ui: &UI) {
    // I want zb_top to be immutable, but I may need to change it just after intialization
    let zb_top = ui.zoombox_top(seq_para.len());
    let zb_bottom = ui.zoombox_bottom(seq_para.len());
    let zb_left = ui.zoombox_left();
    let zb_right = ui.zoombox_right(seq_para[0].spans.len());
    /*
    let mut zb_right: usize =
        (((ui.leftmost_col + ui.seq_para_width()) as f64) * ui.h_ratio()).round() as usize;
    // If w_a < w_p
    if zb_right > ui.app.aln_len() as usize {
        zb_right = ui.app.aln_len() as usize;
    }
    // debug!("w_a: {}, w_p: {}, r_h: {}", ui.app.aln_len(), ui.seq_para_width(), ui.h_ratio());
    debug!(
        "ZB_top: {}, ZB_bot: {}, ZB_lft: {}, ZB_rgt: {}\n",
        zb_top, zb_bottom, zb_left, zb_right
    );
    ui.assert_invariants();
    */

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

// TODO: put inside draw_zoombox_guides(), once tested (possibly as a closure, in fact)
fn g(l: usize, h: usize, b: usize, j: usize) -> usize {
    let left_zb_pos = l as f64;
    let bottom_empty_line = h as f64;
    let nb_empty_lines = (h - b) as f64;
    let slope = left_zb_pos / nb_empty_lines; // actually -slope...

    (-slope * j as f64 + slope * bottom_empty_line).round() as usize
}

// TODO: put inside draw_zoombox_guides(), once tested (possibly as a closure, in fact)
fn rg(w: usize, r: usize, h: usize, b: usize, j: usize) -> usize {
    let sp_width = w as f64;
    let right_zb_pos = r as f64;
    let bottom_zb_pos = b as f64;
    let nb_empty_lines = (h - b) as f64;
let slope = (sp_width - right_zb_pos) / nb_empty_lines;

    (-slope * j as f64 + slope * bottom_empty_line).round() as usize
}

// Draws guides from the scale to the zoom box (hence, only meaningful in one of the zoomed-out
// modes, and only if there are empty lines).
//
fn draw_zoombox_guides(seq_para: &mut Vec<Line>, ui: &UI) {
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
    debug!("ZB_lft: {}, ZB_rgt: {}\n", zb_left, zb_right);

    for j in zb_bottom + 1..ui.seq_para_height() as usize {
        let mut line = String::new();
        for i in 0..ui.seq_para_width() {
            if usize::from(i) == g(zb_left, ui.seq_para_height().into(), zb_bottom, j) {
                line.push('.');
            } else {
                line.push(' ');
            }
        }
        seq_para.push(Line::from(line));
    }
}

// Draws the zoombox, but preserving aspect ratio
//
fn mark_zoombox_ar(seq_para: &mut [Line], ui: &UI) {
    let zb_top = ui.zoombox_top(seq_para.len());
    let zb_bottom = ui.zoombox_bottom(seq_para.len());

    let zb_left = ui.zoombox_left();
    let zb_right = ui.zoombox_right(seq_para[0].spans.len());
    /*
    let mut zb_right: usize =
        (((ui.leftmost_col + ui.seq_para_width()) as f64) * ratio).round() as usize;
    // If w_a < w_p
    if zb_right > aln_para_width as usize {
        zb_right = aln_para_width as usize;
    }
    */
    // debug!("w_a: {}, w_p: {}, r_h: {}", ui.app.aln_len(), ui.seq_para_width(), ratio);
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

// Ticks and tick marks (e.g. for bottom pane)

fn tick_marks(aln_length: usize, primary: Option<char>, secondary: Option<char>) -> String {
    let mut ticks = String::with_capacity(aln_length);
    for i in 0..aln_length {
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
            // TODO: condition on option ui.show_zoombox_guides; also do in AR
            draw_zoombox_guides(&mut sequences, ui);
        }
        ZoomLevel::ZoomedOutAR => {
            sequences = zoom_out_ar_seq_text(ui);
            if ui.show_zoombox {
                mark_zoombox_ar(&mut sequences, ui);
            }
        }
    }

    sequences
}

fn compute_labels_pane_text<'a>(ui: &'a UI<'a>) -> Vec<Line<'a>> {
    let labels: Vec<Line> = match ui.zoom_level {
        ZoomLevel::ZoomedIn => zoom_in_lbl_text(ui),
        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => zoom_out_lbl_text(ui),
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
    let seq = compute_aln_pane_text(ui);
    let aln_block = Block::default().title(title).borders(Borders::ALL);
    let seq_para = Paragraph::new(seq).white().block(aln_block);
    f.render_widget(seq_para, aln_chunk);

    if ui.zoom_level == ZoomLevel::ZoomedIn && ui.show_scrollbars {
        // vertical scrollbar
        if (AlnWRTSeqPane::TooTall == (ui.aln_wrt_seq_pane() & AlnWRTSeqPane::TooTall))
            && ui.seq_para_height() > 2
        {
            let mut v_scrollbar_state = ScrollbarState::default()
                .content_length((ui.app.num_seq() - ui.seq_para_height()).into())
                .viewport_content_length((ui.seq_para_height() - 2).into())
                .position(ui.top_line.into());
            let v_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .thumb_style(Color::Cyan)
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
                .thumb_style(Color::Cyan)
                .thumb_symbol("🬹")
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

fn mark_consensus_zb_pos(consensus: &mut [Span], retained_pos: &[usize]) {
    let highlight = Style::default().reversed();
    for pos in retained_pos {
        let retained_span = consensus[*pos].clone().patch_style(highlight);
        let _ = std::mem::replace(&mut consensus[*pos], retained_span);
    }
}

fn render_bottom_pane(f: &mut Frame, bottom_chunk: Rect, ui: &UI) {
    let btm_block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);

    let mut coloured_consensus: Vec<Span> = ui
        .app
        .alignment
        .consensus
        .chars()
        .map(|c| {
            Span::styled(
                c.to_string(),
                *ui.colour_map.get(&c).unwrap_or(&Color::White),
            )
        })
        .collect();

    if ZoomLevel::ZoomedIn != ui.zoom_level && ui.highlight_retained_cols {
        mark_consensus_zb_pos(&mut coloured_consensus, &retained_col_ndx(ui));
    }

    let btm_text: Vec<Line> = vec![
        Line::from(tick_marks(ui.app.aln_len() as usize, None, Some(':'))),
        Line::from(tick_position(ui.app.aln_len() as usize)),
        Line::from(coloured_consensus),
        Line::from(values_barchart(&product(
            &ui.app.alignment.densities,
            &ones_complement(&normalize(&ui.app.alignment.entropies)),
        )))
        .style(SALMON),
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
        assert_eq!(tm, "|         |         |");
    }

    #[test]
    fn test_tick_marks_02() {
        let tm = tick_marks(21, Some(':'), Some('.'));
        assert_eq!(tm, ":    .    :    .    :");
    }

    use crate::ui::render::g;

    #[test]
    fn test_g() {
        let b: usize = 0;
        let l: usize = 4;
        let h: usize = b + 4;
        assert_eq!(g(l, h, b, 0), 4);
        assert_eq!(g(l, h, b, 1), 3);
        assert_eq!(g(l, h, b, 2), 2);
        assert_eq!(g(l, h, b, 3), 1);
        assert_eq!(g(l, h, b, 4), 0);
    }

    fn test_rg() {
        let w: usize = 8;
        let r: usize = 4;
        let b: usize = 0;
        let h: usize = 4;
        assert_eq!(rg(w, r, h, b, 0), 4);
        assert_eq!(rg(w, r, h, b, 1), 5);
        assert_eq!(rg(w, r, h, b, 2), 6);
        assert_eq!(rg(w, r, h, b, 3), 7);
    }

}
