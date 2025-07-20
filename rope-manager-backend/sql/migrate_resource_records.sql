-- 检查resource_records表是否存在，如果不存在则创建
CREATE TABLE IF NOT EXISTS resource_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    resource_id INTEGER NOT NULL,
    resource_type TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    action TEXT NOT NULL,
    ip_address TEXT,
    old_data TEXT,
    new_data TEXT,
    timestamp INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT
);

-- 为资源记录表创建索引
CREATE INDEX IF NOT EXISTS idx_resource_records_resource ON resource_records(resource_id, resource_type);
CREATE INDEX IF NOT EXISTS idx_resource_records_user ON resource_records(user_id);
CREATE INDEX IF NOT EXISTS idx_resource_records_action ON resource_records(action);
CREATE INDEX IF NOT EXISTS idx_resource_records_timestamp ON resource_records(timestamp); 