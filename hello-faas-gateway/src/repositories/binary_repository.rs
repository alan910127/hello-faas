use std::{fs::Permissions, os::unix::prelude::PermissionsExt, path::PathBuf};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct BinaryRepository;

impl BinaryRepository {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn create(&self, function_id: &str, binary: &[u8]) -> Result<PathBuf> {
        let path = PathBuf::new().join("functions");
        tokio::fs::create_dir_all(&path).await?;
        let path = path.join(function_id);
        let mut file = File::create(&path).await?;
        file.write_all(binary).await?;
        file.set_permissions(Permissions::from_mode(0o777)).await?;
        path.canonicalize().map_err(Into::into)
    }

    pub async fn find(&self, function_id: &str) -> Result<Vec<u8>> {
        let mut file = File::open(format!("functions/{}", function_id)).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        Ok(buffer)
    }
}
