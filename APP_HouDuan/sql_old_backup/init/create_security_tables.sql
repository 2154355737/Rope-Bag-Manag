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

-- 创建IP黑名单表
CREATE TABLE IF NOT EXISTS ip_blacklist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip_address TEXT UNIQUE NOT NULL,
    reason TEXT NOT NULL,
    auto_added BOOLEAN DEFAULT 0,
    expires_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 创建用户黑名单表  
CREATE TABLE IF NOT EXISTS user_blacklist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER UNIQUE NOT NULL,
    reason TEXT NOT NULL,
    auto_added BOOLEAN DEFAULT 0,
    expires_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 创建安全规则配置表
CREATE TABLE IF NOT EXISTS security_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT UNIQUE NOT NULL,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 插入默认安全配置
INSERT OR IGNORE INTO security_config (key, value, description) VALUES
('max_download_ratio', '0.25', '最大下载率（下载量/访问量）'),
('max_downloads_per_hour', '20', '每小时最大下载次数'),
('max_downloads_per_day', '100', '每天最大下载次数'),
('max_views_per_hour', '100', '每小时最大访问次数'),
('max_views_per_day', '1000', '每天最大访问次数'),
('auto_ban_risk_score', '90', '自动封禁的风险评分阈值'),
('auto_ban_duration_minutes', '60', '自动封禁时长（分钟）'),
('enable_anti_fraud', '1', '是否启用防刷系统');

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_security_logs_ip ON security_logs(ip_address);
CREATE INDEX IF NOT EXISTS idx_security_logs_user ON security_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_security_logs_created ON security_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_security_logs_action ON security_logs(action_type);
CREATE INDEX IF NOT EXISTS idx_ip_blacklist_ip ON ip_blacklist(ip_address);
CREATE INDEX IF NOT EXISTS idx_user_blacklist_user ON user_blacklist(user_id);