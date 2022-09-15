use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;

use crate::core::{style, Format, Formatter, MatchFailure, Matcher, ResultFormat};
use crate::matchers::{EqualMatcher, Mismatch};

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

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(EqualMatcher::new(expected), EqualFormat::new())
}
