/* 新增 package_tags 关联表，用于资源（package）与标签（tags）多对多关系 */
CREATE TABLE IF NOT EXISTS package_tags (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    package_id  INTEGER NOT NULL,
    tag_id      INTEGER NOT NULL,
    created_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    FOREIGN KEY(package_id) REFERENCES packages(id) ON DELETE CASCADE,
    FOREIGN KEY(tag_id)    REFERENCES tags(id)     ON DELETE CASCADE
);

/* 建立联合唯一索引，防止重复记录 */
CREATE UNIQUE INDEX IF NOT EXISTS idx_package_tag_unique ON package_tags(package_id, tag_id);

/* 为查询性能建立索引 */
CREATE INDEX IF NOT EXISTS idx_package_tags_package_id ON package_tags(package_id);
CREATE INDEX IF NOT EXISTS idx_package_tags_tag_id     ON package_tags(tag_id); 