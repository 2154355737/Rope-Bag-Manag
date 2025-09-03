-- 数据库修复和字段补齐脚本
-- 用于修复损坏的数据库并添加缺失字段

-- ========================================
-- 1. 检查并修复表结构
-- ========================================

-- 确保 posts 表存在并具有所有必要字段
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    author_name TEXT DEFAULT NULL,
    category_id INTEGER,
    status VARCHAR(20) DEFAULT 'Published',
    view_count INTEGER DEFAULT 0,
    like_count INTEGER DEFAULT 0,
    comment_count INTEGER DEFAULT 0,
    is_pinned BOOLEAN DEFAULT 0,
    is_featured BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    review_status TEXT DEFAULT NULL,
    review_comment TEXT DEFAULT NULL,
    reviewer_id INTEGER DEFAULT NULL,
    reviewed_at DATETIME DEFAULT NULL,
    images TEXT DEFAULT NULL,
    code_snippet TEXT DEFAULT NULL,
    tags TEXT DEFAULT NULL,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- ========================================
-- 2. 添加缺失的字段（如果不存在）
-- ========================================

-- 为现有的 posts 表添加缺失字段
-- 注意：如果字段已存在，这些语句会被忽略

-- 审核相关字段
ALTER TABLE posts ADD COLUMN review_status TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN review_comment TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN reviewer_id INTEGER DEFAULT NULL;
ALTER TABLE posts ADD COLUMN reviewed_at DATETIME DEFAULT NULL;

-- 内容相关字段
ALTER TABLE posts ADD COLUMN images TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN code_snippet TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN tags TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN author_name TEXT DEFAULT NULL;

-- ========================================
-- 3. 更新现有数据
-- ========================================

-- 更新 author_name 字段
UPDATE posts 
SET author_name = (SELECT username FROM users WHERE users.id = posts.author_id)
WHERE author_name IS NULL;

-- 确保 tags 字段有默认值
UPDATE posts 
SET tags = '[]' 
WHERE tags IS NULL;

-- 确保 images 字段有默认值
UPDATE posts 
SET images = '[]' 
WHERE images IS NULL;

-- ========================================
-- 4. 创建索引
-- ========================================

CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_posts_review_status ON posts(review_status);

-- ========================================
-- 5. 验证数据完整性
-- ========================================

-- 检查是否有孤立的帖子（作者不存在）
-- DELETE FROM posts WHERE author_id NOT IN (SELECT id FROM users);

-- ========================================
-- 6. 更新系统设置
-- ========================================

-- 确保 system_settings 表存在
CREATE TABLE IF NOT EXISTS system_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key VARCHAR(100) UNIQUE NOT NULL,
    value TEXT,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('database_repaired', datetime('now'), '数据库修复完成时间'),
('posts_fields_updated', datetime('now'), '帖子字段更新完成时间'); 