use std::fmt;
use std::marker::PhantomData;

use crate::core::{style, Format, Formatter, MatchFailure, Matcher};
use crate::matchers::equal::EqualMatcher;
use crate::matchers::Mismatch;

/// A formatter for [`Mismatch`] values.
///
/// # Examples
///
/// ```
/// # use xpct::format::MismatchFormat;
/// let format: MismatchFormat<String, String> = MismatchFormat::new(
///     "to equal",
///     "to not equal",
/// );
/// ```
///
/// ```
/// # use xpct::format::MismatchFormat;
/// let format: MismatchFormat<u32, u32> = MismatchFormat::new(
///     "to be greater than or equal to",
///     "to not be greater than or equal to",
/// );
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
    /// expecting the matcher to succeed) and the one to use in the *negative* case (when we were
    /// expecting the matcher to fail).
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

    fn fmt(&self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        match value {
            MatchFailure::Pos(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.indented(style::indent(1), |f| {
                    f.write_str(format!("{:?}", mismatch.actual));
                    Ok(())
                })?;
                f.write_char('\n');

                f.set_style(style::important());
                f.write_str(&self.pos_msg);
                f.write_str(":\n");

                f.set_style(style::bad());
                f.indented(style::indent(1), |f| {
                    f.write_str(format!("{:?}", mismatch.expected));
                    Ok(())
                })?;
                f.write_char('\n');
            }
            MatchFailure::Neg(mismatch) => {
                f.set_style(style::important());
                f.write_str("Expected:\n");

                f.set_style(style::bad());
                f.indented(style::indent(1), |f| {
                    f.write_str(format!("{:?}", mismatch.actual));
                    Ok(())
                })?;
                f.write_char('\n');

                f.set_style(style::important());
                f.write_str(&self.neg_msg);
                f.write_str(":\n");

                f.set_style(style::bad());
                f.indented(style::indent(1), |f| {
                    f.write_str(format!("{:?}", mismatch.expected));
                    Ok(())
                })?;
                f.write_char('\n');
            }
        };

        Ok(())
    }
}

/// Succeeds when the actual value equals the expected value.
///
/// # Examples
///
/// ```
/// use xpct::{expect, equal};
///
/// expect!("foobar").to(equal("foobar"));
/// ```
pub fn equal<'a, Actual, Expected>(expected: Expected) -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + PartialEq<Expected> + Eq + 'a,
    Expected: fmt::Debug + 'a,
{
    Matcher::new(
        EqualMatcher::new(expected),
        MismatchFormat::new("to equal", "to not equal"),
    )
}

#[cfg(test)]
mod tests {
    use super::equal;
    use crate::expect;

    #[test]
    fn succeeds_when_equal() {
        expect!("some string").to(equal("some string"));
    }

    #[test]
    fn succeeds_when_not_equal() {
        expect!("some string").to_not(equal("a different string"));
    }

    #[test]
    #[should_panic]
    fn fails_when_equal() {
        expect!("some string").to_not(equal("some string"));
    }

    #[test]
    #[should_panic]
    fn fails_when_not_equal() {
        expect!("some string").to(equal("a different string"));
    }
}
