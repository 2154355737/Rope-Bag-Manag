-- 修复评论点赞系统
-- 确保数据库结构完整和数据一致性

-- 1. 确保comments表有正确的字段（应该使用likes而不是like_count）
-- comments表已经有likes字段，无需修改

-- 2. 确保comment_likes表存在且结构正确
-- comment_likes表已存在，检查是否有索引
CREATE INDEX IF NOT EXISTS idx_comment_likes_comment_id ON comment_likes(comment_id);
CREATE INDEX IF NOT EXISTS idx_comment_likes_user_id ON comment_likes(user_id);

-- 3. 创建或重建触发器确保数据同步
DROP TRIGGER IF EXISTS update_comment_like_count_insert;
DROP TRIGGER IF EXISTS update_comment_like_count_delete;

CREATE TRIGGER update_comment_like_count_insert
AFTER INSERT ON comment_likes
BEGIN
    UPDATE comments SET likes = likes + 1 WHERE id = NEW.comment_id;
END;

CREATE TRIGGER update_comment_like_count_delete
AFTER DELETE ON comment_likes
BEGIN
    UPDATE comments SET likes = likes - 1 WHERE id = OLD.comment_id;
END;

-- 4. 创建用户点赞汇总视图（如果不存在）
DROP VIEW IF EXISTS user_likes_summary;

CREATE VIEW user_likes_summary AS
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

-- 5. 修复现有数据的不一致（重新计算comments的likes数）
UPDATE comments SET likes = (
    SELECT COUNT(*) FROM comment_likes WHERE comment_id = comments.id
);

-- 6. 验证数据一致性
-- 显示不一致的数据（如果有的话）
-- SELECT 
--     c.id, 
--     c.likes as stored_likes, 
--     (SELECT COUNT(*) FROM comment_likes cl WHERE cl.comment_id = c.id) as actual_likes
-- FROM comments c 
-- WHERE c.likes != (SELECT COUNT(*) FROM comment_likes cl WHERE cl.comment_id = c.id); 