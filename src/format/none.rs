use std::convert::Infallible;

use crate::core::{
    DynMatchFailure, Format, Formatter, MatchError, MatchFailure, PosMatcher, ResultFormat,
};
use crate::matchers::{NoneAssertion, NoneMatcher};

#[derive(Debug)]
pub struct NoneFormat;

impl Format for NoneFormat {
    type Value = MatchFailure<DynMatchFailure, Infallible>;
    type Error = Infallible;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> Result<(), Self::Error> {
        match value {
            MatchFailure::Pos(fail) => f.write_fmt(fail.into_fmt()),
            _ => unreachable!(),
        }

        Ok(())
    }
}

impl ResultFormat for NoneFormat {
    type Pos = DynMatchFailure;
    type Neg = Infallible;
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn none<'a, In, Out>(
    block: impl FnOnce(NoneAssertion<In>) -> Result<NoneAssertion<Out>, MatchError> + 'a,
) -> PosMatcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    PosMatcher::new(NoneMatcher::new(block), NoneFormat)
}
