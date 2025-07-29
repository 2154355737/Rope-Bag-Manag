-- =============================================
-- 绳包管理系统 - 完整数据库初始化脚本
-- 版本: v2.0
-- 创建时间: 2025-01-27
-- 说明: 包含项目所需的所有表结构、索引、约束和默认数据
-- =============================================

-- 启用外键约束
PRAGMA foreign_keys = ON;

-- =============================================
-- 核心业务表
-- =============================================

-- 用户表
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

-- 分类表
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(50) NOT NULL,
    description TEXT,
    enabled BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 资源包表（包含审核功能）
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
    status VARCHAR(20) DEFAULT 'pending',
    -- 审核相关字段
    reviewer_id INTEGER DEFAULT NULL,
    reviewed_at DATETIME DEFAULT NULL,
    review_comment TEXT DEFAULT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL,
    FOREIGN KEY (reviewer_id) REFERENCES users(id) ON DELETE SET NULL
);

-- 评论表
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES comments(id) ON DELETE CASCADE
);

-- 评论点赞表
CREATE TABLE IF NOT EXISTS comment_likes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    comment_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(comment_id, user_id)
);

-- 评论点踩表
CREATE TABLE IF NOT EXISTS comment_dislikes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    comment_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (comment_id) REFERENCES comments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(comment_id, user_id)
);

-- =============================================
-- 系统功能表
-- =============================================

-- 系统设置表
CREATE TABLE IF NOT EXISTS system_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key VARCHAR(100) UNIQUE NOT NULL,
    value TEXT,
    description TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 系统日志表
CREATE TABLE IF NOT EXISTS system_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level VARCHAR(20) NOT NULL,
    message TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    details TEXT
);

-- 公告表
CREATE TABLE IF NOT EXISTS announcements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    priority INTEGER DEFAULT 1,
    enabled BOOLEAN DEFAULT 1,
    created_by INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

-- 用户行为日志表
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

-- 资源记录表
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

