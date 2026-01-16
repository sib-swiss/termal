// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

mod common;

use crate::common::utils;

use termal_msa::alignment::Alignment;
use termal_msa::app::App;

#[test]
fn renders_without_panic() {
    let hdrs = vec![
        String::from("R1"),
        String::from("R2"),
        String::from("R3"),
        String::from("R4"),
        String::from("R5"),
    ];
    let seqs = vec![
        String::from("catgcatatg"), // 0 diffs WRT consensus
        String::from("caGgAaCaAg"), // 4 diffs WRT consensus
        String::from("catAcTtatg"), // 2 diffs WRT consensus
        String::from("cCtgcatatg"), // 1 diffs WRT consensus
        String::from("caGgAataAg"), // 3 diffs WRT consensus
    ];
    let aln = Alignment::from_vecs(hdrs, seqs);
    let mut app = App::new("TEST", aln, None);
    let buf = utils::render(&mut app, 40, 30);
    let screen = utils::buffer_text(&buf);

    assert!(!screen.trim().is_empty());
}
