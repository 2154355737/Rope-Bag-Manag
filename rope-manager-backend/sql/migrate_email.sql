-- 邮件验证码记录
CREATE TABLE IF NOT EXISTS email_verifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,
    email TEXT NOT NULL,
    code TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    used INTEGER DEFAULT 0
);

-- 资源订阅表
CREATE TABLE IF NOT EXISTS subscriptions (
    user_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    enabled INTEGER DEFAULT 1,
    PRIMARY KEY(user_id, category_id)
); 