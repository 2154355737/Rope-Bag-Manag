-- ================================
-- 绳包管理器数据库初始化脚本 v2.0
-- ================================

-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(100),
    avatar_url VARCHAR(500),
    bio TEXT,
    role VARCHAR(20) DEFAULT 'user' NOT NULL CHECK (role IN ('user', 'elder', 'moderator', 'admin')),
    status VARCHAR(20) DEFAULT 'active' NOT NULL CHECK (status IN ('active', 'suspended', 'banned')),
    settings JSON DEFAULT '{}',
    stats JSON DEFAULT '{}',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 用户表索引
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_status ON users(status);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

-- 分类表
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    icon VARCHAR(255),
    sort_order INTEGER DEFAULT 0,
    enabled BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 分类表索引
CREATE INDEX IF NOT EXISTS idx_categories_enabled ON categories(enabled);
CREATE INDEX IF NOT EXISTS idx_categories_sort_order ON categories(sort_order);

-- 标签表
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL UNIQUE,
    description TEXT,
    color VARCHAR(7) DEFAULT '#6B7280',
    usage_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 标签表索引
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
CREATE INDEX IF NOT EXISTS idx_tags_usage_count ON tags(usage_count DESC);

-- 资源包表
CREATE TABLE IF NOT EXISTS packages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(200) NOT NULL,
    slug VARCHAR(200) UNIQUE NOT NULL,
    author_id INTEGER NOT NULL,
    version VARCHAR(50) NOT NULL,
    description TEXT,
    readme TEXT,
    category_id INTEGER,
    tags JSON DEFAULT '[]',
    metadata JSON DEFAULT '{}',
    stats JSON DEFAULT '{}',
    status VARCHAR(20) DEFAULT 'pending' NOT NULL CHECK (status IN ('pending', 'active', 'rejected', 'archived')),
    published_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL
);

-- 资源包表索引
CREATE INDEX IF NOT EXISTS idx_packages_author_id ON packages(author_id);
CREATE INDEX IF NOT EXISTS idx_packages_category_id ON packages(category_id);
CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(status);
CREATE INDEX IF NOT EXISTS idx_packages_published_at ON packages(published_at);
CREATE INDEX IF NOT EXISTS idx_packages_slug ON packages(slug);
CREATE INDEX IF NOT EXISTS idx_packages_created_at ON packages(created_at);

-- 全文搜索索引（SQLite FTS5）
CREATE VIRTUAL TABLE IF NOT EXISTS packages_fts USING fts5(
    name,
    description,
    content='packages',
    content_rowid='id'
);

-- FTS5触发器
CREATE TRIGGER IF NOT EXISTS packages_fts_insert AFTER INSERT ON packages
BEGIN
    INSERT INTO packages_fts(rowid, name, description) VALUES (new.id, new.name, new.description);
END;

CREATE TRIGGER IF NOT EXISTS packages_fts_delete AFTER DELETE ON packages
BEGIN
    INSERT INTO packages_fts(packages_fts, rowid, name, description) VALUES ('delete', old.id, old.name, old.description);
END;

CREATE TRIGGER IF NOT EXISTS packages_fts_update AFTER UPDATE ON packages
BEGIN
    INSERT INTO packages_fts(packages_fts, rowid, name, description) VALUES ('delete', old.id, old.name, old.description);
    INSERT INTO packages_fts(rowid, name, description) VALUES (new.id, new.name, new.description);
END;

-- 包文件表
CREATE TABLE IF NOT EXISTS package_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    package_id INTEGER NOT NULL,
    filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type VARCHAR(100),
    file_hash VARCHAR(64) NOT NULL,
    storage_type VARCHAR(20) DEFAULT 'local' CHECK (storage_type IN ('local', 'alist', 's3')),
    storage_config JSON DEFAULT '{}',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);

-- 包文件表索引
CREATE INDEX IF NOT EXISTS idx_package_files_package_id ON package_files(package_id);
CREATE INDEX IF NOT EXISTS idx_package_files_hash ON package_files(file_hash);

-- 包标签关联表
CREATE TABLE IF NOT EXISTS package_tags (
    package_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    PRIMARY KEY (package_id, tag_id),
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- 评论表
CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('package', 'post')),
    target_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    parent_id INTEGER,
    status VARCHAR(20) DEFAULT 'active' NOT NULL CHECK (status IN ('active', 'hidden', 'deleted')),
    likes_count INTEGER DEFAULT 0,
    dislikes_count INTEGER DEFAULT 0,
    is_pinned BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES comments(id) ON DELETE CASCADE
);

