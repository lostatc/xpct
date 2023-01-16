use std::{
    marker::PhantomData,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
};

use crate::core::{Match, MatchOutcome};

/// An integer type which has a non-zero counterpart.
pub trait NonZeroInt: crate::Sealed {
    /// The non-zero counterpart of this integer type.
    type NonZero;

    /// Convert this integer into its non-zero counterpart.
    fn into_non_zero(self) -> Option<Self::NonZero>;

    /// Return whether this integer is `0`.
    fn is_zero(&self) -> bool;
}

impl crate::Sealed for isize {}

impl NonZeroInt for isize {
    type NonZero = NonZeroIsize;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroIsize::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for i8 {}

impl NonZeroInt for i8 {
    type NonZero = NonZeroI8;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroI8::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for i16 {}

impl NonZeroInt for i16 {
    type NonZero = NonZeroI16;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroI16::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for i32 {}

impl NonZeroInt for i32 {
    type NonZero = NonZeroI32;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroI32::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for i64 {}

impl NonZeroInt for i64 {
    type NonZero = NonZeroI64;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroI64::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for i128 {}

impl NonZeroInt for i128 {
    type NonZero = NonZeroI128;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroI128::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for usize {}

impl NonZeroInt for usize {
    type NonZero = NonZeroUsize;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroUsize::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for u8 {}

impl NonZeroInt for u8 {
    type NonZero = NonZeroU8;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroU8::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for u16 {}

impl NonZeroInt for u16 {
    type NonZero = NonZeroU16;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroU16::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for u32 {}

impl NonZeroInt for u32 {
    type NonZero = NonZeroU32;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroU32::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for u64 {}

impl NonZeroInt for u64 {
    type NonZero = NonZeroU64;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroU64::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

impl crate::Sealed for u128 {}

impl NonZeroInt for u128 {
    type NonZero = NonZeroU128;

    fn into_non_zero(self) -> Option<Self::NonZero> {
        NonZeroU128::new(self)
    }

    fn is_zero(&self) -> bool {
        *self == 0
    }
}

/// The matcher for [`be_zero`].
///
/// [`be_zero`]: crate::be_zero
#[derive(Debug, Default)]
pub struct BeZeroMatcher<T> {
    marker: PhantomData<T>,
}

impl<T> BeZeroMatcher<T> {
    /// Create a new [`BeZeroMatcher`].
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<T> Match for BeZeroMatcher<T>
where
    T: NonZeroInt,
{
    type In = T;

    type PosOut = T;
    type NegOut = T::NonZero;

    type PosFail = ();
    type NegFail = ();

    fn match_pos(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::PosOut, Self::PosFail>> {
        if actual.is_zero() {
            Ok(MatchOutcome::Success(actual))
        } else {
            Ok(MatchOutcome::Fail(()))
        }
    }

    fn match_neg(
        self,
        actual: Self::In,
    ) -> crate::Result<MatchOutcome<Self::NegOut, Self::NegFail>> {
        Ok(match T::into_non_zero(actual) {
            Some(value) => MatchOutcome::Success(value),
            None => MatchOutcome::Fail(()),
        })
    }
}
