-- 绳包管理器初始数据
-- 版本: 1.0.0
-- 说明: 系统启动必需的默认数据

-- ========================================
-- 系统设置默认值
-- ========================================

-- 网站基本信息
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('site_name', '绳包管理器', '网站名称'),
('site_description', '专业的绳结资源管理平台', '网站描述'),
('site_keywords', '绳结,绳包,资源管理,社区', '网站关键词'),
('site_logo', '', '网站Logo地址'),
('site_favicon', '', '网站图标地址');

-- 功能开关
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('enable_registration', 'true', '允许用户注册'),
('enable_community', 'true', '启用社区功能'),
('enable_upload', 'true', '允许文件上传'),
('enable_comments', 'true', '启用评论功能'),
('enable_notifications', 'true', '启用通知功能'),
('comment_approval_required', 'false', '评论是否需要审核'),
('post_approval_required', 'false', '帖子是否需要审核');

-- 主题配置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('primary_color', '#409EFF', '主要颜色'),
('secondary_color', '#67C23A', '次要颜色'),
('success_color', '#67C23A', '成功颜色'),
('warning_color', '#E6A23C', '警告颜色'),
('danger_color', '#F56C6C', '危险颜色'),
('info_color', '#909399', '信息颜色'),
('dark_mode', 'false', '深色模式'),
('font_size', '14px', '字体大小'),
('language', 'zh-CN', '系统语言');

-- 文件上传配置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('max_file_size', '10485760', '最大文件上传大小（字节）'),
('allowed_file_types', 'zip,rar,7z,tar,gz', '允许的文件类型'),
('upload_path', 'uploads', '上传文件路径'),
('temp_path', 'temp', '临时文件路径');

-- 安全配置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('max_login_attempts', '5', '最大登录尝试次数'),
('ban_duration', '3600', '封禁时长（秒）'),
('session_timeout', '86400', '会话超时时间（秒）'),
('jwt_expire_time', '86400', 'JWT过期时间（秒）'),
('password_min_length', '6', '密码最小长度'),
('enable_captcha', 'false', '启用验证码');

-- 邮件配置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('mail_enabled', 'false', '启用邮件功能'),
('mail_from_name', '绳包管理器', '邮件发送方名称'),
('smtp_server', '', 'SMTP服务器地址'),
('smtp_port', '587', 'SMTP端口'),
('smtp_username', '', 'SMTP用户名'),
('smtp_password', '', 'SMTP密码'),
('smtp_encryption', 'tls', 'SMTP加密方式');

-- 系统运行模式
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('system_mode', 'Normal', '系统运行模式'),
('maintenance_mode', 'false', '维护模式'),
('maintenance_message', '系统正在维护中，请稍后访问', '维护提示信息'),
('debug_mode', 'false', '调试模式');

-- 分页配置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('default_page_size', '20', '默认分页大小'),
('max_page_size', '100', '最大分页大小'),
('search_result_limit', '1000', '搜索结果限制');

-- ========================================
-- 默认分类
-- ========================================

-- 插入默认分类（仅在不存在时）
INSERT INTO categories (id, name, description, enabled) 
SELECT 7, '资源分享', '用户分享的各类资源', 1
WHERE NOT EXISTS (SELECT 1 FROM categories WHERE id = 7)
UNION ALL
SELECT 8, '其他', '其他未分类的内容', 1
WHERE NOT EXISTS (SELECT 1 FROM categories WHERE id = 8);

-- ========================================
-- 默认标签
-- ========================================

-- 插入默认标签
INSERT OR IGNORE INTO tags (name, description, color, category, is_system) VALUES 
('新手入门', '适合新手学习的内容', '#67C23A', 'level', 1),
('进阶技巧', '需要一定基础的进阶内容', '#E6A23C', 'level', 1),
('专家级', '高难度专业内容', '#F56C6C', 'level', 1),
('热门', '热门推荐内容', '#409EFF', 'status', 1),
('精品', '精品优质内容', '#AB70FF', 'status', 1),
('原创', '原创内容', '#36CFC9', 'source', 1),
('转载', '转载分享内容', '#FAAD14', 'source', 1),
('视频', '视频类型内容', '#FF7875', 'type', 1),
('图文', '图文类型内容', '#73D13D', 'type', 1),
('工具', '工具相关内容', '#40A9FF', 'type', 1);

-- ========================================
-- 默认管理员用户
-- ========================================

-- 注意: 管理员用户已在主初始化脚本中创建，此处跳过以避免重复

-- ========================================
-- 默认公告
-- ========================================

-- 插入欢迎公告
INSERT OR IGNORE INTO announcements (title, content, priority, is_active) VALUES 
('欢迎使用绳包管理器', 
'欢迎来到绳包管理器！这里是专业的绳结资源管理和交流平台。您可以：

1. 浏览和下载各类绳结资源
2. 分享您的绳结作品和经验
3. 参与社区讨论和交流
4. 学习绳结技巧和知识

如有任何问题，请联系管理员。祝您使用愉快！', 
1, 1);

-- ========================================
-- 默认首页设置
-- ========================================

-- 插入默认首页设置
INSERT OR IGNORE INTO homepage_settings (section_name, content, is_active, display_order) VALUES 
('hero_title', '绳包管理器', 1, 1),
('hero_subtitle', '专业的绳结资源管理平台', 1, 2),
('hero_description', '发现、学习、分享绳结技艺，与全球绳艺爱好者交流互动', 1, 3),
('features_title', '平台特色', 1, 4),
('contact_title', '联系我们', 1, 5),
('footer_text', '© 2024 绳包管理器. All rights reserved.', 1, 6);

-- ========================================
-- 默认违禁词（示例）
-- ========================================

-- 插入一些基础的违禁词示例
INSERT OR IGNORE INTO forbidden_words (word, replacement, severity, category, is_active) VALUES 
('垃圾', '***', 'low', 'common', 1),
('废物', '***', 'low', 'common', 1),
('SB', '**', 'medium', 'profanity', 1),
('傻逼', '**', 'medium', 'profanity', 1);

-- ========================================
-- 完成提示
-- ========================================

-- 记录初始化完成时间
INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('database_initialized_at', datetime('now'), '数据库初始化时间'),
('database_version', '1.0.0', '数据库版本号'),
('last_migration', '001_default_data', '最后执行的迁移');