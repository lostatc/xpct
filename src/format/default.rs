use std::fmt;

use crate::{core::Matcher, matchers::BeDefaultMatcher};

use super::MismatchFormat;

/// Succeeds when the actual value equals the default value for the type.
///
/// # Examples
///
/// ```
/// use xpct::{expect, be_default};
///
/// expect!("").to(be_default());
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub fn be_default<'a, Actual>() -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + Default + PartialEq<Actual> + Eq + 'a,
{
    Matcher::simple(
        BeDefaultMatcher::new(),
        MismatchFormat::new("to be the default value", "to not be the default value"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expect;

    #[test]
    fn succeeds_on_default_value() {
        expect!("").to(be_default());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_default_value() {
        expect!("not default").to(be_default());
    }
    #[test]
    #[should_panic]
    fn fails_on_default_value() {
        expect!("").to_not(be_default());
    }

    #[test]
    fn succeeds_when_not_default_value() {
        expect!("not default").to_not(be_default());
    }
}
