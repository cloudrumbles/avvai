use std::path::{Path, PathBuf};
use tokio::fs;

const MEDIA_DIR: &str = "static/media";

#[derive(Debug)]
pub enum MediaError {
    NotFound,
    Io(String),
}

impl MediaError {
    pub fn message(&self) -> String {
        match self {
            Self::NotFound => "File not found".to_string(),
            Self::Io(err) => err.clone(),
        }
    }
}

pub fn media_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(MEDIA_DIR)
}

pub fn sanitize_filename(filename: &str) -> String {
    filename.replace(['/', '\\'], "_")
}

pub async fn list_media() -> Vec<String> {
    let dir = media_root();
    let mut files = Vec::new();

    if let Ok(mut entries) = fs::read_dir(dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if let Ok(file_type) = entry.file_type().await
                && file_type.is_file()
                && let Some(name) = path.file_name()
            {
                files.push(name.to_string_lossy().to_string());
            }
        }
    }

    files.sort();
    files
}

pub async fn create_media_file(filename: &str) -> Result<(PathBuf, tokio::fs::File), MediaError> {
    let root = media_root();
    fs::create_dir_all(&root)
        .await
        .map_err(|err| MediaError::Io(err.to_string()))?;

    let sanitized = sanitize_filename(filename);
    let path = root.join(sanitized);
    let file = tokio::fs::File::create(&path)
        .await
        .map_err(|_| MediaError::Io("Failed to create file".to_string()))?;

    Ok((path, file))
}

pub async fn delete_media(filename: &str) -> Result<(), MediaError> {
    let sanitized = sanitize_filename(filename);
    let path = media_root().join(sanitized);
    if !fs::try_exists(&path).await.unwrap_or(false) {
        return Err(MediaError::NotFound);
    }

    fs::remove_file(&path)
        .await
        .map_err(|err| MediaError::Io(err.to_string()))?;
    Ok(())
}