-- 评论表索引
CREATE INDEX IF NOT EXISTS idx_comments_target ON comments(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments(user_id);
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments(parent_id);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(created_at);

-- 点赞表
CREATE TABLE IF NOT EXISTS likes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('package', 'post', 'comment')),
    target_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(user_id, target_type, target_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 点赞表索引
CREATE INDEX IF NOT EXISTS idx_likes_target ON likes(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_likes_user_id ON likes(user_id);

-- 收藏表
CREATE TABLE IF NOT EXISTS favorites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('package', 'post')),
    target_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(user_id, target_type, target_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 收藏表索引
CREATE INDEX IF NOT EXISTS idx_favorites_target ON favorites(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_favorites_user_id ON favorites(user_id);

-- 下载记录表
CREATE TABLE IF NOT EXISTS downloads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    package_id INTEGER NOT NULL,
    ip_address VARCHAR(45),
    user_agent TEXT,
    download_url VARCHAR(500),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);

-- 下载记录表索引
CREATE INDEX IF NOT EXISTS idx_downloads_package_id ON downloads(package_id);
CREATE INDEX IF NOT EXISTS idx_downloads_user_id ON downloads(user_id);
CREATE INDEX IF NOT EXISTS idx_downloads_created_at ON downloads(created_at);

-- 用户操作日志表
CREATE TABLE IF NOT EXISTS user_actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    action_type VARCHAR(50) NOT NULL,
    target_type VARCHAR(50),
    target_id INTEGER,
    details JSON DEFAULT '{}',
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- 用户操作日志表索引
CREATE INDEX IF NOT EXISTS idx_user_actions_user_id ON user_actions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_actions_type ON user_actions(action_type);
CREATE INDEX IF NOT EXISTS idx_user_actions_created_at ON user_actions(created_at);


-- 系统设置表
CREATE TABLE IF NOT EXISTS system_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key VARCHAR(100) NOT NULL UNIQUE,
    value TEXT,
    description TEXT,
    type VARCHAR(20) DEFAULT 'string' CHECK (type IN ('string', 'number', 'boolean', 'json')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 邮件设置表
CREATE TABLE IF NOT EXISTS mail_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    smtp_host VARCHAR(255),
    smtp_port INTEGER DEFAULT 587,
    username VARCHAR(255),
    password VARCHAR(255),
    from_address VARCHAR(255),
    from_name VARCHAR(255),
    enabled BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 邮件验证表
CREATE TABLE IF NOT EXISTS email_verifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email VARCHAR(255) NOT NULL,
    code VARCHAR(10) NOT NULL,
    type VARCHAR(20) DEFAULT 'register' CHECK (type IN ('register', 'reset_password', 'change_email')),
    expires_at DATETIME NOT NULL,
    used_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 邮件验证表索引
CREATE INDEX IF NOT EXISTS idx_email_verifications_email ON email_verifications(email);
CREATE INDEX IF NOT EXISTS idx_email_verifications_code ON email_verifications(code);
CREATE INDEX IF NOT EXISTS idx_email_verifications_expires_at ON email_verifications(expires_at);

-- 通知表
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    type VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT,
    data JSON DEFAULT '{}',
    read_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 通知表索引
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_read_at ON notifications(read_at);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);

-- 订阅表
CREATE TABLE IF NOT EXISTS subscriptions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('user', 'category', 'tag')),
    target_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE(user_id, target_type, target_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 订阅表索引
CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_target ON subscriptions(target_type, target_id);


-- IP封禁表
CREATE TABLE IF NOT EXISTS ip_bans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip_address VARCHAR(45) NOT NULL UNIQUE,
    reason TEXT,
    banned_by INTEGER,
    expires_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (banned_by) REFERENCES users(id) ON DELETE SET NULL
);

-- IP封禁表索引
CREATE INDEX IF NOT EXISTS idx_ip_bans_ip_address ON ip_bans(ip_address);
CREATE INDEX IF NOT EXISTS idx_ip_bans_expires_at ON ip_bans(expires_at);

-- 下载安全表
CREATE TABLE IF NOT EXISTS download_security (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    package_id INTEGER NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    download_count INTEGER DEFAULT 1,
    first_download_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_download_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);

-- 下载安全表索引
CREATE INDEX IF NOT EXISTS idx_download_security_user_id ON download_security(user_id);
CREATE INDEX IF NOT EXISTS idx_download_security_package_id ON download_security(package_id);
CREATE INDEX IF NOT EXISTS idx_download_security_ip ON download_security(ip_address);
CREATE INDEX IF NOT EXISTS idx_download_security_last_download ON download_security(last_download_at);

-- 插入默认数据
INSERT OR IGNORE INTO categories (id, name, description, icon, sort_order) VALUES
(1, '绳艺教程', '绳艺相关的教程和指导', '📚', 1),
(2, '绳艺图片', '绳艺相关的图片和摄影作品', '🖼️', 2),
(3, '绳艺视频', '绳艺相关的视频内容', '🎥', 3),
(4, '工具软件', '绳艺相关的工具和软件', '🛠️', 4),
(5, '其他资源', '其他类型的绳艺相关资源', '📦', 5);

INSERT OR IGNORE INTO tags (name, description, color) VALUES
('新手教程', '适合新手学习的内容', '#10B981'),
('进阶技巧', '需要一定基础的高级技巧', '#F59E0B'),
('安全提醒', '涉及安全注意事项的内容', '#EF4444'),
('工具介绍', '介绍相关工具的使用', '#6366F1'),
('经典作品', '经典的绳艺作品', '#8B5CF6');

INSERT OR IGNORE INTO system_settings (key, value, description, type) VALUES
('site_name', '绳包管理器', '网站名称', 'string'),
('site_description', '专业的绳艺资源分享平台', '网站描述', 'string'),
('max_file_size', '52428800', '最大上传文件大小（字节）', 'number'),
('allow_register', 'true', '是否允许注册', 'boolean'),
('review_required', 'true', '资源发布是否需要审核', 'boolean'); 