// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod common;

use std::fs;

use crate::common::utils;

use termal_alignment::{seq::fasta, Alignment};
use termal_msa::{
    app::App,
    ui::{render::render_ui, UI},
};

use ratatui::backend::TestBackend;
use ratatui::Terminal;

#[test]
fn user_order_reorders_visible_headers() {
    let seq_file = fasta::read_fasta_file("tests/data/test-user-order.fas").expect("read");
    let aln = Alignment::from_file(seq_file);
    let order = fs::read_to_string("tests/data/test-user-order.order").expect("read order");
    let user_ordering = order.lines().map(str::to_string).collect();

    let mut app = App::new("TEST", aln, Some(user_ordering));
    let mut ui = UI::new(&mut app);

    let backend = TestBackend::new(80, 12);
    let mut terminal = Terminal::new(backend).expect("terminal");
    terminal.draw(|f| render_ui(f, &mut ui)).expect("draw");

    let screen = utils::buffer_text(terminal.backend().buffer());
    let pos_seq3 = screen.find("seq3").expect("seq3 visible");
    let pos_seq1 = screen.find("seq1").expect("seq1 visible");
    let pos_seq2 = screen.find("seq2").expect("seq2 visible");

    assert!(
        pos_seq3 < pos_seq1,
        "seq3 should appear before seq1:\n{}",
        screen
    );
    assert!(
        pos_seq1 < pos_seq2,
        "seq1 should appear before seq2:\n{}",
        screen
    );
}
