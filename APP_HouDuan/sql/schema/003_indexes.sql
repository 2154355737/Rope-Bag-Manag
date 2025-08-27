-- 绳包管理器数据库索引
-- 版本: 1.0.0
-- 说明: 提升查询性能的数据库索引

-- ========================================
-- 用户相关索引
-- ========================================

-- 用户表索引
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_is_active ON users(is_active);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

-- 邮件验证索引
CREATE INDEX IF NOT EXISTS idx_email_verifications_user_id ON email_verifications(user_id);
CREATE INDEX IF NOT EXISTS idx_email_verifications_email ON email_verifications(email);
CREATE INDEX IF NOT EXISTS idx_email_verifications_code ON email_verifications(code);

-- ========================================
-- 资源相关索引
-- ========================================

-- 分类表索引
CREATE INDEX IF NOT EXISTS idx_categories_name ON categories(name);
CREATE INDEX IF NOT EXISTS idx_categories_enabled ON categories(enabled);

-- 绳包表索引
CREATE INDEX IF NOT EXISTS idx_packages_name ON packages(name);
CREATE INDEX IF NOT EXISTS idx_packages_author ON packages(author);
CREATE INDEX IF NOT EXISTS idx_packages_category_id ON packages(category_id);
CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(status);
CREATE INDEX IF NOT EXISTS idx_packages_created_at ON packages(created_at);
CREATE INDEX IF NOT EXISTS idx_packages_download_count ON packages(download_count);
CREATE INDEX IF NOT EXISTS idx_packages_like_count ON packages(like_count);

-- 包标签索引
CREATE INDEX IF NOT EXISTS idx_package_tags_package_id ON package_tags(package_id);
CREATE INDEX IF NOT EXISTS idx_package_tags_tag_name ON package_tags(tag_name);

-- ========================================
-- 社区功能索引
-- ========================================

-- 帖子表索引
CREATE INDEX IF NOT EXISTS idx_posts_author_id ON posts(author_id);
CREATE INDEX IF NOT EXISTS idx_posts_category_id ON posts(category_id);
CREATE INDEX IF NOT EXISTS idx_posts_status ON posts(status);
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts(created_at);
CREATE INDEX IF NOT EXISTS idx_posts_is_pinned ON posts(is_pinned);
CREATE INDEX IF NOT EXISTS idx_posts_is_featured ON posts(is_featured);
CREATE INDEX IF NOT EXISTS idx_posts_view_count ON posts(view_count);

