use std::fs;
use std::path::Path;

pub fn cleanup_temp_files(output_dir: &str, youtube_id: &str) -> std::io::Result<()> {
    let dir = Path::new(output_dir);

    if !dir.exists() {
        return Ok(());
    }

    let entries = fs::read_dir(dir)?;
    let mut deleted_count = 0;

    for entry_result in entries {
        let entry = entry_result?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.contains("temp") && file_name_str.contains(youtube_id) {
            match fs::remove_file(entry.path()) {
                Ok(()) => {
                    println!("Deleted temp file: {}", file_name_str);
                    deleted_count += 1;
                }
                Err(e) => {
                    eprintln!("Failed to delete {}: {}", file_name_str, e);
                }
            }
        }
    }

    if deleted_count > 0 {
        println!("Cleaned up {} temporary files", deleted_count);
    }

    Ok(())
}
