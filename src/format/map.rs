use std::convert::Infallible;

use crate::core::{Format, Formatter, MatchFailure, Matcher};
use crate::matchers::{IterMapMatcher, IterTryMapMatcher, MapMatcher, TryMapMatcher};

use super::FailureFormat;

/// A formatter for matchers that never fail.
///
/// This formatter doesn't actually format anything, because it can never be called. It is mostly
/// useful for matchers like [`map`] that don't actually test anything.
#[derive(Debug, Default)]
pub struct InfallibleFormat;

impl Format for InfallibleFormat {
    type Value = MatchFailure<Infallible>;

    fn fmt(self, _: &mut Formatter, _: Self::Value) -> crate::Result<()> {
        unreachable!()
    }
}

/// Infallibly map the input value to an output value, possibly of a different type.
///
/// This does the same thing as [`Assertion::map`].
///
/// This matcher always succeeds, even when negated. Therefore negating it has no effect.
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use xpct::{expect, map, equal};
///
/// fn do_stuff() -> Result<String, Infallible> {
///     Ok(String::from("foobar"))
/// }
///
/// expect!(do_stuff())
///     .to(map(Result::unwrap))
///     .to(equal("foobar"));
/// ```
///
/// [`Assertion::map`]: crate::core::Assertion::map
pub fn map<'a, In, Out, F>(func: F) -> Matcher<'a, In, Out>
where
    F: FnOnce(In) -> Out + 'a,
    In: 'a,
    Out: 'a,
{
    Matcher::new(MapMatcher::new::<F>(func), InfallibleFormat)
}

/// Fallibly map the input value to an output value, possibly of a different type.
///
/// This does the same thing as [`Assertion::try_map`].
///
/// This matcher always succeeds as long as `func` returns `Ok`, even when negated. Therefore
/// negating it has no effect.
///
/// [`Assertion::try_map`]: crate::core::Assertion::map
pub fn try_map<'a, In, Out>(
    func: impl FnOnce(In) -> crate::Result<Out> + 'a,
) -> Matcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    Matcher::new(TryMapMatcher::new(func), FailureFormat::new())
}

/// Infallibly map each value of an iterator to a different value, possibly of a different type.
///
/// This matcher always succeeds, even when negated. Therefore negating it has no effect.
///
/// # Examples
///
/// This fails to compile if we try to pass `items` by reference.
///
/// ```compile_fail
/// use xpct::{be_some, every, expect, iter_map};
///
/// let items = vec![Some("foo"), Some("bar")];
///
/// let output: Vec<&str> = expect!(&items)
///     .to(every(be_some))
///     .into_inner();
/// ```
///
/// To fix that, we need to call [`Option::as_deref`] on each value.
///
/// ```
/// use xpct::{be_some, every, expect, iter_map};
///
/// let items = vec![Some("foo"), Some("bar")];
///
/// let output: Vec<&str> = expect!(&items)
///     .to(iter_map(Option::as_deref))
///     .to(every(be_some))
///     .into_inner();
/// ```
pub fn iter_map<'a, In, Out, IntoIter, F>(func: F) -> Matcher<'a, IntoIter, Vec<Out>>
where
    IntoIter: IntoIterator<Item = In> + 'a,
    F: Fn(In) -> Out + 'a,
    In: 'a,
    Out: 'a,
{
    Matcher::new(IterMapMatcher::new(func), InfallibleFormat)
}

/// Fallibly each value of an iterator to a different value, possibly of a different type.
///
/// This matcher always succeeds as long as `func` returns `Ok`, even when negated. Therefore
/// negating it has no effect.
pub fn iter_try_map<'a, In, Out, IntoIter, F>(func: F) -> Matcher<'a, IntoIter, Vec<Out>>
where
    IntoIter: IntoIterator<Item = In> + 'a,
    F: Fn(In) -> crate::Result<Out> + 'a,
    In: 'a,
    Out: 'a,
{
    Matcher::new(IterTryMapMatcher::new(func), InfallibleFormat)
}
