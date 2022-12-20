use crate::core::SimpleMatch;

/// The matcher for [`be_true`] and [`be_false`].
///
/// [`be_true`]: crate::be_true
/// [`be_false`]: crate::be_false
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct BeTrueMatcher;

impl BeTrueMatcher {
    /// Create a new [`BeTrueMatcher`].
    pub fn new() -> Self {
        Self
    }
}

impl SimpleMatch<bool> for BeTrueMatcher {
    type Fail = ();

    fn matches(&mut self, actual: &bool) -> crate::Result<bool> {
        Ok(*actual)
    }

    fn fail(self, _: bool) -> Self::Fail {}
}
