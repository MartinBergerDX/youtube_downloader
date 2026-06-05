# youtube_downloader

A small Rust CLI that downloads YouTube videos using [yt-dlp](https://github.com/yt-dlp/yt-dlp) and ffmpeg via the [`yt-dlp`](https://docs.rs/yt-dlp/2.6.0/yt_dlp/) crate.

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- Network access on first run (to download `yt-dlp` and `ffmpeg` into `libs/` if missing)
- **Windows:** [Node.js 22+](https://nodejs.org/) or [Deno 2.3+](https://deno.com/) on `PATH` for YouTube downloads (same as macOS/Linux)

The project pins [`yt-dlp` 2.6.0](https://crates.io/crates/yt-dlp/2.6.0) and patches the yanked `lofty` crate via `Cargo.toml` (see `[patch.crates-io]`).

## Build

Release binary (recommended):

```bash
cargo build --release
```

The executable is at `target/release/youtube_downloader`.

## Usage

Download a single video:

```bash
./target/release/youtube_downloader --url="https://www.youtube.com/watch?v=VIDEO_ID"
```

Download a playlist sequentially (60 seconds between videos by default):

```bash
./target/release/youtube_downloader --list="https://www.youtube.com/playlist?list=PLAYLIST_ID"
```

Watch URLs with `list=` work as well:

```bash
./target/release/youtube_downloader --list="https://www.youtube.com/watch?v=sufUlXZWW6k&list=PLwZdw9DClmZ_Y9bVrondekzpINEkLvTM9"
```

Custom wait between playlist items (seconds):

```bash
./target/release/youtube_downloader --list="https://www.youtube.com/playlist?list=PLAYLIST_ID" --wait-between-playlist-downloads=120
```

Start at a specific playlist index (1-based, matches YouTube’s `index=` in the URL). Uses each entry’s `playlist_index` when present; otherwise falls back to position in the list (needed because flat playlist metadata often omits `playlist_index`):

```bash
./target/release/youtube_downloader --list="https://www.youtube.com/watch?v=...&list=PL...&index=9" --playlist-start=9
```

Age-restricted or sign-in-required videos need browser cookies (log into YouTube in that browser first):

```bash
ytdl --cookies-from-browser=chrome --url="https://www.youtube.com/watch?v=VIDEO_ID"
# or: safari, firefox, brave, edge, ...
```

Optional: `--cookies /path/to/cookies.txt` (Netscape format).

**YouTube JS runtime:** Newer yt-dlp needs Deno or Node for YouTube ([EJS wiki](https://github.com/yt-dlp/yt-dlp/wiki/EJS)). The tool auto-detects `deno` or `node` on `PATH` (including `node.exe` / `deno.exe` on Windows) and writes `libs/yt-dlp.conf` beside the yt-dlp binary. Override with `--js-runtimes node` (Node 22+) or `--js-runtimes deno` (Deno 2.3+).

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

Provide **either** `--url` or `--list` (not both).

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--url` | `-u` | — | Single YouTube watch URL |
| `--list` | `-l` | — | Playlist URL (`playlist?list=` or watch URL with `list=`) |
| `--output` | `-o` | `~/Downloads` (macOS/Linux) or `%USERPROFILE%\Downloads` (Windows) | Download directory (created if missing) |
| `--wait-between-playlist-downloads` | — | `60` | Seconds to wait between playlist downloads |
| `--playlist-start` | — | — | 1-based index to start from (only with `--list`) |
| `--file` | — | — | Output filename override for `--url` only (sanitized; `.mp4` added if omitted) |
| `--cookies-from-browser` | — | — | Browser for YouTube auth cookies (`chrome`, `safari`, `firefox`, …) |
| `--cookies` | — | — | Path to a cookies file (alternative to browser cookies) |
| `--js-runtimes` | — | — | JS runtime for YouTube extractor (`node`, `deno`, …) |
| `--quality` | `-q` | `best` | Defined in CLI; not wired in code yet |

Downloads are saved as `{output}/{sanitized_title}.mp4` (falls back to video ID if the title is empty). Use `--file` to override the name on single-video downloads.

## What happens on run

Uses the [`yt-dlp`](https://docs.rs/yt-dlp/2.6.0/yt_dlp/) **2.6** crate. Subprocess calls (`yt-dlp`, `ffmpeg`) use a **5 minute** timeout (crate default is 30 seconds).

1. Ensures `libs/yt-dlp` and `libs/ffmpeg` exist (`yt-dlp.exe` / `ffmpeg.exe` on Windows; installs if missing).
2. Updates the yt-dlp executable.
3. **Single URL:** fetches metadata, builds filename from title, downloads one file.
4. **Playlist:** fetches playlist entries, downloads each available video one at a time, waits `--wait` seconds between items (not after the last).
5. Cleans up temporary files in the output directory after each video.

## Project layout

- `src/main.rs` — CLI entry point
- `libs/` — `yt-dlp`, `ffmpeg`, and auto-generated `yt-dlp.conf` (created on first run)
- `~/Downloads/` or `%USERPROFILE%\Downloads` — default output (override with `-o`)

## Windows

Build and run from PowerShell or cmd (run from the project directory so `libs/` is created next to the working directory, or pass an absolute `-o`):

```powershell
cargo build --release
.\target\release\youtube_downloader.exe --url="https://www.youtube.com/watch?v=VIDEO_ID"
```

Browser cookies: `--cookies-from-browser=chrome`, `edge`, `brave`, `firefox`, etc. (browser must be installed and you must be logged into YouTube).

## Example

```bash
cargo build --release
./target/release/youtube_downloader --url="https://www.youtube.com/watch?v=bMf3fmAQI8A"
```
