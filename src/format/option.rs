use std::convert::Infallible;
use std::fmt;
use std::marker::PhantomData;

use crate::core::{style, Format, Formatter, MatchFailure, Matcher, NegFormat};
use crate::matchers::option::BeSomeMatcher;
use crate::matchers::Expectation;

/// A formatter for [`Expectation`] values.
///
/// # Examples
///
/// ```
/// # use xpct::format::ExpectationFormat;
/// let format: ExpectationFormat<Option<String>> = ExpectationFormat::new(
///     "to be Some(_)",
///     "to be None",
/// );
/// ```
///
/// ```
/// # use std::io::Error;
/// # use xpct::format::ExpectationFormat;
/// let format: ExpectationFormat<Result<String, Error>> = ExpectationFormat::new(
///     "to be Ok(_)",
///     "to be Err(_)",
/// );
/// ```
#[derive(Debug)]
pub struct ExpectationFormat<Actual> {
    marker: PhantomData<Actual>,
    pos_msg: String,
    neg_msg: String,
}

impl<Actual> ExpectationFormat<Actual> {
    /// Create a new [`ExpectationFormat`].
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

impl<Actual> Format for ExpectationFormat<Actual>
where
    Actual: fmt::Debug,
{
    type Value = MatchFailure<Expectation<Actual>>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        f.set_style(style::important());
        f.write_str("Expected:\n");

        match value {
            MatchFailure::Pos(expectation) => {
                f.set_style(style::bad());
                f.indented(style::indent(1), |f| {
                    f.write_str(format!("{:?}", expectation.actual));
                    Ok(())
                })?;
                f.write_char('\n');

                f.set_style(style::important());
                f.write_str(self.pos_msg);
                f.write_char('\n');
            }
            MatchFailure::Neg(expectation) => {
                f.set_style(style::bad());
                f.indented(style::indent(1), |f| {
                    f.write_str(format!("{:?}", expectation.actual));
                    Ok(())
                })?;
                f.write_char('\n');

                f.set_style(style::important());
                f.write_str(self.neg_msg);
                f.write_char('\n');
            }
        };

        Ok(())
    }
}

fn option_format<T>() -> ExpectationFormat<Option<T>> {
    ExpectationFormat::new("to be Some(_)", "to be None")
}

/// Succeeds when the actual value is [`Some`].
///
/// If this matcher succeeds, it unwraps the [`Some`] value. When negated, it behaves like
/// [`be_none`].
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_some, equal};
///
/// let value = Some("foobar");
///
/// expect!(value)
///     .to(be_some())
///     .to(equal("foobar"));
/// ```
pub fn be_some<'a, T>() -> Matcher<'a, Option<T>, T, Option<Infallible>>
where
    T: fmt::Debug + 'a,
{
    Matcher::transform(BeSomeMatcher::new(), option_format())
}

/// Succeeds when the actual value is [`None`].
///
/// If this matcher succeeds, it converts the value to `Option<Infallible>`. When negated, it
/// behaves like [`be_some`].
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use xpct::{expect, be_none, equal};
///
/// let value: Option<String> = None;
///
/// let output: Option<Infallible> = expect!(value)
///     .to(be_none())
///     .into_inner();
/// ```
pub fn be_none<'a, T>() -> Matcher<'a, Option<T>, Option<Infallible>, T>
where
    T: fmt::Debug + 'a,
{
    Matcher::transform_neg(BeSomeMatcher::new(), NegFormat(option_format()))
}

#[cfg(test)]
mod tests {
    use super::{be_none, be_some};
    use crate::expect;

    fn some() -> Option<()> {
        Some(())
    }

    fn none() -> Option<()> {
        None
    }

    #[test]
    fn succeeds_when_some() {
        expect!(some()).to(be_some());
    }

    #[test]
    fn succeeds_when_not_some() {
        expect!(none()).to_not(be_some());
    }

    #[test]
    #[should_panic]
    fn fails_when_some() {
        expect!(some()).to_not(be_some());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_some() {
        expect!(none()).to(be_some());
    }

    #[test]
    fn succeeds_when_none() {
        expect!(none()).to(be_none());
    }

    #[test]
    fn succeeds_when_not_none() {
        expect!(some()).to_not(be_none());
    }

    #[test]
    #[should_panic]
    fn fails_when_none() {
        expect!(none()).to_not(be_none());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_none() {
        expect!(some()).to(be_none());
    }
}
