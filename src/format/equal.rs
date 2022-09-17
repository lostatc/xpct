use std::fmt;
use std::marker::PhantomData;

use crate::core::{style, Format, Formatter, MatchFailure, Matcher};
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

impl<Actual, Expected> Default for EqualFormat<Actual, Expected> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Actual, Expected> Format for EqualFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
{
    type Value = MatchFailure<Mismatch<Actual, Expected>>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        match value {
            MatchFailure::Pos(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.expected));

                f.set_style(style::important());
                f.write_str("to equal:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.actual));
            }
            MatchFailure::Neg(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.expected));

                f.set_style(style::important());
                f.write_str("to not equal:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.actual));
            }
        }

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(EqualMatcher::new(expected), EqualFormat::new())
}
