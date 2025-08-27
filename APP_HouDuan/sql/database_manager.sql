-- 数据库维护和优化脚本
-- 用于系统启动时的数据库维护操作

-- 1. 数据库分析和优化
ANALYZE;

-- 2. 清理和压缩数据库
VACUUM;

-- 3. 检查数据库完整性（可选，注释掉避免启动时间过长）
-- PRAGMA integrity_check;

-- 4. 更新表统计信息
PRAGMA optimize;

-- 5. 检查和修复可能的数据不一致问题

-- 更新包的点赞数
UPDATE packages SET like_count = (
    SELECT COUNT(*) FROM package_likes WHERE package_id = packages.id
) WHERE EXISTS (SELECT 1 FROM package_likes WHERE package_id = packages.id);

-- 更新包的浏览数
UPDATE packages SET view_count = (
    SELECT COUNT(*) FROM package_views WHERE package_id = packages.id
) WHERE EXISTS (SELECT 1 FROM package_views WHERE package_id = packages.id);

-- 更新帖子的点赞数
UPDATE posts SET like_count = (
    SELECT COUNT(*) FROM post_likes WHERE post_id = posts.id
) WHERE EXISTS (SELECT 1 FROM post_likes WHERE post_id = posts.id);

-- 更新帖子的评论数
UPDATE posts SET comment_count = (
    SELECT COUNT(*) FROM comments WHERE target_type = 'Post' AND target_id = posts.id
) WHERE EXISTS (SELECT 1 FROM comments WHERE target_type = 'Post' AND target_id = posts.id);

-- 更新帖子的浏览数
UPDATE posts SET view_count = (
    SELECT COUNT(*) FROM post_views WHERE post_id = posts.id
) WHERE EXISTS (SELECT 1 FROM post_views WHERE post_id = posts.id);

-- 更新评论的点赞数
UPDATE comments SET likes = (
    SELECT COUNT(*) FROM comment_likes WHERE comment_id = comments.id
) WHERE EXISTS (SELECT 1 FROM comment_likes WHERE comment_id = comments.id);

-- 更新评论的踩数
UPDATE comments SET dislikes = (
    SELECT COUNT(*) FROM comment_dislikes WHERE comment_id = comments.id
) WHERE EXISTS (SELECT 1 FROM comment_dislikes WHERE comment_id = comments.id);

-- 更新标签使用计数
UPDATE tags SET use_count = (
    SELECT 
        COALESCE((SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id), 0) +
        COALESCE((SELECT COUNT(*) FROM package_tags WHERE tag_id = tags.id), 0)
);

-- 更新用户统计信息
UPDATE users SET 
    upload_count = (
        SELECT COUNT(*) FROM packages WHERE author = users.username
    ),
    download_count = (
        SELECT COUNT(*) FROM download_records WHERE user_id = users.id
    )
WHERE EXISTS (
    SELECT 1 FROM packages WHERE author = users.username
    UNION
    SELECT 1 FROM download_records WHERE user_id = users.id
);

-- 6. 清理过期数据

-- 删除过期的邮件验证码（超过24小时）
DELETE FROM email_verifications 
WHERE datetime(expires_at) < datetime('now', '-1 day');

-- 删除过期的IP封禁记录
DELETE FROM ip_bans 
WHERE expires_at IS NOT NULL 
AND datetime(expires_at) < datetime('now');

-- 删除过期的安全操作记录
DELETE FROM security_actions 
WHERE expires_at IS NOT NULL 
AND datetime(expires_at) < datetime('now');

-- 清理旧的系统日志（保留最近30天）
DELETE FROM system_logs 
WHERE datetime(timestamp) < datetime('now', '-30 days');

-- 清理旧的下载安全日志（保留最近30天）
DELETE FROM download_security_logs 
WHERE datetime(timestamp) < datetime('now', '-30 days');

-- 7. 数据库配置优化
PRAGMA cache_size = 10000;
PRAGMA temp_store = memory;
PRAGMA mmap_size = 134217728; -- 128MB
PRAGMA synchronous = NORMAL;
PRAGMA journal_mode = WAL;
PRAGMA wal_autocheckpoint = 1000; 