-- 检查announcements表是否存在
CREATE TABLE IF NOT EXISTS announcements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT 'Info',
    priority INTEGER NOT NULL DEFAULT 5,
    enabled BOOLEAN NOT NULL DEFAULT 1,
    start_time TEXT NOT NULL,
    end_time TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 为现有表添加可能缺失的列
PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

-- 创建临时表
CREATE TABLE IF NOT EXISTS announcements_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT 'Info',
    priority INTEGER NOT NULL DEFAULT 5,
    enabled BOOLEAN NOT NULL DEFAULT 1,
    start_time TEXT NOT NULL DEFAULT (datetime('now')),
    end_time TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 将旧表数据复制到新表
INSERT INTO announcements_new (id, title, content, type, priority, enabled, start_time, created_at, updated_at)
SELECT 
    id, 
    title, 
    content, 
    'Info' AS type, 
    COALESCE(priority, 5) AS priority, 
    1 AS enabled, 
    COALESCE(created_at, datetime('now')) AS start_time, 
    created_at, 
    updated_at 
FROM announcements;

-- 删除旧表
DROP TABLE announcements;

-- 重命名新表
ALTER TABLE announcements_new RENAME TO announcements;

-- 为某些字段创建索引
CREATE INDEX IF NOT EXISTS idx_announcements_priority ON announcements (priority);
CREATE INDEX IF NOT EXISTS idx_announcements_enabled ON announcements (enabled);
CREATE INDEX IF NOT EXISTS idx_announcements_start_time ON announcements (start_time);

-- 添加一条初始公告
INSERT OR IGNORE INTO announcements (title, content, type, priority, enabled, start_time)
VALUES ('欢迎使用绳包管理系统', '这是一个初始公告，您可以在管理后台设置和管理公告。', 'Info', 10, 1, datetime('now'));

COMMIT;

PRAGMA foreign_keys=on; 