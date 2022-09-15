use std::convert::Infallible;

use crate::core::{
    DynMatchFailure, Format, Formatter, MatchError, MatchFailure, Matcher, ResultFormat,
};
use crate::matchers::{EachContext, EachMatcher};

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

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn each<'a, T>(
    block: impl FnOnce(&mut EachContext<T>) -> Result<(), MatchError> + 'a,
) -> Matcher<'a, T, T>
where
    T: 'a,
{
    Matcher::new(EachMatcher::new(block), EachFormat)
}
