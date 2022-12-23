#![cfg(feature = "json")]
#![cfg_attr(docsrs, doc(cfg(all(feature = "fmt", feature = "json"))))]

use std::borrow::Cow;
use std::fmt;

use crate::core::Matcher;
use crate::matchers::MatchJsonMatcher;

use super::MismatchFormat;

/// Succeeds when the actual string and the expected string are equivalent JSON.
///
/// This allows you to compare JSON strings, ignoring whitespace and the order of keys in objects.
/// The order of elements in arrays is still significant, though.
///
/// # Examples
///
/// ```
/// use xpct::{expect, match_json};
///
/// let expected = r#"
///     {
///         "name": "Reál",
///         "code": "IIR"
///     }
/// "#;
///
/// expect!(r#"{"code":"IIR","name":"Reál"}"#).to(match_json(expected));
/// ```
pub fn match_json<'a, Actual>(json: impl Into<Cow<'a, str>>) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<str> + 'a,
{
    Matcher::simple(
        MatchJsonMatcher::new(json),
        MismatchFormat::new("to be equivalent JSON to", "to not be equivalent JSON to"),
    )
}

#[cfg(test)]
mod tests {
    use super::match_json;
    use crate::expect;

    fn expected() -> &'static str {
        r#"
            {
                "key1": "value1",
                "key2": "value2"
            }
        "#
    }

    fn actual() -> &'static str {
        r#"{"key2":"value2","key1":"value1"}"#
    }

    #[test]
    fn succeeds_when_matches_json() {
        expect!(actual()).to(match_json(expected()));
    }

    #[test]
    fn succeeds_when_not_matches_json() {
        expect!(actual()).to_not(match_json(r#""different json""#));
    }

    #[test]
    #[should_panic]
    fn fails_when_matches_json() {
        expect!(actual()).to_not(match_json(expected()));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_matches_json() {
        expect!(actual()).to(match_json(r#""different json""#));
    }
}
