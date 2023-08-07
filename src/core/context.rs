/// A location in a Rust source file.
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
