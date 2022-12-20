use crate::core::{DispatchFormat, MatchError, Matcher};
use crate::matchers::{ChainAssertion, ChainMatcher};

use super::{FailureFormat, MessageFormat};

/// Matches when all of the passed matchers match.
///
/// This method is similar to [`each`], except it short-circuits on the first failed match and
/// chains the output of each matcher into the next.
///
/// This matcher accepts a closure which is passed a [`ChainAssertion`] value, which is similar to
/// the [`Assertion`] value returned by [`expect!`]. You can call [`to`] and [`to_not`] on it to
/// use matchers.
///
/// This matcher has some limitations compared to [`each`] when you negate it (such as with [`not`]):
///
/// 1. If it succeeds (meaning that all the matchers failed), it can't return the transformed value
///    at the end. Matchers don't return the value that was passed into them when they fail.
/// 2. If it fails (meaning that all the matchers succeeded), the output it produces won't be
///    particularly useful. Matchers don't produce failure output when they succeed.
///
/// # Examples
///
/// Unlike [`each`], this matcher lets you chain matchers together to do things like this.
///
/// ```
/// use xpct::{expect, all, equal, be_some};
///
/// fn favorite_clothing() -> Option<String> {
///     Some(String::from("necktie"))
/// }
///
/// expect!(favorite_clothing()).to(all(|ctx| ctx
///     .to(be_some())?
///     .to(equal("necktie"))
/// ));
/// ```
///
/// Instead of returning the transformed value at the end, when negated, it returns `()`.
///
/// ```
/// use xpct::{expect, all, equal, be_some};
///
/// fn favorite_clothing() -> Option<String> {
///     None
/// }
///
/// let result: () = expect!(favorite_clothing())
///     .to_not(all(|ctx| ctx
///         .to(be_some())?
///         .to(equal("necktie"))
///     ))
///     .into_inner();
/// ```
///
/// [`each`]: crate::each
/// [`Assertion`]: crate::core::Assertion
/// [`to`]: crate::matchers::ChainAssertion::to
/// [`to_not`]: crate::matchers::ChainAssertion::to_not
/// [`not`]: crate::not
/// [`expect!`]: crate::expect
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
