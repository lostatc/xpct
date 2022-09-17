use crate::core::SimpleMatch;

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct BeTrueMatcher;

impl BeTrueMatcher {
    pub fn new() -> Self {
        Self
    }
}

impl SimpleMatch<bool> for BeTrueMatcher {
    type Fail = ();

    fn matches(&mut self, actual: &bool) -> anyhow::Result<bool> {
        Ok(*actual)
    }

    fn fail(self, _: bool) -> Self::Fail {
        ()
    }
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct BeFalseMatcher;

impl BeFalseMatcher {
    pub fn new() -> Self {
        Self
    }
}

impl SimpleMatch<bool> for BeFalseMatcher {
    type Fail = ();

    fn matches(&mut self, actual: &bool) -> anyhow::Result<bool> {
        Ok(!actual)
    }

    fn fail(self, _: bool) -> Self::Fail {
        ()
    }
}
