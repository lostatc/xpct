use super::error::DynMatchError;
use super::format::{Formatter, AssertionFormat};
use super::matcher::{Matcher, MatchContext, MatchCase, DynMatch};

#[derive(Debug)]
pub struct Assertion<T, Fmt>
where
    Fmt: AssertionFormat,
{
    value: T,
    ctx: Fmt::Context,
}

fn fail<Fmt>(ctx: Fmt::Context, error: DynMatchError) -> !
where
    Fmt: AssertionFormat,
{
    let error = Fmt::new(ctx, error);
    let mut formatter = Formatter::new();
    
    error.fmt(&mut formatter).expect("Failed to format error message.");
    
    panic!("{}", formatter.as_str());
}

impl<T, Fmt> Assertion<T, Fmt>
where
    Fmt: AssertionFormat,
{
    pub fn to<Out>(self, matcher: impl Into<Matcher<T, Out>>) -> Out {
        let ctx = MatchContext::new(MatchCase::Positive);
        match matcher.into().matches(&ctx, self.value) {
            Ok(value) => value,
            Err(error) => fail::<Fmt>(self.ctx, error),
        }
    }
    
    pub fn to_not<Out>(self, matcher: impl Into<Matcher<T, Out>>) -> Out {
        let ctx = MatchContext::new(MatchCase::Negative);
        match matcher.into().matches(&ctx, self.value) {
            Ok(value) => value,
            Err(error) => fail::<Fmt>(self.ctx, error),
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

macro_rules! fail {
    ($reason:expr) => {
        return MatchError::Fail($reason.into());
    };
}

macro_rules! expect {
    ($actual:expr) => {
        expect($actual).with_ctx(|ctx| {
            ctx.expr = Some(stringify!($actual));
            ctx.location = Some(file_location!());
        })
    };
}
