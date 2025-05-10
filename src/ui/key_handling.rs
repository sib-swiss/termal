// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use log::debug;

use crate::{ZoomLevel, UI};

pub fn handle_key_press(ui: &mut UI, key_event: KeyEvent) -> bool {
    let mut done = false;

    if ui.show_help {
        ui.show_help = false;
    } else {
        // debug!("key event: {:#?}", key_event.code);
        match key_event.code {
            // Help
            KeyCode::Char('?') => {
                ui.show_help = true;
            }

            // ----- Hide/Show panes -----

            // Left pane
            KeyCode::Char('a') => {
                if ui.label_pane_width == 0 {
                    ui.show_label_pane();
                } else {
                    ui.hide_label_pane();
                }
            }

            // Bottom pane
            // Exception: Ctrl-C quits
            KeyCode::Char('c') if !key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    done = true;
                }
                if ui.bottom_pane_height == 0 {
                    ui.show_bottom_pane();
                } else {
                    ui.hide_bottom_pane();
                }
            }

            // Both panes
            KeyCode::Char('f') => {
                if ui.full_screen {
                    ui.show_label_pane();
                    ui.show_bottom_pane();
                    ui.full_screen = false;
                } else {
                    ui.hide_label_pane();
                    ui.hide_bottom_pane();
                    ui.full_screen = true;
                }
            }

            // ----- Motion -----

            // Arrows - late introduction, but might be friendlier to new users.
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                // Non-shifted arrow keys
                if !key_event.modifiers.contains(KeyModifiers::SHIFT) {
                    match key_event.code {
                        KeyCode::Down => match ui.zoom_level() {
                            ZoomLevel::ZoomedIn => ui.scroll_one_line_down(),
                            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                                ui.scroll_zoombox_one_line_down()
                            }
                        },
                        KeyCode::Up => match ui.zoom_level() {
                            ZoomLevel::ZoomedIn => ui.scroll_one_line_up(),
                            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                                ui.scroll_zoombox_one_line_up()
                            }
                        },
                        KeyCode::Right => match ui.zoom_level() {
                            ZoomLevel::ZoomedIn => ui.scroll_one_col_right(),
                            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                                ui.scroll_zoombox_one_col_right()
                            }
                        },
                        KeyCode::Left => match ui.zoom_level() {
                            ZoomLevel::ZoomedIn => ui.scroll_one_col_left(),
                            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                                ui.scroll_zoombox_one_col_left()
                            }
                        },

                        _ => panic!("Expected only arrow keycodes"),
                    }
                } else {
                    // Shifted arrow keys
                    match key_event.code {
                        KeyCode::Down => ui.scroll_one_screen_down(),
                        KeyCode::Up => ui.scroll_one_screen_up(),
                        KeyCode::Right => ui.scroll_one_screen_right(),
                        KeyCode::Left => ui.scroll_one_screen_left(),

                        _ => panic!("Expected only arrow keycodes"),
                    }
                }
            }

            // Down
            KeyCode::Char('j') => match ui.zoom_level() {
                ZoomLevel::ZoomedIn => ui.scroll_one_line_down(),
                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => ui.scroll_zoombox_one_line_down(),
            },
            KeyCode::Char('J') | KeyCode::Char(' ') => ui.scroll_one_screen_down(),
            KeyCode::Char('G') => ui.jump_to_bottom(),

            // Up
            KeyCode::Char('k') => match ui.zoom_level() {
                ZoomLevel::ZoomedIn => ui.scroll_one_line_up(),
                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => ui.scroll_zoombox_one_line_up(),
            },
            KeyCode::Char('K') => ui.scroll_one_screen_up(),
            KeyCode::Char('g') => ui.jump_to_top(),

            // Right
            KeyCode::Char('l') => match ui.zoom_level() {
                ZoomLevel::ZoomedIn => ui.scroll_one_col_right(),
                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => ui.scroll_zoombox_one_col_right(),
            },
            KeyCode::Char('L') => ui.scroll_one_screen_right(),
            KeyCode::Char('$') => ui.jump_to_end(),

            // Left
            KeyCode::Char('h') => match ui.zoom_level() {
                ZoomLevel::ZoomedIn => ui.scroll_one_col_left(),
                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => ui.scroll_zoombox_one_col_left(),
            },
            KeyCode::Char('H') => ui.scroll_one_screen_left(),
            KeyCode::Char('^') => ui.jump_to_begin(),

            // Label Pane width
            // NOTE: for these methods I'm using a more general approach than for
            // motion: pass the argument instead of having separate functions for
            // each increment.
            KeyCode::Char('>') => ui.widen_label_pane(1),
            KeyCode::Char('<') => ui.reduce_label_pane(1),

            // Zoom
            KeyCode::Char('z') => ui.cycle_zoom(),
            // Since there are 3 zoom levels, cycling twice amounts to cycling
            // backwards.
            KeyCode::Char('Z') => {
                ui.cycle_zoom();
                ui.cycle_zoom();
            }
            // Toggle zoom box guides
            KeyCode::Char('v') => {
                ui.set_zoombox_guides(!ui.show_zb_guides);
            }
            // Toggle zoom box visibility
            KeyCode::Char('B') => {
                ui.toggle_zoombox();
            }

            // Bottom pane position (i.e., bottom of screen or stuck to the alignment - when both
            // are possible).
            KeyCode::Char('b') => {
                ui.cycle_bottom_pane_position();
                debug!(
                    "-- Toggling bottom pane position - now {:?}  --",
                    ui.bottom_pane_position
                );
            }

            // ---- Visuals ----

            // Mark consensus positions that are retained in the zoom box
            KeyCode::Char('r') => ui.toggle_hl_retained_cols(),

            // Inverse video
            KeyCode::Char('i') => {
                ui.inverse = !ui.inverse;
            }

            KeyCode::Char('d') => ui.toggle_theme(),

            // Cycle through colormaps
            KeyCode::Char('m') => ui.cycle_colormap(),

            // Sequence Order
            KeyCode::Char('o') => ui.cycle_ordering_criterion(),

            // Metric
            // TODO: this directl< calls the method in App, while the above call a method in UI
            // (which is just a wrapper around an App counterpart). Make up your mind, dude...
            KeyCode::Char('t') => {
                ui.app.cycle_metric();
            }

            // ----  Exit ----
            KeyCode::Char('q') | KeyCode::Char('Q') => done = true,
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                done = true
            }

            _ => {
                // let the user know this key is not bound
                //
                // TODO: there are pros and cons about this - first, the user can probably guess
                // that if nothing happens then the key isn't bound. Second, the message should be
                // disabled after the user presses a bound key, which would force us to either add
                // code to that effect for _every single_ key binding, or do a first match on every
                // valid key (to disable the message) and then match on each individual key to
                // launch the desired action. Not sure it's worth it, frankly.
                //
                // ui.message = format!("Key '{:#?}' is not bound.", key_event.code);
            }
        }
    }

    done
}
