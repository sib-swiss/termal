// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

use std::fs;
use std::path::Path;

use termal_alignment::seq::fasta;
use termal_alignment::Alignment;

#[test]
fn large_alignment_beyond_u16_max() {
    // Create a FASTA file with >65k columns (u16::MAX = 65535)
    // This tests the fix for the "Formatting argument out of range" panic
    // Rust 1.87+ constrains format! width to u16, causing panic at alignment.rs:125

    let test_file = "tests/data/large_alignment_test.fas";

    // Only create if it doesn't exist
    if !Path::new(test_file).exists() {
        let width = 100_000;
        let seq_data = "A".repeat(width);

        let mut content = String::new();
        content.push_str(">seq1\n");
        content.push_str(&seq_data);
        content.push_str("\n>seq2\n");
        content.push_str(&seq_data);
        content.push_str("\n>seq3\n");
        content.push_str(&seq_data);
        content.push('\n');

        fs::write(test_file, content).expect("write test file");
    }

    // This should panic with current code at alignment.rs:125:
    // "thread 'main' panicked at 'Formatting argument out of range'"
    let seq_file = fasta::read_fasta_file(test_file)
        .expect("read fasta");

    let aln = Alignment::from_file(seq_file);

    // If we get here, the fix worked
    assert_eq!(aln.sequences.len(), 3, "should have 3 sequences");
    assert_eq!(aln.sequences[0].len(), 100_000, "sequences should be padded to full width");
}
