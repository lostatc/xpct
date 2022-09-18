use std::fmt;
use std::marker::PhantomData;

use crate::core::{style, Format, Formatter, MatchFailure, Matcher};
use crate::matchers::{EqualMatcher, Mismatch};

#[derive(Debug)]
pub struct MismatchFormat<Actual, Expected> {
    marker: PhantomData<(Actual, Expected)>,
    pos_msg: String,
    neg_msg: String,
}

impl<Actual, Expected> MismatchFormat<Actual, Expected> {
    pub fn new(pos_msg: impl Into<String>, neg_msg: impl Into<String>) -> Self {
        Self {
            marker: PhantomData,
            pos_msg: pos_msg.into(),
            neg_msg: neg_msg.into(),
        }
    }
}
impl<Actual, Expected> Format for MismatchFormat<Actual, Expected>
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
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.actual));

                f.set_style(style::important());
                f.write_str(self.pos_msg);
                f.write_str(":\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.expected));
            }
            MatchFailure::Neg(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.actual));

                f.set_style(style::important());
                f.write_str(self.neg_msg);
                f.write_str(":\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.expected));
            }
        };

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        EqualMatcher::new(expected),
        MismatchFormat::new("to equal", "to not equal"),
    )
}
