-- 163邮箱专用配置修复
-- 解决 "incomplete response" 错误

-- 方案1: 使用465端口 + SSL (推荐)
UPDATE mail_settings 
SET 
    smtp_server = 'smtp.163.com',
    smtp_port = 465,
    use_ssl = 1,
    auth_required = 1,
    enabled = 1
WHERE id = 1;

-- 如果465端口失败，可以手动改为以下配置：
-- UPDATE mail_settings SET smtp_port = 587, use_ssl = 1 WHERE id = 1;
-- 或者尝试：
-- UPDATE mail_settings SET smtp_port = 25, use_ssl = 0 WHERE id = 1;

SELECT 'Current 163 mail config:' as info;
SELECT 
    smtp_server,
    smtp_port,
    username,
    CASE 
        WHEN LENGTH(password) > 0 THEN SUBSTR(password, 1, 3) || '***' || SUBSTR(password, -2, 2)
        ELSE 'NO_PASSWORD'
    END as password_status,
    from_name,
    enabled,
    use_ssl,
    auth_required
FROM mail_settings WHERE id = 1; 