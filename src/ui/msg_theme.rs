// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use crate::app::MessageKind;
use ratatui::style::{Color, Style};

pub const INFO_STYLE: Style = Style::new().fg(Color::White).bg(Color::Black);

pub const WARNING_STYLE: Style = Style::new().fg(Color::Black).bg(Color::Yellow);

pub const ERROR_STYLE: Style = Style::new().fg(Color::White).bg(Color::Red);

pub const DEBUG_STYLE: Style = Style::new().fg(Color::Black).bg(Color::Cyan);

pub const ARGUMENT_STYLE: Style = Style::new().fg(Color::White).bg(Color::Blue);

pub fn style_for(kind: &MessageKind) -> Style {
    match kind {
        MessageKind::Info => INFO_STYLE,
        MessageKind::Warning => WARNING_STYLE,
        MessageKind::Error => ERROR_STYLE,
        MessageKind::Debug => DEBUG_STYLE,
        MessageKind::Argument => ARGUMENT_STYLE,
    }
}
