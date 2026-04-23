// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

pub struct Canvas {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug)]
pub struct Rgba { pub r:u8, pub g:u8, pub b:u8, pub a:u8 }

#[derive(Clone, Debug, Default)]
pub struct TextStyle {
    pub fg: Option<Rgba>,
    pub bg: Option<Rgba>,
    pub bold: bool,
}

pub enum Primitive {
    Rect { x:f32, y:f32, w:f32, h:f32, fill: Option<Rgba> },
    Text { x:f32, y:f32, s:String, style: TextStyle },
    Line { x1:f32, y1:f32, x2:f32, y2:f32, stroke:Rgba, width:f32 },
}

pub struct FigurePlan {
    pub canvas: Canvas,
    pub prims: Vec<Primitive>,
}

pub struct FigureOpts {
    pub cell_w: f32,
    pub cell_h: f32,
    pub margin_x: f32,
    pub margin_y: f32,
    pub show_labels: bool,
    pub show_ruler: bool,
    // later: font_size, palette, ticks, etc.
}

impl Default for FigureOpts { ... }

pub fn build(
    grid: &ResidueGrid,
    opts: &FigureOpts,
) -> FigurePlan

#[test]
fn canvas_size_matches_grid_only() {
    let opts = FigureOpts {
        cell_w: 10.0,
        cell_h: 12.0,
        margin_x: 5.0,
        margin_y: 7.0,
        labels: false,
        ruler: false,
        ..Default::default()
    };

    let grid = fake_grid(2, 3); // 2 rows, 3 cols
    let plan = build_figure_plan(&grid, &opts);

    assert_eq!(plan.canvas.width, 5.0*2.0 + 3.0*10.0);
    assert_eq!(plan.canvas.height, 7.0*2.0 + 2.0*12.0);
}
