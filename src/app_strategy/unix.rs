use std::path::PathBuf;

/// This strategy has no standard or official specification. It has arisen over time through hundreds of Unixy tools. Vim and Cargo are notable examples whose configuration/data/cache directory layouts are similar to those created by this strategy.
///
/// ```
/// use etcetera::app_strategy::AppStrategy;
/// use etcetera::app_strategy::AppStrategyArgs;
/// use etcetera::app_strategy::Unix;
/// use std::path::Path;
///
/// let app_strategy = Unix::new(AppStrategyArgs {
///     top_level_domain: "org".to_string(),
///     author: "Bram Moolenar".to_string(),
///     app_name: "Vim".to_string(),
/// }).unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     app_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".vim/")
/// ));
/// assert_eq!(
///     app_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".vim/data/")
/// ));
/// assert_eq!(
///     app_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".vim/cache/")
/// ));
/// assert_eq!(
///     app_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".vim/state/")
/// ));
/// ```
#[derive(Debug)]
pub struct Unix {
    // This is `.vim` in the above example.
    root_dir: PathBuf,
}

impl super::AppStrategy for Unix {
    type CreationError = crate::HomeDirError;

    fn new(args: super::AppStrategyArgs) -> Result<Self, Self::CreationError> {
        let mut root_dir = crate::home_dir()?;
        root_dir.push(format!(".{}", args.unixy_name()));

        Ok(Self { root_dir })
    }

    fn config_dir(&self) -> PathBuf {
        self.root_dir.clone()
    }

    fn data_dir(&self) -> PathBuf {
        self.root_dir.join("data/")
    }

    fn cache_dir(&self) -> PathBuf {
        self.root_dir.join("cache/")
    }

    fn state_dir(&self) -> Option<PathBuf> {
        Some(self.root_dir.join("state/"))
    }
}
