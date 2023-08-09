use std::convert::Infallible;

use crate::core::{Format, Formatter, MatchFailure, Matcher};
use crate::matchers::map::{IterMap, IterMapMatcher, IterTryMapMatcher, MapMatcher, TryMapMatcher};

use super::FailureFormat;

/// A formatter for matchers that never fail.
///
/// This formatter doesn't actually format anything, because it can never be called. It is mostly
/// useful for matchers like [`MapMatcher`] that can never fail.
#[derive(Debug, Default)]
pub struct InfallibleFormat;

impl Format for InfallibleFormat {
    type Value = MatchFailure<Infallible>;

    fn fmt(&self, _: &mut Formatter, _: Self::Value) -> crate::Result<()> {
        unreachable!()
    }
}

/// Infallibly map the input value by applying a function to it.
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
pub fn map<'a, In, Out>(func: impl FnOnce(In) -> Out + 'a) -> Matcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    Matcher::transform(MapMatcher::new(func), InfallibleFormat)
}

/// Fallibly map the input value by applying a function to it.
///
/// This does the same thing as [`Assertion::try_map`].
///
/// This matcher always succeeds as long as `func` returns `Ok`, even when negated. Therefore
/// negating it has no effect.
///
/// # Examples
///
/// ```
/// use xpct::{expect, equal, try_map};
///
/// expect!(vec![0x43, 0x75, 0x6e, 0x6f])
///     .to(try_map(|bytes| Ok(String::from_utf8(bytes)?)))
///     .to(equal("Cuno"));
/// ```
///
/// [`Assertion::try_map`]: crate::core::Assertion::map
pub fn try_map<'a, In, Out>(
    func: impl FnOnce(In) -> crate::Result<Out> + 'a,
) -> Matcher<'a, In, Out>
where
    In: 'a,
    Out: 'a,
{
    Matcher::transform(TryMapMatcher::new(func), FailureFormat::new())
}

/// Infallibly convert the input value via [`From`]/[`Into`].
///
/// This does the same thing as [`Assertion::into`].
///
/// This matcher always succeeds, even when negated. Therefore negating it has no effect.
///
/// # Examples
///
/// ```
/// use xpct::{expect, equal, into};
///
/// expect!(41u32)
///     .to(into::<_, u64>())
///     .to(equal(41u64));
/// ```
///
/// [`Assertion::into`]: crate::core::Assertion::into
pub fn into<'a, In, Out>() -> Matcher<'a, In, Out>
where
    In: 'a,
    Out: From<In> + 'a,
{
    Matcher::transform(MapMatcher::new(<Out as From<In>>::from), InfallibleFormat)
}

/// Fallibly convert the input value via [`TryFrom`]/[`TryInto`].
///
/// This does the same thing as [`Assertion::try_into`].
///
/// This matcher always succeeds as long as [`TryFrom::try_from`] returns `Ok`, even when negated.
/// Therefore negating it has no effect.
///
/// # Examples
///
/// ```
/// use xpct::{expect, equal, try_into};
///
/// expect!(41u64)
///     .to(try_into::<_, u32>())
///     .to(equal(41u32));
/// ```
///
/// [`Assertion::try_into`]: crate::core::Assertion::try_into
pub fn try_into<'a, In, Out>() -> Matcher<'a, In, Out>
where
    In: 'a,
    Out: TryFrom<In> + 'a,
    <Out as TryFrom<In>>::Error: std::error::Error + Send + Sync + 'static,
{
    Matcher::transform(
        TryMapMatcher::new(|value| {
            <Out as TryFrom<In>>::try_from(value).map_err(crate::Error::new)
        }),
        FailureFormat::new(),
    )
}

/// Infallibly map each value of an iterator by applying a function to it.
///
/// This does the same thing as [`Assertion::iter_map`].
///
/// This matcher always succeeds, even when negated. Therefore negating it has no effect.
///
/// # Examples
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
///
/// [`Assertion::iter_map`]: crate::core::Assertion::iter_map
pub fn iter_map<'a, In, Out, IntoIter>(
    func: impl Fn(In) -> Out + 'a,
) -> Matcher<'a, IntoIter, IterMap<'a, In, Out, IntoIter::IntoIter>>
where
    In: 'a,
    Out: 'a,
    IntoIter: IntoIterator<Item = In> + 'a,
{
    Matcher::transform(IterMapMatcher::new(func), InfallibleFormat)
}

/// Fallibly map each value of an iterator by applying a function to it.
///
/// This does the same thing as [`Assertion::iter_try_map`].
///
/// This matcher always succeeds as long as `func` returns `Ok`, even when negated. Therefore
/// negating it has no effect.
///
/// # Examples
///
/// ```
/// use xpct::{expect, iter_try_map, consist_of};
///
/// let small_integers: [u64; 2] = [41, 57];
///
/// expect!(small_integers)
///     .to(iter_try_map(|value| Ok(u32::try_from(value)?)))
///     .to(consist_of([41u32, 57u32]));
/// ```
///
/// [`Assertion::iter_try_map`]: crate::core::Assertion::iter_try_map
pub fn iter_try_map<'a, In, Out, IntoIter>(
    func: impl Fn(In) -> crate::Result<Out> + 'a,
) -> Matcher<'a, IntoIter, Vec<Out>>
where
    In: 'a,
    Out: 'a,
    IntoIter: IntoIterator<Item = In> + 'a,
{
    Matcher::transform(IterTryMapMatcher::new(func), InfallibleFormat)
}
