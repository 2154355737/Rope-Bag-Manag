-- 关注功能相关表结构
-- 版本: 1.0.0
-- 说明: 用户关注/被关注功能

-- ========================================
-- 用户关注关系表
-- ========================================

-- 用户关注表
CREATE TABLE IF NOT EXISTS user_follows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    follower_id INTEGER NOT NULL,        -- 关注者ID
    followed_id INTEGER NOT NULL,        -- 被关注者ID
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    -- 外键约束
    FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (followed_id) REFERENCES users(id) ON DELETE CASCADE,
    
    -- 唯一约束：同一用户不能重复关注同一个人
    UNIQUE(follower_id, followed_id),
    
    -- 检查约束：用户不能关注自己
    CHECK(follower_id != followed_id)
);

-- 索引优化
CREATE INDEX IF NOT EXISTS idx_user_follows_follower ON user_follows(follower_id);
CREATE INDEX IF NOT EXISTS idx_user_follows_followed ON user_follows(followed_id);
CREATE INDEX IF NOT EXISTS idx_user_follows_created_at ON user_follows(created_at);

-- ========================================
-- 关注统计表（可选，用于快速查询统计数据）
-- ========================================

-- 用户关注统计表
CREATE TABLE IF NOT EXISTS user_follow_stats (
    user_id INTEGER PRIMARY KEY,
    followers_count INTEGER DEFAULT 0,    -- 粉丝数
    following_count INTEGER DEFAULT 0,    -- 关注数
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    
    -- 外键约束
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- ========================================
-- 触发器：自动维护统计数据
-- ========================================

-- 插入关注关系时更新统计
CREATE TRIGGER IF NOT EXISTS update_follow_stats_on_insert
AFTER INSERT ON user_follows
BEGIN
    -- 更新被关注者的粉丝数
    INSERT OR REPLACE INTO user_follow_stats (user_id, followers_count, following_count, updated_at)
    VALUES (
        NEW.followed_id,
        COALESCE((SELECT followers_count FROM user_follow_stats WHERE user_id = NEW.followed_id), 0) + 1,
        COALESCE((SELECT following_count FROM user_follow_stats WHERE user_id = NEW.followed_id), 0),
        datetime('now')
    );
    
    -- 更新关注者的关注数
    INSERT OR REPLACE INTO user_follow_stats (user_id, followers_count, following_count, updated_at)
    VALUES (
        NEW.follower_id,
        COALESCE((SELECT followers_count FROM user_follow_stats WHERE user_id = NEW.follower_id), 0),
        COALESCE((SELECT following_count FROM user_follow_stats WHERE user_id = NEW.follower_id), 0) + 1,
        datetime('now')
    );
END;

-- 删除关注关系时更新统计
CREATE TRIGGER IF NOT EXISTS update_follow_stats_on_delete
AFTER DELETE ON user_follows
BEGIN
    -- 更新被关注者的粉丝数
    UPDATE user_follow_stats 
    SET followers_count = GREATEST(0, followers_count - 1),
        updated_at = datetime('now')
    WHERE user_id = OLD.followed_id;
    
    -- 更新关注者的关注数
    UPDATE user_follow_stats 
    SET following_count = GREATEST(0, following_count - 1),
        updated_at = datetime('now')
    WHERE user_id = OLD.follower_id;
END;

-- ========================================
-- 初始化现有用户的统计数据
-- ========================================

-- 为所有现有用户初始化关注统计
INSERT OR IGNORE INTO user_follow_stats (user_id, followers_count, following_count, updated_at)
SELECT 
    id,
    0,
    0,
    datetime('now')
FROM users;

-- 计算并更新实际的关注统计数据
UPDATE user_follow_stats 
SET followers_count = (
    SELECT COUNT(*) 
    FROM user_follows 
    WHERE followed_id = user_follow_stats.user_id
),
following_count = (
    SELECT COUNT(*) 
    FROM user_follows 
    WHERE follower_id = user_follow_stats.user_id
),
updated_at = datetime('now'); 