use crate::core::{style, Format, Formatter, MatchFailure, Matcher};
use crate::matchers::BeTrueMatcher;

#[derive(Debug)]
pub struct MessageFormat {
    pos_msg: String,
    neg_msg: String,
}

impl MessageFormat {
    pub fn new(pos_msg: impl Into<String>, neg_msg: impl Into<String>) -> Self {
        Self {
            pos_msg: pos_msg.into(),
            neg_msg: neg_msg.into(),
        }
    }
}

impl Format for MessageFormat {
    type Value = MatchFailure<(), ()>;

    fn fmt(self, f: &mut Formatter, value: Self::Value) -> anyhow::Result<()> {
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

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_true() -> Matcher<'static, bool, bool> {
    Matcher::simple(
        BeTrueMatcher::new(),
        MessageFormat::new("Expected this to be true.", "Expected this to be false."),
    )
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_false() -> Matcher<'static, bool, bool> {
    Matcher::simple_neg(
        BeTrueMatcher::new(),
        MessageFormat::new("Expected this to be false.", "Expected this to be true."),
    )
}
