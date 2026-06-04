mod utils;
use utils::*;

use clap::Parser;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use yt_dlp::client::deps::{Libraries, LibraryInstaller};
use yt_dlp::model::Video;
use yt_dlp::utils::validation::sanitize_filename;
use yt_dlp::Downloader;

fn default_output_dir() -> String {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join("Downloads"))
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Downloads".into())
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    url: Option<String>,
    #[arg(short, long)]
    file: Option<String>,
    #[arg(short, long, default_value_t = default_output_dir())]
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

fn output_filename_for_video(video: &Video) -> String {
    let stem = sanitize_filename(&video.title);
    let stem = if stem.is_empty() || stem == "download" {
        video.id.as_str()
    } else {
        stem.as_str()
    };
    format!("{stem}.mp4")
}

#[tokio::main]
async fn main() -> Result<(), TestError> {
    let args = Args::parse();

    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from(&args.output);
    fs::create_dir_all(&output_dir)?;

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    if !youtube.exists() {
        println!("Downloading youtube library.");
        let installer = LibraryInstaller::new(libraries_dir.clone());
        installer.install_youtube(None).await?;
    }
    if !ffmpeg.exists() {
        println!("Downloading ffmpeg library.");
        let installer = LibraryInstaller::new(libraries_dir.clone());
        installer.install_ffmpeg(None).await?;
    }

    let libraries = Libraries::new(youtube, ffmpeg);

    println!("Updating yt-dlp executable.");
    let downloader = Downloader::builder(libraries, output_dir.clone())
        .with_timeout(Duration::from_secs(300))
        .build()
        .await?;
    downloader.update_downloader().await?;
    println!("Done.");

    let url = args
        .url
        .ok_or_else(|| TestError::UrlDownloadError("Error: Url not provided.".to_string()))?;
    println!("YouTube url: {}", url);

    let youtube_id = extract_youtube_id(&url)
        .ok_or_else(|| TestError::UrlDownloadError("Error: YouTube id not found.".to_string()))?;
    println!("YouTube id: {}", youtube_id);

    println!("Fetching video metadata...");
    let video = downloader.fetch_video_infos(&url).await?;
    println!("Title: {}", video.title);

    let output_filename = output_filename_for_video(&video);
    println!("Output filename: {}", output_filename);

    let output_path = output_dir.join(&output_filename);
    println!("Output path: {}", output_path.display());

    if output_path.exists() {
        match fs::remove_file(&output_path) {
            Ok(()) => println!("Deleted existing file: {}", output_path.display()),
            Err(e) => println!("Failed to delete existing file: {}", e),
        }
    }

    println!("Downloading video...");
    let video_path = downloader
        .download_video(&video, &output_filename)
        .await?;
    println!(
        "Successfully downloaded video to: {}",
        video_path.display()
    );

    cleanup_temp_files(
        output_dir
            .to_str()
            .ok_or_else(|| TestError::UrlDownloadError("Invalid output directory".to_string()))?,
        &youtube_id,
    )?;

    Ok(())
}
