use std::fmt;

use crate::core::{DynMatch, Match, MatchOutcome, Matcher};

use super::SomeFailures;

/// The matcher for [`every`].
///
/// [`every`]: crate::every
pub struct EveryMatcher<'a, PosOut, NegOut, IntoIter>
where
    IntoIter: IntoIterator + 'a,
{
    match_func: Box<dyn Fn() -> Matcher<'a, IntoIter::Item, PosOut, NegOut> + 'a>,
}

impl<'a, PosOut, NegOut, IntoIter> fmt::Debug for EveryMatcher<'a, PosOut, NegOut, IntoIter>
where
    IntoIter: IntoIterator + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EveryMatcher").finish_non_exhaustive()
    }
}

impl<'a, PosOut, NegOut, IntoIter> EveryMatcher<'a, PosOut, NegOut, IntoIter>
where
    IntoIter: IntoIterator + 'a,
{
    /// Create a new [`EveryMatcher`] from a function that returns a matcher.
    pub fn new(match_func: impl Fn() -> Matcher<'a, IntoIter::Item, PosOut, NegOut> + 'a) -> Self {
        Self {
            match_func: Box::new(match_func),
        }
    }
}

impl<'a, PosOut, NegOut, IntoIter> Match for EveryMatcher<'a, PosOut, NegOut, IntoIter>
where
    IntoIter: IntoIterator + 'a,
{
    type In = IntoIter;

    type PosOut = Vec<PosOut>;
    type NegOut = Vec<NegOut>;

    type PosFail = SomeFailures;
    type NegFail = SomeFailures;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        let mut failures = Vec::new();
        let mut successes = Vec::new();

        for input in actual {
            match Box::new((self.match_func)()).match_pos(input)? {
                MatchOutcome::Success(success) => {
                    failures.push(None);
                    successes.push(success);
                }
                MatchOutcome::Fail(fail) => {
                    failures.push(Some(fail));
                }
            }
        }

        if failures.iter().any(Option::is_some) {
            return Ok(MatchOutcome::Fail(failures));
        }

        Ok(MatchOutcome::Success(successes))
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        let mut failures = Vec::new();
        let mut successes = Vec::new();

        for input in actual {
            match Box::new((self.match_func)()).match_neg(input)? {
                MatchOutcome::Success(success) => {
                    failures.push(None);
                    successes.push(success);
                }
                MatchOutcome::Fail(fail) => {
                    failures.push(Some(fail));
                }
            }
        }

        if failures.iter().any(Option::is_none) {
            return Ok(MatchOutcome::Success(successes));
        }

        Ok(MatchOutcome::Fail(failures))
    }
}
