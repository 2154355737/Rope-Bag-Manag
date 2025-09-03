-- 修复SQLite触发器中的GREATEST函数问题
-- SQLite不支持GREATEST函数，需要使用MAX函数

-- 删除现有的触发器
DROP TRIGGER IF EXISTS update_follow_stats_on_insert;
DROP TRIGGER IF EXISTS update_follow_stats_on_delete;

-- 重新创建插入触发器
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

-- 重新创建删除触发器（使用MAX函数）
CREATE TRIGGER IF NOT EXISTS update_follow_stats_on_delete
AFTER DELETE ON user_follows
BEGIN
    -- 更新被关注者的粉丝数
    UPDATE user_follow_stats 
    SET followers_count = MAX(0, followers_count - 1),
        updated_at = datetime('now')
    WHERE user_id = OLD.followed_id;
    
    -- 更新关注者的关注数
    UPDATE user_follow_stats 
    SET following_count = MAX(0, following_count - 1),
        updated_at = datetime('now')
    WHERE user_id = OLD.follower_id;
END; 