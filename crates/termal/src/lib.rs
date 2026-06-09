// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

pub mod app;
pub mod errors;
mod runner;
pub mod seq_match;
pub mod ui;
mod vec_f64_aux;

use crate::errors::TermalError;

pub fn run() -> Result<(), TermalError> {
    runner::run()
}
