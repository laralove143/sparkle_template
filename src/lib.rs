#![doc = include_str!("../README.md")]

use std::fmt::{Debug, Display, Formatter};

pub mod parse;

/// Errors returned in this library
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Error {
    /// Interaction doesn't contain data
    MissingInteractionData,
    /// Interaction doesn't contain command data
    MissingCommandData,
    /// Interaction doesn't contain component data
    MissingComponentData,
    /// Interaction doesn't contain modal data
    MissingModalData,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::MissingInteractionData => "interaction doesn't contain data",
            Self::MissingCommandData => "interaction doesn't contain command data",
            Self::MissingComponentData => "interaction doesn't contain component data",
            Self::MissingModalData => "interaction doesn't contain modal data",
        };

        fmt.write_str(msg)
    }
}

impl std::error::Error for Error {}
