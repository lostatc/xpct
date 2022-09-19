use crate::core::{MatchError, PosMatcher};
use crate::matchers::{ChainAssertion, ChainMatcher};

use super::FailureFormat;

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn all<'a, In, Out>(
    block: impl FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a,
) -> PosMatcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    PosMatcher::new(ChainMatcher::new(block), FailureFormat::new())
}
