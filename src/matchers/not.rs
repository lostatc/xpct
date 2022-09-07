use crate::{
    DynMap, Format, Formatter, MapNeg, MapPos, MatchBase, MatchCase, MatchFailure, MatchResult,
    Matcher, Matches, ResultFormat,
};

#[derive(Debug)]
pub struct MaybeFailure(Option<MatchFailure>);

impl Matches for MaybeFailure {
    fn matches(&self) -> bool {
        self.0.is_none()
    }
}

pub struct MaybeFailureFormat {
    result: MaybeFailure,
    case: MatchCase,
}

impl Format for MaybeFailureFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl ResultFormat for MaybeFailureFormat {
    type Res = MaybeFailure;

    fn new(result: Self::Res, case: MatchCase) -> Self {
        Self { result, case }
    }
}

pub struct NotMatcher<In, PosOut, NegOut>(
    Box<dyn DynMap<In = In, PosOut = PosOut, NegOut = NegOut>>,
);

impl<In, PosOut, NegOut> NotMatcher<In, PosOut, NegOut> {
    pub fn new<M, ResFmt>(matcher: Matcher<M, ResFmt>) -> Self
    where
        M: MatchBase<In = In> + MapPos<PosOut = PosOut> + MapNeg<NegOut = NegOut> + 'static,
        ResFmt: ResultFormat<Res = M::Res>,
    {
        NotMatcher(Box::new(matcher))
    }
}

impl<In, PosOut, NegOut> MatchBase for NotMatcher<In, PosOut, NegOut> {
    type In = In;
    type Res = MaybeFailure;
}

impl<In, PosOut, NegOut> MapPos for NotMatcher<In, PosOut, NegOut> {
    type PosOut = NegOut;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::Res>> {
        match self.0.map_neg(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(MaybeFailure(Some(result)))),
            Err(error) => Err(error),
        }
    }
}

impl<In, PosOut, NegOut> MapNeg for NotMatcher<In, PosOut, NegOut> {
    type NegOut = PosOut;

    fn map_neg(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::NegOut, Self::Res>> {
        match self.0.map_pos(actual) {
            Ok(MatchResult::Success(out)) => Ok(MatchResult::Success(out)),
            Ok(MatchResult::Fail(result)) => Ok(MatchResult::Fail(MaybeFailure(Some(result)))),
            Err(error) => Err(error),
        }
    }
}

pub fn not<M, ResFmt>(matcher: Matcher<M, ResFmt>) -> NotMatcher<M::In, M::PosOut, M::NegOut>
where
    M: MapPos + MapNeg + 'static,
    ResFmt: ResultFormat<Res = M::Res>,
{
    NotMatcher::new(matcher)
}
