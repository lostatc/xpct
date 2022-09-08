use std::fmt;

use crate::{Format, Formatter, MapPos, DynMapPos, MapNeg, DynMapNeg, MatchFailure, ResultFormat, Matcher, MatchResult, MatchBase};

#[derive(Debug)]
pub struct OrAssertion<'a, T> {
    value: T,
    failures: &'a mut Vec<Option<MatchFailure>>,
}

impl<'a, T> OrAssertion<'a, T> {
    pub fn to<M, ResultFmt>(self, matcher: &mut Matcher<M, ResultFmt>) -> anyhow::Result<()>
    where
        M: MapPos<In = T>,
        ResultFmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
    {
        match matcher.map_pos(self.value) {
            Ok(MatchResult::Success(_)) => {
                self.failures.push(None);
                Ok(())
            },
            Ok(MatchResult::Fail(result)) => {
                self.failures.push(Some(result));
                Ok(())
            },
            Err(error) => Err(error),
        }
    }

    pub fn to_not<M, ResultFmt>(self, matcher: &mut Matcher<M, ResultFmt>) -> anyhow::Result<()>
    where
        M: MapNeg<In = T>,
        ResultFmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
    {
        match matcher.map_neg(self.value) {
            Ok(MatchResult::Success(_)) => {
                self.failures.push(None);
                Ok(())
            }
            Ok(MatchResult::Fail(result)) => {
                self.failures.push(Some(result));
                Ok(())
            },
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug)]
pub struct OrContext<T> {
    value: T,
    failures: Vec<Option<MatchFailure>>,
}

impl<T> OrContext<T> {
    pub fn by_ref(&mut self) -> OrAssertion<&T> {
        OrAssertion {
            value: &self.value,
            failures: &mut self.failures,
        }
    }
}

impl<T> OrContext<T>
where
    T: Copy,
{
    pub fn copied(&mut self) -> OrAssertion<T> {
        OrAssertion {
            value: self.value,
            failures: &mut self.failures,
        }
    }
}

impl<T> OrContext<T>
where
    T: Clone,
{
    pub fn cloned(&mut self) -> OrAssertion<T> {
        OrAssertion {
            value: self.value.clone(),
            failures: &mut self.failures,
        }
    }
}

pub struct OrMatcher<T>(
    Box<dyn Fn(&mut OrContext<T>) -> anyhow::Result<()>>
);

impl<T> fmt::Debug for OrMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OrMatcher").finish()
    }
}

impl<T> OrMatcher<T> {
    pub fn new(block: impl Fn(&mut OrContext<T>) -> anyhow::Result<()> + 'static) -> Self {
        Self(Box::new(block))
    }
}

impl<T> MatchBase for OrMatcher<T> {
    type In = T;
    type Success = Vec<Option<MatchFailure>>;
    type Fail = Vec<MatchFailure>;
}

impl<T> MapPos for OrMatcher<T> {
    type PosOut = T;

    fn map_pos(
        &mut self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::Fail>> {
        let mut assertion = OrContext {
            value: actual,
            failures: Vec::new(),
        };

        (self.0)(&mut assertion)?;

        if assertion.failures.iter().any(Option::is_none) {
            Ok(MatchResult::Success(assertion.value))
        } else {
            Ok(MatchResult::Fail(assertion.failures.into_iter().filter_map(std::convert::identity).collect()))
        }
    }
}

impl<T> MapNeg for OrMatcher<T> {
    type NegOut = T;

    fn map_neg(&mut self, actual: Self::In)
        -> anyhow::Result<MatchResult<Self::NegOut, Self::Success>> {
        let mut assertion = OrContext {
            value: actual,
            failures: Vec::new(),
        };

        (self.0)(&mut assertion)?;

        if assertion.failures.iter().any(Option::is_none) {
            Ok(MatchResult::Fail(assertion.failures))
        } else {
            Ok(MatchResult::Success(assertion.value))
        }
    }
}

#[derive(Debug)]
pub struct OrFormat(MatchResult<Vec<Option<MatchFailure>>, Vec<MatchFailure>>);

impl Format for OrFormat {
    fn fmt(&self, _: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
}

impl From<MatchResult<Vec<Option<MatchFailure>>, Vec<MatchFailure>>> for OrFormat {
    fn from(result: MatchResult<Vec<Option<MatchFailure>>, Vec<MatchFailure>>) -> Self {
        Self(result)
    }
}

impl ResultFormat for OrFormat {
    type Success = Vec<Option<MatchFailure>>;
    type Fail = Vec<MatchFailure>;
}

pub fn or<T>(block: impl Fn(&mut OrContext<T>) -> anyhow::Result<()> + 'static) -> Matcher<OrMatcher<T>, OrFormat> {
    Matcher::new(OrMatcher::new(block))
}
