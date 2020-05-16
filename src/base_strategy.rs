//! These strategies simply provide the user’s configuration, data and cache directories, without knowing about the application specifically.

use std::path::PathBuf;

/// Provides configuration, data and cache directories of the current user.
pub trait BaseStrategy: Sized {
    /// The error type returned by `new`.
    type CreationError;

    /// Base strategies are constructed without knowledge of the application.
    fn new() -> Result<Self, Self::CreationError>;

    /// Gets the user’s configuration directory.
    fn config_dir(&self) -> PathBuf;

    /// Gets the user’s data directory.
    fn data_dir(&self) -> PathBuf;

    /// Gets the user’s cache directory.
    fn cache_dir(&self) -> PathBuf;
}

mod apple;
pub use apple::Apple;
