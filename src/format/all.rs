use crate::core::{DispatchFormat, MatchError, Matcher};
use crate::matchers::{ChainAssertion, ChainMatcher};

use super::{FailureFormat, MessageFormat};

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn all<'a, In, Out>(
    block: impl FnOnce(ChainAssertion<In>) -> Result<ChainAssertion<Out>, MatchError> + 'a,
) -> Matcher<'a, In, Out, ()>
where
    In: 'a,
    Out: 'a,
{
    let format = DispatchFormat::new(
        FailureFormat::new(),
        MessageFormat::new("", "All the matchers matched."),
    );

    Matcher::new(ChainMatcher::new(block), format)
}
