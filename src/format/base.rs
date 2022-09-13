use crate::{AssertionFailure, MatchFailure};
use std::error::Error;

#[cfg(not(feature = "color"))]
use super::formatter::Formatter;

#[cfg(feature = "color")]
use super::formatter_color::Formatter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OutputStream {
    Stdout,
    Stderr,
}

impl OutputStream {
    pub fn is_stdout(&self) -> bool {
        match self {
            OutputStream::Stdout => true,
            OutputStream::Stderr => false,
        }
    }

    pub fn is_stderr(&self) -> bool {
        match self {
            OutputStream::Stdout => false,
            OutputStream::Stderr => true,
        }
    }
}

pub trait Format {
    type Value;
    type Error: Error + Send + Sync + 'static;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error>;
}

pub trait ResultFormat: Format<Value = MatchFailure<Self::Pos, Self::Neg>> {
    type Pos;
    type Neg;
}

pub trait AssertionFormat: Format<Value = AssertionFailure<Self::Context>> {
    type Context;
}
