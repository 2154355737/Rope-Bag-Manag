use anyhow::Result;
use std::path::Path;
use uuid::Uuid;

pub struct FileUtils {
    upload_path: String,
}

impl FileUtils {
    pub fn new(upload_path: String) -> Self {
        Self { upload_path }
    }

    pub fn save_file(&self, file_data: &[u8], filename: &str) -> Result<String> {
        // 确保上传目录存在
        std::fs::create_dir_all(&self.upload_path)?;

        // 生成唯一文件名
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);
        let file_path = Path::new(&self.upload_path).join(&unique_filename);

        // 保存文件
        std::fs::write(&file_path, file_data)?;

        Ok(unique_filename)
    }

    pub fn get_file_path(&self, filename: &str) -> String {
        Path::new(&self.upload_path).join(filename).to_string_lossy().to_string()
    }

    pub fn delete_file(&self, filename: &str) -> Result<()> {
        let file_path = Path::new(&self.upload_path).join(filename);
        if file_path.exists() {
            std::fs::remove_file(file_path)?;
        }
        Ok(())
    }
} 