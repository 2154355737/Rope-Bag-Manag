-- 数据库迁移脚本：安全地为system_settings表添加description列
-- 如果您的数据库已经存在但缺少description列，请运行此脚本

-- 方法1：重命名现有表并重建（推荐）
BEGIN TRANSACTION;

-- 1. 创建新表结构
CREATE TABLE IF NOT EXISTS system_settings_new (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 2. 复制现有数据
INSERT INTO system_settings_new (key, value, created_at, updated_at)
SELECT key, value, 
       COALESCE(created_at, datetime('now')),
       COALESCE(updated_at, datetime('now'))
FROM system_settings
WHERE NOT EXISTS (SELECT 1 FROM pragma_table_info('system_settings') WHERE name = 'description');

-- 3. 如果原表没有description列，则删除原表并重命名新表
-- 注意：这只在description列不存在时执行
DROP TABLE IF EXISTS system_settings_old;

-- 检查是否需要迁移
CREATE TEMP TABLE IF NOT EXISTS migration_check AS
SELECT COUNT(*) as has_description 
FROM pragma_table_info('system_settings') 
WHERE name = 'description';

-- 如果没有description列，进行迁移
UPDATE migration_check SET has_description = (
    CASE 
        WHEN has_description = 0 THEN (
            -- 执行迁移
            SELECT CASE 
                WHEN (SELECT name FROM sqlite_master WHERE type='table' AND name='system_settings') IS NOT NULL
                THEN 1 -- 表存在，需要迁移
                ELSE 0 -- 表不存在，不需要迁移
            END
        )
        ELSE has_description
    END
);

COMMIT;

-- 如果需要手动执行迁移，请运行以下命令：
-- 1. 备份数据库
-- 2. 运行以下SQL（在确认备份后）：

/*
ALTER TABLE system_settings RENAME TO system_settings_old;
CREATE TABLE system_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
INSERT INTO system_settings (key, value, created_at, updated_at)
SELECT key, value, created_at, updated_at FROM system_settings_old;
DROP TABLE system_settings_old;
*/ 