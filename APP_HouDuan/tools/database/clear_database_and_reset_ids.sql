-- 清空数据库并重置所有表的ID序列
-- 警告：此操作将删除所有数据，请谨慎使用！

-- 1. 禁用外键约束检查（临时）
PRAGMA foreign_keys = OFF;

-- 2. 清空所有表数据（保留表结构）
DELETE FROM comment_dislikes;
DELETE FROM comment_likes;
DELETE FROM comments;
DELETE FROM posts;
DELETE FROM download_security_logs;
DELETE FROM ip_bans;
DELETE FROM resource_records;
DELETE FROM user_actions;
DELETE FROM system_logs;
DELETE FROM email_verifications;
DELETE FROM packages;
DELETE FROM categories;
DELETE FROM users;

-- 3. 重置所有表的自增ID序列
-- 方法1：删除sqlite_sequence表中的记录（推荐）
DELETE FROM sqlite_sequence WHERE name IN (
    'users', 'email_verifications', 'categories', 'packages', 
    'posts', 'comments', 'comment_likes', 'comment_dislikes',
    'system_logs', 'user_actions', 'resource_records', 
    'ip_bans', 'download_security_logs'
);

-- 4. 重新启用外键约束检查
PRAGMA foreign_keys = ON;

-- 5. 重新插入必要的默认数据
-- 系统设置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('site_name', '绳包管理器', '网站名称'),
('site_description', '专业的绳结资源管理平台', '网站描述'),
('enable_registration', 'true', '允许用户注册'),
('enable_comments', 'true', '启用评论功能'),
('primary_color', '#409EFF', '主要颜色'),
('secondary_color', '#67C23A', '次要颜色'),
('max_file_size', '10485760', '最大文件上传大小（字节）'),
('allowed_file_types', 'zip,rar,7z,tar,gz', '允许的文件类型'),
('system_mode', 'Normal', '系统运行模式'),
('database_version', '1.0.0', '数据库版本号'),
('database_cleared_at', datetime('now'), '数据库清空时间');

-- 默认分类
INSERT INTO categories (name, description, enabled) VALUES 
('基础绳结', '基础的绳结技法和教学资源', 1),
('进阶技巧', '进阶绳结技巧和复杂绳法', 1),
('工具器材', '绳结工具和相关器材介绍', 1),
('教学视频', '绳结教学视频和演示', 1),
('图文教程', '详细的图文教程和说明', 1),
('社区讨论', '社区交流和讨论话题', 1);

-- 默认管理员用户 (用户名: admin, 密码: admin123)
INSERT INTO users (
    username, email, password_hash, nickname, role, 
    is_active, is_admin, created_at
) VALUES (
    'admin', 'admin@example.com', 
    '$2b$12$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', 
    '系统管理员', 'admin', 1, 1, CURRENT_TIMESTAMP
);

-- 验证清空结果
SELECT 'users' as table_name, COUNT(*) as record_count FROM users
UNION ALL
SELECT 'categories', COUNT(*) FROM categories
UNION ALL
SELECT 'packages', COUNT(*) FROM packages
UNION ALL
SELECT 'posts', COUNT(*) FROM posts
UNION ALL
SELECT 'comments', COUNT(*) FROM comments;

-- 查看ID序列状态
SELECT * FROM sqlite_sequence; 