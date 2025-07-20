-- 创建系统日志表
CREATE TABLE IF NOT EXISTS system_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    details TEXT
);

-- 创建系统设置表
CREATE TABLE IF NOT EXISTS system_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 插入默认的主题设置
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('primary_color', '#409EFF');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('secondary_color', '#67C23A');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('dark_mode', 'false');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('font_size', '14px');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('language', 'zh-CN');

-- 插入默认的功能开关
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('enable_registration', 'true');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('enable_community', 'true');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('enable_upload', 'true');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('enable_comments', 'true');

-- 插入默认的系统模式
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('system_mode', 'Normal');

-- 插入示例日志
INSERT OR IGNORE INTO system_logs (level, message, details) 
VALUES ('INFO', '系统启动成功', '{"version": "1.0.0", "env": "production"}');

INSERT OR IGNORE INTO system_logs (level, message, details) 
VALUES ('WARN', '发现异常登录尝试', '{"ip": "192.168.1.100", "attempts": 5}');

INSERT OR IGNORE INTO system_logs (level, message, details) 
VALUES ('ERROR', '数据库连接失败', '{"error": "Connection refused", "db": "main"}');

INSERT OR IGNORE INTO system_logs (level, message, details) 
VALUES ('INFO', '用户注册成功', '{"user_id": 10, "username": "test_user"}');

INSERT OR IGNORE INTO system_logs (level, message, details) 
VALUES ('INFO', '文件上传完成', '{"filename": "example.zip", "size": "1.5MB"}'); 