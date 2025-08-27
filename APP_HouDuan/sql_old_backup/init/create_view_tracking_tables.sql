-- 增强点赞和访问统计系统数据库结构
-- 执行时间: 2024年
-- 目的: 修复点赞重复、访问统计、状态检查等问题

-- 1. 为packages表添加view_count字段
ALTER TABLE packages ADD COLUMN view_count INTEGER DEFAULT 0;

-- 2. 创建资源访问记录表
CREATE TABLE IF NOT EXISTS package_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    package_id INTEGER NOT NULL,
    user_id INTEGER, -- 可为空，支持访客访问
    ip_address TEXT,
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- 3. 创建帖子访问记录表（posts已有view_count字段）
CREATE TABLE IF NOT EXISTS post_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    user_id INTEGER, -- 可为空，支持访客访问
    ip_address TEXT,
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- 4. 创建索引优化性能
CREATE INDEX IF NOT EXISTS idx_package_views_package_id ON package_views(package_id);
CREATE INDEX IF NOT EXISTS idx_package_views_user_id ON package_views(user_id);
CREATE INDEX IF NOT EXISTS idx_package_views_ip ON package_views(ip_address);
CREATE INDEX IF NOT EXISTS idx_package_views_created_at ON package_views(created_at);

CREATE INDEX IF NOT EXISTS idx_post_views_post_id ON post_views(post_id);
CREATE INDEX IF NOT EXISTS idx_post_views_user_id ON post_views(user_id);
CREATE INDEX IF NOT EXISTS idx_post_views_ip ON post_views(ip_address);
CREATE INDEX IF NOT EXISTS idx_post_views_created_at ON post_views(created_at);

-- 5. 创建触发器自动更新访问计数
CREATE TRIGGER IF NOT EXISTS update_package_view_count
AFTER INSERT ON package_views
BEGIN
    UPDATE packages SET view_count = view_count + 1 WHERE id = NEW.package_id;
END;

CREATE TRIGGER IF NOT EXISTS update_post_view_count_from_views
AFTER INSERT ON post_views
BEGIN
    UPDATE posts SET view_count = view_count + 1 WHERE id = NEW.post_id;
END;

-- 6. 为package_likes表创建触发器（如果不存在）
CREATE TRIGGER IF NOT EXISTS update_package_like_count_insert
AFTER INSERT ON package_likes
BEGIN
    UPDATE packages SET like_count = like_count + 1 WHERE id = NEW.package_id;
END;

CREATE TRIGGER IF NOT EXISTS update_package_like_count_delete
AFTER DELETE ON package_likes
BEGIN
    UPDATE packages SET like_count = like_count - 1 WHERE id = NEW.package_id;
END;

-- 7. 为comment_likes表创建触发器（如果不存在）
CREATE TRIGGER IF NOT EXISTS update_comment_like_count_insert
AFTER INSERT ON comment_likes
BEGIN
    UPDATE comments SET like_count = COALESCE(like_count, 0) + 1 WHERE id = NEW.comment_id;
END;

CREATE TRIGGER IF NOT EXISTS update_comment_like_count_delete
AFTER DELETE ON comment_likes
BEGIN
    UPDATE comments SET like_count = COALESCE(like_count, 0) - 1 WHERE id = NEW.comment_id;
END;

-- 8. 确保comments表有like_count字段
ALTER TABLE comments ADD COLUMN like_count INTEGER DEFAULT 0;

-- 9. 创建用户点赞汇总视图（用于"我的点赞"页面）
CREATE VIEW IF NOT EXISTS user_likes_summary AS
SELECT 
    u.id as user_id,
    u.username,
    'package' as like_type,
    p.id as target_id,
    p.name as target_title,
    p.description as target_description,
    pl.created_at
FROM users u
JOIN package_likes pl ON u.id = pl.user_id
JOIN packages p ON pl.package_id = p.id
WHERE p.status = 'approved'

UNION ALL

SELECT 
    u.id as user_id,
    u.username,
    'post' as like_type,
    po.id as target_id,
    po.title as target_title,
    SUBSTR(po.content, 1, 100) as target_description,
    pol.created_at
FROM users u
JOIN post_likes pol ON u.id = pol.user_id
JOIN posts po ON pol.post_id = po.id

UNION ALL

SELECT 
    u.id as user_id,
    u.username,
    'comment' as like_type,
    c.id as target_id,
    CASE 
        WHEN c.target_type = 'Package' THEN 'Resource Comment'
        WHEN c.target_type = 'Post' THEN 'Post Comment'
        ELSE 'Comment'
    END as target_title,
    SUBSTR(c.content, 1, 100) as target_description,
    cl.created_at
FROM users u
JOIN comment_likes cl ON u.id = cl.user_id
JOIN comments c ON cl.comment_id = c.id;

-- 10. 更新现有数据的计数（如果需要）
-- 修复packages的like_count
UPDATE packages SET like_count = (
    SELECT COUNT(*) FROM package_likes WHERE package_id = packages.id
);

-- 修复posts的like_count（应该已经正确，但确保一致性）
UPDATE posts SET like_count = (
    SELECT COUNT(*) FROM post_likes WHERE post_id = posts.id
);

-- 修复comments的like_count
UPDATE comments SET like_count = (
    SELECT COUNT(*) FROM comment_likes WHERE comment_id = comments.id
);

-- 完成提示
-- SELECT 'Database enhancement completed successfully!' as status; 