use crate::core::{MatchError, PosMatcher};
use crate::matchers::{AllAssertion, AllMatcher};

use super::FailureFormat;

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn all<'a, In, Out>(
    block: impl FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'a,
) -> PosMatcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    PosMatcher::new(AllMatcher::new(block), FailureFormat::new())
}
