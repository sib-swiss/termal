use anyhow::Result;

use termal_alignment::Alignment;

use crate::ExportOpts;

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

fn svg_open(width: f32, height: f32) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, width, height)
}

fn svg_close() -> String {
    "</svg>".to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_svg_open() {
        assert_eq!(svg_open(200.0, 80.0), r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="80">"#);
    }

    #[test]
    fn test_svg_close() { assert_eq!(svg_close(), "</svg>"); }
}
