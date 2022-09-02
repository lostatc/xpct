use super::format::{DefaultFormat, ErrorFormatter, FormatError};
use std::fmt;

use super::location::AssertionLocation;

#[derive(Debug)]
pub enum FailReason {
    Fail(String),
    Err(anyhow::Error),
}

impl fmt::Display for FailReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fail(msg) => f.write_str(msg),
            Self::Err(error) => error.fmt(f),
        }
    }
}

impl From<String> for FailReason {
    fn from(value: String) -> Self {
        Self::Fail(value)
    }
}

impl From<anyhow::Error> for FailReason {
    fn from(err: anyhow::Error) -> Self {
        Self::Err(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCase {
    Positive,
    Negative,
}

enum FailReasonBuilder {
    Fail(Box<dyn FnOnce(ErrorCase) -> String>),
    Err(anyhow::Error),
}

impl FailReasonBuilder {
    fn build(self, case: ErrorCase) -> FailReason {
        match self {
            Self::Fail(func) => FailReason::Fail(func(case)),
            Self::Err(error) => FailReason::Err(error),
        }
    }
}

impl fmt::Debug for FailReasonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fail(_) => f
                .debug_tuple("Fail")
                .field(&String::from(std::any::type_name::<
                    Box<dyn FnOnce(ErrorCase) -> String>,
                >()))
                .finish(),
            Self::Err(error) => f.debug_tuple("Err").field(error).finish(),
        }
    }
}

#[derive(Debug)]
pub struct MatchError(FailReasonBuilder);

impl MatchError {
    pub fn new(func: impl FnOnce(ErrorCase) -> String + 'static) -> Self {
        Self(FailReasonBuilder::Fail(Box::new(func)))
    }

    fn into_reason(self, case: ErrorCase) -> FailReason {
        self.0.build(case)
    }
}

impl From<anyhow::Error> for MatchError {
    fn from(error: anyhow::Error) -> Self {
        Self(FailReasonBuilder::Err(error))
    }
}

pub trait Matcher {
    type In;
    type Out;

    fn matches(&mut self, actual: Self::In) -> Result<Self::Out, MatchError>;
}

struct AssertionData {
    name: Option<String>,
    location: Option<AssertionLocation>,
    fmt: Box<dyn FormatError>,
}

impl AssertionData {
    fn fail(self, error: MatchError, case: ErrorCase) -> ! {
        let mut formatter = ErrorFormatter::new(error.into_reason(case), self.name, self.location);

        self.fmt.fmt(&mut formatter);

        panic!("{}", formatter.msg());
    }
}

impl Default for AssertionData {
    fn default() -> Self {
        Self {
            name: None,
            location: None,
            fmt: Box::new(DefaultFormat),
        }
    }
}

impl fmt::Debug for AssertionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AssertionData")
            .field("name", &self.name)
            .field("location", &self.location)
            .field(
                "fmt",
                &String::from(std::any::type_name::<Box<dyn FormatError>>()),
            )
            .finish()
    }
}

#[derive(Debug)]
pub struct Assertion<V> {
    value: V,
    data: AssertionData,
}

impl<V> Assertion<V> {
    pub fn to<M: Matcher<In = V>>(self, matcher: &mut M) -> Assertion<M::Out> {
        match matcher.matches(self.value) {
            Ok(value) => Assertion {
                value,
                data: self.data,
            },
            Err(err) => self.data.fail(err, ErrorCase::Positive),
        }
    }

    pub fn to_not<M: Matcher<In = V>>(self, matcher: &mut M) -> Assertion<M::Out> {
        match matcher.matches(self.value) {
            Ok(value) => Assertion {
                value,
                data: self.data,
            },
            Err(err) => self.data.fail(err, ErrorCase::Negative),
        }
    }

    pub fn with_name(self, name: impl Into<String>) -> Assertion<V> {
        Assertion {
            value: self.value,
            data: AssertionData {
                name: Some(name.into()),
                location: self.data.location,
                fmt: self.data.fmt,
            },
        }
    }

    pub fn with_location(self, location: impl Into<AssertionLocation>) -> Assertion<V> {
        Assertion {
            value: self.value,
            data: AssertionData {
                name: self.data.name,
                location: Some(location.into()),
                fmt: self.data.fmt,
            },
        }
    }
}

pub fn expect<V>(actual: V) -> Assertion<V> {
    Assertion {
        value: actual,
        data: Default::default(),
    }
}

macro_rules! expect {
    ($actual:expr) => {
        expect($actual)
            .with_name(stringify!($actual))
            .with_location(file_location!())
    };
}
