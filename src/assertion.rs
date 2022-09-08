use super::format::{AssertionFormat, Formatter, ResultFormat};
use super::matcher::{DynMapNeg, DynMapPos, MapNeg, MapPos, Matcher};
use super::result::{MatchError, MatchResult};

#[derive(Debug)]
pub struct Assertion<T, Fmt>
where
    Fmt: AssertionFormat,
{
    value: T,
    ctx: Fmt::Context,
}

fn fail<Fmt>(ctx: Fmt::Context, error: MatchError) -> !
where
    Fmt: AssertionFormat,
{
    let error = Fmt::new(ctx, error);
    let mut formatter = Formatter::new();

    error
        .fmt(&mut formatter)
        .expect("Failed to format error message.");

    panic!("{}", formatter.as_str());
}

impl<T, Fmt> Assertion<T, Fmt>
where
    Fmt: AssertionFormat,
{
    pub fn to<M, ResultFmt>(self, matcher: &mut Matcher<M, ResultFmt>) -> M::PosOut
    where
        M: MapPos<In = T>,
        ResultFmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
    {
        match matcher.map_pos(self.value) {
            Ok(MatchResult::Success(out)) => out,
            Ok(MatchResult::Fail(result)) => fail::<Fmt>(self.ctx, MatchError::Fail(result)),
            Err(error) => fail::<Fmt>(self.ctx, MatchError::Err(error)),
        }
    }

    pub fn to_not<M, ResultFmt>(self, matcher: &mut Matcher<M, ResultFmt>) -> M::NegOut
    where
        M: MapNeg<In = T>,
        ResultFmt: ResultFormat<Success = M::Success, Fail = M::Fail>,
    {
        match matcher.map_neg(self.value) {
            Ok(MatchResult::Success(out)) => out,
            Ok(MatchResult::Fail(result)) => fail::<Fmt>(self.ctx, MatchError::Fail(result)),
            Err(error) => fail::<Fmt>(self.ctx, MatchError::Err(error)),
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn ctx(&self) -> &Fmt::Context {
        &self.ctx
    }

    pub fn ctx_mut(&mut self) -> &mut Fmt::Context {
        &mut self.ctx
    }

    pub fn with_ctx(mut self, block: impl FnOnce(&mut Fmt::Context)) -> Self {
        block(&mut self.ctx);
        self
    }
}

pub fn expect<T, Fmt>(actual: T) -> Assertion<T, Fmt>
where
    Fmt: AssertionFormat,
    Fmt::Context: Default,
{
    Assertion {
        value: actual,
        ctx: Default::default(),
    }
}

#[macro_export]
macro_rules! fail {
    ($reason:expr) => {
        return Ok(MatchError::Fail($reason.into()));
    };
}

#[macro_export]
macro_rules! expect {
    ($actual:expr) => {
        expect($actual).with_ctx(|ctx| {
            ctx.expr = Some(stringify!($actual));
            ctx.location = Some(file_location!());
        })
    };
}
