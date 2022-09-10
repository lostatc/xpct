use crate::format::AssertionFailure;

use super::format::AssertionFormat;
use super::matcher::{DynMatchNeg, DynMatchPos};
use super::result::{MatchError, MatchResult};

#[derive(Debug)]
pub struct Assertion<T, AssertFmt>
where
    AssertFmt: AssertionFormat,
{
    value: T,
    ctx: AssertFmt::Context,
}

fn fail<AssertFmt>(ctx: AssertFmt::Context, error: MatchError) -> !
where
    AssertFmt: AssertionFormat,
{
    panic!("\n{}", AssertFmt::new(AssertionFailure { ctx, error }));
}

impl<T, AssertFmt> Assertion<T, AssertFmt>
where
    AssertFmt: AssertionFormat,
{
    pub fn to<Out>(
        self,
        matcher: impl DynMatchPos<In = T, PosOut = Out>,
    ) -> Assertion<Out, AssertFmt> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchResult::Success(out)) => Assertion {
                value: out,
                ctx: self.ctx,
            },
            Ok(MatchResult::Fail(result)) => fail::<AssertFmt>(self.ctx, MatchError::Fail(result)),
            Err(error) => fail::<AssertFmt>(self.ctx, MatchError::Err(error)),
        }
    }

    pub fn to_not<Out>(
        self,
        matcher: impl DynMatchNeg<In = T, NegOut = Out>,
    ) -> Assertion<Out, AssertFmt> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchResult::Success(out)) => Assertion {
                value: out,
                ctx: self.ctx,
            },
            Ok(MatchResult::Fail(result)) => fail::<AssertFmt>(self.ctx, MatchError::Fail(result)),
            Err(error) => fail::<AssertFmt>(self.ctx, MatchError::Err(error)),
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn ctx(&self) -> &AssertFmt::Context {
        &self.ctx
    }

    pub fn ctx_mut(&mut self) -> &mut AssertFmt::Context {
        &mut self.ctx
    }

    pub fn with_ctx(mut self, block: impl FnOnce(&mut AssertFmt::Context)) -> Self {
        block(&mut self.ctx);
        self
    }
}

pub fn expect<T, AssertFmt>(actual: T) -> Assertion<T, AssertFmt>
where
    AssertFmt: AssertionFormat,
    AssertFmt::Context: Default,
{
    Assertion {
        value: actual,
        ctx: Default::default(),
    }
}

#[macro_export]
macro_rules! expect {
    ($actual:expr) => {
        expect::<_, $crate::DefaultAssertionFormat>($actual).with_ctx(|ctx| {
            ctx.expr = Some(String::from(stringify!($actual)));
            ctx.location = Some($crate::file_location!());
        })
    };
}
