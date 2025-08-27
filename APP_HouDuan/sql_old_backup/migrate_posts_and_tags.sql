-- 创建帖子表
CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    author_name VARCHAR(100),
    category_id INTEGER,
    status VARCHAR(20) DEFAULT 'Draft',
    view_count INTEGER DEFAULT 0,
    like_count INTEGER DEFAULT 0,
    comment_count INTEGER DEFAULT 0,
    is_pinned BOOLEAN DEFAULT 0,
    is_featured BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
);

-- 创建标签表
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    color VARCHAR(20),
    use_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 创建帖子标签关联表
CREATE TABLE IF NOT EXISTS post_tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(post_id, tag_id)
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_category_id ON posts(category_id);
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_posts_is_pinned ON posts(is_pinned);
CREATE INDEX IF NOT EXISTS idx_posts_is_featured ON posts(is_featured);
CREATE INDEX IF NOT EXISTS idx_posts_view_count ON posts(view_count);
CREATE INDEX IF NOT EXISTS idx_posts_like_count ON posts(like_count);

CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
CREATE INDEX IF NOT EXISTS idx_tags_use_count ON tags(use_count);

CREATE INDEX IF NOT EXISTS idx_post_tags_post_id ON post_tags(post_id);
CREATE INDEX IF NOT EXISTS idx_post_tags_tag_id ON post_tags(tag_id);

-- 插入一些默认标签
INSERT OR IGNORE INTO tags (name, description, color, use_count) VALUES 
('技术分享', '技术相关的分享和讨论', '#1890ff', 0),
('经验交流', '经验和心得的交流', '#52c41a', 0),
('问题求助', '遇到问题寻求帮助', '#faad14', 0),
('资源推荐', '推荐优质资源', '#f5222d', 0),
('教程指南', '教程和指南类内容', '#722ed1', 0),
('社区活动', '社区活动和公告', '#13c2c2', 0),
('Rust', 'Rust编程语言相关', '#ff4d4f', 0),
('Vue', 'Vue.js前端框架相关', '#52c41a', 0),
('数据库', '数据库相关技术', '#1890ff', 0),
('API', 'API设计和开发', '#722ed1', 0);

-- 确保comments表支持帖子评论
-- 如果comments表还没有target_type字段，添加它
-- 注意：这个操作可能需要根据现有数据库结构调整
-- ALTER TABLE comments ADD COLUMN target_type VARCHAR(20) DEFAULT 'Package';

-- 创建触发器来更新帖子的评论计数
CREATE TRIGGER IF NOT EXISTS update_post_comment_count_insert
AFTER INSERT ON comments
WHEN NEW.target_type = 'Post'
BEGIN
    UPDATE posts SET comment_count = comment_count + 1 WHERE id = NEW.target_id;
END;

CREATE TRIGGER IF NOT EXISTS update_post_comment_count_delete
AFTER DELETE ON comments
WHEN OLD.target_type = 'Post'
BEGIN
    UPDATE posts SET comment_count = comment_count - 1 WHERE id = OLD.target_id;
END;

-- 创建触发器来更新标签使用计数
CREATE TRIGGER IF NOT EXISTS update_tag_use_count_insert
AFTER INSERT ON post_tags
BEGIN
    UPDATE tags SET use_count = use_count + 1 WHERE id = NEW.tag_id;
END;

CREATE TRIGGER IF NOT EXISTS update_tag_use_count_delete
AFTER DELETE ON post_tags
BEGIN
    UPDATE tags SET use_count = use_count - 1 WHERE id = OLD.tag_id;
END;

-- 创建帖子点赞表（可选，用于实现点赞功能）
CREATE TABLE IF NOT EXISTS post_likes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(post_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_post_likes_post_id ON post_likes(post_id);
CREATE INDEX IF NOT EXISTS idx_post_likes_user_id ON post_likes(user_id);

-- 创建触发器来更新帖子的点赞计数
CREATE TRIGGER IF NOT EXISTS update_post_like_count_insert
AFTER INSERT ON post_likes
BEGIN
    UPDATE posts SET like_count = like_count + 1 WHERE id = NEW.post_id;
END;

CREATE TRIGGER IF NOT EXISTS update_post_like_count_delete
AFTER DELETE ON post_likes
BEGIN
    UPDATE posts SET like_count = like_count - 1 WHERE id = OLD.post_id;
END; 