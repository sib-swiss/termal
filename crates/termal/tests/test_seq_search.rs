// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crossterm::event::KeyCode;
use termal_alignment::{seq::fasta, Alignment};

use crate::common::utils;

use termal_msa::{
    app::{App, JumpTarget},
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
        |mut ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('/'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(&buffer);
            assert!(
                screen.contains("Sequence search:"),
                "\"Sequence search:\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, utils::keypress('t'));
            key_handling::handle_key_press(ui, utils::keypress('a'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(&buffer);
            assert!(
                screen.contains("Sequence search: tat"),
                "\"Sequence search: tat\" not found on screen:\n{}",
                screen
            );

            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(&buffer);
            assert!(
                screen.contains("match #1/4"),
                "\"match #1/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 5);

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(&buffer);
            assert!(
                screen.contains("match #2/4"),
                "\"match #2/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 0);

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(&buffer);
            assert!(
                screen.contains("match #3/4"),
                "\"match #3/4\" not found on screen:\n{}",
                screen
            );
            assert_eq!(ui.leftmost_col(), 5);

            key_handling::handle_key_press(ui, utils::keypress('p'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer();
            let screen = utils::buffer_text(&buffer);
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
    app.regex_search_seq("tat");
    app.display_current_match();
    let (expected_top_line, expected_leftmost_col) = match app.current_match() {
        Some(JumpTarget::Match(screenline, match_pos)) => {
            (screenline as u16, match_pos.start_col() as u16)
        }
        _ => panic!("expected current sequence match"),
    };
    let expected_initial_message = app.current_message().message.clone();
    app.increment_current_match(1);
    app.display_current_match();
    let (expected_next_top_line, expected_next_leftmost_col) = match app.current_match() {
        Some(JumpTarget::Match(screenline, match_pos)) => {
            (screenline as u16, match_pos.start_col() as u16)
        }
        _ => panic!("expected next sequence match"),
    };
    let expected_next_message = app.current_message().message.clone();

    utils::with_rig(
        "tests/data/test-seq-search.fas",
        SCREEN_WIDTH,
        REORDERED_SCREEN_HEIGHT,
        |mut ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('o'));
            key_handling::handle_key_press(ui, utils::keypress('/'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            key_handling::handle_key_press(ui, utils::keypress('a'));
            key_handling::handle_key_press(ui, utils::keypress('t'));
            key_handling::handle_key_press(ui, KeyCode::Enter.into());
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let screen = utils::buffer_text(terminal.backend().buffer());

            assert!(
                screen.contains(&expected_initial_message),
                "initial sequence-search message did not follow reordered traversal.\nExpected message: {}\nScreen:\n{}",
                expected_initial_message,
                screen
            );
            assert_eq!(ui.top_line(), expected_top_line);
            assert_eq!(
                ui.leftmost_col(),
                expected_leftmost_col.min(ui.max_leftmost_col())
            );

            key_handling::handle_key_press(ui, utils::keypress('n'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let screen = utils::buffer_text(terminal.backend().buffer());

            assert!(
                screen.contains(&expected_next_message),
                "next sequence-search message did not follow reordered traversal.\nExpected message: {}\nScreen:\n{}",
                expected_next_message,
                screen
            );
            assert_eq!(ui.top_line(), expected_next_top_line);
            assert_eq!(
                ui.leftmost_col(),
                expected_next_leftmost_col.min(ui.max_leftmost_col())
            );
        },
    );
}
