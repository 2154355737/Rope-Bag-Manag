-- 为分类表添加订阅锁定字段
ALTER TABLE categories ADD COLUMN subscription_locked INTEGER NOT NULL DEFAULT 0;

-- 创建索引以优化锁定状态查询
CREATE INDEX IF NOT EXISTS idx_categories_subscription_locked ON categories(subscription_locked);
 
-- 添加注释说明
-- subscription_locked: 0 = 未锁定（用户可以订阅/取消订阅）, 1 = 已锁定（用户无法订阅/取消订阅） 