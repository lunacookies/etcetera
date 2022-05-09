//! This is a Rust library that aims to allow you to determine the locations of configuration, cache and data files for your application. Existing Rust libraries generally do not give you a choice in terms of which standards/conventions (Etcetera calls these ‘strategies’) they follow. Etcetera, on the other hand, gives you the choice.
//!
//! If you want to get started quickly, you can use the provided convenience functions that use the default strategies (as determined arbitrarily by yours truly) for each OS.
//!
//! This first example is for when you just want the path to a configuration, data or cache directory (this is called a ‘base strategy’):
//!
//! ```
//! use etcetera::base_strategy;
//! use etcetera::base_strategy::BaseStrategy;
//!
//! let strategy = base_strategy::choose_base_strategy().unwrap();
//!
//! let config_dir = strategy.config_dir();
//! let data_dir = strategy.data_dir();
//! let cache_dir = strategy.cache_dir();
//! let state_dir = strategy.state_dir();
//! ```
//!
//! Here is an example where you provide Etcetera with some basic information about your application, and Etcetera will in turn give you a path that includes this information (this is called an ‘app strategy’).
//!
//! Let’s take an application created by Acme Corp with the name Frobnicator and the top-level domain of `.org` as an example. With Apple’s strategy, configuration files would be located in `~/Library/Preferences/org.acmecorp.Frobnicator/`. XDG would place these in `~/.config/frobnicator/`, on the other hand. Etcetera takes care of the distinctions.
//!
//! ```
//! use etcetera::app_strategy;
//! use etcetera::app_strategy::AppStrategy;
//! use etcetera::app_strategy::AppStrategyArgs;
//!
//! let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
//!     top_level_domain: "org".to_string(),
//!     author: "Acme Corp".to_string(),
//!     app_name: "Frobnicator".to_string(),
//! }).unwrap();
//!
//! let config_dir = strategy.config_dir();
//! let data_dir = strategy.data_dir();
//! let cache_dir = strategy.cache_dir();
//! let state_dir = strategy.state_dir();
//! ```
//!
//! Say you were a hardened Unix veteran, and didn’t want to have any of this XDG nonsense, clutter in the home directory be damned! Instead of using `choose_app_strategy` or `choose_base_strategy`, you can pick a strategy yourself. Here’s an example using the [`Unix`](app_strategy/struct.Unix.html) strategy – see its documentation to see what kind of folder structures it produces:
//!
//! ```
//! use etcetera::app_strategy;
//! use etcetera::app_strategy::AppStrategy;
//! use etcetera::app_strategy::AppStrategyArgs;
//!
//! let strategy = app_strategy::Unix::new(AppStrategyArgs {
//!     top_level_domain: "com".to_string(),
//!     author: "Hardened Unix Veteran Who Likes Short Command Names".to_string(),
//!     app_name: "wry".to_string(),
//! }).unwrap();
//!
//! let config_dir = strategy.config_dir(); // produces ~/.wry/
//! // et cetera.
//! ```
//!
//! Oftentimes the location of a configuration, data or cache directory is needed solely to create a path that starts inside it. For this purpose, [`AppStrategy`](app_strategy/trait.AppStrategy.html) implements a couple of convenience methods for you:
//!
//! ```
//! use etcetera::app_strategy;
//! use etcetera::app_strategy::AppStrategy;
//! use etcetera::app_strategy::AppStrategyArgs;
//!
//! let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
//!     top_level_domain: "org".to_string(),
//!     author: "Acme Corp".to_string(),
//!     app_name: "Frobnicator".to_string(),
//! }).unwrap();
//!
//! // Path to configuration directory.
//! let config_dir = strategy.config_dir();
//!
//! // Path to config.toml inside the configuration directory.
//! let config_file = strategy.in_config_dir("config.toml");
//!
//! assert_eq!(config_dir.join("config.toml"), config_file);
//! ```
//!
//! You aren’t limited to the built-in strategies, however – you can implement the relevant traits yourself. Please consider contributing these back, as the more preset strategies there are, the better.

#![warn(missing_docs, rust_2018_idioms, missing_debug_implementations)]

pub mod app_strategy;
pub mod base_strategy;

/// A convenience function that wraps the [`home_dir`](https://docs.rs/dirs-next/1/dirs_next/fn.home_dir.html) function from the [dirs-next](https://docs.rs/dirs-next) crate.
pub fn home_dir() -> Result<std::path::PathBuf, HomeDirError> {
    dirs_next::home_dir().ok_or(HomeDirError)
}

/// This error occurs when the home directory cannot be located.
#[derive(Debug, thiserror::Error)]
#[error("could not locate home directory")]
pub struct HomeDirError;
