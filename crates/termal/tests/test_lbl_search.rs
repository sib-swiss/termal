// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_alignment::seq::fasta;
use termal_msa::ui::{key_handling, render};
use termal_msa::{
    app::{App, JumpTarget},
    ui::render::render_ui,
};

const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 50;

fn current_header_marker(app: &App) -> String {
    let screen_line = match app.current_match() {
        Some(JumpTarget::HeaderLine(screen_line)) => screen_line,
        _ => panic!("expected current header match"),
    };
    let rank = app.ordering[screen_line];
    let header = &app.alignment.headers[rank];
    let prefix: String = header.chars().take(10).collect();
    format!("{}│{}", rank + 1, prefix)
}

#[test]
/// Tests a whole label search, for a label that is found in the alignment.
fn test_label_search() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            let key_double_quote = utils::keypress('"');
            let last_line_y = SCREEN_HEIGHT - 1;

            // Pressing " should cause "Label search:" to appear on last line

            key_handling::handle_key_press(ui, key_double_quote);
            // Don't forget to draw the UI after the key event...
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            assert!(
                last_line.contains("Label search:"),
                "\"Label search\" not found on last line: {}",
                last_line
            );

            // Pressing K, F, and J should add 'KFJ' to the modeline argument
            //
            key_handling::handle_key_press(ui, utils::keypress('K'));
            key_handling::handle_key_press(ui, utils::keypress('F'));
            key_handling::handle_key_press(ui, utils::keypress('J'));
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            assert!(
                last_line.contains("Label search: KFJ"),
                "\"Label search: KFJ\" not found on last line: {}",
                last_line
            );

            // Pressing Enter should cause (1) a jump to the 1st matching seq (219) and (2) the text
            // "match #1/8" to appear in the modeline. The 1st match happens to be 14 lines from screen
            // bottom.

            let first_match_line_y = SCREEN_HEIGHT - 14;
            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let first_match_line = utils::screen_line(&buffer, first_match_line_y);

            assert!(
                first_match_line.contains("219│KFJ"), // might as well check line #
                "\"KFJ\" not found on l. {}: {}",
                first_match_line_y,
                first_match_line
            );

            let last_line = utils::screen_line(&buffer, last_line_y);

            assert!(
                last_line.contains("match #1/8"),
                "\"match #1/8\" not found on last line: {}",
                last_line
            );

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
        },
    );
}

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

#[test]
fn test_label_search_current_match_survives_reordering() {
    let seq_file = fasta::read_fasta_file("tests/data/test-motion.msa").expect("read");
    let aln = termal_alignment::Alignment::from_file(seq_file);
    let mut app = App::new("TEST", aln, None);
    app.regex_search_labels("KFJ");
    app.increment_current_match(2);
    app.display_current_match();
    app.next_ordering_criterion();
    let expected_current_after_reorder = current_header_marker(&app);
    app.increment_current_match(1);
    app.display_current_match();
    let expected_next_in_new_order = current_header_marker(&app);

    utils::with_rig("tests/data/test-motion.msa", 80, 15, |mut ui, terminal| {
        key_handling::handle_key_press(ui, utils::keypress('"'));
        key_handling::handle_key_press(ui, utils::keypress('K'));
        key_handling::handle_key_press(ui, utils::keypress('F'));
        key_handling::handle_key_press(ui, utils::keypress('J'));
        key_handling::handle_key_press(ui, KeyCode::Enter.into());
        key_handling::handle_key_press(ui, utils::keypress('n'));
        key_handling::handle_key_press(ui, utils::keypress('n'));

        key_handling::handle_key_press(ui, utils::keypress('o'));
        key_handling::handle_key_press(ui, KeyCode::Enter.into());
        terminal.draw(|f| render_ui(f, &mut ui)).expect("update");
        let buffer = terminal.backend().buffer().clone();
        let screen = utils::buffer_text(&buffer);

        assert!(
            screen.contains(&expected_current_after_reorder),
            "current header search match did not survive reordering or could not be jumped to.\nExpected marker: {}\nScreen:\n{}",
            expected_current_after_reorder,
            screen,
        );

        key_handling::handle_key_press(ui, utils::keypress('n'));
        terminal.draw(|f| render_ui(f, &mut ui)).expect("update");
        let buffer = terminal.backend().buffer().clone();
        let screen = utils::buffer_text(&buffer);

        assert!(
            screen.contains(&expected_next_in_new_order),
            "next match after reordering did not follow the new order.\nExpected marker: {}\nScreen:\n{}",
            expected_next_in_new_order,
            screen,
        );
    });
}

#[test]
/// Tests that passing a malformed regex causes the expected error message to appear in the
/// modeline
fn test_label_search_malformed() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            let key_double_quote = utils::keypress('"');
            let last_line_y = SCREEN_HEIGHT - 1;

            // We enter label search ("), then enter a malformed regex "["), then hit Enter. We expect
            // an error message saying that the regex is malformed.

            key_handling::handle_key_press(ui, key_double_quote);
            key_handling::handle_key_press(ui, utils::keypress('['));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal
                .draw(|f| render::render_ui(f, &mut ui))
                .expect("update");
            let buffer = terminal.backend().buffer();
            let last_line = utils::screen_line(&buffer, last_line_y);

            let expected = "ERROR: Malformed regex";
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
