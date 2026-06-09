// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crate::common::utils;

use termal_msa::ui::{key_handling, render};

const SCREEN_WIDTH: u16 = 30;
const SCREEN_HEIGHT: u16 = 12;

#[test]
/// Tests a whole label search, for a label that is found in the alignment.
fn test_header_search() {
    utils::with_rig(
        "tests/data/test-seq-search.fas",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let penultimate_line_y = SCREEN_HEIGHT - 2;

            // Pressing c should cause the column metric to switch to 'coverage'
            key_handling::handle_key_press(ui, utils::keypress('c'));

            // Don't forget to draw the UI after the key event...
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let penultimate_line = utils::screen_line(buffer, penultimate_line_y);

            let expected = "Metric: coverage │██████████";
            assert!(
                penultimate_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                penultimate_line
            );

            // Pressing C should cause the column metric to switch back to 'entropy'
            // NOTE: as long as there are only two metrics, 'c' and 'C' behave identically, but as
            // soon as a third metric is introduced, the won't anymore.

            key_handling::handle_key_press(ui, utils::keypress('C'));

            // Don't forget to draw the UI after the key event...
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let penultimate_line = utils::screen_line(buffer, penultimate_line_y);

            let expected = "Metric: entropy  │▂▂ ▂ ▂▂█ █";
            assert!(
                penultimate_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                penultimate_line
            );
        },
    );
}
