use std::convert::Infallible;

use crate::core::{
    DynMatchFailure, Format, Formatter, MatchError, MatchFailure, PosMatcher, ResultFormat,
};
use crate::matchers::{AllAssertion, AllMatcher};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct AllFormat;

impl AllFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for AllFormat {
    type Value = MatchFailure<DynMatchFailure, Infallible>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        match value {
            MatchFailure::Pos(fail) => f.write_fmt(fail.into_fmt()),
            _ => unreachable!(),
        }

        Ok(())
    }
}

impl ResultFormat for AllFormat {
    type Pos = DynMatchFailure;
    type Neg = Infallible;
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn all<'a, In, Out>(
    block: impl FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'a,
) -> PosMatcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    PosMatcher::new(AllMatcher::new(block), AllFormat::new())
}
