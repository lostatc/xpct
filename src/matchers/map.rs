use std::any::type_name;
use std::fmt;

use crate::core::{FormattedFailure, MatchBase, MatchOutcome, MatchPos};
use crate::success;

pub struct MapMatcher<'a, In, Out> {
    func: Box<dyn FnOnce(In) -> Out + 'a>,
}

impl<'a, In, Out> MapMatcher<'a, In, Out> {
    pub fn new<F>(func: F) -> Self
    where
        F: FnOnce(In) -> Out + 'a,
    {
        Self {
            func: Box::new(func),
        }
    }
}

impl<'a, In, Out> fmt::Debug for MapMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapMatcher")
            .field("func", &type_name::<Box<dyn FnOnce(In) -> Out + 'a>>())
            .finish()
    }
}

impl<'a, In, Out> MatchBase for MapMatcher<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> MatchPos for MapMatcher<'a, In, Out> {
    type PosOut = Out;
    type PosFail = FormattedFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!((self.func)(actual))
    }
}

pub struct TryMapMatcher<'a, In, Out> {
    func: Box<dyn FnOnce(In) -> crate::Result<Out> + 'a>,
}

impl<'a, In, Out> TryMapMatcher<'a, In, Out> {
    pub fn new(func: impl FnOnce(In) -> crate::Result<Out> + 'a) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

impl<'a, In, Out> fmt::Debug for TryMapMatcher<'a, In, Out> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapErrMatcher")
            .field(
                "func",
                &type_name::<Box<dyn FnOnce(In) -> crate::Result<Out>>>(),
            )
            .finish()
    }
}

impl<'a, In, Out> MatchBase for TryMapMatcher<'a, In, Out> {
    type In = In;
}

impl<'a, In, Out> MatchPos for TryMapMatcher<'a, In, Out> {
    type PosOut = Out;
    type PosFail = FormattedFailure;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        success!((self.func)(actual)?)
    }
}
