// SPDX-License-Identifier: MIT
//
// Copyright (c) 2025-2026 Thomas Junier
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::seq_match::SequenceSearchTarget;

use super::{
    ex_command, DiffMode, InputMode, {ZoomLevel, UI},
};

pub fn handle_key_press(ui: &mut UI, key_event: KeyEvent) -> bool {
    let mut done = false;
    let mode = ui.input_mode.clone();
    match mode {
        InputMode::Normal => done = handle_normal_key(ui, key_event),
        InputMode::Help => handle_help_key(ui, key_event),
        InputMode::PendingCount { count } => done = handle_pending_count_key(ui, key_event, count),
        InputMode::LabelSearch { pattern } => handle_label_search(ui, key_event, &pattern),
        InputMode::Search { pattern, target } => {
            handle_sequence_search(ui, key_event, &pattern, target)
        }
        InputMode::SetReference { ref_spec } => handle_set_reference(ui, key_event, &ref_spec),
        InputMode::PaneCmdPrefix => handle_pane_prefix(ui, key_event),
        InputMode::DiffCmdPrefix => handle_diff_prefix(ui, key_event),
        InputMode::SearchCmdPrefix => handle_search_prefix(ui, key_event),
        InputMode::ExCommand {
            cmd,
            history_prefix,
            history_idx,
        } => handle_ex_command(ui, key_event, &cmd, history_prefix, history_idx),
    };
    done
}

fn handle_help_key(ui: &mut UI, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
            ui.input_mode = InputMode::Normal;
        }
        KeyCode::Up | KeyCode::Char('k') => ui.scroll_help_up(1),
        KeyCode::Down | KeyCode::Char('j') => ui.scroll_help_down(1, u16::MAX),
        KeyCode::Char('g') => ui.reset_help_scroll(),
        _ => {}
    }
}

fn handle_normal_key(ui: &mut UI, key_event: KeyEvent) -> bool {
    let mut done = false;
    match key_event.code {
        // 1-9: enter pending count mode
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
            let d = (c as u8 - b'0') as usize;
            ui.input_mode = InputMode::PendingCount { count: d };
            ui.app.clear_msg();
            ui.app.add_argument_char(c);
        }
        KeyCode::Esc => {
            ui.app.reset_lbl_search();
            ui.app.clear_msg();
        }
        // Q, q, and Ctrl-C quit
        KeyCode::Char('q') | KeyCode::Char('Q') => done = true,
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => done = true,
        KeyCode::Char('?') => ui.input_mode = InputMode::Help,
        KeyCode::Char('"') => {
            // shortcut for fh
            ui.input_mode = InputMode::LabelSearch {
                pattern: String::from(""),
            };
            ui.app
                .argument_msg(String::from("Hdr search: "), String::from(""));
        }
        KeyCode::Char('/') => {
            // shortcut for fs
            ui.input_mode = InputMode::Search {
                pattern: String::from(""),
                target: SequenceSearchTarget::BiologicalSequence,
            };
            ui.app
                .argument_msg(String::from("Seq search: "), String::from(""));
        }
        KeyCode::Char('R') => {
            ui.input_mode = InputMode::SetReference {
                ref_spec: String::from(""),
            };
            ui.app
                .argument_msg(String::from("Set ref (#): "), String::from(""));
        }
        KeyCode::Char('w') => {
            ui.input_mode = InputMode::PaneCmdPrefix;
            ui.app.argument_msg(
                String::from("toggle pane: [l]eft [b]ottom [f]ull"),
                String::from(""),
            );
        }
        KeyCode::Char('d') => {
            ui.input_mode = InputMode::DiffCmdPrefix;
            ui.app
                .argument_msg(String::from("diff mode: [d]iff [n]ormal"), String::from(""));
        }
        KeyCode::Char('f') => {
            ui.input_mode = InputMode::SearchCmdPrefix;
            ui.app.argument_msg(
                String::from("find: [h]eaders [s]equences [a]lignment"),
                String::from(""),
            );
        }
        KeyCode::Char(':') => {
            ui.input_mode = InputMode::ExCommand {
                cmd: String::new(),
                history_prefix: None,
                history_idx: None,
            };
            ui.app.argument_msg(String::from(":"), String::from(""));
        }
        // Anything else: dispatch corresponding command, without count
        _ => dispatch_command(ui, key_event, None),
    }
    done
}

