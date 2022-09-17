use std::error::Error;

use crate::core::{AssertionFailure, MatchFailure};

#[cfg(not(feature = "color"))]
use super::formatter::Formatter;

#[cfg(feature = "color")]
use super::formatter_color::Formatter;

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
