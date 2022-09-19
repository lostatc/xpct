/// A location in a Rust source file.
///
/// This type is returned by the [`file_location!`] macro.
///
/// [`file_location!`]: crate::file_location
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLocation {
    /// The file path.
    pub file: String,

    /// The line number.
    pub line: u32,

    /// The column number.
    pub column: u32,
}

/// The context value associated with [`DefaultAssertionFormat`].
///
/// [`DefaultAssertionFormat`]: crate::core::DefaultAssertionFormat
#[non_exhaustive]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AssertionContext {
    /// The location in the file where the assertion was made.
    pub location: Option<FileLocation>,

    /// The expression that the assertion was made against.
    pub expr: Option<String>,
}

/// Return the current [`FileLocation`].
///
/// # Examples
///
/// ```
/// use xpct::file_location;
///
/// let location = file_location!();
/// ```
#[macro_export]
macro_rules! file_location {
    () => {
        $crate::core::FileLocation {
            file: ::std::string::String::from(file!()),
            line: line!(),
            column: column!(),
        }
    };
}
