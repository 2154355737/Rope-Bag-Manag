-- 为packages表添加审核相关字段
-- 添加审核相关字段
ALTER TABLE packages ADD COLUMN reviewer_id INTEGER DEFAULT NULL;
ALTER TABLE packages ADD COLUMN reviewed_at DATETIME DEFAULT NULL;
ALTER TABLE packages ADD COLUMN review_comment TEXT DEFAULT NULL;

-- 更新现有数据：将状态为'active'的改为'pending'，让管理员重新审核
UPDATE packages SET status = 'pending' WHERE status = 'active';

-- 添加外键约束（如果需要的话）
-- ALTER TABLE packages ADD CONSTRAINT fk_packages_reviewer 
-- FOREIGN KEY (reviewer_id) REFERENCES users(id);

-- 创建审核状态索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(status);
CREATE INDEX IF NOT EXISTS idx_packages_reviewer ON packages(reviewer_id); 