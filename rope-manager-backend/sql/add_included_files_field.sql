-- 为packages表添加包含文件字段
-- 执行时间：预期在几秒内完成

-- 添加包含文件字段（JSON格式存储文件信息数组）
ALTER TABLE packages ADD COLUMN included_files TEXT DEFAULT NULL; 