use crate::core::{AssertionFailure, MatchFailure};

use super::Formatter;

pub trait Format {
    type Value;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()>;
}

pub trait ResultFormat: Format<Value = MatchFailure<Self::Pos, Self::Neg>> {
    type Pos;
    type Neg;
}

pub trait AssertionFormat: Format<Value = AssertionFailure<Self::Context>> {
    type Context;
}
