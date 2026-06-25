// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crossterm::event::KeyCode;
use termal_alignment::{seq::fasta, Alignment};

use crate::common::utils;

use termal_msa::{
    app::{App, JumpTarget},
    seq_match::SequenceSearchTarget,
    ui::{key_handling, render},
};

const SCREEN_WIDTH: u16 = 25;
const SCREEN_HEIGHT: u16 = 20;

#[test]
fn test_sequence_search() {
    utils::with_rig(
        "tests/data/test-seq-search.fas",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('/'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Seq search:"),
                "\"Seq search:\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, utils::keypress('t'));
            key_handling::handle_key_press(ui, utils::keypress('a'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Seq search: tat"),
                "\"Seq search: tat\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #1/4"),
                "\"match #1/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 4);

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #2/4"),
                "\"match #2/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 0);

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #3/4"),
                "\"match #3/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 4);

            key_handling::handle_key_press(ui, utils::keypress('p'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #2/4"),
                "\"match #2/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 0);
        },
    );
}

#[test]
fn test_sequence_search_prefix() {
    utils::with_rig(
        "tests/data/test-seq-search.fas",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('f'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            let expected = "find: [h]eaders [s]eq"; // screen is too narrow for whole msg
            assert!(
                screen.contains(expected),
                "\"{}\" not found on screen:\n{}",
                expected,
                screen
            );

            key_handling::handle_key_press(ui, utils::keypress('s'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Seq search:"),
                "\"Seq search:\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, utils::keypress('t'));
            key_handling::handle_key_press(ui, utils::keypress('a'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Seq search: tat"),
                "\"Seq search: tat\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #1/4"),
                "\"match #1/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 4);

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #2/4"),
                "\"match #2/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 0);

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #3/4"),
                "\"match #3/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 4);

            key_handling::handle_key_press(ui, utils::keypress('p'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #2/4"),
                "\"match #2/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 0);
        },
    );
}

#[test]
fn test_sequence_search_respects_reordered_traversal() {
    const REORDERED_SCREEN_HEIGHT: u16 = 8;

    let seq_file = fasta::read_fasta_file("tests/data/test-seq-search.fas").expect("read");
    let aln = Alignment::from_file(seq_file);
    let mut app = App::new("TEST", aln, None);
    app.next_ordering_criterion();
    app.regex_search_seq("tat", SequenceSearchTarget::BiologicalSequence);
    app.display_current_match();
    let (first_screenline, first_start_col) = match app.current_match() {
        Some(JumpTarget::Match(screenline, match_pos)) => (screenline, match_pos.start_col()),
        _ => panic!("expected current sequence match"),
    };
    let expected_initial_message = app.current_message().message.clone();
    app.increment_current_match(1);
    app.display_current_match();
    let (next_screenline, next_start_col) = match app.current_match() {
        Some(JumpTarget::Match(screenline, match_pos)) => (screenline, match_pos.start_col()),
        _ => panic!("expected next sequence match"),
    };
    let expected_next_message = app.current_message().message.clone();

    utils::with_rig(
        "tests/data/test-seq-search.fas",
        SCREEN_WIDTH,
        REORDERED_SCREEN_HEIGHT,
        |ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('o'));
            key_handling::handle_key_press(ui, utils::keypress('/'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            key_handling::handle_key_press(ui, utils::keypress('a'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let screen = utils::buffer_text(terminal.backend().buffer());

            assert!(
                screen.contains(&expected_initial_message),
                "initial sequence-search message did not follow reordered traversal.\nExpected message: {}\nScreen:\n{}",
                expected_initial_message,
                screen
            );
            // Expected positions reflect lazy-center scroll behavior
            let vh = ui.max_nb_seq_shown() as usize;
            let vw = ui.max_nb_col_shown() as usize;
            let expected_top_line =
                (first_screenline.saturating_sub(vh / 2) as u16).min(ui.max_top_line());
            let expected_leftmost_col =
                (first_start_col.saturating_sub(vw / 2) as u16).min(ui.max_leftmost_col());
            assert_eq!(ui.top_line(), expected_top_line);
            assert_eq!(ui.leftmost_col(), expected_leftmost_col);

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let screen = utils::buffer_text(terminal.backend().buffer());

            assert!(
                screen.contains(&expected_next_message),
                "next sequence-search message did not follow reordered traversal.\nExpected message: {}\nScreen:\n{}",
                expected_next_message,
                screen
            );
            let expected_next_top_line =
                (next_screenline.saturating_sub(vh / 2) as u16).min(ui.max_top_line());
            let expected_next_leftmost_col =
                (next_start_col.saturating_sub(vw / 2) as u16).min(ui.max_leftmost_col());
            assert_eq!(ui.top_line(), expected_next_top_line);
            assert_eq!(ui.leftmost_col(), expected_next_leftmost_col);
        },
    );
}

#[test]
/// Checks that / (= fs) searches ignore gaps
fn test_biol_seq_search() {
    utils::with_rig(
        "tests/data/test-seq-search-gapped.fas",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('/'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Seq search:"),
                "\"Seq search:\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, utils::keypress('c'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Seq search: ct"),
                "\"Seq search: ct\" not found on screen:\n{}",
                screen
            );

            // After pressing Enter, the modeline should display 'match #1/2' <- there are 2
            // matches and we're on the first one. Note that both matches (c-t and c---t, on l. 2)
            // have gaps

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #1/2"),
                "\"match #1/2\" not found on screen:\n{}",
                screen
            );
        },
    );
}

#[test]
/// Checks that alignment (= fa) searches do NOT ignore gaps
fn test_alignment_search() {
    utils::with_rig(
        "tests/data/test-seq-search-gapped.fas",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('f'));
            key_handling::handle_key_press(ui, utils::keypress('a'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Aln search:"),
                "\"Aln search:\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, utils::keypress('c'));
            key_handling::handle_key_press(ui, utils::keypress('-'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Aln search: c-t"),
                "\"Aln search: c-t\" not found on screen:\n{}",
                screen
            );

            // After pressing Enter, the modeline should display 'match #1/1' <- there is exactly 1
            // match.

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("match #1/1"),
                "\"match #1/1\" not found on screen:\n{}",
                screen
            );

            // Now we match 'ct' against the alignment, i.e. taking gaps into account - there
            // should be no match.

            key_handling::handle_key_press(ui, utils::keypress('f'));
            key_handling::handle_key_press(ui, utils::keypress('a'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Aln search:"),
                "\"Aln search:\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, utils::keypress('c'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            assert!(
                screen.contains("Aln search: ct"),
                "\"Aln search: ct\" not found on screen:\n{}",
                screen
            );

            // After pressing Enter, the modeline should display 'No match'.

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(buffer);
            let expected = "No match.";
            assert!(
                screen.contains(expected),
                "\"{}\" not found on screen:\n{}",
                expected,
                screen
            );
        },
    );
}
