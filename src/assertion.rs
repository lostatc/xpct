use super::format::{AssertionFormat, Formatter, ResultFormat};
use super::matcher::{DynMapNeg, DynMapPos, MapNeg, MapPos, Matcher};
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
    let error = AssertFmt::new(ctx, error);
    let mut formatter = Formatter::new();

    error
        .fmt(&mut formatter)
        .expect("Failed to format error message.");

    panic!("{}", formatter.as_str());
}

impl<T, AssertFmt> Assertion<T, AssertFmt>
where
    AssertFmt: AssertionFormat,
{
    pub fn to<M, ResultFmt>(self, matcher: &mut Matcher<M, ResultFmt>) -> M::PosOut
    where
        M: MapPos<In = T>,
        ResultFmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
    {
        match matcher.map_pos(self.value) {
            Ok(MatchResult::Success(out)) => out,
            Ok(MatchResult::Fail(result)) => fail::<AssertFmt>(self.ctx, MatchError::Fail(result)),
            Err(error) => fail::<AssertFmt>(self.ctx, MatchError::Err(error)),
        }
    }

    pub fn to_not<M, ResultFmt>(self, matcher: &mut Matcher<M, ResultFmt>) -> M::NegOut
    where
        M: MapNeg<In = T>,
        ResultFmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
    {
        match matcher.map_neg(self.value) {
            Ok(MatchResult::Success(out)) => out,
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
