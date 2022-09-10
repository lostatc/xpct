use super::format::{Format, Formatter, ResultFormat};

#[derive(Debug)]
pub enum MatchFailure<Pos, Neg = Pos> {
    Pos(Pos),
    Neg(Neg),
}

impl<Pos, Neg> MatchFailure<Pos, Neg> {
    pub fn is_pos(&self) -> bool {
        match self {
            Self::Pos(_) => true,
            Self::Neg(_) => false,
        }
    }

    pub fn is_neg(&self) -> bool {
        match self {
            Self::Pos(_) => false,
            Self::Neg(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct DynMatchFailure(String);

impl DynMatchFailure {
    pub fn new<Fmt, PosFail, NegFail>(fail: MatchFailure<PosFail, NegFail>) -> Self
    where
        Fmt: ResultFormat<Pos = PosFail, Neg = NegFail>,
    {
        let mut formatter = Formatter::new();
        Fmt::new(fail).fmt(&mut formatter);
        Self(formatter.into_inner())
    }
}

impl Format for DynMatchFailure {
    fn fmt(&self, f: &mut Formatter) {
        f.write_str(self.0.as_str());
    }
}

#[derive(Debug)]
pub enum MatchResult<T, Fail> {
    Success(T),
    Fail(Fail),
}

#[derive(Debug)]
pub enum MatchError {
    Fail(DynMatchFailure),
    Err(anyhow::Error),
}
