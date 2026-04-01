// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_msa::ui::{key_handling, render};

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
