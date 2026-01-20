//! Error types for TwiML

use std::fmt;

/// Result type alias for TwiML operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for TwiML operations
#[derive(Debug)]
pub enum Error {
    /// TwiML validation failed
    Validation(String),

    /// Invalid parameter
    InvalidParameter {
        /// Parameter name
        param: String,
        /// Reason for invalidity
        reason: String,
    },

    /// Generic error
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Validation(msg) => write!(f, "TwiML validation failed: {}", msg),
            Error::InvalidParameter { param, reason } => {
                write!(f, "Invalid parameter '{}': {}", param, reason)
            }
            Error::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a new invalid parameter error
    pub fn invalid_parameter(param: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidParameter {
            param: param.into(),
            reason: reason.into(),
        }
    }

    /// Create a new generic error
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}
