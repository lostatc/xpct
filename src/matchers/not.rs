use std::fmt;

use crate::{
    DynMapNeg, Format, Formatter, MapNeg, MapPos, MatchBase, MatchResult,
    Matcher, MatchFailure, ResultFormat,
};

#[derive(Debug)]
pub struct NotFormat(MatchResult<(), MatchFailure>);

impl Format for NotFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl From<MatchResult<(), MatchFailure>> for NotFormat {
    fn from(result: MatchResult<(), MatchFailure>) -> Self {
        Self(result)
    }
}

impl ResultFormat for NotFormat {
    type Success = ();
    type Fail = MatchFailure;
}

pub struct NotMatcher<In, Out>(
    Box<dyn DynMapNeg<In = In, NegOut = Out>>,
);

impl<In, Out> fmt::Debug for NotMatcher<In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NotMatcher").finish()
    }
}

impl<In, Out> NotMatcher<In, Out> {
    pub fn new<M, Fmt>(matcher: Matcher<M, Fmt>) -> Self
    where
        M: MapNeg<In = In, NegOut = Out> + 'static,
        Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
    {
        NotMatcher(Box::new(matcher))
    }
}

impl<In, Out> MatchBase for NotMatcher<In, Out> {
    type In = In;
    type Success = ();
    type Fail = MatchFailure;
}

impl<In, Out> MapPos for NotMatcher<In, Out> {
    type PosOut = Out;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::Fail>> {
        self.0.map_neg(actual)
    }
}

pub fn not<M, Fmt>(matcher: Matcher<M, Fmt>) -> Matcher<NotMatcher<M::In, M::NegOut>, NotFormat>
where
    M: MapNeg + 'static,
    Fmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
{
    Matcher::new(NotMatcher::new(matcher))
}
