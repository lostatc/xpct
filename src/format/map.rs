use crate::core::Matcher;
use crate::matchers::{MapMatcher, TryMapMatcher};

use super::FailureFormat;

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn map<'a, In, Out, F>(func: F) -> Matcher<'a, In, Out>
where
    F: FnOnce(In) -> Out + 'a,
    In: 'a,
    Out: 'a,
{
    Matcher::new(MapMatcher::new::<F>(func), FailureFormat::new())
}

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
