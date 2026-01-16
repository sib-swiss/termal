// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

// A record for sequences, consisting of some description and a raw sequence. Meant to be
// format-agnostic - should work for FastA, Stockholm, GenBank, etc - though in the last cases it
// won't contain annotations.

#[derive(Debug)]
pub struct SeqRecord {
    pub header: String,
    pub sequence: String,
}
