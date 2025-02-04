pub fn extract_youtube_id(url: &str) -> Option<String> {
    // Use regex to match YouTube video ID
    let re = regex::Regex::new(r"(?:v=|\/)([0-9A-Za-z_-]{11})").unwrap();

    if let Some(captures) = re.captures(url) {
        captures.get(1).map(|m| m.as_str().to_string())
    } else {
        None
    }
}
