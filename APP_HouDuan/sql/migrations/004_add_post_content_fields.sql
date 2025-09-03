-- 迁移脚本: 为帖子表添加新的内容字段
-- 版本: 004
-- 说明: 添加审核相关字段、图片、代码片段等字段

-- ========================================
-- 为 posts 表添加新字段
-- ========================================

-- 添加审核相关字段
ALTER TABLE posts ADD COLUMN review_status TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN review_comment TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN reviewer_id INTEGER DEFAULT NULL;
ALTER TABLE posts ADD COLUMN reviewed_at DATETIME DEFAULT NULL;

-- 添加内容相关字段
ALTER TABLE posts ADD COLUMN images TEXT DEFAULT NULL; -- JSON格式存储图片URLs
ALTER TABLE posts ADD COLUMN code_snippet TEXT DEFAULT NULL; -- 代码片段
ALTER TABLE posts ADD COLUMN tags TEXT DEFAULT NULL; -- JSON格式存储标签列表

-- 添加author_name字段（如果不存在）
ALTER TABLE posts ADD COLUMN author_name TEXT DEFAULT NULL;

-- ========================================
-- 更新现有帖子的author_name
-- ========================================

-- 根据author_id更新author_name
UPDATE posts 
SET author_name = (SELECT username FROM users WHERE users.id = posts.author_id)
WHERE author_name IS NULL;

-- ========================================
-- 创建索引以提高查询性能
-- ========================================

CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_posts_review_status ON posts(review_status);

-- ========================================
-- 记录迁移完成
-- ========================================

INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('migration_004_completed', datetime('now'), '迁移004完成时间'),
('last_migration', '004_add_post_content_fields', '最后执行的迁移'); 