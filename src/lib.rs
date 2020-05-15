//! This is a Rust library that aims to allow you to determine the locations of configuration, cache and data files for your application. Existing Rust libraries generally do not give you a choice in terms of which standards/conventions (Etcetera calls these ‘strategies’) they follow. Etcetera, on the other hand, gives you the choice.

#![warn(missing_docs, rust_2018_idioms, missing_debug_implementations)]

pub mod strategy;

/// A convenience function that wraps the [`home_dir`](https://docs.rs/dirs/2/dirs/fn.home_dir.html) function from the [dirs](https://docs.rs/dirs) crate.
pub fn home_dir() -> Result<std::path::PathBuf, HomeDirError> {
    dirs::home_dir().ok_or(HomeDirError)
}

/// This error occurs when the home directory cannot be located.
#[derive(Debug, thiserror::Error)]
#[error("could not locate home directory")]
pub struct HomeDirError;
