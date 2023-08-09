use std::borrow::Cow;
use std::fmt;

use crate::core::{style, Format, FormattedFailure, Formatter, MatchFailure, Matcher};

enum WhyFormatReason<'a> {
    Eager(Cow<'a, str>),
    Lazy(Box<dyn Fn() -> Cow<'a, str> + 'a>),
}

impl<'a> fmt::Debug for WhyFormatReason<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eager(reason) => f.debug_tuple("Eager").field(reason).finish(),
            Self::Lazy(_) => f.debug_tuple("Lazy").finish(),
        }
    }
}

/// A formatter that prints a context string provided by [`why`] or [`why_lazy`].
#[derive(Debug)]
pub struct WhyFormat<'a> {
    reason: WhyFormatReason<'a>,
}

impl<'a> WhyFormat<'a> {
    /// Create a new [`WhyFormat`] from the given string.
    pub fn new(reason: impl Into<Cow<'a, str>>) -> Self {
        Self {
            reason: WhyFormatReason::Eager(reason.into()),
        }
    }

    /// Create a new [`WhyFormat`] from the given function, which will be called lazily if the
    /// matcher fails.
    pub fn lazy(reason: impl Fn() -> Cow<'a, str> + 'a) -> Self {
        Self {
            reason: WhyFormatReason::Lazy(Box::new(reason)),
        }
    }
}

impl<'a> Format for WhyFormat<'a> {
    type Value = MatchFailure<FormattedFailure>;

    fn fmt(&self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        f.set_style(style::info());
        f.write_str(style::WHY_SYMBOL);
        f.write_str(" ");

        match &self.reason {
            WhyFormatReason::Eager(reason) => {
                f.write_str(reason.as_ref());
            }
            WhyFormatReason::Lazy(func) => {
                let reason = (func)();
                f.write_str(reason.as_ref());
            }
        };

        f.reset_style();
        f.write_char('\n');
        f.write_fmt(value);

        Ok(())
    }
}

/// Attaches a context string to a matcher that will appear in the failure output.
///
/// # Examples
///
/// ```
/// use xpct::{expect, why, be_ge};
///
/// let index = 2_i32;
///
/// expect!(index).to(why(
///     be_ge(0),
///     "indices must be positive"
/// ));
/// ```
pub fn why<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
    reason: impl Into<Cow<'a, str>>,
) -> Matcher<In, PosOut, NegOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    matcher.wrapped(WhyFormat::new(reason))
}

/// Attaches a lazily evaluated context string to a matcher that will appear in the failure output.
///
/// This serves the same purpose as [`why`], except it can be used when the context value would be
/// expensive to compute.
///
/// # Example
///
/// ```
/// use std::borrow::Cow;
/// use xpct::{expect, why_lazy, be_ge};
///
/// // Imagine this is expensive to compute.
/// fn expensive_context() -> Cow<'static, str> {
///     Cow::Borrowed("indices must be positive")
/// }
///
/// let index = 2_i32;
///
/// expect!(index).to(why_lazy(
///     be_ge(0),
///     expensive_context
/// ));
/// ```
pub fn why_lazy<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
    reason: impl Fn() -> Cow<'a, str> + 'a,
) -> Matcher<In, PosOut, NegOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    matcher.wrapped(WhyFormat::lazy(reason))
}
