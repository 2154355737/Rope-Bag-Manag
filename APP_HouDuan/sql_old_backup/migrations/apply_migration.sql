-- 发布页面数据库迁移脚本
-- 直接在SQLite中执行

-- 检查当前packages表结构
.schema packages

-- 添加新字段到packages表
ALTER TABLE packages ADD COLUMN screenshots TEXT DEFAULT NULL;
ALTER TABLE packages ADD COLUMN cover_image TEXT DEFAULT NULL;
ALTER TABLE packages ADD COLUMN requirements TEXT DEFAULT NULL;

-- 检查posts表是否存在并添加字段
.schema posts

-- 如果posts表存在，添加新字段
ALTER TABLE posts ADD COLUMN images TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN code_snippet TEXT DEFAULT NULL;
ALTER TABLE posts ADD COLUMN tags TEXT DEFAULT NULL;

-- 验证迁移结果
.schema packages
.schema posts

-- 显示完成信息
SELECT '✅ 数据库迁移完成！新字段已添加到packages和posts表。' as message; 