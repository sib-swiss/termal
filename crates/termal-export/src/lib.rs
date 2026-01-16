use anyhow::Result;
//use termal_alignment::{Alignment, Region};
// NOTE: Regions will be implemented later
use termal_alignment::Alignment;

#[derive(Clone, Debug)]
pub struct ExportOpts {
    pub cell_w: f32,
    pub cell_h: f32,
    pub margin_x: f32,
    pub margin_y: f32,
}

impl Default for ExportOpts {
    fn default() -> Self {
        Self {
            cell_w: 12.0,
            cell_h: 14.0,
            margin_x: 10.0,
            margin_y: 10.0,
        }
    }
}

//pub fn export_svg(aln: &Alignment, region: &Region, opts: &ExportOpts) -> Result<String> {
pub fn export_svg(aln: &Alignment, opts: &ExportOpts) -> Result<String> {
    // Placeholder pipeline:
    // 1) paint_region(aln, region) -> ResidueGrid (ratatui-free)
    // 2) figure_plan::build(grid, opts) -> FigurePlan
    // 3) svg::render(plan) -> String

    // For now, return a minimal SVG stub so the CLI works end-to-end.
    //let _ = (aln, region, opts);
    let _ = (aln, opts);
    Ok(r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="80">
  <rect x="0" y="0" width="200" height="80" fill="white"/>
  <text x="10" y="30" font-family="monospace" font-size="14">termal-export stub</text>
</svg>
"#.to_string())
}
