-- 发布页面数据库结构检查和修复脚本
-- 手动执行或通过sqlite3命令行工具执行

-- 检查packages表现有结构
.schema packages

-- 为packages表添加发布页面需要的新字段（如果不存在）
-- 注意：SQLite的ALTER TABLE ADD COLUMN如果字段已存在会报错，所以需要先检查

-- 方法1：直接添加（如果字段不存在）
-- ALTER TABLE packages ADD COLUMN screenshots TEXT DEFAULT NULL;
-- ALTER TABLE packages ADD COLUMN cover_image TEXT DEFAULT NULL;  
-- ALTER TABLE packages ADD COLUMN requirements TEXT DEFAULT NULL;

-- 方法2：安全添加（检查后添加）
-- 由于SQLite限制，我们使用一个更安全的方法：

-- 1. 创建一个临时表用于检查
CREATE TEMP TABLE IF NOT EXISTS field_check (
    table_name TEXT,
    field_name TEXT,
    exists INTEGER DEFAULT 0
);

-- 2. 检查screenshots字段
INSERT INTO field_check (table_name, field_name, exists)
SELECT 'packages', 'screenshots', COUNT(*)
FROM pragma_table_info('packages') 
WHERE name = 'screenshots';

-- 3. 检查cover_image字段  
INSERT INTO field_check (table_name, field_name, exists)
SELECT 'packages', 'cover_image', COUNT(*)
FROM pragma_table_info('packages')
WHERE name = 'cover_image';

-- 4. 检查requirements字段
INSERT INTO field_check (table_name, field_name, exists)
SELECT 'packages', 'requirements', COUNT(*)
FROM pragma_table_info('packages')
WHERE name = 'requirements';

-- 显示检查结果
SELECT 'packages表字段检查结果:' as info;
SELECT field_name, 
       CASE WHEN exists > 0 THEN '✅ 已存在' ELSE '❌ 缺失' END as status
FROM field_check 
WHERE table_name = 'packages';

-- 手动添加缺失字段的SQL（请根据上面的检查结果手动执行）
SELECT '-- 请根据检查结果手动执行以下SQL:' as instruction;
SELECT '-- ALTER TABLE packages ADD COLUMN screenshots TEXT DEFAULT NULL;' as sql_1;
SELECT '-- ALTER TABLE packages ADD COLUMN cover_image TEXT DEFAULT NULL;' as sql_2;
SELECT '-- ALTER TABLE packages ADD COLUMN requirements TEXT DEFAULT NULL;' as sql_3;

-- 检查posts表（如果存在）
SELECT '检查posts表...' as info;

SELECT CASE 
    WHEN EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='posts') 
    THEN '✅ posts表存在'
    ELSE '❌ posts表不存在'
END as posts_table_status;

-- 如果posts表存在，检查字段
INSERT INTO field_check (table_name, field_name, exists)
SELECT 'posts', 'images', COUNT(*)
FROM pragma_table_info('posts') 
WHERE name = 'images'
AND EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='posts');

INSERT INTO field_check (table_name, field_name, exists)
SELECT 'posts', 'code_snippet', COUNT(*)
FROM pragma_table_info('posts')
WHERE name = 'code_snippet'
AND EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='posts');

INSERT INTO field_check (table_name, field_name, exists)
SELECT 'posts', 'tags', COUNT(*)
FROM pragma_table_info('posts')
WHERE name = 'tags'
AND EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='posts');

-- 显示posts表检查结果
SELECT 'posts表字段检查结果:' as info;
SELECT field_name,
       CASE WHEN exists > 0 THEN '✅ 已存在' ELSE '❌ 缺失' END as status
FROM field_check 
WHERE table_name = 'posts';

-- posts表缺失字段的SQL
SELECT '-- posts表缺失字段的SQL:' as instruction;
SELECT '-- ALTER TABLE posts ADD COLUMN images TEXT DEFAULT NULL;' as sql_1;
SELECT '-- ALTER TABLE posts ADD COLUMN code_snippet TEXT DEFAULT NULL;' as sql_2;
SELECT '-- ALTER TABLE posts ADD COLUMN tags TEXT DEFAULT NULL;' as sql_3;

-- 清理
DROP TABLE IF EXISTS field_check; 