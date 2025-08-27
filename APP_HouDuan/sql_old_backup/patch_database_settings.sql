-- ========================================
-- 数据库修补脚本 - 配置管理系统
-- 版本: v1.1.0
-- 创建时间: 2024-01-XX
-- 描述: 修补system_settings表结构并添加新的配置项
-- ========================================

-- 开始事务
BEGIN TRANSACTION;

-- ========================================
-- 第一步：检查并修复system_settings表结构
-- ========================================

-- 创建临时表用于数据迁移（如果需要）
CREATE TABLE IF NOT EXISTS system_settings_temp (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 检查原表是否存在数据，如果存在则需要迁移数据
-- 先尝试从原表复制数据到临时表
INSERT OR IGNORE INTO system_settings_temp (key, value, description, created_at, updated_at)
SELECT 
    key, 
    value, 
    COALESCE(description, ''),
    COALESCE(updated_at, datetime('now')),  -- 使用existing updated_at作为created_at
    COALESCE(updated_at, datetime('now'))   -- 保持updated_at
FROM system_settings 
WHERE EXISTS (SELECT 1 FROM pragma_table_info('system_settings') WHERE name = 'key');

-- 删除原表（如果存在结构问题）
DROP TABLE IF EXISTS system_settings;

-- 创建正确结构的system_settings表
CREATE TABLE system_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 从临时表恢复数据
INSERT OR REPLACE INTO system_settings (key, value, description, created_at, updated_at)
SELECT key, value, description, created_at, updated_at
FROM system_settings_temp;

-- 删除临时表
DROP TABLE IF EXISTS system_settings_temp;

-- ========================================
-- 第二步：插入基础系统设置
-- ========================================

-- 主题设置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('primary_color', '#409EFF', '主要颜色'),
('secondary_color', '#67C23A', '次要颜色'),
('dark_mode', 'false', '深色模式'),
('font_size', '14px', '字体大小'),
('language', 'zh-CN', '系统语言');

-- 功能开关
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('enable_registration', 'true', '允许用户注册'),
('enable_community', 'true', '启用社区功能'),
('enable_upload', 'true', '允许文件上传'),
('enable_comments', 'true', '启用评论功能'),
('enable_qq_binding', 'true', '启用QQ绑定');

-- 系统模式
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('system_mode', 'Normal', '系统运行模式');

-- ========================================
-- 第三步：添加新的首页配置项
-- ========================================

-- 首页标题和副标题
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('hero_title', '绳包管理器', '首页主标题'),
('hero_subtitle', '专业的资源管理与分享平台', '首页副标题');

-- 主页显示模块配置（JSON数组格式）
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('homepage_sections', '["hero_section","stats_section","popular_tags","recent_resources","community_posts","announcements"]', '主页显示模块配置');

-- 分页配置
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('resources_per_page', '12', '每页显示资源数量'),
('posts_per_page', '10', '每页显示帖子数量');

-- 默认排序方式
INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('default_sort', 'latest', '默认排序方式');

-- ========================================
-- 第四步：添加页脚配置项
-- ========================================

INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('copyright_text', '© 2024 绳包管理器. All rights reserved.', '版权信息'),
('icp_number', '', 'ICP备案号'),
('footer_show_links', 'true', '页脚显示友情链接'),
('footer_show_stats', 'true', '页脚显示统计信息');

-- ========================================
-- 第五步：添加SEO配置项
-- ========================================

INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('seo_keywords', '绳包管理器,资源管理,文件分享,社区', 'SEO关键词'),
('seo_description', '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。', 'SEO描述'),
('seo_author', '绳包管理器团队', 'SEO作者信息');

-- ========================================
-- 第六步：添加社区设置项
-- ========================================

INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('site_title', '绳包管理器', '网站标题'),
('site_subtitle', '专业的资源管理与分享平台', '网站副标题'),
('site_description', '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。', '网站描述'),
('welcome_message', '欢迎来到绳包管理器！在这里您可以分享和发现各种有价值的资源。', '欢迎消息'),
('footer_text', '© 2024 绳包管理器. All rights reserved.', '页脚文本'),
('contact_email', 'contact@example.com', '联系邮箱');

-- ========================================
-- 第七步：添加后端配置项
-- ========================================

INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('proxy_address', '', '代理地址'),
('api_timeout', '30', 'API超时时间（秒）'),
('max_upload_size', '100', '最大上传文件大小（MB）');

-- ========================================
-- 第八步：添加备份配置项
-- ========================================

INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('enable_auto_backup', 'false', '启用自动备份'),
('backup_interval_hours', '24', '备份间隔时间（小时）'),
('backup_location', './backups', '备份文件存储路径'),
('max_backup_files', '10', '最大备份文件数量');

-- ========================================
-- 第九步：添加全局公告配置项
-- ========================================

INSERT OR IGNORE INTO system_settings (key, value, description) VALUES 
('global_announcement_enabled', 'false', '启用全局公告'),
('global_announcement_title', '', '全局公告标题'),
('global_announcement_content', '', '全局公告内容'),
('global_announcement_type', 'Info', '全局公告类型'),
('global_announcement_priority', '5', '全局公告优先级');

-- ========================================
-- 第十步：确保系统日志表存在
-- ========================================

CREATE TABLE IF NOT EXISTS system_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    details TEXT
);

-- 插入修补日志
INSERT INTO system_logs (level, message, details) VALUES 
('INFO', '数据库修补完成', '{"patch_version": "v1.1.0", "timestamp": "' || datetime('now') || '", "items_added": "配置管理系统设置项"}');

-- 提交事务
COMMIT;

-- ========================================
-- 修补完成提示
-- ========================================
-- 此脚本已完成以下操作：
-- 1. 修复了system_settings表结构
-- 2. 添加了完整的配置管理系统设置项
-- 3. 确保了数据的向后兼容性
-- 4. 添加了系统日志记录
-- ======================================== 