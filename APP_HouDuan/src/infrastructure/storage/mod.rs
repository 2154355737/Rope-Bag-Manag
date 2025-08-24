use std::path::{Path, PathBuf};
use std::fs;
use chrono::Local;
use sha2::{Sha256, Digest};

// 统一存储抽象
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    async fn write(&self, path: &str, content: Vec<u8>) -> anyhow::Result<()>;
    async fn read(&self, path: &str) -> anyhow::Result<Vec<u8>>;
}

// OpenDAL 适配
pub struct OpenDalStorage {
    op: opendal::Operator,
}

impl OpenDalStorage {
    pub fn new_fs(root: &str) -> anyhow::Result<Self> {
        let builder = opendal::services::Fs::default().root(root);
        let op = opendal::Operator::new(builder)?.finish();
        Ok(Self { op })
    }
}

#[async_trait::async_trait]
impl Storage for OpenDalStorage {
    async fn write(&self, path: &str, content: Vec<u8>) -> anyhow::Result<()> {
        if let Some((parent, _)) = path.rsplit_once('/') {
            if !parent.is_empty() {
                let _ = self.op.create_dir(parent).await;
            }
        }
        self.op.write(path, content).await?;
        Ok(())
    }

    async fn read(&self, path: &str) -> anyhow::Result<Vec<u8>> {
        let bs = self.op.read(path).await?;
        Ok(bs.to_vec())
    }
}

// 过去的本地存储工具中非IO的辅助函数，保留为公用工具
pub struct LocalStorage;

impl LocalStorage {
    pub fn ensure_base_dir(base: &Path) -> anyhow::Result<()> {
        if !base.exists() { fs::create_dir_all(base)?; }
        Ok(())
    }

    pub fn sanitize_filename(name: &str) -> String {
        let mut s = name.replace(['\\', '/', ':', '*', '?', '"', '<', '>', '|'], "_");
        s.truncate(200);
        if s.is_empty() { s = "file".into(); }
        s
    }

    pub fn partitioned_dir(base: &Path) -> PathBuf {
        let now = Local::now();
        let year = now.format("%Y").to_string();
        let month = now.format("%m").to_string();
        let day = now.format("%d").to_string();
        base.join(year).join(month).join(day)
    }

    pub fn hash_path_component(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let out = hasher.finalize();
        hex::encode(&out)[0..8].to_string()
    }

    pub fn sha256_hex(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hex::encode(hasher.finalize())
    }
} 