// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_msa::ui::{key_handling, render};

const SCREEN_WIDTH: u16 = 60;
const SCREEN_HEIGHT: u16 = 12;

#[test]
fn help_dialog_opens_scrolls_resets_and_closes() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |mut ui, terminal| {
            key_handling::handle_key_press(ui, utils::keypress('?'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer().clone();
            let screen = utils::buffer_text(&buffer);

            assert!(
                screen.contains("j/k or arrows: scroll"),
                "help banner not rendered:\n{}",
                screen
            );
            assert!(
                screen.contains("# Main Key Bindings"),
                "top of help text not rendered:\n{}",
                screen
            );
            assert!(
                !screen.contains("## Zooming"),
                "lower help content should not be visible before scrolling:\n{}",
                screen
            );

            for _ in 0..20 {
                key_handling::handle_key_press(ui, utils::keypress('j'));
            }
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer().clone();
            let scrolled_screen = utils::buffer_text(&buffer);

            assert!(
                scrolled_screen.contains("## Zooming"),
                "scrolling did not reveal lower help content:\n{}",
                scrolled_screen
            );
            assert!(
                !scrolled_screen.contains("# Main Key Bindings"),
                "top of help text should have scrolled out of view:\n{}",
                scrolled_screen
            );

            key_handling::handle_key_press(ui, utils::keypress('g'));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer().clone();
            let reset_screen = utils::buffer_text(&buffer);

            assert!(
                reset_screen.contains("# Main Key Bindings"),
                "g did not reset help dialog to the top:\n{}",
                reset_screen
            );

            key_handling::handle_key_press(ui, utils::key(KeyCode::Esc));
            terminal.draw(|f| render::render_ui(f, &mut ui)).expect("update");
            let buffer = terminal.backend().buffer().clone();
            let closed_screen = utils::buffer_text(&buffer);

            assert!(
                !closed_screen.contains("j/k or arrows: scroll"),
                "help banner still visible after closing help:\n{}",
                closed_screen
            );
            assert!(
                !closed_screen.contains("# Main Key Bindings"),
                "help content still visible after closing help:\n{}",
                closed_screen
            );
        },
    );
}
