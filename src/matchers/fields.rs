use std::any::type_name;
use std::collections::HashMap;
use std::fmt;

use crate::core::{DynMatchFailure, MatchBase, MatchPos, MatchResult};

pub type FailuresByField = HashMap<&'static str, Option<DynMatchFailure>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByMatchMode {
    Any,
    All,
}

pub struct ByFieldMatcher<'a, T> {
    func: Box<dyn FnOnce(T) -> anyhow::Result<FailuresByField> + 'a>,
    mode: ByMatchMode,
}

impl<'a, T> fmt::Debug for ByFieldMatcher<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ByFieldMatcher")
            .field(
                "func",
                &type_name::<Box<dyn FnOnce(T) -> anyhow::Result<FailuresByField> + 'a>>(),
            )
            .field("mode", &self.mode)
            .finish()
    }
}

impl<'a, T> ByFieldMatcher<'a, T> {
    pub fn new(
        mode: ByMatchMode,
        func: impl FnOnce(T) -> anyhow::Result<FailuresByField> + 'a,
    ) -> Self {
        Self {
            func: Box::new(func),
            mode,
        }
    }
}

impl<'a, T> MatchBase for ByFieldMatcher<'a, T> {
    type In = T;
}

impl<'a, T> MatchPos for ByFieldMatcher<'a, T> {
    type PosOut = ();
    type PosFail = FailuresByField;

    fn match_pos(
        self,
        actual: Self::In,
    ) -> anyhow::Result<MatchResult<Self::PosOut, Self::PosFail>> {
        let failures = (self.func)(actual)?;
        match self.mode {
            ByMatchMode::Any => {
                if failures.values().any(Option::is_none) {
                    Ok(MatchResult::Success(()))
                } else {
                    Ok(MatchResult::Fail(failures))
                }
            }
            ByMatchMode::All => {
                if failures.values().all(Option::is_none) {
                    Ok(MatchResult::Success(()))
                } else {
                    Ok(MatchResult::Fail(failures))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! fields {
    (
        $struct_type:ty {
            $(
                $field_name:tt: $matcher:expr
            ),+
            $(,)?
        }
    ) => {
        |input: $struct_type| {
            Ok(std::collections::HashMap::from([$(
                (
                    stringify!($field_name),
                    match $crate::core::DynMatchPos::match_pos(Box::new($matcher), input.$field_name)? {
                        $crate::core::MatchResult::Success(_) => None,
                        $crate::core::MatchResult::Fail(fail) => Some(fail),
                    },
                ),
            )+]))
        }
    };
}
