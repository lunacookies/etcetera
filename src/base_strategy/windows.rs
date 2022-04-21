use std::path::PathBuf;

/// This strategy follows Windows’ conventions. It seems that all Windows GUI apps, and some command-line ones follow this pattern.
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
/// ```
#[derive(Debug)]
pub struct Windows {
    appdata: PathBuf,
}

impl super::BaseStrategy for Windows {
    type CreationError = crate::HomeDirError;

    fn new() -> Result<Self, Self::CreationError> {
        Ok(Self {
            appdata: crate::home_dir()?.join("AppData/"),
        })
    }

    fn config_dir(&self) -> PathBuf {
        self.appdata.join("Roaming/")
    }

    fn data_dir(&self) -> PathBuf {
        self.appdata.join("Roaming/")
    }

    fn cache_dir(&self) -> PathBuf {
        self.appdata.join("Local/")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        None
    }
}
