// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use super::{JumpMode, UI};

pub fn execute(ui: &mut UI, cmd: &str) {
    ui.app.clear_msg();
    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["set", "case"] => ui.app.set_case_sensitive(true),
        ["set", "nocase"] => ui.app.set_case_sensitive(false),
        ["set", "jump", "lazy"] => ui.options.jump_mode = JumpMode::LazyCentered,
        ["set", "jump", "center"] => ui.options.jump_mode = JumpMode::AlwaysCenter,
        ["set", "lohi-threshold", val] | ["set", "lt", val] => match val.parse::<f64>() {
            Ok(v) if (0.0..=1.0).contains(&v) => {
                ui.app.options.lohi_high_threshold = v;
                ui.app.update_hi_metric_regions();
            }
            _ => ui.app.warning_msg(format!(
                "lohi-threshold: expected float in [0,1], got '{val}'"
            )),
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