fn handle_pending_count_key(ui: &mut UI, key_event: KeyEvent, count: usize) -> bool {
    let mut done = false;
    match key_event.code {
        KeyCode::Char(c) if c.is_ascii_digit() => {
            let d = (c as u8 - b'0') as usize;
            let updated_count = count.saturating_mul(10).saturating_add(d);
            ui.input_mode = InputMode::PendingCount {
                count: updated_count,
            };
            ui.app.add_argument_char(c);
        }
        // Q, q, and Ctrl-C quit (TODO: should they really quit when in pending-count mode?)
        KeyCode::Char('q') | KeyCode::Char('Q') => done = true,
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => done = true,
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        _ => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
            dispatch_command(ui, key_event, Some(count));
        }
    }
    done
}

fn handle_label_search(ui: &mut UI, key_event: KeyEvent, pattern: &str) {
    match key_event.code {
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char(c) if c.is_ascii_graphic() || ' ' == c => {
            ui.app.add_argument_char(c);
            let mut updated_pattern = pattern.to_string();
            updated_pattern.push(c);
            ui.input_mode = InputMode::LabelSearch {
                pattern: updated_pattern,
            }
        }
        KeyCode::Delete | KeyCode::Backspace => {
            ui.app.pop_argument_char();
            let mut updated_pattern = pattern.to_string();
            updated_pattern.pop();
            ui.input_mode = InputMode::LabelSearch {
                pattern: updated_pattern,
            };
        }
        KeyCode::Enter => {
            ui.app.regex_search_labels(pattern);
            ui.input_mode = InputMode::Normal;
            if ui.app.search_state.is_some() {
                ui.jump_to_next_match(0);
            }
        }
        _ => {}
    }
}

fn handle_sequence_search(
    ui: &mut UI,
    key_event: KeyEvent,
    pattern: &str,
    target: SequenceSearchTarget,
) {
    match key_event.code {
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char(c) if c.is_ascii_graphic() || ' ' == c => {
            ui.app.add_argument_char(c);
            let mut updated_pattern = pattern.to_string();
            updated_pattern.push(c);
            ui.input_mode = InputMode::Search {
                pattern: updated_pattern,
                target,
            }
        }
        KeyCode::Delete | KeyCode::Backspace => {
            ui.app.pop_argument_char();
            let mut updated_pattern = pattern.to_string();
            updated_pattern.pop();
            ui.input_mode = InputMode::Search {
                pattern: updated_pattern,
                target,
            };
        }
        KeyCode::Enter => {
            ui.app.regex_search_seq(pattern, target);
            ui.input_mode = InputMode::Normal;
            if ui.app.search_state.is_some() {
                ui.jump_to_next_match(0);
            }
        }
        _ => {}
    }
}

