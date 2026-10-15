use async_trait::async_trait;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[async_trait]
pub trait Checkpoint: Send + Sync {
    async fn load(&self) -> Result<Option<u64>, CheckpointError>;

    /// Stores the next slot to process.
    async fn save(&self, next_slot: u64) -> Result<(), CheckpointError>;
}

#[derive(Debug)]
pub enum CheckpointError {
    IoError(std::io::Error),
    Other(String),
}

pub struct FileCheckpoint {
    path: PathBuf,
    save_lock: Mutex<()>,
}

impl FileCheckpoint {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            save_lock: Mutex::new(()),
        }
    }

    fn temporary_path(&self) -> PathBuf {
        let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let filename = self.path.file_name().unwrap_or_default().to_string_lossy();

        let temporary_name = format!(".{filename}.{}.{}.tmp", std::process::id(), counter);

        self.path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(temporary_name)
    }
}

#[async_trait]
impl Checkpoint for FileCheckpoint {
    async fn load(&self) -> Result<Option<u64>, CheckpointError> {
        let contents = match fs::read_to_string(&self.path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(error) => return Err(CheckpointError::IoError(error)),
        };

        let next_slot = contents.trim().parse::<u64>().map_err(|error| {
            CheckpointError::Other(format!(
                "invalid checkpoint in {}: {error}",
                self.path.display()
            ))
        })?;

        Ok(Some(next_slot))
    }

    async fn save(&self, next_slot: u64) -> Result<(), CheckpointError> {
        let _guard = self
            .save_lock
            .lock()
            .map_err(|_| CheckpointError::Other("checkpoint mutex poisoned".into()))?;

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(CheckpointError::IoError)?;
        }

        let temporary_path = self.temporary_path();

        let result = (|| {
            let file = fs::File::create(&temporary_path).map_err(CheckpointError::IoError)?;

            use std::io::Write;
            let mut writer = std::io::BufWriter::new(file);

            writeln!(writer, "{next_slot}").map_err(CheckpointError::IoError)?;

            writer.flush().map_err(CheckpointError::IoError)?;

            let file = writer
                .into_inner()
                .map_err(|error| CheckpointError::IoError(error.into_error()))?;

            file.sync_all().map_err(CheckpointError::IoError)?;

            fs::rename(&temporary_path, &self.path).map_err(CheckpointError::IoError)?;

            Ok(())
        })();

        if result.is_err() {
            let _ = fs::remove_file(&temporary_path);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_checkpoint_has_no_saved_slot() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("checkpoint.txt");

        let checkpoint = FileCheckpoint::new(&path);
        assert_eq!(checkpoint.load().await.unwrap(), None);
    }

    #[tokio::test]
    async fn save_stores_next_slot() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("checkpoint.txt");

        let checkpoint = FileCheckpoint::new(&path);

        checkpoint.save(100).await.unwrap();

        assert_eq!(checkpoint.load().await.unwrap(), Some(100));
    }

    #[tokio::test]
    async fn save_overwrites_previous_slot() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("checkpoint.txt");

        let checkpoint = FileCheckpoint::new(&path);

        checkpoint.save(100).await.unwrap();
        checkpoint.save(200).await.unwrap();

        assert_eq!(checkpoint.load().await.unwrap(), Some(200));
    }

    #[tokio::test]
    async fn file_checkpoint_returns_none_when_file_is_missing() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("checkpoint.txt");

        let checkpoint = FileCheckpoint::new(&path);

        assert_eq!(checkpoint.load().await.unwrap(), None);
    }

    #[tokio::test]
    async fn file_checkpoint_persists_next_slot() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("checkpoint.txt");

        {
            let checkpoint = FileCheckpoint::new(&path);
            checkpoint.save(100).await.unwrap();
        }

        // A new instance simulates application restart.
        let checkpoint = FileCheckpoint::new(&path);
        assert_eq!(checkpoint.load().await.unwrap(), Some(100));

        fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn file_checkpoint_overwrites_previous_slot() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("checkpoint.txt");

        let checkpoint = FileCheckpoint::new(&path);

        checkpoint.save(100).await.unwrap();
        checkpoint.save(200).await.unwrap();

        assert_eq!(checkpoint.load().await.unwrap(), Some(200));

        fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn file_checkpoint_rejects_invalid_contents() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("checkpoint.txt");

        fs::write(&path, "not-a-slot").unwrap();

        let checkpoint = FileCheckpoint::new(&path);

        assert!(matches!(
            checkpoint.load().await,
            Err(CheckpointError::Other(_))
        ));

    }
}
