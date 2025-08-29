-- 迁移002: 添加安全检查日志表
-- 为现有数据库添加反欺诈系统所需的表结构

-- 创建安全检查日志表
CREATE TABLE IF NOT EXISTS security_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    ip_address TEXT NOT NULL,
    action_type TEXT NOT NULL,
    resource_id INTEGER,
    is_allowed BOOLEAN NOT NULL,
    risk_score INTEGER NOT NULL,
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- 创建安全日志索引
CREATE INDEX IF NOT EXISTS idx_security_logs_ip ON security_logs(ip_address);
CREATE INDEX IF NOT EXISTS idx_security_logs_user ON security_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_security_logs_created ON security_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_security_logs_action ON security_logs(action_type);

-- 更新数据库版本
INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('migration_002_applied', datetime('now'), '安全日志表迁移应用时间'); 