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
///
/// You could also use it to test for an enum variant while ignoring its fields.
///
/// ```
/// use xpct::{expect, match_pattern, pattern};
///
/// #[derive(Debug)]
/// enum Command {
///     Create(String),
///     Update(String),
///     Delete,
/// }
///
/// let command = Command::Create("foo".into());
///
/// expect!(command).to(match_pattern(pattern!(
///     Command::Create(_) | Command::Delete
/// )));
/// ```
pub fn match_pattern<'a, Actual>(spec: Pattern<'a, Actual>) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + 'a,
{
    Matcher::new(
        PatternMatcher::new(spec),
        MismatchFormat::new("to match the pattern", "to not match the pattern"),
    )
}
