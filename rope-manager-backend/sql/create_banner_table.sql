-- 创建轮播图表
CREATE TABLE IF NOT EXISTS banners (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    image_url TEXT NOT NULL,
    link_type TEXT NOT NULL,  -- "resource", "category", "url", "none"
    link_target TEXT,         -- 资源ID/分类ID/外部URL
    priority INTEGER NOT NULL DEFAULT 0,
    enabled BOOLEAN NOT NULL DEFAULT 1,
    start_time TEXT NOT NULL, -- ISO 8601 格式时间字符串
    end_time TEXT,           -- ISO 8601 格式时间字符串
    created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
    updated_at TEXT
); 