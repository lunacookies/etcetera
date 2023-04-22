use crate::base_strategy;
use crate::base_strategy::BaseStrategy;
use std::path::{Path, PathBuf};

/// This strategy follows Windows’ conventions. It seems that all Windows GUI apps, and some command-line ones follow this pattern. The specification is available [here](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid).
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Windows;
/// use std::path::Path;
///
/// let app_strategy = Windows::new(AppStrategyArgs {
///     top_level_domain: "org".to_string(),
///     author: "Acme Corp".to_string(),
///     app_name: "Frobnicator Plus".to_string(),
/// }).unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     app_strategy.home_dir(),
///     &home_dir
/// );
/// assert_eq!(
///     app_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/Acme Corp/Frobnicator Plus/config"))
/// );
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/Acme Corp/Frobnicator Plus/data"))
/// );
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Local/Acme Corp/Frobnicator Plus/cache"))
/// );
/// assert_eq!(
///     app_strategy.state_dir(),
///     None
/// );
/// assert_eq!(
///     app_strategy.runtime_dir(),
///     None
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Windows {
    base_strategy: base_strategy::Windows,
    author_app_name_path: PathBuf,
}

macro_rules! dir_method {
    ($self: ident, $base_strategy_method: ident, $subfolder_name: expr) => {{
        let mut path = $self.base_strategy.$base_strategy_method();
        path.push(&$self.author_app_name_path);
        path.push($subfolder_name);

        path
    }};
}

impl super::AppStrategy for Windows {
    type CreationError = crate::HomeDirError;

    fn new(args: super::AppStrategyArgs) -> Result<Self, Self::CreationError> {
        Ok(Self {
            base_strategy: base_strategy::Windows::new()?,
            author_app_name_path: PathBuf::from(format!("{}/{}", args.author, args.app_name)),
        })
    }

    fn home_dir(&self) -> &Path {
        self.base_strategy.home_dir()
    }

    fn config_dir(&self) -> PathBuf {
        dir_method!(self, config_dir, "config/")
    }

    fn data_dir(&self) -> PathBuf {
        dir_method!(self, data_dir, "data/")
    }

    fn cache_dir(&self) -> PathBuf {
        dir_method!(self, cache_dir, "cache/")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        None
    }

    fn runtime_dir(&self) -> Option<PathBuf> {
        None
    }
}
