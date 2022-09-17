use std::borrow::Cow;
use std::fmt;

use crate::core::{style, DynMatchFailure, Format, Formatter, MatchFailure, Matcher, ResultFormat};

enum WhyFormatReason<'a> {
    Eager(Cow<'a, str>),
    Lazy(Box<dyn FnOnce() -> Cow<'a, str> + 'a>),
}

impl<'a> fmt::Debug for WhyFormatReason<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eager(reason) => f.debug_tuple("Eager").field(reason).finish(),
            Self::Lazy(_) => f
                .debug_tuple("Lazy")
                .field(&std::any::type_name::<Box<dyn FnOnce() -> String>>())
                .finish(),
        }
    }
}

#[derive(Debug)]
pub struct WhyFormat<'a> {
    reason: WhyFormatReason<'a>,
}

impl<'a> WhyFormat<'a> {
    pub fn new(reason: impl Into<Cow<'a, str>>) -> Self {
        Self {
            reason: WhyFormatReason::Eager(reason.into()),
        }
    }

    pub fn lazy(reason: impl FnOnce() -> Cow<'a, str> + 'a) -> Self {
        Self {
            reason: WhyFormatReason::Lazy(Box::new(reason)),
        }
    }
}

impl<'a> Format for WhyFormat<'a> {
    type Value = MatchFailure<DynMatchFailure>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::info());
        f.write_str(style::INFO_SYMBOL);
        f.write_str(" ");

        match self.reason {
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

impl<'a> ResultFormat for WhyFormat<'a> {
    type Pos = DynMatchFailure;
    type Neg = DynMatchFailure;
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn why<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
    reason: impl Into<Cow<'a, str>>,
) -> Matcher<In, PosOut, NegOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::wrap(matcher, WhyFormat::new(reason))
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn why_lazy<'a, In, PosOut, NegOut>(
    matcher: Matcher<'a, In, PosOut, NegOut>,
    reason: impl FnOnce() -> Cow<'a, str> + 'a,
) -> Matcher<In, PosOut, NegOut>
where
    In: 'a,
    PosOut: 'a,
    NegOut: 'a,
{
    Matcher::wrap(matcher, WhyFormat::lazy(reason))
}