-- 评论表索引
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments(user_id);
CREATE INDEX IF NOT EXISTS idx_comments_target_type_target_id ON comments(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments(parent_id);
CREATE INDEX IF NOT EXISTS idx_comments_status ON comments(status);
CREATE INDEX IF NOT EXISTS idx_comments_created_at ON comments(created_at);
CREATE INDEX IF NOT EXISTS idx_comments_is_pinned ON comments(is_pinned);

-- 评论点赞/点踩索引
CREATE INDEX IF NOT EXISTS idx_comment_likes_comment_id ON comment_likes(comment_id);
CREATE INDEX IF NOT EXISTS idx_comment_likes_user_id ON comment_likes(user_id);
CREATE INDEX IF NOT EXISTS idx_comment_dislikes_comment_id ON comment_dislikes(comment_id);
CREATE INDEX IF NOT EXISTS idx_comment_dislikes_user_id ON comment_dislikes(user_id);

-- ========================================
-- 标签系统索引
-- ========================================

-- 标签表索引
CREATE INDEX IF NOT EXISTS idx_tags_name ON tags(name);
CREATE INDEX IF NOT EXISTS idx_tags_category ON tags(category);
CREATE INDEX IF NOT EXISTS idx_tags_usage_count ON tags(usage_count);
CREATE INDEX IF NOT EXISTS idx_tags_is_system ON tags(is_system);

-- 帖子标签索引
CREATE INDEX IF NOT EXISTS idx_post_tags_post_id ON post_tags(post_id);
CREATE INDEX IF NOT EXISTS idx_post_tags_tag_name ON post_tags(tag_name);

-- ========================================
-- 订阅和通知索引
-- ========================================

-- 订阅表索引
CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_category_id ON subscriptions(category_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_enabled ON subscriptions(enabled);

-- 通知表索引
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_type ON notifications(type);
CREATE INDEX IF NOT EXISTS idx_notifications_read_status ON notifications(read_status);
CREATE INDEX IF NOT EXISTS idx_notifications_created_at ON notifications(created_at);

-- ========================================
-- 系统相关索引
-- ========================================

-- 系统设置索引
CREATE INDEX IF NOT EXISTS idx_system_settings_key ON system_settings(key);

-- 系统日志索引
CREATE INDEX IF NOT EXISTS idx_system_logs_level ON system_logs(level);
CREATE INDEX IF NOT EXISTS idx_system_logs_timestamp ON system_logs(timestamp);

-- 用户行为日志索引
CREATE INDEX IF NOT EXISTS idx_user_actions_user_id ON user_actions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_actions_action_type ON user_actions(action_type);
CREATE INDEX IF NOT EXISTS idx_user_actions_target_type_id ON user_actions(target_type, target_id);
CREATE INDEX IF NOT EXISTS idx_user_actions_created_at ON user_actions(created_at);
CREATE INDEX IF NOT EXISTS idx_user_actions_ip_address ON user_actions(ip_address);

-- 资源记录索引
CREATE INDEX IF NOT EXISTS idx_resource_records_resource ON resource_records(resource_id, resource_type);
CREATE INDEX IF NOT EXISTS idx_resource_records_user_id ON resource_records(user_id);
CREATE INDEX IF NOT EXISTS idx_resource_records_action ON resource_records(action);
CREATE INDEX IF NOT EXISTS idx_resource_records_timestamp ON resource_records(timestamp);

-- ========================================
-- 安全相关索引
-- ========================================

-- IP封禁索引
CREATE INDEX IF NOT EXISTS idx_ip_bans_ip_address ON ip_bans(ip_address);
CREATE INDEX IF NOT EXISTS idx_ip_bans_expires_at ON ip_bans(expires_at);

-- 下载安全记录索引
CREATE INDEX IF NOT EXISTS idx_download_security_logs_user_id ON download_security_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_download_security_logs_ip_address ON download_security_logs(ip_address);
CREATE INDEX IF NOT EXISTS idx_download_security_logs_status ON download_security_logs(status);
CREATE INDEX IF NOT EXISTS idx_download_security_logs_timestamp ON download_security_logs(timestamp);

-- 安全动作索引
CREATE INDEX IF NOT EXISTS idx_security_actions_action_type ON security_actions(action_type);
CREATE INDEX IF NOT EXISTS idx_security_actions_user_id ON security_actions(user_id);
CREATE INDEX IF NOT EXISTS idx_security_actions_ip_address ON security_actions(ip_address);
CREATE INDEX IF NOT EXISTS idx_security_actions_created_at ON security_actions(created_at);
CREATE INDEX IF NOT EXISTS idx_security_actions_severity ON security_actions(severity);

-- ========================================
-- 内容管理索引
-- ========================================

-- 公告表索引
CREATE INDEX IF NOT EXISTS idx_announcements_priority ON announcements(priority);
CREATE INDEX IF NOT EXISTS idx_announcements_is_active ON announcements(is_active);
CREATE INDEX IF NOT EXISTS idx_announcements_created_at ON announcements(created_at);

-- 横幅表索引
CREATE INDEX IF NOT EXISTS idx_banners_position ON banners(position);
CREATE INDEX IF NOT EXISTS idx_banners_is_active ON banners(is_active);
CREATE INDEX IF NOT EXISTS idx_banners_display_order ON banners(display_order);

-- 首页设置索引
CREATE INDEX IF NOT EXISTS idx_homepage_settings_section_name ON homepage_settings(section_name);
CREATE INDEX IF NOT EXISTS idx_homepage_settings_is_active ON homepage_settings(is_active);
CREATE INDEX IF NOT EXISTS idx_homepage_settings_display_order ON homepage_settings(display_order);

-- ========================================
-- 邮件系统索引
-- ========================================

-- 邮件设置索引
CREATE INDEX IF NOT EXISTS idx_mail_settings_is_active ON mail_settings(is_active);

-- ========================================
-- 违禁词索引
-- ========================================

-- 违禁词索引
CREATE INDEX IF NOT EXISTS idx_forbidden_words_word ON forbidden_words(word);
CREATE INDEX IF NOT EXISTS idx_forbidden_words_severity ON forbidden_words(severity);
CREATE INDEX IF NOT EXISTS idx_forbidden_words_category ON forbidden_words(category);
CREATE INDEX IF NOT EXISTS idx_forbidden_words_is_active ON forbidden_words(is_active);