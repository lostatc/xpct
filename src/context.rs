#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLocation {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

#[non_exhaustive]
#[derive(Debug, Default)]
pub struct AssertionContext {
    pub location: Option<FileLocation>,
    pub expr: Option<String>,
}

#[macro_export]
macro_rules! file_location {
    () => {
        $crate::FileLocation {
            file: String::from(file!()),
            line: line!(),
            column: column!(),
        }
    };
}
