use std::path::PathBuf;

/// This is the strategy created by Apple for use on macOS and iOS devices. It is always used by GUI apps on macOS, and is sometimes used by command-line applications there too. iOS only has GUIs, so all iOS applications follow this strategy. The specification is available [here](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW1).
///
/// ```
/// use etcetera::base_strategy::Apple;
/// use etcetera::base_strategy::BaseStrategy;
/// use std::path::Path;
///
/// let base_strategy = Apple::new().unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     base_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new("Library/Preferences/")
/// ));
/// assert_eq!(
///     base_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new("Library/Application Support/")
/// ));
/// assert_eq!(
///     base_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new("Library/Caches/")
/// ));
/// assert_eq!(
///     base_strategy.state_dir(),
///     None
/// );
/// ```
#[derive(Debug)]
pub struct Apple {
    library_path: PathBuf,
}

impl super::BaseStrategy for Apple {
    type CreationError = crate::HomeDirError;

    fn new() -> Result<Self, Self::CreationError> {
        let mut library_path = crate::home_dir()?;
        library_path.push("Library/");

        Ok(Self { library_path })
    }

    fn config_dir(&self) -> PathBuf {
        self.library_path.join("Preferences/")
    }

    fn data_dir(&self) -> PathBuf {
        self.library_path.join("Application Support/")
    }

    fn cache_dir(&self) -> PathBuf {
        self.library_path.join("Caches/")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        None
    }
}