-- 禁用词表
CREATE TABLE IF NOT EXISTS forbidden_words (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT UNIQUE NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- 邮件系统表
-- =============================================

-- 邮件配置表
CREATE TABLE IF NOT EXISTS mail_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    smtp_server TEXT NOT NULL DEFAULT 'smtp.qq.com',
    smtp_port INTEGER NOT NULL DEFAULT 465,
    username TEXT NOT NULL DEFAULT '',
    password TEXT NOT NULL DEFAULT '',
    from_name TEXT NOT NULL DEFAULT '绳包管理器',
    enabled INTEGER NOT NULL DEFAULT 0,
    use_ssl INTEGER NOT NULL DEFAULT 1,
    auth_required INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 邮件发送记录表
CREATE TABLE IF NOT EXISTS mail_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    to_email TEXT NOT NULL,
    subject TEXT NOT NULL,
    mail_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    error_message TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    sent_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 邮件模板表
CREATE TABLE IF NOT EXISTS mail_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    template_type TEXT NOT NULL UNIQUE,
    subject TEXT NOT NULL,
    content TEXT NOT NULL,
    variables TEXT,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 邮件验证码记录表
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

-- =============================================
-- 订阅功能表
-- =============================================

-- 资源订阅表
CREATE TABLE IF NOT EXISTS subscriptions (
    user_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    enabled INTEGER DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY(user_id, category_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- =============================================
-- 创建索引以优化查询性能
-- =============================================

-- 用户表索引
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_ban_status ON users(ban_status);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

-- 资源包表索引
CREATE INDEX IF NOT EXISTS idx_packages_category_id ON packages(category_id);
CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(status);
CREATE INDEX IF NOT EXISTS idx_packages_reviewer ON packages(reviewer_id);
CREATE INDEX IF NOT EXISTS idx_packages_created_at ON packages(created_at);
CREATE INDEX IF NOT EXISTS idx_packages_author ON packages(author);

-- 评论表索引
CREATE INDEX IF NOT EXISTS idx_comments_target_type_target_id ON comments(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments(user_id);
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments(parent_id);
CREATE INDEX IF NOT EXISTS idx_comments_status ON comments(status);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(created_at);

-- 评论点赞/点踩索引
CREATE INDEX IF NOT EXISTS idx_comment_likes_comment_id ON comment_likes(comment_id);
CREATE INDEX IF NOT EXISTS idx_comment_likes_user_id ON comment_likes(user_id);
CREATE INDEX IF NOT EXISTS idx_comment_dislikes_comment_id ON comment_dislikes(comment_id);
CREATE INDEX IF NOT EXISTS idx_comment_dislikes_user_id ON comment_dislikes(user_id);

-- 用户行为日志索引
CREATE INDEX IF NOT EXISTS idx_user_actions_user_id ON user_actions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_actions_action_type ON user_actions(action_type);
CREATE INDEX IF NOT EXISTS idx_user_actions_created_at ON user_actions(created_at);
CREATE INDEX IF NOT EXISTS idx_user_actions_target ON user_actions(target_type, target_id);

-- 资源记录表索引
CREATE INDEX IF NOT EXISTS idx_resource_records_resource ON resource_records(resource_id, resource_type);
CREATE INDEX IF NOT EXISTS idx_resource_records_user ON resource_records(user_id);
CREATE INDEX IF NOT EXISTS idx_resource_records_action ON resource_records(action);
CREATE INDEX IF NOT EXISTS idx_resource_records_timestamp ON resource_records(timestamp);

-- 邮件相关索引
CREATE INDEX IF NOT EXISTS idx_mail_logs_to_email ON mail_logs(to_email);
CREATE INDEX IF NOT EXISTS idx_mail_logs_status ON mail_logs(status);
CREATE INDEX IF NOT EXISTS idx_mail_logs_mail_type ON mail_logs(mail_type);
CREATE INDEX IF NOT EXISTS idx_mail_logs_created_at ON mail_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_email_verifications_email ON email_verifications(email);
CREATE INDEX IF NOT EXISTS idx_email_verifications_code ON email_verifications(code);

-- 系统表索引
CREATE INDEX IF NOT EXISTS idx_system_settings_key ON system_settings(key);
CREATE INDEX IF NOT EXISTS idx_system_logs_level ON system_logs(level);
CREATE INDEX IF NOT EXISTS idx_system_logs_timestamp ON system_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_announcements_priority ON announcements(priority);
CREATE INDEX IF NOT EXISTS idx_announcements_enabled ON announcements(enabled);

-- =============================================
-- 插入默认数据
-- =============================================

-- 插入默认系统设置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('site_name', '绳包管理器', '网站名称'),
('site_description', '专业的绳结资源管理平台', '网站描述'),
('max_file_size', '10485760', '最大文件上传大小（字节）'),
('allowed_file_types', 'zip,rar,7z,tar,gz', '允许的文件类型'),
('registration_enabled', 'true', '是否允许用户注册'),
('comment_approval_required', 'false', '评论是否需要审核'),
('resource_approval_required', 'true', '资源是否需要审核'),
('primary_color', '#409EFF', '主题主色调'),
('secondary_color', '#67C23A', '主题次色调'),
('success_color', '#67C23A', '成功色'),
('warning_color', '#E6A23C', '警告色'),
('danger_color', '#F56C6C', '危险色'),
('info_color', '#909399', '信息色'),
('dark_mode', 'false', '深色模式'),
('font_size', '14px', '字体大小'),
('language', 'zh-CN', '语言设置'),
('items_per_page', '20', '每页显示条目数'),
('backup_enabled', 'true', '是否启用自动备份'),
('backup_interval', '24', '备份间隔（小时）'),
('log_retention_days', '30', '日志保留天数');

-- 插入默认管理员用户 (用户名: admin, 密码: admin123)
INSERT OR IGNORE INTO users (id, username, email, password_hash, nickname, role, is_active, is_admin, created_at) VALUES 
(1, 'admin', 'admin@example.com', '$2b$12$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi', '系统管理员', 'admin', 1, 1, CURRENT_TIMESTAMP);

-- 插入默认分类
INSERT OR IGNORE INTO categories (id, name, description, enabled, created_at) VALUES 
(1, '绳结技法', '各种绳结技法和教程', 1, CURRENT_TIMESTAMP),
(2, '工具资源', '绳结相关工具和资源', 1, CURRENT_TIMESTAMP),
(3, '视频教程', '绳结技法视频教程', 1, CURRENT_TIMESTAMP),
(4, '图文教程', '绳结技法图文说明', 1, CURRENT_TIMESTAMP),
(5, '进阶技法', '高级绳结技法和创新', 1, CURRENT_TIMESTAMP),
(6, '安全知识', '绳结安全使用知识', 1, CURRENT_TIMESTAMP),
(7, '社区分享', '用户分享和交流', 1, CURRENT_TIMESTAMP);

-- 插入默认邮件配置
INSERT OR IGNORE INTO mail_settings (id, smtp_server, smtp_port, username, password, from_name, enabled)
VALUES (1, 'smtp.qq.com', 465, '', '', '绳包管理器', 0);

-- 插入默认邮件模板
INSERT OR IGNORE INTO mail_templates (template_type, subject, content, variables) VALUES 
('verification', '【绳包管理器】邮箱验证码', 
'<div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
<h2 style="color: #409EFF; text-align: center;">邮箱验证</h2>
<p>您好！</p>
<p>您的验证码是：</p>
<div style="text-align: center; margin: 20px 0;">
<span style="display: inline-block; padding: 10px 20px; background-color: #f5f7fa; border: 2px dashed #409EFF; font-size: 24px; font-weight: bold; color: #409EFF; letter-spacing: 3px;">{{code}}</span>
</div>
<p style="color: #E6A23C;">此验证码5分钟内有效，请及时使用。</p>
<p>如果您没有注册账号，请忽略此邮件。</p>
<hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
<p style="font-size: 12px; color: #999; text-align: center;">绳包管理器团队</p>
</div>', 
'{"code": "验证码"}'),

('reset_password', '【绳包管理器】密码重置', 
'<div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
<h2 style="color: #409EFF; text-align: center;">密码重置</h2>
<p>您好！</p>
<p>您已请求重置密码，请点击以下链接重置密码：</p>
<div style="text-align: center; margin: 20px 0;">
<a href="{{reset_link}}" style="display: inline-block; padding: 12px 24px; background-color: #409EFF; color: white; text-decoration: none; border-radius: 4px;">重置密码</a>
</div>
<p style="color: #E6A23C;">此链接24小时内有效。</p>
<p>如果您没有请求重置密码，请忽略此邮件。</p>
<hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
<p style="font-size: 12px; color: #999; text-align: center;">绳包管理器团队</p>
</div>', 
'{"reset_link": "重置密码链接"}'),

('notification', '【绳包管理器】新资源通知', 
'<div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
<h2 style="color: #409EFF; text-align: center;">新资源发布</h2>
<p>您好！</p>
<p>有新的资源发布，您可能会感兴趣：</p>
<div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 20px 0; background-color: #f9f9f9;">
<h3 style="color: #303133; margin: 0 0 10px 0;">{{resource_name}}</h3>
<p style="color: #606266; margin: 0 0 10px 0;">{{resource_description}}</p>
<div style="text-align: center;">
<a href="{{resource_link}}" style="display: inline-block; padding: 8px 16px; background-color: #67C23A; color: white; text-decoration: none; border-radius: 4px;">查看详情</a>
</div>
</div>
<hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
<p style="font-size: 12px; color: #999; text-align: center;">绳包管理器团队</p>
</div>', 
'{"resource_name": "资源名称", "resource_description": "资源描述", "resource_link": "资源链接"}'),

('test', '【绳包管理器】测试邮件', 
'<div style="font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
<h2 style="color: #409EFF; text-align: center;">邮件服务测试</h2>
<p>您好！</p>
<p>这是一封测试邮件，如果您收到此邮件，说明邮件服务配置正确。</p>
<div style="background-color: #f0f9ff; border: 1px solid #409EFF; border-radius: 8px; padding: 16px; margin: 20px 0;">
<p style="margin: 0; color: #409EFF;"><strong>✓ 邮件服务运行正常</strong></p>
<p style="margin: 10px 0 0 0; font-size: 14px; color: #666;">发送时间：{{send_time}}</p>
</div>
<hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
<p style="font-size: 12px; color: #999; text-align: center;">绳包管理器团队</p>
</div>', 
'{"send_time": "发送时间"}');

-- 插入默认公告
INSERT OR IGNORE INTO announcements (id, title, content, priority, enabled, created_by, created_at) VALUES 
(1, '欢迎使用绳包管理器', '感谢您使用绳包管理器！本平台致力于为绳结爱好者提供专业的资源管理服务。', 1, 1, 1, CURRENT_TIMESTAMP);

-- 插入一些常见的禁用词
INSERT OR IGNORE INTO forbidden_words (word, created_at) VALUES 
('垃圾', datetime('now')),
('废物', datetime('now')),
('傻逼', datetime('now')),
('白痴', datetime('now')),
('蠢货', datetime('now'));

-- =============================================
-- 创建触发器以自动更新时间戳
-- =============================================

-- 用户表更新时间触发器
CREATE TRIGGER IF NOT EXISTS users_update_timestamp 
AFTER UPDATE ON users
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- 资源包表更新时间触发器
CREATE TRIGGER IF NOT EXISTS packages_update_timestamp 
AFTER UPDATE ON packages
BEGIN
    UPDATE packages SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- 评论表更新时间触发器
CREATE TRIGGER IF NOT EXISTS comments_update_timestamp 
AFTER UPDATE ON comments
BEGIN
    UPDATE comments SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- 分类表更新时间触发器
CREATE TRIGGER IF NOT EXISTS categories_update_timestamp 
AFTER UPDATE ON categories
BEGIN
    UPDATE categories SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- 公告表更新时间触发器
CREATE TRIGGER IF NOT EXISTS announcements_update_timestamp 
AFTER UPDATE ON announcements
BEGIN
    UPDATE announcements SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- =============================================
-- 数据完整性检查视图
-- =============================================

-- 创建用户统计视图
CREATE VIEW IF NOT EXISTS user_stats AS
SELECT 
    u.id,
    u.username,
    u.nickname,
    u.role,
    u.created_at,
    COUNT(DISTINCT p.id) as package_count,
    COUNT(DISTINCT c.id) as comment_count,
    COALESCE(SUM(p.download_count), 0) as total_downloads
FROM users u
LEFT JOIN packages p ON u.username = p.author OR u.id = p.reviewer_id
LEFT JOIN comments c ON u.id = c.user_id
GROUP BY u.id;

-- 创建包统计视图
CREATE VIEW IF NOT EXISTS package_stats AS
SELECT 
    p.*,
    cat.name as category_name,
    COUNT(DISTINCT c.id) as comment_count,
    reviewer.username as reviewer_name
FROM packages p
LEFT JOIN categories cat ON p.category_id = cat.id
LEFT JOIN comments c ON p.id = c.target_id AND c.target_type = 'Package'
LEFT JOIN users reviewer ON p.reviewer_id = reviewer.id
GROUP BY p.id;

-- =============================================
-- 脚本执行完成提示
-- =============================================

-- 插入初始化完成日志
INSERT INTO system_logs (level, message, details) VALUES 
('INFO', '数据库初始化完成', 'complete_database_init.sql 执行成功，所有表结构、索引、默认数据已创建完成');

-- 显示初始化信息
SELECT 
    '数据库初始化完成！' as status,
    datetime('now') as completion_time,
    '请使用以下默认账号登录：' as note,
    'admin / admin123' as default_account; 