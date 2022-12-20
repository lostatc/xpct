use crate::core::Matcher;
use crate::matchers::{MapMatcher, TryMapMatcher};

use super::FailureFormat;

/// Infallibly map the input value to an output value, possibly of a different type.
///
/// This does the same thing as [`Assertion::map`].
///
/// [`Assertion::map`]: crate::core::Assertion::map
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn map<'a, In, Out, F>(func: F) -> Matcher<'a, In, Out>
where
    F: FnOnce(In) -> Out + 'a,
    In: 'a,
    Out: 'a,
{
    Matcher::new(MapMatcher::new::<F>(func), FailureFormat::new())
}

/// Fallibly map the input value to an output value, possibly of a different type.
///
/// This does the same thing as [`Assertion::try_map`].
///
/// [`Assertion::try_map`]: crate::core::Assertion::map
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn try_map<'a, In, Out>(
    func: impl FnOnce(In) -> crate::Result<Out> + 'a,
) -> Matcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    Matcher::new(TryMapMatcher::new(func), FailureFormat::new())
}