fn handle_set_reference(ui: &mut UI, key_event: KeyEvent, ref_spec: &str) {
    match key_event.code {
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            ui.app.add_argument_char(c);
            let mut updated_ref_spec = ref_spec.to_string();
            updated_ref_spec.push(c);
            ui.input_mode = InputMode::SetReference {
                ref_spec: updated_ref_spec,
            }
        }
        KeyCode::Delete | KeyCode::Backspace => {
            ui.app.pop_argument_char();
            let mut updated_ref_spec = ref_spec.to_string();
            updated_ref_spec.pop();
            ui.input_mode = InputMode::SetReference {
                ref_spec: updated_ref_spec,
            };
        }
        KeyCode::Enter => {
            ui.app.set_aln_ref(ref_spec);
            ui.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_pane_prefix(ui: &mut UI, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char('l') => {
            ui.toggle_label_pane();
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char('b') => {
            ui.toggle_bottom_pane();
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char('f') => {
            ui.toggle_fullscreen();
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        _ => {}
    }
}

fn handle_diff_prefix(ui: &mut UI, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char('n') => {
            ui.diff_mode = DiffMode::Original;
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char('d') => {
            ui.diff_mode = DiffMode::DiffWRTRef;
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        _ => {}
    }
}

fn handle_search_prefix(ui: &mut UI, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char('h') => {
            ui.input_mode = InputMode::LabelSearch {
                pattern: String::from(""),
            };
            ui.app
                .argument_msg(String::from("Hdr search: "), String::from(""));
        }
        KeyCode::Char('s') => {
            ui.input_mode = InputMode::Search {
                pattern: String::from(""),
                target: SequenceSearchTarget::BiologicalSequence,
            };
            ui.app
                .argument_msg(String::from("Seq search: "), String::from(""));
        }
        KeyCode::Char('a') => {
            ui.input_mode = InputMode::Search {
                pattern: String::from(""),
                target: SequenceSearchTarget::AlignmentRow,
            };
            ui.app
                .argument_msg(String::from("Aln search: "), String::from(""));
        }
        _ => {}
    }
}

fn handle_ex_command(
    ui: &mut UI,
    key_event: KeyEvent,
    cmd: &str,
    history_prefix: Option<String>,
    history_idx: Option<usize>,
) {
    match key_event.code {
        KeyCode::Esc => {
            ui.input_mode = InputMode::Normal;
            ui.app.clear_msg();
        }
        KeyCode::Char(c) if c.is_ascii_graphic() || ' ' == c => {
            // Typing cancels history browsing.
            let mut updated_cmd = cmd.to_string();
            updated_cmd.push(c);
            ui.app.argument_msg(":", updated_cmd.clone());
            ui.input_mode = InputMode::ExCommand {
                cmd: updated_cmd,
                history_prefix: None,
                history_idx: None,
            };
        }
        KeyCode::Delete | KeyCode::Backspace => {
            // Backspace also cancels history browsing.
            let mut updated_cmd = cmd.to_string();
            updated_cmd.pop();
            ui.app.argument_msg(":", updated_cmd.clone());
            ui.input_mode = InputMode::ExCommand {
                cmd: updated_cmd,
                history_prefix: None,
                history_idx: None,
            };
        }
        KeyCode::Enter => {
            ui.app.push_ex_history(cmd);
            ex_command::execute(ui, cmd);
            ui.input_mode = InputMode::Normal;
        }
        KeyCode::Up => {
            // First Up press: snapshot the current buffer as the search prefix.
            let prefix = history_prefix.unwrap_or_else(|| cmd.to_string());
            // checked_sub returns None instead of wrapping/panicking on underflow, so
            // len().checked_sub(1) is None when the history is empty (len = 0) rather than
            // saturating to 0 and producing a bogus index into an empty Vec.
            let search_start = history_idx
                .and_then(|i| i.checked_sub(1))
                .or_else(|| ui.app.ex_history.len().checked_sub(1));
            // Search backward (newest first) for an entry starting with the prefix.
            let found = search_start.and_then(|start| {
                (0..=start)
                    .rev()
                    .find(|&i| ui.app.ex_history[i].starts_with(&prefix))
            });
            if let Some(idx) = found {
                let entry = ui.app.ex_history[idx].clone();
                ui.app.argument_msg(":", entry.clone());
                ui.input_mode = InputMode::ExCommand {
                    cmd: entry,
                    history_prefix: Some(prefix),
                    history_idx: Some(idx),
                };
            } else {
                // No match: keep current state but remember the prefix.
                ui.input_mode = InputMode::ExCommand {
                    cmd: cmd.to_string(),
                    history_prefix: Some(prefix),
                    history_idx,
                };
            }
        }
        KeyCode::Down => {
            if let (Some(prefix), Some(idx)) = (history_prefix, history_idx) {
                // Search forward from the entry after the current one.
                let found = (idx + 1..ui.app.ex_history.len())
                    .find(|&i| ui.app.ex_history[i].starts_with(&prefix));
                if let Some(next_idx) = found {
                    let entry = ui.app.ex_history[next_idx].clone();
                    ui.app.argument_msg(":", entry.clone());
                    ui.input_mode = InputMode::ExCommand {
                        cmd: entry,
                        history_prefix: Some(prefix),
                        history_idx: Some(next_idx),
                    };
                } else {
                    // Reached the present: restore the original prefix as the buffer.
                    ui.app.argument_msg(":", prefix.clone());
                    ui.input_mode = InputMode::ExCommand {
                        cmd: prefix,
                        history_prefix: None,
                        history_idx: None,
                    };
                }
            }
        }
        KeyCode::Tab => {
            // later: implement completion
        }
        _ => {}
    }
}

fn dispatch_command(ui: &mut UI, key_event: KeyEvent, count_arg: Option<usize>) {
    let count = count_arg.unwrap_or(1);

    // debug!("key event: {:#?}", key_event.code);
    match key_event.code {
        // ----- Motion -----

        // Arrows - late introduction, but might be friendlier to new users.
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
            // Non-shifted arrow keys
            if !key_event.modifiers.contains(KeyModifiers::SHIFT) {
                match key_event.code {
                    KeyCode::Down => match ui.zoom_level() {
                        ZoomLevel::ZoomedIn => ui.scroll_one_line_down(count as u16),
                        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                            ui.scroll_zoombox_one_line_down(count as u16)
                        }
                    },
                    KeyCode::Up => match ui.zoom_level() {
                        ZoomLevel::ZoomedIn => ui.scroll_one_line_up(count as u16),
                        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                            ui.scroll_zoombox_one_line_up(count as u16)
                        }
                    },
                    KeyCode::Right => match ui.zoom_level() {
                        ZoomLevel::ZoomedIn => ui.scroll_one_col_right(count as u16),
                        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                            ui.scroll_zoombox_one_col_right(count as u16)
                        }
                    },
                    KeyCode::Left => match ui.zoom_level() {
                        ZoomLevel::ZoomedIn => ui.scroll_one_col_left(count as u16),
                        ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                            ui.scroll_zoombox_one_col_left(count as u16)
                        }
                    },

                    _ => panic!("Expected only arrow keycodes"),
                }
            } else {
                // Shifted arrow keys
                match key_event.code {
                    KeyCode::Up => ui.scroll_one_screen_up(count as u16),
                    KeyCode::Left => ui.scroll_one_screen_left(count as u16),
                    KeyCode::Down => ui.scroll_one_screen_down(count as u16),
                    KeyCode::Right => ui.scroll_one_screen_right(count as u16),

                    _ => panic!("Expected only arrow keycodes"),
                }
            }
        }

        // Up
        KeyCode::Char('k') => match ui.zoom_level() {
            ZoomLevel::ZoomedIn => ui.scroll_one_line_up(count as u16),
            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                ui.scroll_zoombox_one_line_up(count as u16)
            }
        },
        KeyCode::Char('K') => ui.scroll_one_screen_up(count as u16),
        KeyCode::Char('g') => ui.jump_to_top(),

        // Left
        KeyCode::Char('h') => match ui.zoom_level() {
            ZoomLevel::ZoomedIn => ui.scroll_one_col_left(count as u16),
            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                ui.scroll_zoombox_one_col_left(count as u16)
            }
        },
        KeyCode::Char('H') => ui.scroll_one_screen_left(count as u16),
        KeyCode::Char('^') => ui.jump_to_begin(),

        // Down
        KeyCode::Char('j') => match ui.zoom_level() {
            ZoomLevel::ZoomedIn => ui.scroll_one_line_down(count as u16),
            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                ui.scroll_zoombox_one_line_down(count as u16)
            }
        },
        KeyCode::Char('J') | KeyCode::Char(' ') => ui.scroll_one_screen_down(count as u16),
        KeyCode::Char('G') => ui.jump_to_bottom(),

        // Right
        KeyCode::Char('l') => match ui.zoom_level() {
            ZoomLevel::ZoomedIn => ui.scroll_one_col_right(count as u16),
            ZoomLevel::ZoomedOut | ZoomLevel::ZoomedOutAR => {
                ui.scroll_zoombox_one_col_right(count as u16)
            }
        },
        KeyCode::Char('L') => ui.scroll_one_screen_right(count as u16),
        KeyCode::Char('$') => ui.jump_to_end(),

        // Absolute Positions

        // Visible line
        KeyCode::Char('-') => ui.jump_to_line((count as u16) - 1), // -1: user is 1-based

        // Rank
        KeyCode::Char('=') => ui.jump_to_rank((count as u16) - 1), // -1: user is 1-based

        // Column
        KeyCode::Char('|') => ui.jump_to_col(count as u16),

        // Relative positions

        // Vertical
        KeyCode::Char('%') => ui.jump_to_pct_line(count as u16),

        // Horizontal
        KeyCode::Char('#') => ui.jump_to_pct_col(count as u16),

        // To search matches
        KeyCode::Char('n') => ui.jump_to_next_match(count as i16),
        KeyCode::Char('p') => ui.jump_to_next_match(-(count as i16)),
        KeyCode::Char('N') => ui.jump_to_next_vertical_match(count as i16),
        KeyCode::Char('P') => ui.jump_to_next_vertical_match(-(count as i16)),
        KeyCode::Enter => ui.jump_to_current_match(),

        // To regions of high column metric
        KeyCode::Char(')') => ui.jump_to_next_hi_col_metric_region(count as i16),
        KeyCode::Char('(') => ui.jump_to_next_hi_col_metric_region(-(count as i16)),

        // ---- Left Pane width ----

        // TODO: use just one function and pass negative count, like 'n' & 'p' or ')' and '('.
        KeyCode::Char('>') => ui.widen_label_pane(count as u16),
        KeyCode::Char('<') => ui.reduce_label_pane(count as u16),

        // ---- Zoom ----
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
        // TODO: not sure we're keeping the "bottom" position. Seems much better to stick it to the
        // last seq in the alignment.
        KeyCode::Char('b') => {
            ui.cycle_bottom_pane_position();
        }

        // ---- Visuals ----

        // Mark consensus positions that are retained in the zoom box
        KeyCode::Char('r') => ui.toggle_hl_retained_cols(),

        // Inverse video
        KeyCode::Char('i') => {
            ui.toggle_video_mode();
        }

        KeyCode::Char('s') => ui.next_color_scheme(),
        KeyCode::Char('S') => ui.prev_color_scheme(),

        // Switch to next/previous colormap in the list
        KeyCode::Char('m') => ui.next_colormap(),
        KeyCode::Char('M') => ui.prev_colormap(),

        // Sequence Order
        KeyCode::Char('o') => ui.app.next_ordering_criterion(),
        KeyCode::Char('O') => ui.app.prev_ordering_criterion(),

        // Sequence Metric
        KeyCode::Char('t') => ui.app.next_seq_metric(),
        KeyCode::Char('T') => ui.app.prev_seq_metric(),

        // Column Metric
        KeyCode::Char('c') => ui.app.next_col_metric(),
        KeyCode::Char('C') => ui.app.prev_col_metric(),

        // ----- Search -----
        KeyCode::Char('?') => ui.input_mode = InputMode::Help,
        KeyCode::Char(']') => ui.app.warning_msg("Search not implemented yet"),
        KeyCode::Char('[') => ui.app.warning_msg("Search not implemented yet"),

        // ----- Editing -----
        // Filter alignment through external command (à la Vim's '!')
        KeyCode::Char('!') => ui.app.warning_msg("Filtering not implemented yet"),

        // ----- Diff Mode -----
        KeyCode::Char('D') => ui.diff_mode = DiffMode::Original,

        _ => {
            // let the user know this key is not bound
            //
            // TODO: there are pros and cons about this - first, the user can probably guess
            // that if nothing happens then the key isn't bound. Second, the message should be
            // disabled after the user presses a bound key, which would force us to either add
            // code to that effect for _every single_ key binding, or do a first match on every
            // valid key (to disable the message) and then match on each individual key to
            // launch the desired action. Not sure it's worth it, frankly.
            // ui.warning_msg(format!("'{}' not bound", c));
        }
    }
}
