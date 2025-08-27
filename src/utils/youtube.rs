pub fn extract_youtube_id(url: &str) -> Option<&str> {
    if url.contains("youtu.be/") {
        return url.split("youtu.be/").nth(1)?.split("?").next();
    }

    if url.contains("v=") {
        return url.split("v=").nth(1)?.split('&').next();
    }
    
    None
}