// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_msa::ui::{key_handling, render};

// More than enough, but shouldn't harm.
const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 12;

#[test]
/// Checks the behaviour of the 'wl' command.
fn test_wl() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            // Pressing w should cause "w... [lbf<Esc>]" to appear on last line

            key_handling::handle_key_press(ui, utils::keypress('w'));
            // Don't forget to draw the UI after the key event...
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(buffer, last_line_y);

            let expected = "toggle pane: [l]eft [b]ottom [f]ull";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing l should cause the left pane to disappear

            key_handling::handle_key_press(ui, utils::keypress('l'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(buffer, last_line_y);

            // The last line should contain nothing but border, in particular, no vertical separator
            let expected =
                "└──────────────────────────────────────────────────────────────────────────────┘";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing wl again should restore the left panel

            key_handling::handle_key_press(ui, utils::keypress('w'));
            key_handling::handle_key_press(ui, utils::keypress('l'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(buffer, last_line_y);

            // The last line should show the separator again
            let expected = "└─────────────────└─";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );
        },
    );
}

#[test]
/// Checks the behaviour of the 'wb' command.
// See also test_reference_specifier() in alignment.rs
fn test_wb() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;
            let sep_line_y = 6;

            // Initially, line 6 should contain a separator between the top and bottom panes.

            // Draw the UI
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let sep_line = utils::screen_line(buffer, sep_line_y);

            let expected = "└───└──────────└──└";
            assert!(
                sep_line.contains(expected),
                "\"{}\" not found on sep line: {}",
                expected,
                sep_line
            );

            // Pressing w should cause "w... [lbf<Esc>]" to appear on last line

            key_handling::handle_key_press(ui, utils::keypress('w'));
            // Don't forget to draw the UI after the key event...
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(buffer, last_line_y);

            let expected = "toggle pane: [l]eft [b]ottom [f]ull";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing b should cause the bottom pane to disappear, and line sep_line_y should
            // contain header and sequence.

            key_handling::handle_key_press(ui, utils::keypress('b'));
            // Draw the UI
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let sep_line = utils::screen_line(buffer, sep_line_y);

            let expected =
                "│  6│LEACOLDL_0│█▊│------------MSDT-----------------------NSTSQNNTNS--------CGC║";
            assert!(
                sep_line.contains(expected),
                "\"{}\" not found on sep line: {}",
                expected,
                sep_line
            );

            // Pressing wb again should restore the bottom panel

            key_handling::handle_key_press(ui, utils::keypress('w'));
            key_handling::handle_key_press(ui, utils::keypress('b'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let sep_line = utils::screen_line(buffer, sep_line_y);

            let expected = "└───└──────────└──└";
            assert!(
                sep_line.contains(expected),
                "\"{}\" not found on sep line: {}",
                expected,
                sep_line
            );
        },
    );
}

#[test]
/// Checks the behaviour of the 'wf' command.
fn test_wf() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;
            let sep_line_y = 6;

            // Pressing w should cause "w... [lbf<Esc>]" to appear on last line

            key_handling::handle_key_press(ui, utils::keypress('w'));
            // Don't forget to draw the UI after the key event...
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(buffer, last_line_y);

            let expected = "toggle pane: [l]eft [b]ottom [f]ull";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing f should trigger full-screen mode

            key_handling::handle_key_press(ui, utils::keypress('f'));
            // Draw the UI
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let sep_line = utils::screen_line(buffer, sep_line_y);

            // Line 6 should now contain sequence and no header
            let expected =
                "------------MSDT-----------------------NSTSQNNTNS--------CGCGKT-VPKYPEQVTGMYLL║";
            assert!(
                sep_line.contains(expected),
                "\"{}\" not found on sep line: {}",
                expected,
                sep_line
            );

            // Pressing wf again should exit full-screen mode

            key_handling::handle_key_press(ui, utils::keypress('w'));
            key_handling::handle_key_press(ui, utils::keypress('f'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let sep_line = utils::screen_line(buffer, sep_line_y);

            // Line 6 should contain the separator again
            let expected = "└───└──────────└──└";
            assert!(
                sep_line.contains(expected),
                "\"{}\" not found on sep line: {}",
                expected,
                sep_line
            );
        },
    );
}

#[test]
/// Checks the behaviour of 'w<Esc>'.
fn test_w_esc() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            // Pressing w should cause "w... [lbf<Esc>]" to appear on last line

            key_handling::handle_key_press(ui, utils::keypress('w'));
            // Don't forget to draw the UI after the key event...
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(buffer, last_line_y);

            let expected = "toggle pane: [l]eft [b]ottom [f]ull";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing Esc should quit pane prefix mode
            //
            key_handling::handle_key_press(ui, KeyCode::Esc.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(buffer, last_line_y);

            // There should be no modeline message anymore
            let expected = "└─────────────────└─";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );
        },
    );
}

// Roadmap: "When the horizontal scrollbar is visible, pushing the labels pane all the way to the
// right (admittedly not a very frequent situation) causes a panic."
// test-motion.msa is 226 seqs x 60 cols, so on a narrow screen both scrollbars are shown.
#[test]
fn test_widen_label_pane_to_max_does_not_panic() {
    utils::with_rig("tests/data/test-motion.msa", 40, 12, |ui, terminal| {
        for i in 0..60 {
            key_handling::handle_key_press(ui, utils::keypress('>'));
            terminal
                .draw(|f| render::render_ui(f, ui))
                .unwrap_or_else(|e| panic!("draw failed after {} presses of '>': {}", i + 1, e));
        }
    });
}
