// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use crate::common::utils;

use crossterm::event::KeyCode;

use termal_msa::ui::key_handling;

#[test]
fn cap_g_moves_to_bottom() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        assert_eq!(0, ui.top_line());
        let key_cap_g = utils::keypress('G');
        key_handling::handle_key_press(ui, key_cap_g);
        assert_eq!(ui.max_top_line(), ui.top_line());
        // Idempotence at bottom
        key_handling::handle_key_press(ui, key_cap_g);
        assert_eq!(ui.max_top_line(), ui.top_line());
    });
}

#[test]
fn g_moves_to_top() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        let key_cap_g = utils::keypress('G');
        key_handling::handle_key_press(ui, key_cap_g);
        assert_eq!(ui.max_top_line(), ui.top_line());
        let key_g = utils::keypress('g');
        key_handling::handle_key_press(ui, key_g);
        assert_eq!(0, ui.top_line());
        // Idempotence at top
        key_handling::handle_key_press(ui, key_g);
        assert_eq!(0, ui.top_line());
    });
}

#[test]
fn jump_to_reference_when_consensus() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        // Reference is consensus by default
        let key_cap_g = utils::keypress('G');
        key_handling::handle_key_press(ui, key_cap_g);
        let initial_top = ui.top_line();
        assert!(initial_top > 0, "scroll down first");

        // vR should be no-op when reference is consensus
        key_handling::handle_key_press(ui, utils::keypress('v'));
        key_handling::handle_key_press(ui, utils::keypress('R'));
        assert_eq!(initial_top, ui.top_line(), "vR should be no-op for consensus");
    });
}

#[test]
fn jump_to_reference_when_specific_seq() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        // Set reference to seq 5 using R command
        key_handling::handle_key_press(ui, utils::keypress('R'));
        key_handling::handle_key_press(ui, utils::keypress('5'));
        key_handling::handle_key_press(ui, KeyCode::Enter.into());

        // Scroll far from seq 5
        let key_cap_g = utils::keypress('G');
        key_handling::handle_key_press(ui, key_cap_g);
        assert!(ui.top_line() > 5, "scroll down past seq 5");

        // vR should jump to seq 5 (and center it)
        key_handling::handle_key_press(ui, utils::keypress('v'));
        key_handling::handle_key_press(ui, utils::keypress('R'));

        // Reference should be visible and centered
        let ref_screenline = ui.app().rank_to_screenline(5);
        let top = ui.top_line() as usize;
        let visible = ui.max_nb_seq_shown() as usize;
        assert!(ref_screenline >= top && ref_screenline < top + visible,
                "reference seq 5 should be visible on screen");
    });
}

#[test]
fn jump_to_reference_ignores_prefix_count() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        // Set reference to seq 3 using R command
        key_handling::handle_key_press(ui, utils::keypress('R'));
        key_handling::handle_key_press(ui, utils::keypress('3'));
        key_handling::handle_key_press(ui, KeyCode::Enter.into());

        // Scroll down
        let key_cap_g = utils::keypress('G');
        key_handling::handle_key_press(ui, key_cap_g);
        assert!(ui.top_line() > 3);

        // Type "5vR" - the 5 should be ignored
        key_handling::handle_key_press(ui, utils::keypress('5'));
        key_handling::handle_key_press(ui, utils::keypress('v'));
        key_handling::handle_key_press(ui, utils::keypress('R'));

        // Reference should be visible and centered (prefix count ignored)
        let ref_screenline = ui.app().rank_to_screenline(3);
        let top = ui.top_line() as usize;
        let visible = ui.max_nb_seq_shown() as usize;
        assert!(ref_screenline >= top && ref_screenline < top + visible,
                "reference seq 3 should be visible; prefix count should not affect vault");
    });
}

#[test]
fn jump_leftmost() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        // Scroll right first
        let key_dollar = utils::keypress('$');
        key_handling::handle_key_press(ui, key_dollar);
        assert!(ui.leftmost_col() > 0, "scroll right first");

        // vl should vault to leftmost
        key_handling::handle_key_press(ui, utils::keypress('v'));
        key_handling::handle_key_press(ui, utils::keypress('l'));
        assert_eq!(0, ui.leftmost_col());
    });
}

#[test]
fn jump_rightmost() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        // Start at leftmost
        assert_eq!(0, ui.leftmost_col());

        // vr should vault to rightmost
        key_handling::handle_key_press(ui, utils::keypress('v'));
        key_handling::handle_key_press(ui, utils::keypress('r'));
        assert_eq!(ui.max_leftmost_col(), ui.leftmost_col());
    });
}

#[test]
fn jump_top() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        // Scroll down first
        let key_cap_g = utils::keypress('G');
        key_handling::handle_key_press(ui, key_cap_g);
        assert!(ui.top_line() > 0);

        // vt should vault to top
        key_handling::handle_key_press(ui, utils::keypress('v'));
        key_handling::handle_key_press(ui, utils::keypress('t'));
        assert_eq!(0, ui.top_line());
    });
}

#[test]
fn jump_bottom() {
    utils::with_rig("tests/data/test-motion.msa", 80, 50, |ui, _terminal| {
        // Start at top
        assert_eq!(0, ui.top_line());

        // vb should vault to bottom
        key_handling::handle_key_press(ui, utils::keypress('v'));
        key_handling::handle_key_press(ui, utils::keypress('b'));
        assert_eq!(ui.max_top_line(), ui.top_line());
    });
}
