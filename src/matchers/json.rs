#![cfg(feature = "json")]

use std::borrow::Cow;

use serde_json::{from_str as json_from_str, Value as JsonValue};

use crate::core::SimpleMatch;

use super::Mismatch;

/// The matcher for [`match_json`].
///
/// [`match_json`]: crate::match_json
#[derive(Debug)]
pub struct MatchJsonMatcher<'a> {
    expected_json: Cow<'a, str>,
}

impl<'a> MatchJsonMatcher<'a> {
    /// Create a new [`MatchJsonMatcher`] from the expected JSON string.
    pub fn new(json: impl Into<Cow<'a, str>>) -> Self {
        Self {
            expected_json: json.into(),
        }
    }
}

impl<'a, Actual> SimpleMatch<Actual> for MatchJsonMatcher<'a>
where
    Actual: AsRef<str>,
{
    type Fail = Mismatch<Cow<'a, str>, Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        let expected_value: JsonValue = json_from_str(&self.expected_json)?;
        let actual_value: JsonValue = json_from_str(actual.as_ref())?;
        Ok(actual_value == expected_value)
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Mismatch {
            expected: self.expected_json,
            actual,
        }
    }
}