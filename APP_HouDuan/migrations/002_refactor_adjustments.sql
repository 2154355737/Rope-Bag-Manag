-- ================================
-- 数据库迁移脚本 002 - 架构调整
-- 1) 删除无用表
-- 2) 新增 posts 表及 FTS5 索引与触发器
-- 3) 可选 post_tags 关联表
-- 4) 调整 packages 与 comments 表字段与索引
-- ================================

-- 1) 删除无用表
DROP TABLE IF EXISTS announcements;
DROP TABLE IF EXISTS forbidden_words;

-- 2) 新增 posts 表
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    like_count INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    comment_count INTEGER DEFAULT 0,
    status VARCHAR(20) DEFAULT 'published' NOT NULL CHECK (status IN ('draft', 'published', 'archived', 'banned')),
    is_featured BOOLEAN DEFAULT 0,
    is_pinned BOOLEAN DEFAULT 0,
    tags JSON DEFAULT '[]',
    images JSON DEFAULT '[]',
    code_snippet TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);

-- posts 表索引
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_posts_featured ON posts(is_featured);
CREATE INDEX IF NOT EXISTS idx_posts_pinned ON posts(is_pinned);

-- posts 全文搜索索引（SQLite FTS5）
CREATE VIRTUAL TABLE IF NOT EXISTS posts_fts USING fts5(
    title,
    content,
    content='posts',
    content_rowid='id'
);

-- posts FTS5 触发器
CREATE TRIGGER IF NOT EXISTS posts_fts_insert AFTER INSERT ON posts
BEGIN
    INSERT INTO posts_fts(rowid, title, content) VALUES (new.id, new.title, new.content);
END;

CREATE TRIGGER IF NOT EXISTS posts_fts_delete AFTER DELETE ON posts
BEGIN
    INSERT INTO posts_fts(posts_fts, rowid, title, content) VALUES ('delete', old.id, old.title, old.content);
END;

CREATE TRIGGER IF NOT EXISTS posts_fts_update AFTER UPDATE ON posts
BEGIN
    INSERT INTO posts_fts(posts_fts, rowid, title, content) VALUES ('delete', old.id, old.title, old.content);
    INSERT INTO posts_fts(rowid, title, content) VALUES (new.id, new.title, new.content);
END;

-- 3) 可选 post_tags 关联表
CREATE TABLE IF NOT EXISTS post_tags (
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- 4) packages 表字段调整
-- 注意：如果字段已经存在，这些ALTER语句会失败，但这是正常的
-- 我们在迁移逻辑中会处理这些错误

-- 尝试添加新字段，如果已存在则忽略错误
-- file_path 字段已存在于某些版本中，跳过以避免重复
-- ALTER TABLE packages ADD COLUMN file_path VARCHAR(500);
-- ALTER TABLE packages ADD COLUMN file_size INTEGER DEFAULT 0;
-- ALTER TABLE packages ADD COLUMN download_count INTEGER DEFAULT 0;
-- ALTER TABLE packages ADD COLUMN like_count INTEGER DEFAULT 0;
-- ALTER TABLE packages ADD COLUMN view_count INTEGER DEFAULT 0;
-- ALTER TABLE packages ADD COLUMN comment_count INTEGER DEFAULT 0;
-- ALTER TABLE packages ADD COLUMN rating REAL DEFAULT 0.0;
-- ALTER TABLE packages ADD COLUMN is_featured BOOLEAN DEFAULT 0;
-- ALTER TABLE packages ADD COLUMN is_pinned BOOLEAN DEFAULT 0;
-- ALTER TABLE packages ADD COLUMN requirements JSON DEFAULT '[]';
-- ALTER TABLE packages ADD COLUMN screenshots JSON DEFAULT '[]';

-- 这些字段已经在实际使用中添加过了，暂时注释掉以避免迁移冲突

-- packages 新增字段索引
CREATE INDEX IF NOT EXISTS idx_packages_featured ON packages(is_featured);
CREATE INDEX IF NOT EXISTS idx_packages_pinned ON packages(is_pinned);
CREATE INDEX IF NOT EXISTS idx_packages_rating ON packages(rating DESC);
CREATE INDEX IF NOT EXISTS idx_packages_download_count ON packages(download_count DESC);
CREATE INDEX IF NOT EXISTS idx_packages_like_count ON packages(like_count DESC);

-- 5) comments 表字段调整
ALTER TABLE comments ADD COLUMN helpful_count INTEGER DEFAULT 0;
ALTER TABLE comments ADD COLUMN is_helpful BOOLEAN DEFAULT 0; 