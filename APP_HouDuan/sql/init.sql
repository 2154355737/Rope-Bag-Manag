-- 绳包管理器数据库初始化脚本
-- 版本: 1.0.0
-- 说明: 完整的数据库结构和初始数据

-- ========================================
-- 执行顺序说明
-- ========================================
-- 1. 创建核心表结构
-- 2. 创建系统表结构  
-- 3. 创建索引
-- 4. 插入默认数据

-- ========================================
-- 1. 创建核心表结构
-- ========================================

-- 用户相关表
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(100),
    role VARCHAR(20) DEFAULT 'user' NOT NULL,
    star INTEGER DEFAULT 0,
    ban_status VARCHAR(20) DEFAULT 'normal',
    ban_reason TEXT,
    qq_number VARCHAR(20),
    avatar_url VARCHAR(255),
    bio TEXT,
    is_active BOOLEAN DEFAULT 1,
    is_admin BOOLEAN DEFAULT 0,
    login_count INTEGER DEFAULT 0,
    upload_count INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    last_login DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS email_verifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    email TEXT NOT NULL,
    code TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    used INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 资源相关表
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    description TEXT,
    enabled BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS packages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(200) NOT NULL,
    author VARCHAR(100),
    version VARCHAR(50),
    description TEXT,
    file_url VARCHAR(500),
    file_size INTEGER,
    download_count INTEGER DEFAULT 0,
    like_count INTEGER DEFAULT 0,
    favorite_count INTEGER DEFAULT 0,
    category_id INTEGER,
    status VARCHAR(20) DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- 社区功能表
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

CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    target_type VARCHAR(20) NOT NULL DEFAULT 'Package',
    target_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    status VARCHAR(20) DEFAULT 'Active' NOT NULL,
    parent_id INTEGER,
    likes INTEGER DEFAULT 0,
    dislikes INTEGER DEFAULT 0,
    is_pinned BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES comments(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS comment_likes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    comment_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(comment_id, user_id)
);

CREATE TABLE IF NOT EXISTS comment_dislikes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    comment_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(comment_id, user_id)
);

-- ========================================
-- 2. 创建系统表结构
-- ========================================

CREATE TABLE IF NOT EXISTS system_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS system_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    details TEXT
);

CREATE TABLE IF NOT EXISTS user_actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    action_type VARCHAR(50) NOT NULL,
    target_type VARCHAR(50),
    target_id INTEGER,
    details TEXT,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS resource_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    resource_id INTEGER NOT NULL,
    resource_type TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    action TEXT NOT NULL,
    ip_address TEXT,
    old_data TEXT,
    new_data TEXT,
    timestamp INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 安全相关表
CREATE TABLE IF NOT EXISTS ip_bans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip_address TEXT NOT NULL UNIQUE,
    reason TEXT,
    banned_by INTEGER,
    expires_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (banned_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS download_security_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    resource_id INTEGER,
    ip_address TEXT NOT NULL,
    user_agent TEXT,
    status VARCHAR(20) NOT NULL,
    reason TEXT,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- ========================================
-- 3. 创建核心索引
-- ========================================

-- 用户表索引
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);

-- 包表索引
CREATE INDEX IF NOT EXISTS idx_packages_category_id ON packages(category_id);
CREATE INDEX IF NOT EXISTS idx_packages_created_at ON packages(created_at);
CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(status);

-- 评论表索引
CREATE INDEX IF NOT EXISTS idx_comments_target_type_target_id ON comments(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments(user_id);
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments(parent_id);
CREATE INDEX IF NOT EXISTS idx_comments_status ON comments(status);

-- 行为日志索引
CREATE INDEX IF NOT EXISTS idx_user_actions_user_id ON user_actions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_actions_created_at ON user_actions(created_at);

-- ========================================
-- 4. 插入默认数据
-- ========================================

-- 系统设置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('site_name', '绳包管理器', '网站名称'),
('site_description', '专业的绳结资源管理平台', '网站描述'),
('enable_registration', 'true', '允许用户注册'),
('enable_comments', 'true', '启用评论功能'),
('primary_color', '#409EFF', '主要颜色'),
('secondary_color', '#67C23A', '次要颜色'),
('max_file_size', '10485760', '最大文件上传大小（字节）'),
('allowed_file_types', 'zip,rar,7z,tar,gz', '允许的文件类型'),
('system_mode', 'Normal', '系统运行模式'),
('database_version', '1.0.0', '数据库版本号');

-- 默认分类（仅在表为空时插入）
INSERT INTO categories (id, name, description, enabled) 
SELECT 1, '基础绳结', '基础的绳结技法和教学资源', 1
WHERE NOT EXISTS (SELECT 1 FROM categories LIMIT 1)
UNION ALL
SELECT 2, '进阶技巧', '进阶绳结技巧和复杂绳法', 1
WHERE NOT EXISTS (SELECT 1 FROM categories LIMIT 1)
UNION ALL
SELECT 3, '工具器材', '绳结工具和相关器材介绍', 1
WHERE NOT EXISTS (SELECT 1 FROM categories LIMIT 1)
UNION ALL
SELECT 4, '教学视频', '绳结教学视频和演示', 1
WHERE NOT EXISTS (SELECT 1 FROM categories LIMIT 1)
UNION ALL
SELECT 5, '图文教程', '详细的图文教程和说明', 1
WHERE NOT EXISTS (SELECT 1 FROM categories LIMIT 1)
UNION ALL
SELECT 6, '社区讨论', '社区交流和讨论话题', 1
WHERE NOT EXISTS (SELECT 1 FROM categories LIMIT 1);

-- 默认管理员用户 (用户名: admin, 密码: admin123) - 仅在没有管理员时创建
INSERT INTO users (
    id, username, email, password_hash, nickname, role, 
    is_active, is_admin, created_at
)
SELECT 1, 'admin', 'admin@example.com', 
    '$2b$12$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', 
    '系统管理员', 'admin', 1, 1, CURRENT_TIMESTAMP
WHERE NOT EXISTS (SELECT 1 FROM users WHERE role = 'admin' OR is_admin = 1);

-- 记录初始化完成
INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('database_initialized_at', datetime('now'), '数据库初始化时间');