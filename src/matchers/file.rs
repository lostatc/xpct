use std::path::Path;
use std::{fs, io};

use crate::core::SimpleMatch;

use super::Expectation;

/// How an [`FileExistsMatcher`] should match.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FileExistsMode {
    /// Succeeds if the file exists.
    ///
    /// This mode follows symlinks.
    Exists,

    /// Succeeds if the file exists and is a regular file.
    ///
    /// This mode follows symlinks.
    RegularFile,

    /// Succeeds if the file exists and is a directory.
    ///
    /// This mode follows symlinks.
    Directory,

    /// Succeeds if the file exists and is a symbolic link.
    Symlink,
}

/// The matcher for [`be_existing_file`], [`be_regular_file`], [`be_directory`], and [`be_symlink`].
///
/// [`be_existing_file`]: crate::be_existing_file
/// [`be_regular_file`]: crate::be_regular_file
/// [`be_directory`]: crate::be_directory
/// [`be_symlink`]: crate::be_symlink
#[derive(Debug)]
pub struct FileExistsMatcher {
    mode: FileExistsMode,
}

impl FileExistsMatcher {
    /// Create a new [`FileExistsMatcher`] with the given `mode`.
    pub fn new(mode: FileExistsMode) -> Self {
        Self { mode }
    }
}

impl<Actual> SimpleMatch<Actual> for FileExistsMatcher
where
    Actual: AsRef<Path>,
{
    type Fail = Expectation<Actual>;

    fn matches(&mut self, actual: &Actual) -> crate::Result<bool> {
        let metadata_result = if self.mode == FileExistsMode::Symlink {
            fs::symlink_metadata(actual)
        } else {
            fs::metadata(actual.as_ref())
        };

        let metadata = match metadata_result {
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
            Ok(metadata) => metadata,
        };

        match self.mode {
            FileExistsMode::Exists => Ok(true),
            FileExistsMode::RegularFile if metadata.is_file() => Ok(true),
            FileExistsMode::Directory if metadata.is_dir() => Ok(true),
            FileExistsMode::Symlink if metadata.is_symlink() => Ok(true),
            _ => Ok(false),
        }
    }

    fn fail(self, actual: Actual) -> Self::Fail {
        Expectation { actual }
    }
}
