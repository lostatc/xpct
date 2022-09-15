use std::convert::Infallible;

use crate::core::{DynMatchFailure, Format, Formatter, MatchFailure, Matcher, ResultFormat};
use crate::matchers::NotMatcher;

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

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn not<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
) -> Matcher<In, NegOut, PosOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::new(NotMatcher::new(matcher), FailFormat)
}
