// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

pub mod alignment;
pub mod app;
pub mod errors;
mod runner;
pub mod seq;
pub mod ui;
mod vec_f64_aux;

use crate::errors::TermalError;

pub fn run() -> Result<(), TermalError> {
    runner::run()
}
