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
/// std::env::remove_var("XDG_STATE_HOME");
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
/// assert_eq!(
///     base_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".local/state")
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
/// // We need to conditionally set these to ensure that they are absolute paths both on Windows and other systems.
/// let config_path = if cfg!(windows) {
///     "C:\\foo\\"
/// } else {
///     "/foo/"
/// };
/// let data_path = if cfg!(windows) {
///     "C:\\bar\\"
/// } else {
///     "/bar/"
/// };
/// let cache_path = if cfg!(windows) {
///     "C:\\baz\\"
/// } else {
///     "/baz/"
/// };
/// let state_path = if cfg!(windows) {
///     "C:\\foobar\\"
/// } else {
///     "/foobar/"
/// };
///
/// std::env::set_var("XDG_CONFIG_HOME", config_path);
/// std::env::set_var("XDG_DATA_HOME", data_path);
/// std::env::set_var("XDG_CACHE_HOME", cache_path);
/// std::env::set_var("XDG_STATE_HOME", state_path);
///
/// let base_strategy = Xdg::new().unwrap();
///
/// assert_eq!(base_strategy.config_dir(), Path::new(config_path));
/// assert_eq!(base_strategy.data_dir(), Path::new(data_path));
/// assert_eq!(base_strategy.cache_dir(), Path::new(cache_path));
/// assert_eq!(base_strategy.state_dir().unwrap(), Path::new(state_path));
/// ```
///
/// The specification states that:
///
/// > All paths set in these environment variables must be absolute. If an implementation encounters a relative path in any of these variables it should consider the path invalid and ignore it.
///
/// In this example the environment variables have been given relative values. The strategy behaves correctly and uses the defaults:
///
/// ```
/// use etcetera::base_strategy::BaseStrategy;
/// use etcetera::base_strategy::Xdg;
/// use std::path::Path;
///
/// std::env::set_var("XDG_CONFIG_HOME", "foo/");
/// std::env::set_var("XDG_DATA_HOME", "bar/");
/// std::env::set_var("XDG_CACHE_HOME", "baz/");
/// std::env::set_var("XDG_STATE_HOME", "foobar/");
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
/// assert_eq!(
///     base_strategy.state_dir().unwrap().strip_prefix(&home_dir),
///     Ok(Path::new(".local/state/")
/// ));
/// ```
#[derive(Debug)]
pub struct Xdg {
    home_dir: PathBuf,
}

impl Xdg {
    fn env_var_or_default(&self, env_var: &str, default: impl AsRef<Path>) -> PathBuf {
        std::env::var(env_var)
            .ok()
            .and_then(|path| {
                let path = PathBuf::from(path);

                // Return None if the path obtained from the environment variable isn’t absolute.
                if path.is_absolute() {
                    Some(path)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| self.home_dir.join(default))
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

    fn state_dir(&self) -> Option<PathBuf> {
        Some(self.env_var_or_default("XDG_STATE_HOME", ".local/state/"))
    }
}
