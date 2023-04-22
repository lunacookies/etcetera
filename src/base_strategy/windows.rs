use std::path::{Path, PathBuf};

/// This strategy follows Windows’ conventions. It seems that all Windows GUI apps, and some command-line ones follow this pattern. The specification is available [here](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid).
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Windows;
/// use std::path::Path;
///
/// let base_strategy = Windows::new().unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     base_strategy.home_dir(),
///     &home_dir
/// );
/// assert_eq!(
///     base_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/"))
/// );
/// assert_eq!(
///     base_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/"))
/// );
/// assert_eq!(
///     base_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Local/"))
/// );
/// assert_eq!(
///     base_strategy.state_dir(),
///     None
/// );
/// assert_eq!(
///     base_strategy.runtime_dir(),
///     None
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Windows {
    home_dir: PathBuf,
}

impl super::BaseStrategy for Windows {
    type CreationError = crate::HomeDirError;

    fn new() -> Result<Self, Self::CreationError> {
        Ok(Self {
            home_dir: crate::home_dir()?,
        })
    }

    fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    fn config_dir(&self) -> PathBuf {
        self.home_dir.join("AppData/Roaming/")
    }

    fn data_dir(&self) -> PathBuf {
        self.home_dir.join("AppData/Roaming/")
    }

    fn cache_dir(&self) -> PathBuf {
        self.home_dir.join("AppData/Local/")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        None
    }

    fn runtime_dir(&self) -> Option<PathBuf> {
        None
    }
}
