-- 启用邮件服务
UPDATE mail_settings SET enabled = 1 WHERE id = 1;

-- 查看更新后的配置
SELECT * FROM mail_settings WHERE id = 1; 