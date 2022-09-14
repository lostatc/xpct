#![cfg(feature = "fmt")]

use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;

use crate::{format::style, DynMatchFailure, Format, Formatter, MatchFailure, ResultFormat};

use super::{AllFailures, Mismatch, SomeFailures};

pub struct AllFailuresFormat;

impl Format for AllFailuresFormat {
    type Value = AllFailures;
    type Error = Infallible;

    fn fmt(self, _: &mut Formatter, _: Self::Value) -> Result<(), Self::Error> {
        todo!()
    }
}

pub struct SomeFailuresFormat;

impl Format for SomeFailuresFormat {
    type Value = SomeFailures;
    type Error = Infallible;

    fn fmt(self, _: &mut Formatter, _: Self::Value) -> Result<(), Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct AnyFormat;

impl Format for AnyFormat {
    type Value = MatchFailure<AllFailures, SomeFailures>;
    type Error = Infallible;

    fn fmt(self, _: &mut Formatter, _: Self::Value) -> Result<(), Self::Error> {
        todo!()
    }
}

impl ResultFormat for AnyFormat {
    type Pos = AllFailures;
    type Neg = SomeFailures;
}

#[derive(Debug)]
pub struct AllFormat;

impl Format for AllFormat {
    type Value = MatchFailure<DynMatchFailure, ()>;
    type Error = Infallible;

    fn fmt(self, _: &mut Formatter, _: Self::Value) -> Result<(), Self::Error> {
        todo!()
    }
}

impl ResultFormat for AllFormat {
    type Pos = DynMatchFailure;
    type Neg = ();
}

#[derive(Debug)]
pub struct EachFormat;

impl Format for EachFormat {
    type Value = MatchFailure<DynMatchFailure, ()>;
    type Error = Infallible;

    fn fmt(self, _: &mut Formatter, _: Self::Value) -> Result<(), Self::Error> {
        todo!()
    }
}

impl ResultFormat for EachFormat {
    type Pos = DynMatchFailure;
    type Neg = ();
}

#[derive(Debug)]
pub struct EqualFormat<Actual, Expected> {
    marker: PhantomData<(Actual, Expected)>,
}

impl<Actual, Expected> EqualFormat<Actual, Expected> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<Actual, Expected> Format for EqualFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
{
    type Value = MatchFailure<Mismatch<Actual, Expected>>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        match value {
            MatchFailure::Pos(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(), mismatch.expected));

                f.set_style(style::important());
                f.write_str("to equal:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(), mismatch.actual));
            }
            MatchFailure::Neg(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(), mismatch.expected));

                f.set_style(style::important());
                f.write_str("to not equal:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(), mismatch.actual));
            }
        }

        Ok(())
    }
}

impl<Actual, Expected> ResultFormat for EqualFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
{
    type Pos = Mismatch<Actual, Expected>;
    type Neg = Mismatch<Actual, Expected>;
}

#[derive(Debug)]
pub struct FailFormat;

impl Format for FailFormat {
    type Value = MatchFailure<DynMatchFailure>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        let fail = match value {
            MatchFailure::Pos(fail) => fail,
            MatchFailure::Neg(fail) => fail,
        };

        f.write_fmt(fail.into_fmt());

        Ok(())
    }
}

impl ResultFormat for FailFormat {
    type Pos = DynMatchFailure;
    type Neg = DynMatchFailure;
}

enum WhyFormatReason<'a> {
    Eager(Cow<'a, str>),
    Lazy(Box<dyn FnOnce() -> Cow<'a, str> + 'a>),
}

impl<'a> fmt::Debug for WhyFormatReason<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eager(reason) => f.debug_tuple("Eager").field(reason).finish(),
            Self::Lazy(_) => f
                .debug_tuple("Lazy")
                .field(&std::any::type_name::<Box<dyn FnOnce() -> String>>())
                .finish(),
        }
    }
}

#[derive(Debug)]
pub struct WhyFormat<'a> {
    reason: WhyFormatReason<'a>,
}

impl<'a> WhyFormat<'a> {
    pub fn new(reason: impl Into<Cow<'a, str>>) -> Self {
        Self {
            reason: WhyFormatReason::Eager(reason.into()),
        }
    }

    pub fn lazy(reason: impl FnOnce() -> Cow<'a, str> + 'a) -> Self {
        Self {
            reason: WhyFormatReason::Lazy(Box::new(reason)),
        }
    }
}

impl<'a> Format for WhyFormat<'a> {
    type Value = MatchFailure<DynMatchFailure>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        match self.reason {
            WhyFormatReason::Eager(reason) => {
                f.write_str(reason.as_ref());
            }
            WhyFormatReason::Lazy(func) => {
                let reason = (func)();
                f.write_str(reason.as_ref());
            }
        };

        f.write_fmt(value);

        Ok(())
    }
}

impl<'a> ResultFormat for WhyFormat<'a> {
    type Pos = DynMatchFailure;
    type Neg = DynMatchFailure;
}
