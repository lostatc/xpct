use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertionLocation {
    File(FileLocation),
    Raw(String),
}

impl From<FileLocation> for AssertionLocation {
    fn from(value: FileLocation) -> Self {
        Self::File(value)
    }
}

impl From<String> for AssertionLocation {
    fn from(value: String) -> Self {
        Self::Raw(value)
    }
}

impl fmt::Display for AssertionLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(FileLocation { file, line, column }) => {
                f.write_fmt(format_args!("{}:{}:{}", file, line, column))
            }
            Self::Raw(string) => f.write_str(string),
        }
    }
}

macro_rules! file_location {
    () => {
        FileLocation {
            file: String::from(file!()),
            line: line!(),
            column: column!(),
        }
    };
}
