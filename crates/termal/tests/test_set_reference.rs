// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_msa::ui::{key_handling, render};
use termal_msa::{
    app::{App, JumpTarget},
    ui::render::render_ui,
};
use termal_alignment::seq::fasta;

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

            // The last sequence should now be seq 4, header "corone"...
            let expected = "4│corone      │██│tatgcatatg";
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

            /* 
             *

            // Pressing 'n' should cause the modeline to change to "match #2/8"

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            assert!(
                last_line.contains("match #2/8"),
                "\"match #2/8\" not found on last line: {}",
                last_line
            );

            // Pressing 'n' another 7 times should cause the modeline to cycle back to "match #1/8"

            key_handling::handle_key_press(ui, utils::keypress('n'));
            key_handling::handle_key_press(ui, utils::keypress('n'));
            key_handling::handle_key_press(ui, utils::keypress('n'));
            key_handling::handle_key_press(ui, utils::keypress('n'));
            key_handling::handle_key_press(ui, utils::keypress('n'));
            key_handling::handle_key_press(ui, utils::keypress('n'));
            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            assert!(
                last_line.contains("match #1/8"),
                "\"match #1/8\" not found on last line: {}",
                last_line
            );

            // Pressing 'p' should cause the modeline to change to "match #8/8"

            key_handling::handle_key_press(ui, utils::keypress('p'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "match #8/8";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing 'n' another 7 times should cause the modeline to cycle back to "match #1/8"

            key_handling::handle_key_press(ui, utils::keypress('p'));
            key_handling::handle_key_press(ui, utils::keypress('p'));
            key_handling::handle_key_press(ui, utils::keypress('p'));
            key_handling::handle_key_press(ui, utils::keypress('p'));
            key_handling::handle_key_press(ui, utils::keypress('p'));
            key_handling::handle_key_press(ui, utils::keypress('p'));
            key_handling::handle_key_press(ui, utils::keypress('p'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "match #1/8";
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
            */
        },
    );
}

/*
#[test]
/// Tests a label search, for a label that is NOT found in the alignment.
fn test_missing_label_search() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            let key_double_quote = utils::keypress('"');
            let last_line_y = SCREEN_HEIGHT - 1;

            // We enter label search ("), then enter a label that's NOT in the alignment ("MISS")

            key_handling::handle_key_press(ui, key_double_quote);
            key_handling::handle_key_press(ui, utils::keypress('M'));
            key_handling::handle_key_press(ui, utils::keypress('I'));
            key_handling::handle_key_press(ui, utils::keypress('S'));
            key_handling::handle_key_press(ui, utils::keypress('S'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "Label search: MISS";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing Enter should cause "No match." to appear in the modeline

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "No match.";
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

#[test]
/// Tests that the Del key works as expected
fn test_label_search_del() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            let key_double_quote = utils::keypress('"');
            let last_line_y = SCREEN_HEIGHT - 1;

            // We enter label search ("), then enter a label "MISS")

            key_handling::handle_key_press(ui, key_double_quote);
            key_handling::handle_key_press(ui, utils::keypress('M'));
            key_handling::handle_key_press(ui, utils::keypress('I'));
            key_handling::handle_key_press(ui, utils::keypress('S'));
            key_handling::handle_key_press(ui, utils::keypress('S'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "Label search: MISS";
            assert!(
                last_line.contains(expected),
                "\"{}\" not found on last line: {}",
                expected,
                last_line
            );

            // Pressing Del then 'T' "Label search: MIST" to show in the modeline

            key_handling::handle_key_press(ui, KeyCode::Delete.into());
            key_handling::handle_key_press(ui, utils::keypress('T'));

            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "Label search: MIST";
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

*/
