use std::convert::Infallible;

use crate::{
    core::{style, Format, MatchFailure, Matcher, NegFormat},
    matchers::BeSomeMatcher,
};

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct BeSomeFormat;

impl BeSomeFormat {
    pub fn new() -> Self {
        Self
    }

    pub fn neg() -> NegFormat<Self> {
        NegFormat(Self)
    }
}

impl Format for BeSomeFormat {
    type Value = MatchFailure<(), ()>;

    fn fmt(self, f: &mut crate::core::Formatter, value: Self::Value) -> anyhow::Result<()> {
        f.set_style(style::bad());
        f.write_str(match value {
            MatchFailure::Pos(_) => "Expected this to be Some.\n",
            MatchFailure::Neg(_) => "Expected this to be None.\n",
        });
        f.reset_style();

        Ok(())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_some<'a, T>() -> Matcher<'a, Option<T>, T, Option<Infallible>>
where
    T: 'a,
{
    Matcher::new(BeSomeMatcher::new(), BeSomeFormat::new())
}

#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_none<'a, T>() -> Matcher<'a, Option<T>, Option<Infallible>, T>
where
    T: 'a,
{
    Matcher::neg(BeSomeMatcher::new(), BeSomeFormat::neg())
}
