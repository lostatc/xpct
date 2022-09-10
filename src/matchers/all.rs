use crate::{DynMatchPos, MatchError, MatchResult, DynMatchNeg, MatchPos, DynMatchFailure, MatchBase, MatchNeg, MatchFailure, Format, Formatter, ResultFormat, Matcher};

pub struct AllAssertion<T> {
    value: T,
}

impl<T> AllAssertion<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> AllAssertion<T> {
    pub fn to<Out>(self, matcher: impl DynMatchPos<In = T, PosOut = Out>) -> Result<AllAssertion<Out>, MatchError> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchResult::Success(out)) => Ok(AllAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }

    pub fn to_not<Out>(self, matcher: impl DynMatchNeg<In = T, NegOut = Out>) -> Result<AllAssertion<Out>, MatchError> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchResult::Success(out)) => Ok(AllAssertion::new(out)),
            Ok(MatchResult::Fail(fail)) => Err(MatchError::Fail(fail)),
            Err(error) => Err(MatchError::Err(error)),
        }
    }
}

pub struct AllMatcher<In, Out>(Box<dyn FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError>>);

impl<In, Out> AllMatcher<In, Out> {
    pub fn new(block: impl FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'static) -> Self {
        Self(Box::new(block))
    }
}

impl<In, Out> MatchBase for AllMatcher<In, Out> {
    type In = In;
}

impl<In, Out> MatchPos for AllMatcher<In, Out> {
    type PosOut = Out;
    type PosFail = DynMatchFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        match (self.0)(AllAssertion::new(actual)) {
            Ok(assertion) => Ok(MatchResult::Success(assertion.value)),
            Err(MatchError::Fail(fail)) => Ok(MatchResult::Fail(fail)),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}

impl<In, Out> MatchNeg for AllMatcher<In, Out> {
    type NegOut = ();
    type NegFail = ();

    fn match_neg(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::NegFail>> {
        match (self.0)(AllAssertion::new(actual)) {
            Ok(_) => Ok(MatchResult::Fail(())),
            Err(MatchError::Fail(_)) => Ok(MatchResult::Success(())),
            Err(MatchError::Err(error)) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct AllFormat(MatchFailure<DynMatchFailure, ()>);

impl Format for AllFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl ResultFormat for AllFormat {
    type Pos = DynMatchFailure;
    type Neg = ();

    fn new(fail: MatchFailure<Self::Pos, Self::Neg>) -> Self {
        Self(fail)
    }
}

pub fn all<In, Out>(
    block: impl FnOnce(AllAssertion<In>) -> Result<AllAssertion<Out>, MatchError> + 'static,
) -> Matcher<In, Out, ()>
where
    In: 'static,
    Out: 'static,
{
    Matcher::new::<AllFormat, _>(AllMatcher::new(block))
}
