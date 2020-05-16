//! These strategies require you to provide some information on your application, and they will in turn locate the configuration/data/cache directory specifically for your application.

use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

/// The arguments to the creator method of an [`AppStrategy`](trait.AppStrategy.html).
#[derive(Debug)]
pub struct AppStrategyArgs {
    /// The top level domain of the application, e.g. `com` or `org`.
    pub top_level_domain: String,
    /// The name of the author of the application.
    pub author: String,
    /// The application’s name. This should be capitalised if appropriate.
    pub app_name: String,
}

impl AppStrategyArgs {
    /// Constructs a bunde identifier from an `AppStrategyArgs`.
    ///
    /// ```
    /// use etcetera::app_strategy::AppStrategyArgs;
    ///
    /// let strategy_args = AppStrategyArgs {
    ///     top_level_domain: "com".to_string(),
    ///     author: "Apple".to_string(),
    ///     app_name: "Safari".to_string(),
    /// };
    ///
    /// assert_eq!(strategy_args.bundle_id(), "com.apple.Safari".to_string());
    /// ```
    pub fn bundle_id(&self) -> String {
        format!(
            "{}.{}.{}",
            self.top_level_domain,
            self.author.to_lowercase(),
            self.app_name
        )
    }

    /// Returns a ‘unixy’ version of the application’s name, akin to what would usually be used as a binary name.
    ///
    /// ```
    /// use etcetera::app_strategy::AppStrategyArgs;
    ///
    /// let strategy_args = AppStrategyArgs {
    ///     top_level_domain: "org".to_string(),
    ///     author: "Mozilla".to_string(),
    ///     app_name: "Firefox Developer Edition".to_string(),
    /// };
    ///
    /// assert_eq!(strategy_args.unixy_name(), "firefox-developer-edition".to_string());
    /// ```
    pub fn unixy_name(&self) -> String {
        self.app_name.to_lowercase().replace(' ', "-")
    }
}

macro_rules! file_method_body {
    ($self: ident, $file_name: ident, $dir_method_name: ident) => {{
        let mut path = $self.$dir_method_name();
        path.push(Path::new(&$file_name));

        path
    }};
}

/// Allows applications to retrieve the paths of configuration, data and cache directories specifically for them.
pub trait AppStrategy: Sized {
    /// The error type returned by `new`.
    type CreationError;

    /// The constructor requires access to some basic information about your application.
    fn new(args: AppStrategyArgs) -> Result<Self, Self::CreationError>;

    /// Gets the configuration directory for your application.
    fn config_dir(&self) -> PathBuf;

    /// Gets the data directory for your application.
    fn data_dir(&self) -> PathBuf;

    /// Gets the cache directory for your application.
    fn cache_dir(&self) -> PathBuf;

    /// Constructs a path inside your application’s configuration directory to which a file name of your choice has been appended.
    fn config_file<FileName: AsRef<OsStr>>(&self, file_name: FileName) -> PathBuf {
        file_method_body!(self, file_name, config_dir)
    }

    /// Constructs a path inside your application’s data directory to which a file name of your choice has been appended.
    fn data_file<FileName: AsRef<OsStr>>(&self, file_name: FileName) -> PathBuf {
        file_method_body!(self, file_name, data_dir)
    }

    /// Constructs a path inside your application’s cache directory to which a file name of your choice has been appended.
    fn cache_file<FileName: AsRef<OsStr>>(&self, file_name: FileName) -> PathBuf {
        file_method_body!(self, file_name, cache_dir)
    }
}

macro_rules! create_choose_app_strategy {
    ($name: ident, $ty: ty) => {
        /// Returns the current OS’s default [`AppStrategy`](trait.AppStrategy.html). This uses the [`Windows`](struct.Windows.html) strategy on Windows, the [`Apple`](struct.Apple.html) strategy on iOS, and [`Xdg`](struct.Xdg.html) everywhere else.
        pub fn $name(args: AppStrategyArgs) -> Result<$ty, <$ty as AppStrategy>::CreationError> {
            <$ty>::new(args)
        }
    };
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        create_choose_app_strategy!(choose_app_strategy, Windows);
    } else if #[cfg(target_os = "ios")] {
        create_choose_app_strategy!(choose_app_strategy, Apple);
    } else {
        create_choose_app_strategy!(choose_app_strategy, Xdg);
    }
}

mod apple;
mod unix;
mod windows;
mod xdg;

pub use apple::Apple;
pub use unix::Unix;
pub use windows::Windows;
pub use xdg::Xdg;
