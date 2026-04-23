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
/// Checks the behaviour of the 'R' command.
// See also test_reference_specifier() in alignment.rs
fn test_set_reference() {
    utils::with_rig(
        "tests/data/test-set-ref.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;
            let ref_line_y = SCREEN_HEIGHT - 3;
            let last_seq_line_y = 5;

            // Before anything, the reference line should contain the consensus, namely
            // "tATGCATATG".

            // Draw the UI
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let ref_line = utils::screen_line(&buffer, ref_line_y);

            // Check the consensus
            assert!(
                ref_line.contains("tATGCATATG"),
                "\"tATGCATATG\" not found on ref line: {}",
                ref_line
            );

            // Pressing R should cause "Set ref:" to appear on last line

            key_handling::handle_key_press(ui, utils::keypress('R'));
            // Don't forget to draw the UI after the key event...
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            assert!(
                last_line.contains("Set ref:"),
                "\"Set ref:\" not found on last line: {}",
                last_line
            );

            // Pressing 1 should add '1' to the modeline argument
            //
            key_handling::handle_key_press(ui, utils::keypress('1'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            assert!(
                last_line.contains("Set ref: 1"),
                "\"Set ref: 1\" not found on last line: {}",
                last_line
            );

            // Pressing Enter should cause (1) the reference sequence to match the sequence of rank
            // 1, and (2) "Ref: #1" to appear in the bottom-left pane.

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let ref_line = utils::screen_line(&buffer, ref_line_y);

            // Check for "Ref: #1"
            assert!(
                ref_line.contains("Ref: #1"),
                "\"Ref: #1\" not found on last line: {}",
                ref_line
            );

            assert!(
                ref_line.contains("catgcatatg"), // rk 1 is now ref
                "\"catgcatatg\" not found on l. {}: {}",
                ref_line_y,
                ref_line
            );

            // Pressing 'o' should now cause the ref sequence (i.e. seq 1, 'frugilegus') to move to
            // _last_ position, because the order is now by increasing similarity to the reference,
            // namely sequence 1 itself.

            key_handling::handle_key_press(ui, utils::keypress('o'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_seq_line = utils::screen_line(&buffer, last_seq_line_y);

            // The last sequence should be seq 1, header "frugilegus" and sequence "catgcatatg", and
            // maximal similarity (██):
            let expected = "│1│frugilegus  │██│catgcatatg";
            assert!(
                last_seq_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_seq_line
            );

            // Pressing 'R<Enter>' should revert to the consensus (but NOT restore the original
            // ordering., which is by file).

            key_handling::handle_key_press(ui, utils::keypress('R'));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_seq_line = utils::screen_line(&buffer, last_seq_line_y);
            let ref_line = utils::screen_line(&buffer, ref_line_y);

            // The last sequence should now be seq 4, header "corone"... OR seq 3 "corax" - they
            // have the same sequence. So I'm not going to check the header.
            let expected = "│██│tatgcatatg";
            assert!(
                last_seq_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_seq_line
            );

            // The ref should revert to the consensus
            assert!(
                ref_line.contains("tATGCATATG"),
                "\"tATGCATATG\" not found on ref line: {}",
                ref_line
            );
        },
    );
}

#[test]
/// Checks the behaviour is the user enters an invalid (too small (0) or too large) ref number.
fn test_invalid_ref() {
    utils::with_rig(
        "tests/data/test-set-ref.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            // Pressing R0<Enter> should trigger a warning that no such ref exists, since there 
            // is no sequence #0 (for the user, that is).

            key_handling::handle_key_press(ui, utils::keypress('R'));
            key_handling::handle_key_press(ui, utils::keypress('0'));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            // Don't forget to draw the UI after the key event...
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expect = "Ref # must be > 0";
            assert!(
                last_line.contains(expect),
                "\"{}\" not found on last line: {}",
                expect,
                last_line
            );

            // Pressing R6<Enter> should trigger a warning that no such ref exists, since there are
            // only 5 sequences.

            key_handling::handle_key_press(ui, utils::keypress('R'));
            key_handling::handle_key_press(ui, utils::keypress('6'));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            // Don't forget to draw the UI after the key event...
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expect = "Ref # too large (max 5)";
            assert!(
                last_line.contains(expect),
                "\"{}\" not found on last line: {}",
                expect,
                last_line
            );


        },
    );
}

#[test]
/// Tests that the Del and Esc key work as expected
fn test_ref_spec_del_esc() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            // We enter ref spec (R), then start entering a ref #

            key_handling::handle_key_press(ui, utils::keypress('R'));
            key_handling::handle_key_press(ui, utils::keypress('1'));
            key_handling::handle_key_press(ui, utils::keypress('2'));
            key_handling::handle_key_press(ui, utils::keypress('3'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "Set ref: 123";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing Del then 'T' "Label search: MIST" to show in the modeline

            key_handling::handle_key_press(ui, KeyCode::Delete.into());
            key_handling::handle_key_press(ui, utils::keypress('9'));

            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "Set ref: 129";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing Esc should clear modeline

            key_handling::handle_key_press(ui, KeyCode::Esc.into());
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

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
