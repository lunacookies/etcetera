[![crates.io version](https://img.shields.io/crates/v/etcetera?style=for-the-badge)](https://crates.io/crates/etcetera)
[![crates.io revdeps](https://img.shields.io/crates/d/etcetera?style=for-the-badge)](https://crates.io/crates/etcetera/reverse_dependencies)
[![documentation](https://img.shields.io/docsrs/etcetera?style=for-the-badge)](https://docs.rs/etcetera)
![license](https://img.shields.io/crates/l/etcetera?style=for-the-badge)

# Etcetera

This is a Rust library that allows you to determine the locations of configuration, data, cache & other files for your application. Existing Rust libraries generally do not give you a choice in terms of which standards/conventions (Etcetera calls these ‘strategies’) they follow. Etcetera, on the other hand, gives you the choice.

Etcetera supports the following strategies:
- the [XDG base directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html)
- the [Known Folder Locations](https://msdn.microsoft.com/en-us/library/windows/desktop/dd378457.aspx)
- the [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html)
- the "Unix Single-folder Strategy" i.e. everything in `~/.myapp`

Etcetera has 2 modes of operation: `BaseStrategy` & `AppStrategy`:
- With `BaseStrategy`, you just get the location of the respective directory. For eg. for `config_dir()`:
  - XDG: `~/.config`
  - Apple: `~/Library/Preferences`
  - Windows: `~\AppData\Roaming`
- With `AppStrategy`, you provide additional information to get the location of your app directory.
  For eg. if you provide the following details: `{ top_level_domain: "org", author: "Acme Corp", app_name: "Frobnicator Plus" }`, you'll get:
  - XDG: `~/.config/frobnicator-plus`
  - Unix: `~/.frobnicator-plus`
  - Apple: `~/Library/Preferences/org.acmecorp.FrobnicatorPlus`
  - Windows: `~\AppData\Roaming\Acme Corp\Frobnicator Plus`

Note: the location of the home (~) is determined by the [`home`](https://docs.rs/home/0.5.4/home/fn.home_dir.html) crate.

Etcetera also provides convenience functions for selecting the appropriate strategy on each platform:
- `base_strategy::choose_base_strategy` & `app_strategy::choose_app_strategy`: Uses `Windows` on Windows & `XDG` everywhere else.
  This is used by most CLI tools & some GUI tools on each platform.
- `base_strategy::choose_native_strategy` & `app_strategy::choose_native_strategy`: Uses `Windows` on Windows, `Apple` on macOS/iOS, & `XDG` everywhere else.
  This is used by most GUI applications on each platform.

See the documentation for examples.
