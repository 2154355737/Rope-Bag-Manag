-- 为announcements表添加缺少的字段
ALTER TABLE announcements ADD COLUMN type TEXT NOT NULL DEFAULT 'Info';
ALTER TABLE announcements ADD COLUMN enabled BOOLEAN NOT NULL DEFAULT 1;
ALTER TABLE announcements ADD COLUMN start_time TEXT NOT NULL DEFAULT (datetime('now'));
ALTER TABLE announcements ADD COLUMN end_time TEXT; 