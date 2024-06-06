use std::path::Path;
use tokio::fs;
use tokio::fs::read_to_string;

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
