// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use std::{fmt, io};

#[derive(Debug)]
pub enum TermalError {
    Io(io::Error),
    Format(String),
}

// These allow conversion to TermalError, required for main() to return Result<()> and for '?' to
// work.

impl From<io::Error> for TermalError {
    fn from(e: io::Error) -> Self {
        TermalError::Io(e)
    }
}

impl From<String> for TermalError {
    fn from(s: String) -> Self {
        TermalError::Format(s)
    }
}

impl fmt::Display for TermalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermalError::Io(e) => write!(f, "I/O error: {}", e),
            TermalError::Format(msg) => write!(f, "Format error: {}", msg),
        }
    }
}
