use rusqlite::{Connection, Result as SqliteResult, params};
use crate::models::{Tag, CreateTagRequest, UpdateTagRequest, TagQueryParams, TagListResponse};
use chrono::{DateTime, Utc};

// 辅助函数，解析RFC3339时间戳，若失败则返回当前时间，避免panic
fn parse_timestamp(ts: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&ts)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

pub struct TagService {
    db_path: String,
}

impl TagService {
    pub fn new(db_path: String) -> Self {
        Self { db_path }
    }

    // 创建标签
    pub async fn create_tag(&self, req: CreateTagRequest) -> SqliteResult<i32> {
        let conn = Connection::open(&self.db_path)?;
        
        // 先检查标签名是否已存在
        let existing_tag = conn.query_row(
            "SELECT id FROM tags WHERE name = ?",
            params![req.name],
            |row| row.get::<_, i32>(0)
        );
        
        if existing_tag.is_ok() {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some(format!("标签名称 '{}' 已存在", req.name))
            ));
        }
        
        let now_str = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tags (name, description, color, use_count, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
            params![
                req.name,
                req.description,
                req.color,
                0, // use_count
                now_str,
                now_str
            ]
        )?;

        let tag_id = conn.last_insert_rowid() as i32;

        Ok(tag_id)
    }

    // 更新标签
    pub async fn update_tag(&self, tag_id: i32, req: UpdateTagRequest) -> SqliteResult<bool> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(name) = req.name {
            updates.push("name = ?");
            params.push(Box::new(name));
        }

        if let Some(description) = req.description {
            updates.push("description = ?");
            params.push(Box::new(description));
        }

        if let Some(color) = req.color {
            updates.push("color = ?");
            params.push(Box::new(color));
        }

        if !updates.is_empty() {
            updates.push("updated_at = ?");
            params.push(Box::new(Utc::now().to_rfc3339()));

            let sql = format!("UPDATE tags SET {} WHERE id = ?", updates.join(", "));
            params.push(Box::new(tag_id));

            let mut stmt = conn.prepare(&sql)?;
            stmt.execute(rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())))?;
        }

        Ok(true)
    }

    // 获取标签列表
    pub async fn get_tags(&self, query: TagQueryParams) -> SqliteResult<TagListResponse> {
        let conn = Connection::open(&self.db_path)?;
        
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(50);
        let offset = (page - 1) * page_size;

        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(search) = query.search {
            conditions.push("(name LIKE ? OR description LIKE ?)");
            let search_pattern = format!("%{}%", search);
            params.push(Box::new(search_pattern.clone()));
            params.push(Box::new(search_pattern));
        }

        let where_clause = if conditions.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 确定排序
        let sort_by = query.sort_by.unwrap_or_else(|| "use_count".to_string());
        let sort_order = query.sort_order.unwrap_or_else(|| "desc".to_string());
        let order_clause = format!("ORDER BY {} {}", sort_by, sort_order.to_uppercase());

        // 获取总数
        let count_sql = format!("SELECT COUNT(*) FROM tags {}", where_clause);
        let total: i64 = if params.is_empty() {
            conn.query_row(&count_sql, [], |row| row.get(0))?
        } else {
            let mut stmt = conn.prepare(&count_sql)?;
            stmt.query_row(rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())), |row| row.get(0))?
        };

        // 获取标签列表
        let sql = format!(
            "SELECT id, name, description, color, use_count, created_at, updated_at FROM tags {} {} LIMIT ? OFFSET ?",
            where_clause, order_clause
        );

        let mut stmt = conn.prepare(&sql)?;
        let mut params_with_limit = params;
        params_with_limit.push(Box::new(page_size as i32));
        params_with_limit.push(Box::new(offset as i32));

        let tags = stmt.query_map(
            rusqlite::params_from_iter(params_with_limit.iter().map(|p| p.as_ref())),
            |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    color: row.get(3)?,
                    use_count: row.get(4)?,
                    created_at: parse_timestamp(row.get::<_, String>(5)?),
                    updated_at: parse_timestamp(row.get::<_, String>(6)?),
                })
            }
        )?.collect::<Result<Vec<_>, _>>()?;

        Ok(TagListResponse {
            list: tags,
            total,
            page: page as i32,
            size: page_size as i32,
        })
    }

    // 获取单个标签
    pub async fn get_tag(&self, tag_id: i32) -> SqliteResult<Option<Tag>> {
        let conn = Connection::open(&self.db_path)?;
        
        let sql = "SELECT id, name, description, color, use_count, created_at, updated_at FROM tags WHERE id = ?";
        
        let result = conn.query_row(sql, params![tag_id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                use_count: row.get(4)?,
                created_at: parse_timestamp(row.get::<_, String>(5)?),
                updated_at: parse_timestamp(row.get::<_, String>(6)?),
            })
        });

        match result {
            Ok(tag) => Ok(Some(tag)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // 根据名称获取标签
    pub async fn get_tag_by_name(&self, name: &str) -> SqliteResult<Option<Tag>> {
        let conn = Connection::open(&self.db_path)?;
        
        let sql = "SELECT id, name, description, color, use_count, created_at, updated_at FROM tags WHERE name = ?";
        
        let result = conn.query_row(sql, params![name], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                color: row.get(3)?,
                use_count: row.get(4)?,
                created_at: parse_timestamp(row.get::<_, String>(5)?),
                updated_at: parse_timestamp(row.get::<_, String>(6)?),
            })
        });

        match result {
            Ok(tag) => Ok(Some(tag)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // 删除标签
    pub async fn delete_tag(&self, tag_id: i32) -> SqliteResult<bool> {
        let conn = Connection::open(&self.db_path)?;
        
        // 先删除标签关联
        conn.execute("DELETE FROM post_tags WHERE tag_id = ?", params![tag_id])?;
        
        // 删除标签
        let result = conn.execute("DELETE FROM tags WHERE id = ?", params![tag_id])?;
        Ok(result > 0)
    }

    // 增加标签使用次数
    pub async fn increment_use_count(&self, tag_id: i32) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("UPDATE tags SET use_count = use_count + 1 WHERE id = ?", params![tag_id])?;
        Ok(())
    }

    // 获取热门标签
    pub async fn get_popular_tags(&self, limit: Option<i32>) -> SqliteResult<Vec<Tag>> {
        let conn = Connection::open(&self.db_path)?;
        
        let limit = limit.unwrap_or(10);
        // 修改查询：如果没有使用次数大于0的标签，则返回所有标签
        let sql = "SELECT id, name, description, color, use_count, created_at, updated_at FROM tags ORDER BY use_count DESC, created_at DESC LIMIT ?";
        
        let tags = conn.prepare(sql)?
            .query_map(params![limit], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    color: row.get(3)?,
                    use_count: row.get(4)?,
                    created_at: parse_timestamp(row.get::<_, String>(5)?),
                    updated_at: parse_timestamp(row.get::<_, String>(6)?),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(tags)
    }

    // 获取所有标签（用于下拉选择）
    pub async fn get_all_tags(&self) -> SqliteResult<Vec<Tag>> {
        let conn = Connection::open(&self.db_path)?;
        
        let sql = "SELECT id, name, description, color, use_count, created_at, updated_at FROM tags ORDER BY name";
        
        let tags = conn.prepare(sql)?
            .query_map([], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    color: row.get(3)?,
                    use_count: row.get(4)?,
                    created_at: parse_timestamp(row.get::<_, String>(5)?),
                    updated_at: parse_timestamp(row.get::<_, String>(6)?),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(tags)
    }

    // 更新所有标签的使用次数 - 基于真实关联数据统计
    pub async fn update_all_tag_counts(&self) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        
        // 检查 package_tags 表是否存在
        let package_tags_exists = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='package_tags'")
            .and_then(|mut stmt| stmt.exists([]))
            .unwrap_or(false);
        
        // 根据表是否存在来构建不同的SQL
        let sql = if package_tags_exists {
            "UPDATE tags 
             SET use_count = (
                 COALESCE((
                     SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id
                 ), 0) + 
                 COALESCE((
                     SELECT COUNT(*) FROM package_tags WHERE tag_id = tags.id
                 ), 0)
             ),
             updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')"
        } else {
            "UPDATE tags 
             SET use_count = (
                 COALESCE((
                     SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id
                 ), 0)
             ),
             updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')"
        };
        
        conn.execute(sql, [])?;
        log::info!("已更新所有标签的使用次数");
        
        Ok(())
    }

    // 获取标签使用统计信息
    pub async fn get_tag_usage_stats(&self) -> SqliteResult<TagUsageStats> {
        let conn = Connection::open(&self.db_path)?;
        
        // 检查 package_tags 表是否存在
        let package_tags_exists = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='package_tags'")
            .and_then(|mut stmt| stmt.exists([]))
            .unwrap_or(false);
        
        // 统计总体信息
        let total_tags: i32 = conn.query_row("SELECT COUNT(*) FROM tags", [], |row| row.get(0))?;
        let used_tags: i32 = conn.query_row("SELECT COUNT(*) FROM tags WHERE use_count > 0", [], |row| row.get(0))?;
        let total_usage: i32 = conn.query_row("SELECT SUM(use_count) FROM tags", [], |row| row.get(0)).unwrap_or(0);
        
        // 统计帖子标签使用情况
        let post_tag_usage: i32 = conn.query_row("SELECT COUNT(*) FROM post_tags", [], |row| row.get(0))?;
        
        // 统计资源包标签使用情况（如果表存在）
        let package_tag_usage: i32 = if package_tags_exists {
            conn.query_row("SELECT COUNT(*) FROM package_tags", [], |row| row.get(0)).unwrap_or(0)
        } else {
            0
        };
        
        Ok(TagUsageStats {
            total_tags,
            used_tags,
            unused_tags: total_tags - used_tags,
            total_usage,
            post_tag_usage,
            package_tag_usage,
        })
    }
}

// 标签使用统计信息
#[derive(Debug, serde::Serialize)]
pub struct TagUsageStats {
    pub total_tags: i32,        // 总标签数
    pub used_tags: i32,         // 已使用标签数
    pub unused_tags: i32,       // 未使用标签数
    pub total_usage: i32,       // 总使用次数
    pub post_tag_usage: i32,    // 帖子标签使用次数
    pub package_tag_usage: i32, // 资源包标签使用次数
} 