use std::path::Path;
use tokio::fs;
use tokio::fs::read_to_string;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn path_exists<T: AsRef<Path>>(path: T) -> bool {
    fs::metadata(path).await.is_ok()
}

pub async fn read_file<T: AsRef<Path>>(path: T) -> Result<String, ()> {
    let path = path.as_ref();
    if path_exists(path).await {
        return Ok(read_to_string(path).await.unwrap());
    }
    Err(())
}

pub async fn write_file<T: AsRef<Path>>(path: T, content: &str) {
    let mut file = File::create(path).await.unwrap();
    file.write_all(&content.as_bytes()).await.unwrap();
}

pub async fn ensure_dir<T: AsRef<Path>>(path: T) {
    let path = path.as_ref();
    fs::create_dir_all(path).await.unwrap();
}
