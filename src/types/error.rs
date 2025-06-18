use std::{io::Error as IoError, result::Result as StdResult};

use rand::distr::uniform::Error as RandomError;

/// Main error type for the sysz library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid type syntax during parsing.
    #[error("Invalid type syntax: {0}")]
    InvalidSyntax(String),

    /// Type validation mismatch.
    #[error(
        "Type validation error: expected {:?}, found {:?}{:?}",
        expected,
        actual,
        context
    )]
    ValidationError {
        /// Expected type.
        expected: String,
        /// Actual type.
        actual:   String,
        /// Additional context.
        context:  Option<String>,
    },

    /// Error during random generation.
    #[error("Random generation error: {0}")]
    RandomErrorWrapper(#[from] RandomError),

    /// Sysz I/O error.
    #[error("I/O error: {0}")]
    IoErrorWrapper(#[from] IoError),

    /// Error during random generation.
    #[error("Random generation error: {0}")]
    RandomError(String),

    /// Sysz I/O error.
    #[error("I/O error: {0}")]
    IoError(String),
}

/// Result type for sysz library functions.
pub type Result<T> = StdResult<T, Error>;
