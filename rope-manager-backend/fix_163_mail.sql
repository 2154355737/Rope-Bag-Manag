-- 修复163邮箱配置
-- 163邮箱推荐使用587端口的STARTTLS模式，而不是25端口

UPDATE mail_settings 
SET 
    smtp_port = 587,  -- 使用587端口替代25端口
    use_ssl = 1,      -- 启用SSL/TLS
    enabled = 1       -- 确保已启用
WHERE id = 1;

-- 查看更新后的配置
SELECT 
    id,
    smtp_server,
    smtp_port,
    username,
    SUBSTR(password, 1, 3) || '***' as password_masked,
    from_name,
    enabled,
    use_ssl,
    auth_required
FROM mail_settings WHERE id = 1; 