use std::path::Path;

use crate::core::Matcher;
use crate::matchers::{FileExistsMatcher, FileExistsMode};

use super::MessageFormat;

/// Succeeds when the actual value is an existing file.
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
    Actual: AsRef<Path> + 'a,
{
    Matcher::simple(
        FileExistsMatcher::new(FileExistsMode::Exists),
        MessageFormat::new(
            "Expected this file to exist",
            "Expected this file to not exist",
        ),
    )
}

/// Succeeds when the actual value is an existing regular file.
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
    Actual: AsRef<Path> + 'a,
{
    Matcher::simple(
        FileExistsMatcher::new(FileExistsMode::RegularFile),
        MessageFormat::new(
            "Expected this to be a regular file",
            "Expected this to not be a regular file",
        ),
    )
}

/// Succeeds when the actual value is an existing directory.
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
    Actual: AsRef<Path> + 'a,
{
    Matcher::simple(
        FileExistsMatcher::new(FileExistsMode::Directory),
        MessageFormat::new(
            "Expected this to be a directory",
            "Expected this to not be a directory",
        ),
    )
}

/// Succeeds when the actual value is an existing symbolic link.
///
/// This does not follow symbolic links.
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
    Actual: AsRef<Path> + 'a,
{
    Matcher::simple(
        FileExistsMatcher::new(FileExistsMode::Symlink),
        MessageFormat::new(
            "Expected this to be a symbolic link",
            "Expected this to not be a symbolic link",
        ),
    )
}
