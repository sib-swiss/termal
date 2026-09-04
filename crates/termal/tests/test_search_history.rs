// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_msa::ui::{key_handling, render};

const SCREEN_WIDTH: u16 = 80;
const SCREEN_HEIGHT: u16 = 50;

fn type_label_search(ui: &mut termal_msa::ui::UI, pattern: &str) {
    key_handling::handle_key_press(ui, utils::keypress('"'));
    for c in pattern.chars() {
        key_handling::handle_key_press(ui, utils::keypress(c));
    }
    key_handling::handle_key_press(ui, KeyCode::Enter.into());
}

fn type_seq_search(ui: &mut termal_msa::ui::UI, pattern: &str) {
    key_handling::handle_key_press(ui, utils::keypress('/'));
    for c in pattern.chars() {
        key_handling::handle_key_press(ui, utils::keypress(c));
    }
    key_handling::handle_key_press(ui, KeyCode::Enter.into());
}

#[test]
fn label_search_history_up_recalls_most_recent() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_label_search(ui, "KFJ");
            type_label_search(ui, "GGG");

            key_handling::handle_key_press(ui, utils::keypress('"'));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Hdr search: GGG"),
                "expected most recent search pattern recalled, got: {}",
                last_line
            );
        },
    );
}

#[test]
fn label_search_history_up_twice_recalls_older() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_label_search(ui, "KFJ");
            type_label_search(ui, "GGG");

            key_handling::handle_key_press(ui, utils::keypress('"'));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Hdr search: KFJ"),
                "expected older search pattern recalled, got: {}",
                last_line
            );
        },
    );
}

#[test]
fn label_search_history_up_stops_at_oldest() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_label_search(ui, "KFJ");
            type_label_search(ui, "GGG");
            type_label_search(ui, "AAA");

            key_handling::handle_key_press(ui, utils::keypress('"'));
            for _ in 0..5 {
                key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            }
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Hdr search: KFJ"),
                "expected to stay on oldest history entry rather than wrapping, got: {}",
                last_line
            );
        },
    );
}

#[test]
fn label_search_history_down_after_up_restores_buffer() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_label_search(ui, "KFJ");

            key_handling::handle_key_press(ui, utils::keypress('"'));
            key_handling::handle_key_press(ui, utils::keypress('X'));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Down));
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Hdr search: X"),
                "expected original in-progress buffer 'X' restored after Down, got: {}",
                last_line
            );
        },
    );
}

#[test]
fn seq_search_history_up_recalls_most_recent() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_seq_search(ui, "ACG");
            type_seq_search(ui, "TGC");

            key_handling::handle_key_press(ui, utils::keypress('/'));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Seq search: TGC"),
                "expected most recent seq search recalled, got: {}",
                last_line
            );
        },
    );
}

#[test]
fn seq_search_history_up_twice_recalls_older() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_seq_search(ui, "ACG");
            type_seq_search(ui, "TGC");

            key_handling::handle_key_press(ui, utils::keypress('/'));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Seq search: ACG"),
                "expected older seq search recalled, got: {}",
                last_line
            );
        },
    );
}

#[test]
fn seq_search_history_down_after_up_restores_buffer() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_seq_search(ui, "ACG");

            key_handling::handle_key_press(ui, utils::keypress('/'));
            key_handling::handle_key_press(ui, utils::keypress('T'));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Up));
            key_handling::handle_key_press(ui, utils::key(KeyCode::Down));
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Seq search: T"),
                "expected original in-progress buffer 'T' restored after Down, got: {}",
                last_line
            );
        },
    );
}
