use std::fmt;

use crate::core::Matcher;
use crate::matchers::{Pattern, PatternMatcher};

use super::MismatchFormat;

/// Succeeds when the given pattern matches.
///
/// This matcher is used with the [`pattern!`] macro.
///
/// [`pattern!`]: crate::pattern
///
/// # Examples
///
/// Pattern matching lets you do things you can't do with other matchers, like test for a specific
/// enum variant when the enum doesn't implement `Eq`.
///
/// ```
/// use xpct::{expect, match_pattern, pattern};
///
/// #[derive(Debug)]
/// enum ConnectionError {
///     Disconnected,
///     Unavailable,
///     Unknown,
/// }
///
/// fn connect() -> Result<(), ConnectionError> {
///     Err(ConnectionError::Unavailable)
/// }
///
/// expect!(connect()).to(match_pattern(
///     pattern!(Err(ConnectionError::Unavailable))
/// ));
/// ```
pub fn match_pattern<'a, Actual>(spec: Pattern<'a, Actual>) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + 'a,
{
    Matcher::simple(
        PatternMatcher::new(spec),
        MismatchFormat::new("to match the pattern", "to not match the pattern"),
    )
}
