-- 修复用户行为记录表结构
-- 首先删除现有表（如果存在）
DROP TABLE IF EXISTS user_actions;

-- 重新创建用户行为记录表，支持可选的user_id
CREATE TABLE user_actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER,  -- 改为可选，支持访客用户
    action_type TEXT NOT NULL,
    target_type TEXT,
    target_id INTEGER,
    details TEXT,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- 创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_user_actions_user_id ON user_actions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_actions_created_at ON user_actions(created_at);
CREATE INDEX IF NOT EXISTS idx_user_actions_action_type ON user_actions(action_type);
CREATE INDEX IF NOT EXISTS idx_user_actions_target ON user_actions(target_type, target_id);

-- 插入一些测试数据
INSERT INTO user_actions (user_id, action_type, target_type, target_id, details, created_at) VALUES
(1, 'Login', 'User', 1, '管理员登录', datetime('now', '-1 hours')),
(NULL, 'PageView', 'Package', 1, '访客查看资源详情', datetime('now', '-30 minutes')),
(1, 'Upload', 'Package', 1, '管理员上传资源: 测试资源', datetime('now', '-15 minutes')),
(NULL, 'Download', 'Package', 1, '访客下载资源', datetime('now', '-5 minutes'));