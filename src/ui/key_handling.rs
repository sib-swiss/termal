use crossterm::event::{KeyCode, KeyEvent};

use log::debug;

use crate::{ZoomLevel, UI};

pub fn handle_key_press(ui: &mut UI, key_event: KeyEvent) -> bool {
    let mut done = false;

    if ui.show_help {
        ui.show_help = false;
    } else {
        match key_event.code {
            // Help
            KeyCode::Char('?') => {
                ui.show_help = true;
            }
            // ----- Motion -----

            // Down
            KeyCode::Char('j') => match ui.zoom_level() {
                ZoomLevel::ZoomedIn => ui.scroll_one_line_down(),
                ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => ui.scroll_zoombox_one_line_down(),
            },
            KeyCode::Char('J') => ui.scroll_one_screen_down(),
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

            // Bottom pane position (i.e., bottom of screen or stuck to the alignment - when both
            // are possible).
            KeyCode::Char('b') => {
                ui.cycle_bottom_pane_position();
                debug!(
                    "-- Toggling bottom pane position - now {:?}  --",
                    ui.bottom_pane_position
                );
            }

            // Mark consensus positions that are retained in the zoom box
            KeyCode::Char('r') => ui.toggle_hl_retained_cols(),
            // Exit
            KeyCode::Char('q') => done = true,
            KeyCode::Char('Q') => done = true,
            _ => {}
        }
    }

    done
}
