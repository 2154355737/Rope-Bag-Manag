-- 创建系统日志表
CREATE TABLE IF NOT EXISTS system_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    details TEXT
);

-- 创建系统设置表（包含所有需要的列）
CREATE TABLE IF NOT EXISTS system_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 插入默认的主题设置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('primary_color', '#409EFF', '主要颜色');
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('secondary_color', '#67C23A', '次要颜色');
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('dark_mode', 'false', '深色模式');
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('font_size', '14px', '字体大小');
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('language', 'zh-CN', '系统语言');

-- 插入默认的功能开关
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('enable_registration', 'true', '允许用户注册');
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('enable_community', 'true', '启用社区功能');
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('enable_upload', 'true', '允许文件上传');
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('enable_comments', 'true', '启用评论功能');

-- 插入默认的系统模式
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES ('system_mode', 'Normal', '系统运行模式'); 