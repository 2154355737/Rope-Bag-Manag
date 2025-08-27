-- 初始化主页配置设置
-- 如果不存在则插入默认的主页配置项

-- 首页标题和副标题
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('hero_title', '绳包管理器');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('hero_subtitle', '专业的资源管理与分享平台');

-- 主页显示模块配置（JSON数组格式）
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('homepage_sections', '["hero_section","stats_section","popular_tags","recent_resources","community_posts","announcements"]');

-- 分页配置
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('resources_per_page', '12');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('posts_per_page', '10');

-- 默认排序方式
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('default_sort', 'latest');

-- 页脚配置
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('copyright_text', '© 2024 绳包管理器. All rights reserved.');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('icp_number', '');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('footer_show_links', 'true');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('footer_show_stats', 'true');

-- SEO配置
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('seo_keywords', '绳包管理器,资源管理,文件分享,社区');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('seo_description', '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。');
INSERT OR IGNORE INTO system_settings (key, value) VALUES ('seo_author', '绳包管理器团队'); 