-- 创建邮件配置表
CREATE TABLE IF NOT EXISTS mail_settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    smtp_server TEXT NOT NULL DEFAULT 'smtp.qq.com',
    smtp_port INTEGER NOT NULL DEFAULT 465,
    username TEXT NOT NULL DEFAULT '',
    password TEXT NOT NULL DEFAULT '',
    from_name TEXT NOT NULL DEFAULT '绳包管理器',
    enabled INTEGER NOT NULL DEFAULT 0,  -- 是否启用邮件服务
    use_ssl INTEGER NOT NULL DEFAULT 1,  -- 是否使用SSL/TLS
    auth_required INTEGER NOT NULL DEFAULT 1,  -- 是否需要认证
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 创建邮件发送记录表
CREATE TABLE IF NOT EXISTS mail_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    to_email TEXT NOT NULL,
    subject TEXT NOT NULL,
    mail_type TEXT NOT NULL,  -- 'verification', 'reset_password', 'notification', 'test'
    status TEXT NOT NULL DEFAULT 'pending',  -- 'pending', 'sent', 'failed'
    error_message TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0,
    sent_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 创建邮件模板表
CREATE TABLE IF NOT EXISTS mail_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    template_type TEXT NOT NULL UNIQUE,  -- 'verification', 'reset_password', 'notification'
    subject TEXT NOT NULL,
    content TEXT NOT NULL,  -- 支持HTML内容
    variables TEXT,  -- JSON格式的可用变量说明
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 插入默认邮件配置
INSERT OR IGNORE INTO mail_settings (id, smtp_server, smtp_port, username, password, from_name, enabled)
VALUES (1, 'smtp.qq.com', 465, '', '', '绳包管理器', 0);

-- 插入默认邮件模板
INSERT OR IGNORE INTO mail_templates (template_type, subject, content, variables) VALUES 
('verification', '【绳包管理器】邮箱验证码', 
'<h2>验证码</h2><p>您的验证码是：<strong style="color: #409EFF; font-size: 24px;">{{code}}</strong></p><p>此验证码5分钟内有效，请及时使用。</p>', 
'{"code": "验证码"}'),

('reset_password', '【绳包管理器】密码重置', 
'<h2>密码重置</h2><p>您已请求重置密码，请点击以下链接重置：</p><p><a href="{{reset_link}}" style="color: #409EFF;">重置密码</a></p><p>如果您没有请求重置密码，请忽略此邮件。</p>', 
'{"reset_link": "重置密码链接"}'),

('notification', '【绳包管理器】新资源通知', 
'<h2>新资源发布</h2><p>有新的资源发布：</p><h3>{{resource_name}}</h3><p>{{resource_description}}</p><p><a href="{{resource_link}}" style="color: #409EFF;">查看详情</a></p>', 
'{"resource_name": "资源名称", "resource_description": "资源描述", "resource_link": "资源链接"}'),

('test', '【绳包管理器】测试邮件', 
'<h2>测试邮件</h2><p>这是一封测试邮件，如果您收到此邮件，说明邮件服务配置正确。</p><p>发送时间：{{send_time}}</p>', 
'{"send_time": "发送时间"}');

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_mail_logs_to_email ON mail_logs(to_email);
CREATE INDEX IF NOT EXISTS idx_mail_logs_status ON mail_logs(status);
CREATE INDEX IF NOT EXISTS idx_mail_logs_mail_type ON mail_logs(mail_type);
CREATE INDEX IF NOT EXISTS idx_mail_logs_created_at ON mail_logs(created_at); 