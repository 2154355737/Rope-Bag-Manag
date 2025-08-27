-- 为packages表添加前端发布页面需要的字段
-- 执行时间：预期在几秒内完成

-- 添加截图字段（JSON格式存储URL数组）
ALTER TABLE packages ADD COLUMN screenshots TEXT DEFAULT NULL;

-- 添加封面图片字段
ALTER TABLE packages ADD COLUMN cover_image TEXT DEFAULT NULL;

-- 添加系统要求字段（JSON格式存储要求数组）
ALTER TABLE packages ADD COLUMN requirements TEXT DEFAULT NULL;

-- 为posts表添加前端发布页面需要的字段
-- 检查posts表是否存在，如果存在则添加字段
-- 添加图片字段（JSON格式存储URL数组）
ALTER TABLE posts ADD COLUMN images TEXT DEFAULT NULL;

-- 添加代码片段字段
ALTER TABLE posts ADD COLUMN code_snippet TEXT DEFAULT NULL;

-- 记录迁移
INSERT INTO schema_migrations (version, applied_at) 
VALUES ('add_package_publish_fields', datetime('now'))
ON CONFLICT(version) DO NOTHING; 