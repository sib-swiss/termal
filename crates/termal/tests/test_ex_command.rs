// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crossterm::event::KeyCode;

use crate::common::utils;

use termal_msa::ui::{key_handling, render, JumpMode};

const SCREEN_WIDTH: u16 = 60;
const SCREEN_HEIGHT: u16 = 12;

fn type_ex_command(ui: &mut termal_msa::ui::UI, cmd: &str) {
    key_handling::handle_key_press(ui, utils::keypress(':'));
    for c in cmd.chars() {
        key_handling::handle_key_press(ui, utils::keypress(c));
    }
    key_handling::handle_key_press(ui, KeyCode::Enter.into());
}

#[test]
fn colon_enters_ex_command_mode() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            key_handling::handle_key_press(ui, utils::keypress(':'));
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains(':'),
                "':' not shown in modeline after entering ExCommand mode: {}",
                last_line
            );
        },
    );
}

#[test]
fn ex_command_esc_returns_to_normal() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            key_handling::handle_key_press(ui, utils::keypress(':'));
            key_handling::handle_key_press(ui, utils::keypress('s'));
            key_handling::handle_key_press(ui, KeyCode::Esc.into());
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            // Modeline should show no command prompt
            assert!(
                !last_line.contains(":s"),
                "command prompt still visible after Esc: {}",
                last_line
            );
        },
    );
}

#[test]
fn set_jump_center_changes_jump_mode() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, _terminal| {
            assert_eq!(
                ui.options.jump_mode,
                JumpMode::LazyCentered,
                "default should be LazyCentered"
            );
            type_ex_command(ui, "set jump center");
            assert_eq!(ui.options.jump_mode, JumpMode::AlwaysCenter);
        },
    );
}

#[test]
fn set_jump_lazy_restores_default() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, _terminal| {
            type_ex_command(ui, "set jump center");
            assert_eq!(ui.options.jump_mode, JumpMode::AlwaysCenter);
            type_ex_command(ui, "set jump lazy");
            assert_eq!(ui.options.jump_mode, JumpMode::LazyCentered);
        },
    );
}

#[test]
fn set_lohi_threshold_updates_option() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, _terminal| {
            type_ex_command(ui, "set lohi-threshold 0.5");
            assert!(
                (ui.app().options.lohi_high_threshold - 0.5).abs() < 1e-10,
                "expected 0.5, got {}",
                ui.app().options.lohi_high_threshold
            );
        },
    );
}

#[test]
fn set_lohi_gap_updates_option() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, _terminal| {
            type_ex_command(ui, "set lohi-gap 7");
            assert_eq!(ui.app().options.lohi_gap_threshold, 7);
        },
    );
}

#[test]
fn unknown_command_shows_warning() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;

            type_ex_command(ui, "frobnicate");
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("Unknown command"),
                "expected warning for unknown command, got: {}",
                last_line
            );
        },
    );
}

#[test]
fn invalid_lohi_threshold_shows_warning() {
    utils::with_rig(
        "tests/data/test-motion.msa",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        |ui, terminal| {
            let last_line_y = SCREEN_HEIGHT - 1;
            let original = ui.app().options.lohi_high_threshold;

            type_ex_command(ui, "set lohi-threshold banana");
            terminal.draw(|f| render::render_ui(f, ui)).expect("draw");
            let last_line = utils::screen_line(terminal.backend().buffer(), last_line_y);

            assert!(
                last_line.contains("lohi-threshold"),
                "expected error message mentioning 'lohi-threshold', got: {}",
                last_line
            );
            assert_eq!(
                ui.app().options.lohi_high_threshold,
                original,
                "threshold should be unchanged after bad input"
            );
        },
    );
}
