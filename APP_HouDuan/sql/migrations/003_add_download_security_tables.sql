-- 迁移003: 下载安全与访问统计相关表
-- 说明: 创建 download_records、download_rate_limits、download_anomalies、resource_access_stats、ip_whitelist 表

-- 下载记录表
CREATE TABLE IF NOT EXISTS download_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    package_id INTEGER NOT NULL,
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    download_time TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);

-- 下载频率限制规则表
CREATE TABLE IF NOT EXISTS download_rate_limits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rule_type TEXT NOT NULL, -- 'user' | 'ip' | 'resource' | 'global'
    target_id INTEGER,
    time_window INTEGER NOT NULL, -- 秒
    max_downloads INTEGER NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_download_rate_limits_active ON download_rate_limits(is_active);

-- 下载异常表
CREATE TABLE IF NOT EXISTS download_anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    anomaly_type TEXT NOT NULL,
    user_id INTEGER,
    package_id INTEGER,
    ip_address TEXT,
    details TEXT,
    severity TEXT NOT NULL,
    is_resolved INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    resolved_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_download_anomalies_time ON download_anomalies(created_at);
CREATE INDEX IF NOT EXISTS idx_download_anomalies_ip ON download_anomalies(ip_address);

-- 资源访问统计（日维度）
CREATE TABLE IF NOT EXISTS resource_access_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    package_id INTEGER NOT NULL,
    date TEXT NOT NULL,
    view_count INTEGER NOT NULL DEFAULT 0,
    download_count INTEGER NOT NULL DEFAULT 0,
    unique_visitors INTEGER NOT NULL DEFAULT 0,
    unique_downloaders INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(package_id, date),
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_resource_access_stats_pkg_date ON resource_access_stats(package_id, date);

-- IP 白名单
CREATE TABLE IF NOT EXISTS ip_whitelist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip_address TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by TEXT
);

-- 记录迁移执行
INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('migration_003_applied', datetime('now'), '下载安全与访问统计表已创建'); 