# youtube_downloader

A small Rust CLI that downloads YouTube videos using [yt-dlp](https://github.com/yt-dlp/yt-dlp) and ffmpeg via the [`yt-dlp`](https://docs.rs/yt-dlp/latest/yt_dlp/) crate.

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- Network access on first run (to download `yt-dlp` and `ffmpeg` into `libs/` if missing)

The project pins [`yt-dlp` 2.6.0](https://crates.io/crates/yt-dlp/2.6.0) and patches the yanked `lofty` crate via `Cargo.toml` (see `[patch.crates-io]`).

## Build

Release binary (recommended):

```bash
cargo build --release
```

The executable is at `target/release/youtube_downloader`.

## Usage

Download a video by URL:

```bash
./target/release/youtube_downloader --url="https://www.youtube.com/watch?v=VIDEO_ID"
```

During development:

```bash
cargo run -- --url="https://www.youtube.com/watch?v=VIDEO_ID"
```

### Optional shell alias

```bash
echo "alias ytdl='$(pwd)/target/release/youtube_downloader'" >> ~/.zshrc
source ~/.zshrc
ytdl --url="https://www.youtube.com/watch?v=VIDEO_ID"
```

## CLI flags

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--url` | `-u` | — | **Required.** YouTube watch URL |
| `--output` | `-o` | `~/Downloads` | Download directory (created if missing) |
| `--quality` | `-q` | `best` | Defined in CLI; not wired in code yet |
| `--file` | `-f` | — | Defined in CLI; not wired in code yet |

Downloads are saved as `{output}/{sanitized_title}.mp4` (falls back to video ID if the title is empty).

## What happens on run

Uses the [`yt-dlp`](https://docs.rs/yt-dlp/2.6.0/yt_dlp/) **2.6** crate. Subprocess calls (`yt-dlp`, `ffmpeg`) use a **5 minute** timeout (crate default is 30 seconds).

1. Ensures `libs/yt-dlp` and `libs/ffmpeg` exist (installs them if missing).
2. Updates the yt-dlp executable.
3. Fetches metadata and builds a filename from the video title (unsafe characters removed via the crate’s `sanitize_filename`).
4. Downloads the video to `{output}/{title}.mp4` (replaces an existing file with the same name).
5. Cleans up temporary files in the output directory.

## Project layout

- `src/main.rs` — CLI entry point
- `libs/` — `yt-dlp` and `ffmpeg` binaries (created on first run)
- `~/Downloads/` — default location for downloaded videos (override with `-o`)

## Example

```bash
cargo build --release
./target/release/youtube_downloader --url="https://www.youtube.com/watch?v=bMf3fmAQI8A"
```
