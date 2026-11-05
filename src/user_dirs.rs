#[cfg(all(unix, not(target_os = "macos")))]
use std::path::Path;
use std::path::PathBuf;

/// Returns the user's downloads directory, or `None` if it cannot be found.
///
/// - Linux and other Unix: `XDG_DOWNLOAD_DIR`, from the environment or from
///   `$XDG_CONFIG_HOME/user-dirs.dirs` (see [xdg-user-dirs](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)),
///   falling back to `~/Downloads`.
/// - macOS: `~/Downloads`.
/// - Windows: the `FOLDERID_Downloads` known folder.
///
/// ```
/// let downloads = etcetera::download_dir();
/// ```
pub fn download_dir() -> Option<PathBuf> {
    imp::download_dir()
}

/// Parses `XDG_DOWNLOAD_DIR` out of the contents of a `user-dirs.dirs` file.
#[cfg(all(unix, not(target_os = "macos")))]
fn parse_user_dirs(contents: &str, home: &Path) -> Option<PathBuf> {
    contents.lines().find_map(|line| {
        let value = line.trim().strip_prefix("XDG_DOWNLOAD_DIR=")?;
        resolve(value.strip_prefix('"')?.strip_suffix('"')?, home)
    })
}

/// Values are either `$HOME/...` or an absolute path.
#[cfg(all(unix, not(target_os = "macos")))]
fn resolve(value: &str, home: &Path) -> Option<PathBuf> {
    if value == "$HOME" {
        return Some(home.to_path_buf());
    }
    if let Some(rest) = value.strip_prefix("$HOME/") {
        let rest = rest.trim_end_matches('/');
        return Some(if rest.is_empty() {
            home.to_path_buf()
        } else {
            home.join(rest)
        });
    }
    let path = PathBuf::from(value);
    path.is_absolute().then_some(path)
}

#[cfg(all(unix, not(target_os = "macos")))]
mod imp {
    use std::path::PathBuf;

    pub(super) fn download_dir() -> Option<PathBuf> {
        let home = crate::home_dir().ok()?;
        if let Some(dir) = std::env::var("XDG_DOWNLOAD_DIR")
            .ok()
            .and_then(|v| super::resolve(&v, &home))
        {
            return Some(dir);
        }
        let config = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .unwrap_or_else(|| home.join(".config"));
        std::fs::read_to_string(config.join("user-dirs.dirs"))
            .ok()
            .and_then(|contents| super::parse_user_dirs(&contents, &home))
            .or_else(|| Some(home.join("Downloads")))
    }
}

#[cfg(target_os = "macos")]
mod imp {
    use std::path::PathBuf;

    pub(super) fn download_dir() -> Option<PathBuf> {
        crate::home_dir().ok().map(|home| home.join("Downloads"))
    }
}

#[cfg(all(windows, not(target_vendor = "uwp")))]
mod imp {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::path::PathBuf;
    use std::{ptr, slice};

    use windows_sys::Win32::Foundation::S_OK;
    use windows_sys::Win32::System::Com::CoTaskMemFree;
    use windows_sys::Win32::UI::Shell::{
        FOLDERID_Downloads, KF_FLAG_DONT_VERIFY, SHGetKnownFolderPath,
    };

    unsafe extern "C" {
        fn wcslen(buf: *const u16) -> usize;
    }

    pub(super) fn download_dir() -> Option<PathBuf> {
        unsafe {
            let mut path = ptr::null_mut();
            let result = SHGetKnownFolderPath(
                &FOLDERID_Downloads,
                KF_FLAG_DONT_VERIFY as u32,
                ptr::null_mut(),
                &mut path,
            );
            let dir = (result == S_OK).then(|| {
                PathBuf::from(OsString::from_wide(slice::from_raw_parts(
                    path,
                    wcslen(path),
                )))
            });
            // A null ptr is a no-op for `CoTaskMemFree`.
            CoTaskMemFree(path.cast());
            dir
        }
    }
}

#[cfg(not(any(unix, all(windows, not(target_vendor = "uwp")))))]
mod imp {
    use std::path::PathBuf;

    pub(super) fn download_dir() -> Option<PathBuf> {
        None
    }
}

#[cfg(all(test, unix, not(target_os = "macos")))]
mod tests {
    use super::*;

    #[test]
    fn parses_user_dirs() {
        let home = Path::new("/home/user");
        let contents = "# comment\nXDG_DESKTOP_DIR=\"$HOME/Desktop\"\nXDG_DOWNLOAD_DIR=\"$HOME/Téléchargements\"\n";
        assert_eq!(
            parse_user_dirs(contents, home),
            Some(home.join("Téléchargements"))
        );
        assert_eq!(
            parse_user_dirs("XDG_DOWNLOAD_DIR=\"/data/dl\"", home),
            Some(PathBuf::from("/data/dl"))
        );
        assert_eq!(
            parse_user_dirs("XDG_DOWNLOAD_DIR=\"$HOME/\"", home),
            Some(home.to_path_buf())
        );
        assert_eq!(parse_user_dirs("XDG_DOWNLOAD_DIR=\"relative\"", home), None);
        assert_eq!(
            parse_user_dirs("#XDG_DOWNLOAD_DIR=\"$HOME/Old\"", home),
            None
        );
    }
}
