-- 确保categories表中有所有必需字段
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT
);

-- 检查是否存在categories表
INSERT OR IGNORE INTO categories (id, name, description, enabled, created_at)
VALUES (1, '默认分类', '系统默认分类', 1, datetime('now')); 