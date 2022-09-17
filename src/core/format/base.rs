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

impl<T, Pos, Neg> ResultFormat for T
where
    T: Format<Value = MatchFailure<Pos, Neg>>,
{
    type Pos = Pos;
    type Neg = Neg;
}

pub trait AssertionFormat: Format<Value = AssertionFailure<Self::Context>> {
    type Context;
}

impl<T, Context> AssertionFormat for T
where
    T: Format<Value = AssertionFailure<Context>>,
{
    type Context = Context;
}
