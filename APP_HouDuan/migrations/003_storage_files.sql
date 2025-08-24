-- ================================
-- 003 - 文件元数据与去重索引
-- ================================

CREATE TABLE IF NOT EXISTS storage_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    sha256 VARCHAR(64) NOT NULL UNIQUE,
    size INTEGER NOT NULL,
    mime VARCHAR(100),
    original_name VARCHAR(255),
    ext VARCHAR(20),
    relative_path VARCHAR(500) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_storage_files_user_id ON storage_files(user_id);
CREATE INDEX IF NOT EXISTS idx_storage_files_created_at ON storage_files(created_at); 