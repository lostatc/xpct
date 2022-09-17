use crate::core::{style, Format, MatchFailure, Matcher};
use crate::matchers::{BeFalseMatcher, BeTrueMatcher};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct BeTrueFormat;

impl BeTrueFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for BeTrueFormat {
    type Value = MatchFailure<(), ()>;

    fn fmt(self, f: &mut crate::core::Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::bad());
        f.write_str(match value {
            MatchFailure::Pos(_) => "Expected this to be true.\n",
            MatchFailure::Neg(_) => "Expected this to be false.\n",
        });
        f.reset_style();

        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct BeFalseFormat;

impl BeFalseFormat {
    pub fn new() -> Self {
        Self
    }
}

impl Format for BeFalseFormat {
    type Value = MatchFailure<(), ()>;

    fn fmt(self, f: &mut crate::core::Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::bad());
        f.write_str(match value {
            MatchFailure::Pos(_) => "Expected this to be false.\n",
            MatchFailure::Neg(_) => "Expected this to be true.\n",
        });
        f.reset_style();

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_true() -> Matcher<'static, bool, bool> {
    Matcher::simple(BeTrueMatcher::new(), BeTrueFormat::new())
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_false() -> Matcher<'static, bool, bool> {
    Matcher::simple(BeFalseMatcher::new(), BeFalseFormat::new())
}
