use std::fmt;
use std::marker::PhantomData;

use crate::core::{style, Format, Formatter, MatchFailure, Matcher};
use crate::matchers::{EqualMatcher, Mismatch};

/// A formatter for [`Mismatch`] types.
///
/// This formatter can be used to implement custom matchers without having to manually implement a
/// formatter.
///
/// # Examples
///
/// ```
/// # use xpct::{core::{Matcher, SimpleMatch}, matchers::Mismatch, format::MismatchFormat};
/// #
/// # struct SharesPrefixMatcher;
/// #
/// # impl SharesPrefixMatcher {
/// #     pub fn new() -> Self {
/// #         Self
/// #     }
/// # }
/// #
/// # impl SimpleMatch<String> for SharesPrefixMatcher {
/// #     type Fail = Mismatch<String, String>;
/// #
/// #   fn matches(&mut self, actual: &String) -> anyhow::Result<bool> {
/// #       unimplemented!()
/// #   }
/// #
/// #   fn fail(self, actual: String) -> Self::Fail {
/// #       unimplemented!()
/// #   }
/// # }
/// #
/// pub fn shares_prefix_with<'a>(expected: String) -> Matcher<'a, String, String> {
///     Matcher::simple(
///         SharesPrefixMatcher::new(),
///         MismatchFormat::new(
///             "to share a common prefix with",
///             "to not share a common prefix with"
///         ),
///     )
/// }
/// ```
#[derive(Debug)]
pub struct MismatchFormat<Actual, Expected> {
    marker: PhantomData<(Actual, Expected)>,
    pos_msg: String,
    neg_msg: String,
}

impl<Actual, Expected> MismatchFormat<Actual, Expected> {
    /// Create a new [`MismatchFormat`].
    ///
    /// This accepts two error messages: the one to use in the *positive* case (when we were
    /// expecting the matcher to match) and the one to use in the *negative* case (when we were
    /// expecting the matcher to not match).
    pub fn new(pos_msg: impl Into<String>, neg_msg: impl Into<String>) -> Self {
        Self {
            marker: PhantomData,
            pos_msg: pos_msg.into(),
            neg_msg: neg_msg.into(),
        }
    }
}

impl<Actual, Expected> Format for MismatchFormat<Actual, Expected>
where
    Actual: fmt::Debug,
    Expected: fmt::Debug,
{
    type Value = MatchFailure<Mismatch<Actual, Expected>>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        match value {
            MatchFailure::Pos(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.actual));

                f.set_style(style::important());
                f.write_str(self.pos_msg);
                f.write_str(":\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.expected));
            }
            MatchFailure::Neg(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.actual));

                f.set_style(style::important());
                f.write_str(self.neg_msg);
                f.write_str(":\n");

                f.set_style(style::bad());
                f.write_str(format!("{}{:?}\n", style::indent(1), mismatch.expected));
            }
        };

        Ok(())
    }
}

/// Matches when two values are equal.
///
/// # Examples
///
/// ```
/// use xpct::{expect, equal};
///
/// expect!(Some("oblivion")).to(equal(Some("oblivion")));
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::simple(
        EqualMatcher::new(expected),
        MismatchFormat::new("to equal", "to not equal"),
    )
}
