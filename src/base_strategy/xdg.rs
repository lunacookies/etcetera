use std::path::Path;
use std::path::PathBuf;

/// This strategy implements the [XDG Base Directories Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html). It is the most common on Linux, but is increasingly being adopted elsewhere.
///
/// In this first example the relevant XDG environment variables have been unset so as to demonstrate the strategy’s behaviour when it has to fall back to the defaults.
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Xdg;
/// use std::path::Path;
///
/// std::env::remove_var("XDG_CONFIG_HOME");
/// std::env::remove_var("XDG_DATA_HOME");
/// std::env::remove_var("XDG_CACHE_HOME");
///
/// let base_strategy = Xdg::new().unwrap();
///
/// let home_dir = etcetera::home_dir().unwrap();
///
/// assert_eq!(
///     base_strategy.config_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".config/")
/// ));
/// assert_eq!(
///     base_strategy.data_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".local/share/")
/// ));
/// assert_eq!(
///     base_strategy.cache_dir().strip_prefix(&home_dir),
///     Ok(Path::new(".cache/")
/// ));
/// ```
///
/// And here is another example with the environment variables set to demonstrate that the strategy really does read them:
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Xdg;
/// use std::path::Path;
///
/// std::env::set_var("XDG_CONFIG_HOME", "/foo/");
/// std::env::set_var("XDG_DATA_HOME", "/bar/");
/// std::env::set_var("XDG_CACHE_HOME", "/baz/");
///
/// let base_strategy = Xdg::new().unwrap();
///
/// assert_eq!(base_strategy.config_dir(), Path::new("/foo/"));
/// assert_eq!(base_strategy.data_dir(), Path::new("/bar/"));
/// assert_eq!(base_strategy.cache_dir(), Path::new("/baz/"));
/// ```
#[derive(Debug)]
pub struct Xdg {
    home_dir: PathBuf,
}

impl Xdg {
    fn env_var_or_default(&self, env_var: &str, default: impl AsRef<Path>) -> PathBuf {
        std::env::var(env_var).map_or(self.home_dir.join(default), PathBuf::from)
    }
}

impl super::BaseStrategy for Xdg {
    type CreationError = crate::HomeDirError;

    fn new() -> Result<Self, Self::CreationError> {
        Ok(Self {
            home_dir: crate::home_dir()?,
        })
    }

    fn config_dir(&self) -> PathBuf {
        self.env_var_or_default("XDG_CONFIG_HOME", ".config/")
    }

    fn data_dir(&self) -> PathBuf {
        self.env_var_or_default("XDG_DATA_HOME", ".local/share/")
    }

    fn cache_dir(&self) -> PathBuf {
        self.env_var_or_default("XDG_CACHE_HOME", ".cache/")
    }
}
