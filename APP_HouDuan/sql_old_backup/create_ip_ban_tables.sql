-- IP封禁表
CREATE TABLE IF NOT EXISTS ip_bans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip_address TEXT NOT NULL,
    reason TEXT NOT NULL,
    ban_type TEXT NOT NULL DEFAULT 'temporary', -- temporary, permanent, download_only
    severity TEXT NOT NULL DEFAULT 'medium', -- low, medium, high, critical
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT, -- NULL表示永久封禁
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_by TEXT NOT NULL DEFAULT 'system',
    notes TEXT
);

-- IP白名单表
CREATE TABLE IF NOT EXISTS ip_whitelist (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip_address TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by TEXT NOT NULL DEFAULT 'system'
);

-- 安全操作记录表
CREATE TABLE IF NOT EXISTS security_actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    action_type TEXT NOT NULL, -- ip_ban, ip_unban, ip_whitelist_add, ip_whitelist_remove
    target_type TEXT NOT NULL, -- ip, user, resource
    target_id TEXT NOT NULL,
    reason TEXT NOT NULL,
    severity TEXT NOT NULL DEFAULT 'medium',
    duration_hours INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT,
    created_by TEXT NOT NULL,
    notes TEXT
);

-- 下载异常记录表
CREATE TABLE IF NOT EXISTS download_anomalies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    package_id INTEGER,
    ip_address TEXT,
    anomaly_type TEXT NOT NULL, -- high_frequency, suspicious_pattern, etc.
    severity TEXT NOT NULL DEFAULT 'medium', -- low, medium, high, critical
    details TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE SET NULL
);

-- 下载记录表
CREATE TABLE IF NOT EXISTS download_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    package_id INTEGER NOT NULL,
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    download_time TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_ip_bans_ip ON ip_bans(ip_address);
CREATE INDEX IF NOT EXISTS idx_ip_bans_active ON ip_bans(is_active);
CREATE INDEX IF NOT EXISTS idx_ip_bans_created ON ip_bans(created_at);
CREATE INDEX IF NOT EXISTS idx_ip_whitelist_ip ON ip_whitelist(ip_address);
CREATE INDEX IF NOT EXISTS idx_security_actions_type ON security_actions(action_type);
CREATE INDEX IF NOT EXISTS idx_security_actions_target ON security_actions(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_download_anomalies_ip ON download_anomalies(ip_address);
CREATE INDEX IF NOT EXISTS idx_download_anomalies_created ON download_anomalies(created_at);
CREATE INDEX IF NOT EXISTS idx_download_records_ip ON download_records(ip_address);
CREATE INDEX IF NOT EXISTS idx_download_records_user ON download_records(user_id);
CREATE INDEX IF NOT EXISTS idx_download_records_package ON download_records(package_id);
CREATE INDEX IF NOT EXISTS idx_download_records_time ON download_records(download_time); 