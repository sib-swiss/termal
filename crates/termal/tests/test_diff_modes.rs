// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_msa::ui::{key_handling, render};

// More than enough, but shouldn't harm.
const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 10;

#[test]
/// Checks the behaviour of the 'D and d*' commands.
fn test_diff_mode() {
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

            let expect = "diff mode: [d]iff [n]ormal";
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

            // Pressing 'D' should revert to normal mode 

            key_handling::handle_key_press(ui, utils::keypress('D'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();

            // Seq 1 should be AGGCTC
            let line_1 = utils::screen_line(&buffer, 1);
            let expect = "AGGCTC";
            assert!(
                line_1.contains(expect),
                "\"{}\" not found in seq 1: {}",
                expect,
                line_1
            );

            // Seq 2 should be AGGCAC

            let line_2 = utils::screen_line(&buffer, 2);
            let expect = "AGGCAC";
            assert!(
                line_2.contains(expect),
                "\"{}\" not found in seq 2: {}",
                expect,
                line_2
            );

            // Seq 3 should be AGCTTC (not because it's the ref (which it still is), but because
            // we're back in normal mode).

            let line_3 = utils::screen_line(&buffer, 3);
            let expect = "ACGTTC";
            assert!(
                line_3.contains(expect),
                "\"{}\" not found in seq 3: {}",
                expect,
                line_3
            );

            // Another way to switch to normal mode is 'dn' (of which D is just a shortcut). So if
            // we briefly enter diff mode again, then hit dn, we should be back in normal:

            // switch to diff
            key_handling::handle_key_press(ui, utils::keypress('d'));
            key_handling::handle_key_press(ui, utils::keypress('d'));
            // back to normal
            key_handling::handle_key_press(ui, utils::keypress('d'));
            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();

            // Seq 1 should be AGGCTC
            let line_1 = utils::screen_line(&buffer, 1);
            let expect = "AGGCTC";
            assert!(
                line_1.contains(expect),
                "\"{}\" not found in seq 1: {}",
                expect,
                line_1
            );

            // Seq 2 should be AGGCAC

            let line_2 = utils::screen_line(&buffer, 2);
            let expect = "AGGCAC";
            assert!(
                line_2.contains(expect),
                "\"{}\" not found in seq 2: {}",
                expect,
                line_2
            );

            // Seq 3 should be AGCTTC (not because it's the ref (which it still is), but because
            // we're back in normal mode).

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

#[test]
/// Checks the behaviour of diff mode under reorderings
fn test_diff_mode_reorder() {
    utils::with_rig(
        "tests/data/test-diff-modes.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {

            let last_line_y = 9;

            // Select seq 3 as reference

            key_handling::handle_key_press(ui, utils::keypress('R'));
            key_handling::handle_key_press(ui, utils::keypress('3'));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());

            // Enter diff mode

            key_handling::handle_key_press(ui, utils::keypress('d'));
            key_handling::handle_key_press(ui, utils::keypress('d'));

            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();

            // No reordering yet: rank order is 1, 2, 3
            
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

            // Seq 3 should be unchanged, since it is the reference

            let line_3 = utils::screen_line(&buffer, 3);
            let expect = "ACGTTC";
            assert!(
                line_3.contains(expect),
                "\"{}\" not found in seq 3: {}",
                expect,
                line_3
            );

            // Order by increasing similarity to reference: rank order is now 2, 1, 3

            key_handling::handle_key_press(ui, utils::keypress('o'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();

            // Seq 1 should be rank 2 (Epipactis) -G-CA-

            let line_1 = utils::screen_line(&buffer, 1);
            let expect = "-G-CA-";
            assert!(
                line_1.contains(expect),
                "\"{}\" not found in seq 1: {}",
                expect,
                line_1
            );

            // Seq 2 should be rank 1,  -G-C--

            let line_2 = utils::screen_line(&buffer, 2);
            let expect = "-G-C--";
            assert!(
                line_2.contains(expect),
                "\"{}\" not found in seq 2: {}",
                expect,
                line_2
            );

            // Seq 3 should be ACGTTC because it's the ref

            let line_3 = utils::screen_line(&buffer, 3);
            let expect = "ACGTTC";
            assert!(
                line_3.contains(expect),
                "\"{}\" not found in seq 3: {}",
                expect,
                line_3
            );

            // Order by decreasing similarity to reference: rank order is now 3, 1, 2

            key_handling::handle_key_press(ui, utils::keypress('o'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();

            // Seq 1 should be rank 1 (Limodorum), IOW the reference

            let line_1 = utils::screen_line(&buffer, 1);
            let expect = "ACGTTC";
            assert!(
                line_1.contains(expect),
                "\"{}\" not found in seq 1: {}",
                expect,
                line_1
            );

            // Seq 2 should be rank 1 (Anacamptis),  -G-C--

            let line_2 = utils::screen_line(&buffer, 2);
            let expect = "-G-C--";
            assert!(
                line_2.contains(expect),
                "\"{}\" not found in seq 2: {}",
                expect,
                line_2
            );

            // Seq 3 should be rank 2 (Epipactis) -G-CA- 

            let line_3 = utils::screen_line(&buffer, 3);
            let expect = "-G-CA-";
            assert!(
                line_3.contains(expect),
                "\"{}\" not found in seq 3: {}",
                expect,
                line_3
            );

        },
    );
}
