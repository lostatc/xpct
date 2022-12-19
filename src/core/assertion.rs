use std::convert::TryInto;

use super::{
    AssertionFailure, AssertionFormat, DynMatch, FormattedOutput, MatchError, MatchOutcome,
};

#[derive(Debug)]
pub struct Assertion<In, AssertFmt>
where
    AssertFmt: AssertionFormat,
{
    value: In,
    format: AssertFmt,
    ctx: AssertFmt::Context,
}

fn fail<Context, AssertFmt>(ctx: Context, error: MatchError, format: AssertFmt) -> !
where
    AssertFmt: AssertionFormat<Context = Context>,
{
    FormattedOutput::new(AssertionFailure { ctx, error }, format)
        .expect("failed to format matcher output")
        .fail();
}

impl<In, AssertFmt> Assertion<In, AssertFmt>
where
    AssertFmt: AssertionFormat,
{
    pub fn to<Out>(
        self,
        matcher: impl DynMatch<In = In, PosOut = Out>,
    ) -> Assertion<Out, AssertFmt> {
        match Box::new(matcher).match_pos(self.value) {
            Ok(MatchOutcome::Success(out)) => Assertion {
                value: out,
                format: self.format,
                ctx: self.ctx,
            },
            Ok(MatchOutcome::Fail(result)) => fail(self.ctx, MatchError::Fail(result), self.format),
            Err(error) => fail(self.ctx, MatchError::Err(error), self.format),
        }
    }

    pub fn to_not<Out>(
        self,
        matcher: impl DynMatch<In = In, NegOut = Out>,
    ) -> Assertion<Out, AssertFmt> {
        match Box::new(matcher).match_neg(self.value) {
            Ok(MatchOutcome::Success(out)) => Assertion {
                value: out,
                format: self.format,
                ctx: self.ctx,
            },
            Ok(MatchOutcome::Fail(result)) => fail(self.ctx, MatchError::Fail(result), self.format),
            Err(error) => fail(self.ctx, MatchError::Err(error), self.format),
        }
    }

    pub fn map<Out>(self, func: impl FnOnce(In) -> Out) -> Assertion<Out, AssertFmt> {
        Assertion {
            value: func(self.value),
            format: self.format,
            ctx: self.ctx,
        }
    }

    pub fn try_map<Out>(
        self,
        func: impl FnOnce(In) -> crate::Result<Out>,
    ) -> Assertion<Out, AssertFmt> {
        match func(self.value) {
            Ok(out) => Assertion {
                value: out,
                format: self.format,
                ctx: self.ctx,
            },
            Err(error) => fail(self.ctx, MatchError::Err(error), self.format),
        }
    }

    pub fn into<Out>(self) -> Assertion<Out, AssertFmt>
    where
        Out: From<In>,
    {
        Assertion {
            value: self.value.into(),
            format: self.format,
            ctx: self.ctx,
        }
    }

    pub fn try_into<Out>(self) -> Assertion<Out, AssertFmt>
    where
        Out: TryFrom<In>,
        <Out as TryFrom<In>>::Error: std::error::Error + Send + Sync + 'static,
    {
        Assertion {
            value: match self.value.try_into() {
                Ok(out) => out,
                Err(error) => fail(self.ctx, MatchError::Err(error.into()), self.format),
            },
            format: self.format,
            ctx: self.ctx,
        }
    }

    pub fn into_inner(self) -> In {
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

/// Make an assertion.
///
/// Typically you'll want to use the [`expect!`] macro instead, because it does nice things like
/// capture the file name, line number, and the stringified expression that was passed to it.
///
/// However, if you want to use a custom [`AssertionFormat`], then this function allows you to do
/// it.
///
/// [`expect!`]: crate::expect
pub fn expect<In, AssertFmt>(actual: In) -> Assertion<In, AssertFmt>
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

/// Make an assertion.
///
/// This macro accepts an expression and returns an [`Assertion`], which allows you to make
/// assertions on that value.
///
/// See the crate-level docs for more information and examples.
///
/// # Examples
///
/// ```
/// use xpct::{expect, equal};
///
/// expect!("disco").to(equal("disco"));
/// ```
#[macro_export]
macro_rules! expect {
    ($actual:expr) => {
        $crate::core::expect::<_, $crate::core::DefaultAssertionFormat>($actual).with_ctx(|ctx| {
            ctx.expr =
                ::std::option::Option::Some(::std::string::String::from(stringify!($actual)));
            ctx.location = ::std::option::Option::Some($crate::file_location!());
        })
    };
}
