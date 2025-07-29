-- 为评论表添加置顶字段
ALTER TABLE comments ADD COLUMN pinned INTEGER NOT NULL DEFAULT 0;

-- 创建索引以优化置顶评论查询
CREATE INDEX IF NOT EXISTS idx_comments_pinned ON comments(pinned);
 
-- 创建复合索引优化排序查询
CREATE INDEX IF NOT EXISTS idx_comments_target_pinned_created ON comments(target_type, target_id, pinned DESC, created_at DESC); 