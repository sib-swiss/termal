// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use ratatui::{buffer::Buffer, layout::Rect, prelude::Position, style::Style};

pub fn draw_zoombox_border(
    buf: &mut Buffer,
    area: Rect,
    zb_top: usize,
    zb_bottom: usize, // exclusive
    zb_left: usize,
    zb_right: usize, // exclusive
    style: Style,
) {
    let pane_h = area.height as usize;
    let pane_w = area.width as usize;
    if pane_h == 0 || pane_w == 0 {
        return;
    }

    let w = zb_right.saturating_sub(zb_left); // in cells
    let h = zb_bottom.saturating_sub(zb_top); // in cells

    let x0 = area.x + zb_left as u16;
    let y0 = area.y + zb_top as u16;
    let x1 = area.x + (zb_right.saturating_sub(1)) as u16;
    let y1 = area.y + (zb_bottom.saturating_sub(1)) as u16;

    // 1x1 (or degenerate) => point marker
    if w <= 1 && h <= 1 {
        draw_zoombox_border_point(buf, x0, y0, style);
        return;
    }

    // single column
    if w <= 1 {
        draw_zoombox_border_zero_width(buf, x0, y0, y1, style);
        return;
    }

    // single row
    if h <= 1 {
        draw_zoombox_border_zero_height(buf, x0, x1, y0, style);
        return;
    }

    // general case (>= 2x2)
    draw_zoombox_border_general_case(buf, x0, x1, y0, y1, style);
}

fn draw_zoombox_border_general_case(
    buf: &mut Buffer,
    zb_left: u16,
    zb_right: u16,
    zb_top: u16,
    zb_bottom: u16,
    style: Style,
) {
    // Top edge
    buf.cell_mut(Position::from((zb_left, zb_top)))
        .expect("Wrong position")
        .set_char('┌')
        .set_style(style);
    for x in (zb_left + 1)..zb_right {
        buf.cell_mut(Position::from((x, zb_top)))
            .expect("Wrong position")
            .set_char('─')
            .set_style(style);
    }
    buf.cell_mut(Position::from((zb_right, zb_top)))
        .expect("Wrong position")
        .set_char('┐')
        .set_style(style);

    // Sides
    for y in (zb_top + 1)..zb_bottom {
        buf.cell_mut(Position::from((zb_left, y)))
            .expect("Wrong position")
            .set_char('│')
            .set_style(style);
        buf.cell_mut(Position::from((zb_right, y)))
            .expect("Wrong position")
            .set_char('│')
            .set_style(style);
    }

    // Bottom edge
    buf.cell_mut(Position::from((zb_left, zb_bottom)))
        .expect("Wrong position")
        .set_char('└')
        .set_style(style);
    for x in (zb_left + 1)..zb_right {
        buf.cell_mut(Position::from((x, zb_bottom)))
            .expect("Wrong position")
            .set_char('─')
            .set_style(style);
    }
    buf.cell_mut(Position::from((zb_right, zb_bottom)))
        .expect("Wrong position")
        .set_char('┘')
        .set_style(style);
}

fn draw_zoombox_border_point(buf: &mut Buffer, zb_left: u16, zb_top: u16, style: Style) {
    buf.cell_mut(Position::from((zb_left, zb_top)))
        .expect("Wrong position")
        .set_char('▯')
        .set_style(style);
}

fn draw_zoombox_border_zero_width(
    buf: &mut Buffer,
    zb_left: u16, // zb_right == zb_left
    zb_top: u16,
    zb_bottom: u16,
    style: Style,
) {
    // Top cell
    buf.cell_mut(Position::from((zb_left, zb_top)))
        .expect("Wrong position")
        .set_char('╿')
        .set_style(style);
    // Inner cells
    for y in (zb_top + 1)..zb_bottom {
        buf.cell_mut(Position::from((zb_left, y)))
            .expect("Wrong position")
            .set_char('│')
            .set_style(style);
    }
    // Bottom cell
    buf.cell_mut(Position::from((zb_left, zb_bottom)))
        .expect("Wrong position")
        .set_char('╽')
        .set_style(style);
}

fn draw_zoombox_border_zero_height(
    buf: &mut Buffer,
    zb_left: u16,
    zb_right: u16,
    zb_top: u16, // zb_bottom = zb_top
    style: Style,
) {
    // Leftmost col
    buf.cell_mut(Position::from((zb_left, zb_top)))
        .expect("Wrong position")
        .set_char('╾')
        .set_style(style);
    // Inner cells
    for x in (zb_left + 1)..zb_right {
        buf.cell_mut(Position::from((x, zb_top)))
            .expect("Wrong position")
            .set_char('─')
            .set_style(style);
    }
    // Bottom edge
    buf.cell_mut(Position::from((zb_right, zb_top)))
        .expect("Wrong position")
        .set_char('╼')
        .set_style(style);
}
