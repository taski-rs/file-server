fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .unwrap();
    taski_file_server::serve(&path, None)
}
