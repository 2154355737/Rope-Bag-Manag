-- 迁移脚本: 添加缺失的列
-- 版本: 001
-- 说明: 为现有表添加新版本需要的列

-- ========================================
-- 用户表添加缺失列
-- ========================================

-- 检查并添加 is_pinned 列到 comments 表
ALTER TABLE comments ADD COLUMN is_pinned BOOLEAN DEFAULT 0;

-- 检查并添加标签相关列到 packages 表  
-- ALTER TABLE packages ADD COLUMN tags TEXT DEFAULT '';

-- ========================================
-- 帖子表相关（如果不存在则创建）
-- ========================================

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    author_id INTEGER NOT NULL,
    category_id INTEGER,
    status VARCHAR(20) DEFAULT 'published',
    view_count INTEGER DEFAULT 0,
    like_count INTEGER DEFAULT 0,
    comment_count INTEGER DEFAULT 0,
    is_pinned BOOLEAN DEFAULT 0,
    is_featured BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- ========================================
-- 标签系统（如果不存在则创建）
-- ========================================

CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    color VARCHAR(10),
    category VARCHAR(50),
    usage_count INTEGER DEFAULT 0,
    is_system BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS package_tags (
    package_id INTEGER NOT NULL,
    tag_name VARCHAR(50) NOT NULL,
    PRIMARY KEY (package_id, tag_name),
    FOREIGN KEY (package_id) REFERENCES packages(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS post_tags (
    post_id INTEGER NOT NULL,
    tag_name VARCHAR(50) NOT NULL,
    PRIMARY KEY (post_id, tag_name),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

-- ========================================
-- 通知系统（如果不存在则创建）
-- ========================================

CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    type VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT,
    data TEXT,
    read_status BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- ========================================
-- 订阅系统更新
-- ========================================

-- 检查并添加 subscription_lock 列到 subscriptions 表
-- ALTER TABLE subscriptions ADD COLUMN subscription_lock INTEGER DEFAULT 0;

-- ========================================
-- 公告和横幅系统（如果不存在则创建）
-- ========================================

CREATE TABLE IF NOT EXISTS announcements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    priority INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS banners (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL,
    image_url VARCHAR(500),
    link_url VARCHAR(500),
    description TEXT,
    position VARCHAR(50) DEFAULT 'home',
    is_active BOOLEAN DEFAULT 1,
    display_order INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ========================================
-- 邮件系统（如果不存在则创建）
-- ========================================

CREATE TABLE IF NOT EXISTS mail_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    smtp_server VARCHAR(255) NOT NULL,
    smtp_port INTEGER NOT NULL,
    username VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    from_name VARCHAR(255),
    from_email VARCHAR(255),
    enable_ssl BOOLEAN DEFAULT 1,
    enable_tls BOOLEAN DEFAULT 1,
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ========================================
-- 违禁词系统（如果不存在则创建）
-- ========================================

CREATE TABLE IF NOT EXISTS forbidden_words (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word VARCHAR(100) NOT NULL UNIQUE,
    replacement VARCHAR(100),
    severity VARCHAR(20) DEFAULT 'medium',
    category VARCHAR(50),
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ========================================
-- 记录迁移完成
-- ========================================

INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('migration_001_completed', datetime('now'), '迁移001完成时间'),
('last_migration', '001_add_missing_columns', '最后执行的迁移');