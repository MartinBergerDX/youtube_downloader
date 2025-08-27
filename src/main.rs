mod utils;
// use utils::extract_youtube_id;
use utils::*;

use clap::Parser;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use yt_dlp::fetcher::deps::LibraryInstaller;
use yt_dlp::{Youtube, fetcher::deps::Libraries};

// Build the release version
// cargo build --release

// ./youtube_downloader --url="https://www.youtube.com/watch?v=bMf3fmAQI8A"

// VS Code format code shortcut: SHIFT + OPTION + F

// echo "alias ytdl='/Users/martinberger/Dev/Rust/youtube_downloader/target/release/youtube_downloader'" >> ~/.zshrc
// source ~/.zshrc

// https://docs.rs/yt-dlp/latest/yt_dlp/

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,
    #[arg(short, long)]
    file: Option<String>,
    #[clap(short, long, default_value = "./downloads")]
    output: String,
    #[clap(short, long, default_value = "best")]
    quality: String,
}

#[derive(Debug)]
enum TestError {
    UrlDownloadError(String),
    YtDlpError(String),
    IOError(String),
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestError::UrlDownloadError(description) => write!(f, "Generic error: {}", description),
            TestError::YtDlpError(description) => {
                write!(f, "YouTube downloader error: {}", description)
            }
            TestError::IOError(description) => write!(f, "IO error: {}", description),
        }
    }
}

impl Error for TestError {}

impl From<yt_dlp::error::Error> for TestError {
    fn from(err: yt_dlp::error::Error) -> Self {
        TestError::YtDlpError(err.to_string())
    }
}

impl From<std::io::Error> for TestError {
    fn from(err: std::io::Error) -> Self {
        TestError::IOError(err.to_string())
    }
}

// async fn download_url(url: &str, output: &str, quality: &str) -> Result<(), yt_dlp::error::Error> {
//     let libraries_dir = PathBuf::from("libs");
//     let output_dir = PathBuf::from(output);
//     let youtube = libraries_dir.join("yt-dlp");
//     let ffmpeg = libraries_dir.join("ffmpeg");

//     println!("Paths: {:?}", libraries_dir.display());
//     println!("Paths: {:?}", output_dir.display());
//     println!("Paths: {:?}", youtube.display());
//     println!("Paths: {:?}", ffmpeg.display());

//     println!("Step: {}", 1);
//     let libraries = Libraries::new(youtube, ffmpeg);
//     println!("Step: {}", 2);

//     let fetcher = Youtube::new(libraries, output_dir)?;
//     println!("Step: {}", 3);
//     fetcher.update_downloader().await?;
//     println!("Step: {}", 4);

//     let video = fetcher.fetch_video_infos(url.to_string()).await?;
//     println!("Step: {}", 5);

//     // let format = video
//     //     .best_video_format()
//     //     .ok_or_else(|| Err("No video format found"))?;
//     let format = video
//         .best_video_format()
//         .ok_or_else(|| yt_dlp::error::Error::FormatNotFound("No video format found".to_string()))?;

//     let filename = format!("%(title)s.%(ext)s");
//     fetcher.download_format(format, &filename).await?;
//     println!("Downloaded: {} to {}/{}", url, output, filename);
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), TestError> {
    let args = Args::parse();

    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    if !youtube.exists() {
        println!("Downloading youtube library.");
        let destination = PathBuf::from("libs");
        let installer = LibraryInstaller::new(destination);
        installer.install_youtube(None).await.unwrap();
    }
    if !ffmpeg.exists() {
        println!("Downloading ffmpeg library.");
        let destination = PathBuf::from("libs");
        let installer = LibraryInstaller::new(destination);
        installer.install_ffmpeg(None).await.unwrap();
    }

    let libraries = Libraries::new(youtube, ffmpeg);

    println!("Updating yt-dlp executable.");
    let fetcher = Youtube::new(libraries, output_dir)?;
    fetcher.update_downloader().await?;
    println!("Done.");

    let url = args
        .url
        .ok_or_else(|| TestError::UrlDownloadError("Error: Url not provided.".to_string()))?;
    println!("YouTube url: {}", url);

    let youtube_id = extract_youtube_id(&url)
        .ok_or_else(|| TestError::UrlDownloadError("Error: YouTube id not found.".to_string()))?;
    println!("YouTube id: {}", youtube_id);

    let output_filename = format!("{}.mp4", youtube_id);
    println!("Output filename: {}", output_filename);

    let output_dir = "output";

    let output_path = Path::new(output_dir).join(&output_filename);
    println!("Output path: {}", output_path.display());

    if output_path.exists() {
        match fs::remove_file(&output_path) {
            Ok(()) => println!("Deleted existing file: {}", output_path.display()),
            Err(e) => println!("Failed to delete existing file: {}", e),
        }
    }

    let video_path = fetcher
        .download_video_from_url(url.clone(), output_filename)
        .await?;
    println!("Successfully downloaded video to: {}", video_path.display());

    cleanup_temp_files(&output_dir, &youtube_id)?;

    Ok(())
}
