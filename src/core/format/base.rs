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

#[derive(Debug, Default)]
pub struct NegFormat<Fmt>(pub Fmt);

impl<Fmt, Fail> Format for NegFormat<Fmt>
where
    Fmt: Format<Value = MatchFailure<Fail, Fail>>,
{
    type Value = Fmt::Value;

    fn fmt(self, f: &mut super::Formatter, value: Self::Value) -> anyhow::Result<()> {
        match value {
            MatchFailure::Pos(fail) => self.0.fmt(f, MatchFailure::Neg(fail)),
            MatchFailure::Neg(fail) => self.0.fmt(f, MatchFailure::Pos(fail)),
        }
    }
}
