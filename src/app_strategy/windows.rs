use crate::base_strategy;
use crate::base_strategy::BaseStrategy;
use std::path::PathBuf;

/// This strategy follows Windows’ conventions. It seems that all Windows GUI apps, and some command-line ones follow this pattern.
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Windows;
/// use std::path::Path;
///
/// let app_strategy = Windows::new(AppStrategyArgs {
///     top_level_domain: "com".to_string(),
///     author: "Microsoft".to_string(),
///     app_name: "File Explorer".to_string(),
/// }).unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     app_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/Microsoft/File Explorer/config"))
/// );
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Roaming/Microsoft/File Explorer/data"))
/// );
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new("AppData/Local/Microsoft/File Explorer/cache"))
/// );
/// assert_eq!(
///     app_strategy.state_dir(),
///     None
/// );
/// ```
#[derive(Debug)]
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
}
