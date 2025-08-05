-- 修复packages表，添加缺失的字段

-- 添加审核相关字段（如果不存在）
ALTER TABLE packages ADD COLUMN reviewer_id INTEGER DEFAULT NULL;
ALTER TABLE packages ADD COLUMN reviewed_at DATETIME DEFAULT NULL;
ALTER TABLE packages ADD COLUMN review_comment TEXT DEFAULT NULL;

-- 添加置顶和精华字段（如果不存在）
ALTER TABLE packages ADD COLUMN is_pinned BOOLEAN DEFAULT 0;
ALTER TABLE packages ADD COLUMN is_featured BOOLEAN DEFAULT 0;

-- 创建package_tags关联表（如果不存在）
CREATE TABLE IF NOT EXISTS package_tags (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    package_id  INTEGER NOT NULL,
    tag_id      INTEGER NOT NULL,
    created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY(package_id) REFERENCES packages(id) ON DELETE CASCADE,
    FOREIGN KEY(tag_id)    REFERENCES tags(id)     ON DELETE CASCADE
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_packages_status ON packages(status);
CREATE INDEX IF NOT EXISTS idx_packages_reviewer ON packages(reviewer_id);
CREATE INDEX IF NOT EXISTS idx_packages_is_pinned ON packages(is_pinned);
CREATE INDEX IF NOT EXISTS idx_packages_is_featured ON packages(is_featured);

-- 为package_tags表建立索引
CREATE UNIQUE INDEX IF NOT EXISTS idx_package_tag_unique ON package_tags(package_id, tag_id);
CREATE INDEX IF NOT EXISTS idx_package_tags_package_id ON package_tags(package_id);
CREATE INDEX IF NOT EXISTS idx_package_tags_tag_id ON package_tags(tag_id);