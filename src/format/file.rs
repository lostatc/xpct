use std::fmt;
use std::path::Path;

use crate::core::Matcher;
use crate::matchers::{FileExistsMatcher, FileExistsMode};

use super::ExpectationFormat;

/// Succeeds when the actual value is the path of an existing file.
///
/// This follows symbolic links.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xpct::{expect, be_existing_file};
///
/// expect!(Path::new("/path/to/file")).to(be_existing_file());
/// ```
pub fn be_existing_file<'a, Actual>() -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<Path> + 'a,
{
    Matcher::new(
        FileExistsMatcher::new(FileExistsMode::Exists),
        ExpectationFormat::new(
            "to exist in the filesystem",
            "to not exist in the filesystem",
        ),
    )
}

/// Succeeds when the actual value is the path of an existing regular file.
///
/// This follows symbolic links.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xpct::{expect, be_regular_file};
///
/// expect!(Path::new("/path/to/regular/file")).to(be_regular_file());
/// ```
pub fn be_regular_file<'a, Actual>() -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<Path> + 'a,
{
    Matcher::new(
        FileExistsMatcher::new(FileExistsMode::RegularFile),
        ExpectationFormat::new("to exist and be a regular file", "to not be a regular file"),
    )
}

/// Succeeds when the actual value is the path of an existing directory.
///
/// This follows symbolic links.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xpct::{expect, be_directory};
///
/// expect!(Path::new("/path/to/directory")).to(be_directory());
/// ```
pub fn be_directory<'a, Actual>() -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<Path> + 'a,
{
    Matcher::new(
        FileExistsMatcher::new(FileExistsMode::Directory),
        ExpectationFormat::new("to exist and be a directory", "to not be a directory"),
    )
}

/// Succeeds when the actual value is the path of an existing symbolic link.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use xpct::{expect, be_symlink};
///
/// expect!(Path::new("/path/to/symlink")).to(be_symlink());
/// ```
pub fn be_symlink<'a, Actual>() -> Matcher<'a, Actual, Actual>
where
    Actual: fmt::Debug + AsRef<Path> + 'a,
{
    Matcher::new(
        FileExistsMatcher::new(FileExistsMode::Symlink),
        ExpectationFormat::new(
            "to exist and be a symbolic link",
            "to not be a symbolic link",
        ),
    )
}
