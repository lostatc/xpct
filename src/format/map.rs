use crate::core::PosMatcher;
use crate::matchers::{MapMatcher, TryMapMatcher};

use super::FailureFormat;

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn map<'a, In, Out, F>(func: F) -> PosMatcher<'a, In, Out>
where
    F: FnOnce(In) -> Out + 'a,
    In: 'a,
    Out: 'a,
{
    PosMatcher::new(MapMatcher::new::<F>(func), FailureFormat::new())
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn try_map<'a, In, Out>(
    func: impl FnOnce(In) -> anyhow::Result<Out> + 'a,
) -> PosMatcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    PosMatcher::new(TryMapMatcher::new(func), FailureFormat::new())
}
