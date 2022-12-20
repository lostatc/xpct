use std::marker::PhantomData;

use crate::core::{style, Format, Formatter, MatchFailure, Matcher, NegFormat};
use crate::matchers::BeTrueMatcher;

/// A formatter which prints a static string message.
///
/// # Examples
///
/// ```
/// # use xpct::format::MessageFormat;
/// let format: MessageFormat = MessageFormat::new(
///     "Expected this to be true.",
///     "Expected this to be false."
/// );
/// ```
#[derive(Debug)]
pub struct MessageFormat<PosFail = (), NegFail = ()> {
    marker: PhantomData<(PosFail, NegFail)>,
    pos_msg: String,
    neg_msg: String,
}

impl<PosFail, NegFail> MessageFormat<PosFail, NegFail> {
    /// Create a new [`MessageFormat`].
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

impl<PosFail, NegFail> Format for MessageFormat<PosFail, NegFail> {
    type Value = MatchFailure<PosFail, NegFail>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> crate::Result<()> {
        f.set_style(style::bad());
        f.write_str(match value {
            MatchFailure::Pos(_) => self.pos_msg,
            MatchFailure::Neg(_) => self.neg_msg,
        });
        f.reset_style();
        f.write_char('\n');

        Ok(())
    }
}

fn bool_format() -> MessageFormat {
    MessageFormat::new("Expected this to be true.", "Expected this to be false.")
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_true() -> Matcher<'static, bool, bool> {
    Matcher::simple(BeTrueMatcher::new(), bool_format())
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_false() -> Matcher<'static, bool, bool> {
    Matcher::simple_neg(BeTrueMatcher::new(), NegFormat(bool_format()))
}
