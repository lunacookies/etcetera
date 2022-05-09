//! These strategies simply provide the user’s configuration, data and cache directories, without knowing about the application specifically.

use std::path::PathBuf;

/// Provides configuration, data and cache directories of the current user.
pub trait BaseStrategy: Sized {
    /// The error type returned by `new`.
    type CreationError: std::error::Error;

    /// Base strategies are constructed without knowledge of the application.
    fn new() -> Result<Self, Self::CreationError>;

    /// Gets the user’s configuration directory.
    fn config_dir(&self) -> PathBuf;

    /// Gets the user’s data directory.
    fn data_dir(&self) -> PathBuf;

    /// Gets the user’s cache directory.
    fn cache_dir(&self) -> PathBuf;

    /// Gets the user’s state directory.
    /// State directory may not exist for all platforms.
    fn state_dir(&self) -> Option<PathBuf>;
}

macro_rules! create_choose_base_strategy {
    ($name: ident, $ty: ty) => {
        /// Returns the current OS’s default [`BaseStrategy`](trait.BaseStrategy.html). This uses the [`Windows`](struct.Windows.html) strategy on Windows, and [`Xdg`](struct.Xdg.html) everywhere else.
        pub fn $name() -> Result<$ty, <$ty as BaseStrategy>::CreationError> {
            <$ty>::new()
        }
    };
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        create_choose_base_strategy!(choose_base_strategy, Windows);
    } else {
        create_choose_base_strategy!(choose_base_strategy, Xdg);
    }
}

mod apple;
mod windows;
mod xdg;

pub use apple::Apple;
pub use windows::Windows;
pub use xdg::Xdg;
