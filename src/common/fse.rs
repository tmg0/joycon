use std::path::Path;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

pub async fn copy(from: String, to: String) -> io::Result<()> {
    let mut source = File::open(from).await?;
    let mut destination = File::create(to).await?;

    let mut buffer = vec![0; 8192]; // 定义缓冲区大小
    loop {
        let n = source.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        destination.write_all(&buffer[..n]).await?;
    }

    Ok(())
}

pub async fn path_exists<T: AsRef<Path>>(path: T) -> bool {
    fs::metadata(path).await.is_ok()
}

pub async fn ensure_file<T: AsRef<Path>>(path: T) {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.unwrap();
    }

    if fs::metadata(path).await.is_err() {
        File::create(path).await.unwrap();
    }
}
