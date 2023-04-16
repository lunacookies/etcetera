//! This is a Rust library that allows you to determine the locations of configuration, data, cache & other files for your application. Existing Rust libraries generally do not give you a choice in terms of which standards/conventions (Etcetera calls these ‘strategies’) they follow. Etcetera, on the other hand, gives you the choice.
//!
//! # Strategies
//! If you want to get started quickly, you can use the provided convenience functions that use the default strategies (as determined arbitrarily by yours truly) or the native strategies for each OS.
//!
//! ## BaseStrategy
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
//! let runtime_dir = strategy.runtime_dir();
//! ```
//!
//! ## AppStrategy
//! Here is an example where you provide Etcetera with some basic information about your application, and Etcetera will in turn give you a path that includes this information (this is called an ‘app strategy’).
//!
//! Let’s take an application created by Acme Corp with the name Frobnicator Plus and the top-level domain of `.org` as an example.
//! - XDG strategy would place these in `~/.config/frobnicator-plus`.
//! - Unix strategy would place these in `~/.frobnicator-plus`.
//! - Apple strategy would place these in `~/Library/Preferences/org.acmecorp.FrobnicatorPlus`.
//! - Windows strategy would place these in `~\AppData\Roaming\Acme Corp\Frobnicator Plus`.
//!
//! Etcetera takes care of the distinctions.
//!
//! ```
//! use etcetera::app_strategy;
//! use etcetera::app_strategy::AppStrategy;
//! use etcetera::app_strategy::AppStrategyArgs;
//!
//! let strategy = app_strategy::choose_app_strategy(AppStrategyArgs {
//!     top_level_domain: "org".to_string(),
//!     author: "Acme Corp".to_string(),
//!     app_name: "Frobnicator Plus".to_string(),
//! }).unwrap();
//!
//! let config_dir = strategy.config_dir();
//! let data_dir = strategy.data_dir();
//! let cache_dir = strategy.cache_dir();
//! let state_dir = strategy.state_dir();
//! let runtime_dir = strategy.runtime_dir();
//! ```
//!
//! ## Native Strategy
//!
//! `choose_base_strategy()` and `choose_app_strategy()` will use the `XDG` strategy on Linux & macOS, and the `Windows` strategy on Windows.
//! This is used by most CLI tools & some GUI tools on each platform.
//!
//! If you're developing a GUI application, you might want to use the "Standard directories" on macOS by using `choose_native_strategy()` instead.
//! Note that if your application expects the user to modify the configuration files, you still should prefer the `XDG` strategy on macOS.
//!
//! ## Custom Strategies
//!
//! You aren’t limited to the built-in strategies, however – you can implement the relevant traits yourself. Please consider contributing these back, as the more preset strategies there are, the better.
//!
//! # More Examples
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

#![warn(missing_docs, rust_2018_idioms, missing_debug_implementations)]

pub mod app_strategy;
pub mod base_strategy;

/// A convenience function that wraps the [`home_dir`](https://docs.rs/home/0.5.4/home/fn.home_dir.html) function from the [home](https://docs.rs/home) crate.
pub fn home_dir() -> Result<std::path::PathBuf, HomeDirError> {
    home::home_dir().ok_or(HomeDirError)
}

/// This error occurs when the home directory cannot be located.
#[derive(Debug)]
pub struct HomeDirError;

impl std::fmt::Display for HomeDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not locate home directory")
    }
}

impl std::error::Error for HomeDirError {}
