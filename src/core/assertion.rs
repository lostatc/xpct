use std::convert::TryInto;

use crate::matchers::IterMap;

use super::{
    AssertionFailure, AssertionFormat, DynMatch, FormattedOutput, MatchError, MatchOutcome,
};

/// An assertion, the starting point in a chain of matchers.
///
/// This is the value returned by [`expect!`]. You can use the [`to`] and [`to_not`] methods to use
/// matchers, chaining the output of each into the input of the next.
///
/// [`expect!`]: crate::expect
/// [`to`]: crate::core::Assertion::to
/// [`to_not`]: crate::core::Assertion::to_not
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
    /// Make an assertion with the given `matcher`.
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

    /// Same as [`to`], but negated.
    ///
    /// This does the same thing as the [`not`] matcher.
    ///
    /// This tests that the given matcher does *not* succeed.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::{expect, equal};
    ///
    /// expect!("foo").to_not(equal("bar"));
    /// ```
    ///
    /// [`to`]: crate::core::Assertion::to
    /// [`not`]: crate::not
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

    /// Infallibly map the input value by applying a function to it.
    ///
    /// This does the same thing as the [`map`] matcher.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::Infallible;
    /// use xpct::{expect, equal};
    ///
    /// fn do_stuff() -> Result<String, Infallible> {
    ///     Ok(String::from("foobar"))
    /// }
    ///
    /// expect!(do_stuff())
    ///     .map(Result::unwrap)
    ///     .to(equal("foobar"));
    /// ```
    ///
    /// [`map`]: crate::map
    pub fn map<Out>(self, func: impl FnOnce(In) -> Out) -> Assertion<Out, AssertFmt> {
        Assertion {
            value: func(self.value),
            format: self.format,
            ctx: self.ctx,
        }
    }

    /// Fallibly map the input value by applying a function to it.
    ///
    /// This does the same thing as the [`try_map`] matcher.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::{expect, equal};
    ///
    /// expect!(vec![0x43, 0x75, 0x6e, 0x6f])
    ///     .try_map(|bytes| Ok(String::from_utf8(bytes)?))
    ///     .to(equal("Cuno"));
    /// ```
    ///
    /// [`try_map`]: crate::map
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

    /// Infallibly convert the input value via [`From`]/[`Into`].
    ///
    /// This does the same thing as the [`into`] matcher.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::{expect, equal};
    ///
    /// expect!(41u32)
    ///     .into::<u64>()
    ///     .to(equal(41u64));
    /// ```
    ///
    /// [`into`]: crate::into
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

    /// Fallibly convert the input value via [`TryFrom`]/[`TryInto`].
    ///
    /// This does the same thing as the [`try_into`] matcher.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::{expect, equal};
    ///
    /// expect!(41u64)
    ///     .try_into::<u32>()
    ///     .to(equal(41u32));
    /// ```
    ///
    /// [`try_into`]: crate::try_into
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

    /// Consume this assertion and return the value passed to [`expect!`].
    ///
    /// If the value has been transformed by any matchers like [`be_ok`] or [`be_some`], this
    /// returns the final value.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::{expect, be_some, equal};
    /// let result: &str = expect!(Some("disco"))
    ///     .to(be_some())
    ///     .to(equal("disco"))
    ///     .into_inner();
    /// ```
    ///
    /// [`expect!`]: crate::expect
    /// [`be_ok`]: crate::be_ok
    /// [`be_some`]: crate::be_some
    pub fn into_inner(self) -> In {
        self.value
    }

    /// Get the context value associated with this assertion.
    pub fn ctx(&self) -> &AssertFmt::Context {
        &self.ctx
    }

    /// Get a mutable reference to the context value associated with this assertion.
    pub fn ctx_mut(&mut self) -> &mut AssertFmt::Context {
        &mut self.ctx
    }

    /// Apply a function to change the context value associated with this function.
    pub fn with_ctx(mut self, block: impl FnOnce(&mut AssertFmt::Context)) -> Self {
        block(&mut self.ctx);
        self
    }

    /// Get the formatter associated with this assertion.
    pub fn fmt(&self) -> &AssertFmt {
        &self.format
    }

    /// Get a mutable reference to the formatter associated with this assertion.
    pub fn fmt_mut(&mut self) -> &mut AssertFmt {
        &mut self.format
    }

    /// Apply a function to change the formatter associated with this function.
    pub fn with_fmt(mut self, block: impl FnOnce(&mut AssertFmt)) -> Self {
        block(&mut self.format);
        self
    }
}

