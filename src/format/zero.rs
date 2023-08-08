use std::fmt;

use crate::core::Matcher;
use crate::matchers::numbers::{BeZeroMatcher, NonZeroInt};

use super::ExpectationFormat;

/// Succeeds when the actual integer value is `0`.
///
/// When negated, this matcher converts the value to its non-zero counterpart ([`NonZeroU8`],
/// [`NonZeroI32`], etc.). Otherwise, it behaves like `equal(0)`.
///
/// # Examples
///
/// ```
/// use std::num::NonZeroU32;
/// use xpct::{expect, be_zero};
///
/// let result: NonZeroU32 = expect!(10u32)
///     .to_not(be_zero())
///     .into_inner();
/// ```
///
/// [`NonZeroU8`]: std::num::NonZeroU8
/// [`NonZeroI32`]: std::num::NonZeroI32
pub fn be_zero<'a, T>() -> Matcher<'a, T, T, T::NonZero>
where
    T: fmt::Debug + NonZeroInt + 'a,
{
    Matcher::transform(
        BeZeroMatcher::new(),
        ExpectationFormat::new("to be 0", "to not be 0"),
    )
}

#[cfg(test)]
mod tests {
    use super::be_zero;
    use crate::expect;

    #[test]
    fn succeeds_when_zero() {
        expect!(0).to(be_zero());
    }

    #[test]
    fn succeeds_when_not_zero() {
        expect!(10).to_not(be_zero());
    }

    #[test]
    #[should_panic]
    fn fails_when_zero() {
        expect!(0).to_not(be_zero());
    }

    #[test]
    #[should_panic]
    fn fails_when_not_zero() {
        expect!(10).to(be_zero());
    }
}
