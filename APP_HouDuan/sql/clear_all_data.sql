-- 清空所有表数据并重置ID序列
-- 警告：此操作将删除所有数据，请谨慎使用！
-- 使用方法：sqlite3 data.db.1 < clear_all_data.sql

-- 1. 禁用外键约束检查
PRAGMA foreign_keys = OFF;

-- 2. 清空所有表数据（按依赖关系排序）
-- 注意：user_likes_summary是视图，不需要清空
DELETE FROM comment_dislikes;
DELETE FROM comment_likes;
DELETE FROM post_likes;
DELETE FROM post_tags;
DELETE FROM package_likes;
DELETE FROM package_tags;
DELETE FROM package_views;
DELETE FROM post_views;
DELETE FROM comments;
DELETE FROM posts;
DELETE FROM download_records;
DELETE FROM download_rate_limits;
DELETE FROM download_anomalies;
DELETE FROM resource_access_stats;
DELETE FROM ip_bans;
DELETE FROM ip_whitelist;
DELETE FROM security_actions;
DELETE FROM security_config;
DELETE FROM download_security_logs;
DELETE FROM resource_records;
DELETE FROM user_actions;
DELETE FROM system_logs;
DELETE FROM email_verifications;
DELETE FROM mail_logs;
DELETE FROM mail_templates;
DELETE FROM mail_settings;
DELETE FROM subscriptions;
DELETE FROM forbidden_words;
DELETE FROM backups;
DELETE FROM notifications;
DELETE FROM announcements;
DELETE FROM banners;
DELETE FROM app_launches;
DELETE FROM user_check_ins;
DELETE FROM packages;
DELETE FROM categories;
DELETE FROM tags;
DELETE FROM users;

-- 3. 重置所有表的自增ID序列
DELETE FROM sqlite_sequence WHERE name IN (
    'users', 'categories', 'tags', 'packages', 'posts', 'comments',
    'comment_likes', 'comment_dislikes', 'post_likes', 'post_tags',
    'package_likes', 'package_tags', 'package_views', 'post_views',
    'email_verifications', 'system_logs', 'user_actions', 'resource_records',
    'download_records', 'download_rate_limits', 'download_anomalies',
    'resource_access_stats', 'ip_bans', 'ip_whitelist', 'security_actions',
    'security_config', 'download_security_logs', 'mail_settings',
    'mail_logs', 'mail_templates', 'subscriptions', 'forbidden_words',
    'backups', 'notifications', 'announcements', 'banners',
    'app_launches', 'user_check_ins'
);

-- 4. 重新启用外键约束检查
PRAGMA foreign_keys = ON;

-- 5. 验证清空结果
SELECT 'Table清空验证:' as info;
SELECT 'users' as table_name, COUNT(*) as record_count FROM users
UNION ALL SELECT 'categories', COUNT(*) FROM categories
UNION ALL SELECT 'tags', COUNT(*) FROM tags
UNION ALL SELECT 'packages', COUNT(*) FROM packages
UNION ALL SELECT 'posts', COUNT(*) FROM posts
UNION ALL SELECT 'comments', COUNT(*) FROM comments
UNION ALL SELECT 'notifications', COUNT(*) FROM notifications
UNION ALL SELECT 'user_actions', COUNT(*) FROM user_actions;

-- 6. 查看ID序列重置状态
SELECT 'ID序列重置状态:' as info;
SELECT name, seq FROM sqlite_sequence ORDER BY name;