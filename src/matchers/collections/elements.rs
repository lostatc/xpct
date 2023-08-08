use crate::core::{MatchOutcome, Matcher, TransformMatch};
use crate::matchers::SomeFailures;

/// The matcher for [`match_elements`].
///
/// [`match_elements`]: crate::match_elements
#[derive(Debug)]
pub struct MatchElementsMatcher<'a, PosOut, NegOut, IntoIter>
where
    IntoIter: IntoIterator + 'a,
{
    matchers: Vec<Matcher<'a, IntoIter::Item, PosOut, NegOut>>,
}

impl<'a, PosOut, NegOut, IntoIter> MatchElementsMatcher<'a, PosOut, NegOut, IntoIter>
where
    IntoIter: IntoIterator + 'a,
{
    /// Create a new [`MatchElementsMatcher`] from the given matchers.
    pub fn new(
        matchers: impl IntoIterator<Item = Matcher<'a, IntoIter::Item, PosOut, NegOut>>,
    ) -> Self {
        Self {
            matchers: matchers.into_iter().collect::<Vec<_>>(),
        }
    }
}

impl<'a, PosOut, NegOut, IntoIter> TransformMatch
    for MatchElementsMatcher<'a, PosOut, NegOut, IntoIter>
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

        for (item, matcher) in actual.into_iter().zip(self.matchers) {
            match matcher.into_box().match_pos(item)? {
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

        for (item, matcher) in actual.into_iter().zip(self.matchers) {
            match matcher.into_box().match_neg(item)? {
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
