use core::fmt;

use super::format::{Display, Formatter};

#[derive(Debug)]
pub enum MatchError<Reason> {
    Fail(Reason),
    Err(anyhow::Error),
}

impl<Reason> From<anyhow::Error> for MatchError<Reason> {
    fn from(error: anyhow::Error) -> Self {
        Self::Err(error)
    }
}


pub struct DynMatchError(Box<dyn Display>);

impl DynMatchError {
    pub(crate) fn new<Reason, ReasonFmt, ErrorFmt>(error: MatchError<Reason>) -> Self
    where
        ReasonFmt: Display + From<Reason> + 'static,
        ErrorFmt: Display + From<anyhow::Error> + 'static,
    {
        Self(match error {
            MatchError::Fail(reason) => Box::new(ReasonFmt::from(reason)),
            MatchError::Err(error) => Box::new(ErrorFmt::from(error)),
        })
    }
}

impl fmt::Debug for DynMatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynMatchError").finish()
    }
}

impl Display for DynMatchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
