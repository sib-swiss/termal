// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use super::{JumpMode, UI};

#[allow(dead_code)]
fn show_regions(ui: &mut UI) {
    let regions = &ui.app.hi_col_metric_regions;
    if regions.is_empty() {
        ui.app.debug_msg("hi-metric regions: (none)".to_string());
    } else {
        // s and e are 0-based (start, length); convert to 1-based inclusive end for display.
        let list: Vec<String> = regions.iter().map(|(s, e)| format!("{}..{}", s+1, s+e)).collect();
        ui.app
            .debug_msg(format!("{} region(s): {}", list.len(), list.join("  ")));
    }
}

pub fn execute(ui: &mut UI, cmd: &str) {
    ui.app.clear_msg();
    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["set", "jump", "lazy"] => ui.options.jump_mode = JumpMode::LazyCentered,
        ["set", "jump", "center"] => ui.options.jump_mode = JumpMode::AlwaysCenter,
        ["set", "lohi-threshold", val] | ["set", "lt", val] => match val.parse::<f64>() {
            Ok(v) if (0.0..=1.0).contains(&v) => {
                ui.app.options.lohi_high_threshold = v;
                ui.app.update_hi_metric_regions();
            }
            _ => ui
                .app
                .warning_msg(format!("lohi-threshold: expected float in [0,1], got '{val}'")),
        },
        ["set", "lohi-gap", val] | ["set", "lg", val] => match val.parse::<usize>() {
            Ok(v) => {
                ui.app.options.lohi_gap_threshold = v;
                ui.app.update_hi_metric_regions();
            }
            _ => ui
                .app
                .warning_msg(format!("lohi-gap: expected integer, got '{val}'")),
        },
        _ => ui.app.warning_msg(format!("Unknown command: '{cmd}'")),
    }
}
