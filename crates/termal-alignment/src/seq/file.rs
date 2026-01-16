// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use crate::seq::record::SeqRecord;

// For our purposes, a sequence file is just a Vec of sequence records.
//

pub type SeqFile = Vec<SeqRecord>;
