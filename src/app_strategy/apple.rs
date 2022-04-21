use crate::base_strategy;
use crate::base_strategy::BaseStrategy;
use std::path::PathBuf;

/// This is the strategy created by Apple for use on macOS and iOS devices. It is always used by GUI apps on macOS, and is sometimes used by command-line applications there too. iOS only has GUIs, so all iOS applications follow this strategy. The specification is available [here](https://developer.apple.com/library/archive/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW1).
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Apple;
/// use std::path::Path;
///
/// let app_strategy = Apple::new(AppStrategyArgs {
///     top_level_domain: "com".to_string(),
///     author: "Apple".to_string(),
///     app_name: "Safari".to_string(),
/// }).unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     app_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new("Library/Preferences/com.apple.Safari/")
/// ));
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new("Library/Application Support/com.apple.Safari/")
/// ));
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new("Library/Caches/com.apple.Safari/")
/// ));
/// assert_eq!(
///     app_strategy.state_dir(),
///     None
/// );
/// ```
#[derive(Debug)]
pub struct Apple {
    base_strategy: base_strategy::Apple,
    bundle_id: String,
}

impl super::AppStrategy for Apple {
    type CreationError = crate::HomeDirError;

    fn new(args: super::AppStrategyArgs) -> Result<Self, Self::CreationError> {
        Ok(Self {
            base_strategy: base_strategy::Apple::new()?,
            bundle_id: args.bundle_id(),
        })
    }

    fn config_dir(&self) -> PathBuf {
        self.base_strategy.config_dir().join(&self.bundle_id)
    }

    fn data_dir(&self) -> PathBuf {
        self.base_strategy.data_dir().join(&self.bundle_id)
    }

    fn cache_dir(&self) -> PathBuf {
        self.base_strategy.cache_dir().join(&self.bundle_id)
    }

    fn state_dir(&self) -> Option<PathBuf> {
        None
    }
}
