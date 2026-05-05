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
/// Checks the behaviour of the 'D and d*' commands.
// See also test_reference_specifier() in alignment.rs
fn test_set_reference() {
    utils::with_rig(
        "tests/data/test-diff-modes.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {

            let last_line_y = 9;

            // Pressing d should cause "d... [dn]" to appear on last line

            key_handling::handle_key_press(ui, utils::keypress('d'));
            // Don't forget to draw the UI after the key event...
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expect = "d... [dn]";
            assert!(
                last_line.contains(expect),
                "\"{}\" not found on last line: {}",
                expect,
                last_line
            );

            // Pressing d should switch to difference mode. Seq 1 should now show only -----, as it
            // happens to be identical to the reference (which by default is the consensus).
            //
            key_handling::handle_key_press(ui, utils::keypress('d'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();

            let line_1 = utils::screen_line(&buffer, 1);
            let expect = "-----";
            assert!(
                line_1.contains(expect),
                "\"{}\" not found in seq 1: {}",
                expect,
                line_1
            );

            // Seq 2 should be ----A-, as it only differs from the ref at position 5.

            let line_2 = utils::screen_line(&buffer, 2);
            let expect = "---A-";
            assert!(
                line_2.contains(expect),
                "\"{}\" not found in seq 2: {}",
                expect,
                line_2
            );

            // Seq 3 should be -C-T--, as it differs at 2 and 4

            let line_3 = utils::screen_line(&buffer, 3);
            let expect = "-C-T-";
            assert!(
                line_3.contains(expect),
                "\"{}\" not found in seq 3: {}",
                expect,
                line_3
            );

            // Now let us change the reference to sequence 3.

            key_handling::handle_key_press(ui, utils::keypress('R'));
            key_handling::handle_key_press(ui, utils::keypress('3'));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());

            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();

            // Seq 1 should be -G-C--
            let line_1 = utils::screen_line(&buffer, 1);
            let expect = "-G-C-";
            assert!(
                line_1.contains(expect),
                "\"{}\" not found in seq 1: {}",
                expect,
                line_1
            );

            // Seq 2 should be -G-CA-, as it only differs from the ref at position 5.

            let line_2 = utils::screen_line(&buffer, 2);
            let expect = "-G-CA-";
            assert!(
                line_2.contains(expect),
                "\"{}\" not found in seq 2: {}",
                expect,
                line_2
            );

            // Seq 3 should be unchanged, since it is the now reference

            let line_3 = utils::screen_line(&buffer, 3);
            let expect = "ACGTTC";
            assert!(
                line_3.contains(expect),
                "\"{}\" not found in seq 3: {}",
                expect,
                line_3
            );

        },
    );
}