impl<In, AssertFmt> Assertion<In, AssertFmt>
where
    In: IntoIterator,
    AssertFmt: AssertionFormat,
{
    /// Infallibly map each value of an iterator by applying a function to it.
    ///
    /// This does the same thing as the [`iter_map`] matcher.
    ///
    /// # Examples
    ///
    /// This fails to compile if we try to pass `items` by reference.
    ///
    /// ```compile_fail
    /// use xpct::{be_some, every, expect};
    ///
    /// let items = vec![Some("foo"), Some("bar")];
    ///
    /// let output: Vec<&str> = expect!(&items)
    ///     .to(every(be_some))
    ///     .into_inner();
    /// ```
    ///
    /// To fix that, we need to call [`Option::as_deref`] on each value.
    ///
    /// ```
    /// use xpct::{be_some, every, expect};
    ///
    /// let items = vec![Some("foo"), Some("bar")];
    ///
    /// let output: Vec<&str> = expect!(&items)
    ///     .iter_map(Option::as_deref)
    ///     .to(every(be_some))
    ///     .into_inner();
    /// ```
    ///
    /// [`iter_map`]: crate::iter_map
    pub fn iter_map<'a, Out>(
        self,
        func: impl Fn(In::Item) -> Out + 'a,
    ) -> Assertion<IterMap<'a, In::Item, Out, In::IntoIter>, AssertFmt> {
        Assertion {
            value: IterMap::new(self.value.into_iter(), Box::new(func)),
            format: self.format,
            ctx: self.ctx,
        }
    }

    /// Fallibly map each value of an iterator by applying a function to it.
    ///
    /// This does the same thing as the [`iter_try_map`] matcher.
    ///
    /// # Examples
    ///
    /// ```
    /// use xpct::{expect, consist_of};
    ///
    /// let small_integers: [u64; 2] = [41, 57];
    ///
    /// expect!(small_integers)
    ///     .iter_try_map(|value| Ok(u32::try_from(value)?))
    ///     .to(consist_of([41u32, 57u32]));
    /// ```
    ///
    /// [`iter_try_map`]: crate::iter_try_map
    pub fn iter_try_map<'a, Out>(
        self,
        func: impl Fn(In::Item) -> crate::Result<Out> + 'a,
    ) -> Assertion<Vec<Out>, AssertFmt> {
        let mapped_values = self
            .value
            .into_iter()
            .map(func)
            .collect::<Result<Vec<_>, _>>();

        Assertion {
            value: match mapped_values {
                Ok(vec) => vec,
                Err(error) => fail(self.ctx, MatchError::Err(error), self.format),
            },
            format: self.format,
            ctx: self.ctx,
        }
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
/// # Examples
///
/// You can use this function to write your *own* function or macro like [`expect!`] which hooks
/// into your custom [`AssertionFormat`].
///
/// ```
/// use xpct::core::{expect, Assertion, Format, Formatter, AssertionFailure};
///
/// #[derive(Debug, Default)]
/// struct MyAssertionFormat;
///
/// #[derive(Debug, Default)]
/// struct MyAssertionContext {
///     some_value: String,
/// }
///
/// impl Format for MyAssertionFormat {
///     type Value = AssertionFailure<MyAssertionContext>;
///
///     fn fmt(self, f: &mut Formatter, value: Self::Value) -> xpct::Result<()> {
///         todo!()
///     }
/// }
///
/// fn my_expect<In>(actual: In) -> Assertion<In, MyAssertionFormat> {
///     expect::<In, MyAssertionFormat>(actual).with_ctx(|ctx| {
///         ctx.some_value = String::from("Some value");
///     })
/// }
/// ```
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
/// Under the hood, this macro calls the [`expect`] function using [`DefaultAssertionFormat`]. If
/// you want to use a custom [`AssertionFormat`] instead, you can call that function directly or
/// write your own macro that calls it.
///
/// # Examples
///
/// ```
/// use xpct::{expect, equal};
///
/// expect!("disco").to(equal("disco"));
/// ```
///
/// [`expect`]: crate::core::expect
/// [`AssertionFormat`]: crate::core::AssertionFormat
/// [`DefaultAssertionFormat`]: crate::core::DefaultAssertionFormat
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
