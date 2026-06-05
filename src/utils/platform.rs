use std::env;
use std::path::{Path, PathBuf};

/// User home directory (`HOME` on Unix, `USERPROFILE` on Windows).
pub fn user_home_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        env::var_os("USERPROFILE").map(PathBuf::from)
    } else {
        env::var_os("HOME").map(PathBuf::from)
    }
}

/// Default download folder: `~/Downloads` or `%USERPROFILE%\Downloads`.
pub fn default_output_dir() -> String {
    user_home_dir()
        .map(|home| home.join("Downloads"))
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Downloads".into())
}

/// Locate an executable on `PATH` (cross-platform; no `which` / `where` required).
pub fn find_on_path(name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    let candidates: Vec<String> = if cfg!(windows) {
        vec![
            format!("{name}.exe"),
            format!("{name}.cmd"),
            name.to_string(),
        ]
    } else {
        vec![name.to_string()]
    };

    for dir in env::split_paths(&path_var) {
        for file_name in &candidates {
            let path = dir.join(file_name);
            if path.is_file() {
                return Some(path);
            }
        }
    }
    None
}

/// Path to a bundled yt-dlp or ffmpeg binary under `libs/`.
pub fn bundled_executable(libraries_dir: &Path, base_name: &str) -> PathBuf {
    let file_name = yt_dlp::utils::find_executable(base_name);
    libraries_dir.join(file_name)
}
