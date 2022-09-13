use super::format::{AssertionFormat, FormattedOutput, OutputStream};
use super::matcher::{DynMatchNeg, DynMatchPos};
use super::result::{AssertionFailure, MatchError, MatchResult};

#[derive(Debug)]
pub struct Assertion<T, AssertFmt>
where
    AssertFmt: AssertionFormat,
{
    value: T,
    format: AssertFmt,
    ctx: AssertFmt::Context,
}

fn fail<Context, AssertFmt>(ctx: Context, error: MatchError, format: AssertFmt) -> !
where
    AssertFmt: AssertionFormat<Context = Context>,
{
    let output = FormattedOutput::new(AssertionFailure { ctx, error }, format);
    output
        .expect("failed to format matcher failure output")
        .print(OutputStream::Stderr)
        .expect("failed to write output to stderr");
    panic!();
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
                format: self.format,
                ctx: self.ctx,
            },
            Ok(MatchResult::Fail(result)) => fail(self.ctx, MatchError::Fail(result), self.format),
            Err(error) => fail(self.ctx, MatchError::Err(error), self.format),
        }
    }

    pub fn to_not<Out>(
        self,
        matcher: impl DynMatchNeg<In = T, NegOut = Out>,
    ) -> Assertion<Out, AssertFmt> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchResult::Success(out)) => Assertion {
                value: out,
                format: self.format,
                ctx: self.ctx,
            },
            Ok(MatchResult::Fail(result)) => fail(self.ctx, MatchError::Fail(result), self.format),
            Err(error) => fail(self.ctx, MatchError::Err(error), self.format),
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

    pub fn fmt(&self) -> &AssertFmt {
        &self.format
    }

    pub fn fmt_mut(&mut self) -> &mut AssertFmt {
        &mut self.format
    }

    pub fn with_fmt(mut self, block: impl FnOnce(&mut AssertFmt)) -> Self {
        block(&mut self.format);
        self
    }
}

pub fn expect<T, AssertFmt>(actual: T) -> Assertion<T, AssertFmt>
where
    AssertFmt: AssertionFormat + Default,
    AssertFmt::Context: Default,
{
    Assertion {
        value: actual,
        format: Default::default(),
        ctx: Default::default(),
    }
}

#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! expect {
    ($actual:expr) => {
        expect::<_, $crate::DefaultAssertionFormat>($actual).with_ctx(|ctx| {
            ctx.expr = Some(String::from(stringify!($actual)));
            ctx.location = Some($crate::file_location!());
        })
    };
}
