use rusqlite::{Connection, Result as SqliteResult, params};
use crate::models::{Post, CreatePostRequest, UpdatePostRequest, PostQueryParams, PostListResponse, Tag};
use crate::repositories::user_repo::UserRepository;
use chrono::{DateTime, Utc};
use crate::services::notification_service::NotificationService;
use serde_json;

pub struct PostService {
    db_path: String,
    notifier: Option<NotificationService>,
}

impl PostService {
    pub fn new(db_path: String) -> Self {
        Self { db_path, notifier: None }
    }

    pub fn db_path(&self) -> &str { &self.db_path }

    pub fn with_notifier(mut self, notifier: NotificationService) -> Self {
        self.notifier = Some(notifier);
        self
    }

    // 创建帖子
    pub async fn create_post(&self, req: CreatePostRequest, author_id: i32) -> SqliteResult<i32> {
        let conn = Connection::open(&self.db_path)?;
        
        // 重复提交保护：60秒内相同作者与标题，直接返回已有帖子ID
        if let Ok(existing_id) = conn.query_row(
            "SELECT id FROM posts WHERE author_id = ? AND title = ? AND julianday('now') - julianday(created_at) < (60.0/86400.0) ORDER BY id DESC LIMIT 1",
            params![author_id, req.title],
            |r| r.get::<_, i32>(0),
        ) {
            return Ok(existing_id);
        }
        
        // 获取作者信息
        let user_repo = UserRepository::new(&self.db_path).map_err(|e| rusqlite::Error::InvalidPath(std::path::PathBuf::from(e.to_string())))?;
        let author = user_repo.find_by_id(author_id).await.map_err(|e| rusqlite::Error::InvalidPath(std::path::PathBuf::from(e.to_string())))?;
        
        let author_name = match author {
            Some(user) => user.nickname.unwrap_or(user.username),
            None => return Err(rusqlite::Error::InvalidPath(std::path::PathBuf::from("User not found"))),
        };
        
        let now = Utc::now();
        let status = req.status.unwrap_or_default();
        
        conn.execute(
            "INSERT INTO posts (title, content, author_id, author_name, category_id, status, view_count, like_count, comment_count, is_pinned, is_featured, created_at, updated_at, review_status, images, code_snippet, tags) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                req.title,
                req.content,
                author_id,
                author_name,
                req.category_id,
                format!("{:?}", status),
                0, // view_count
                0, // like_count
                0, // comment_count
                false, // is_pinned
                false, // is_featured
                now,
                now,
                "pending",
                req.images.as_ref().map(|v| serde_json::to_string(v).unwrap_or("[]".to_string())).unwrap_or("[]".to_string()),
                req.code_snippet,
                req.tags.as_ref().map(|v| serde_json::to_string(v).unwrap_or("[]".to_string())).unwrap_or("[]".to_string()),
            ]
        )?;
        let post_id = conn.last_insert_rowid() as i32;

        // 处理标签
        if let Some(tags) = req.tags {
            self.add_tags_to_post(post_id, &tags).await?;
        }
        
        Ok(post_id)
    }

    // 更新帖子
    pub async fn update_post(&self, post_id: i32, req: UpdatePostRequest) -> SqliteResult<bool> {
        let conn = Connection::open(&self.db_path)?;
        
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        // 记录置顶/精华变更以便稍后发通知
        let mut pin_changed: Option<bool> = None;
        let mut feat_changed: Option<bool> = None;
        // 读取现状
        let (old_pinned, old_featured): (bool, bool) = {
            let row: (bool, bool) = conn.query_row("SELECT is_pinned, is_featured FROM posts WHERE id = ?", params![post_id], |r| Ok((r.get(0)?, r.get(1)?)))?;
            row
        };

        if let Some(title) = req.title {
            updates.push("title = ?");
            params.push(Box::new(title));
        }

        if let Some(content) = req.content {
            updates.push("content = ?");
            params.push(Box::new(content));
        }

        if let Some(images) = req.images {
            let json = serde_json::to_string(&images).unwrap_or("[]".to_string());
            updates.push("images = ?");
            params.push(Box::new(json));
        }

        if let Some(code_snippet) = req.code_snippet {
            updates.push("code_snippet = ?");
            params.push(Box::new(code_snippet));
        }

        if let Some(category_id) = req.category_id {
            updates.push("category_id = ?");
            params.push(Box::new(category_id));
        }

        if let Some(status) = req.status {
            // 同步业务状态，并根据业务状态调整审核状态
            let status_str = format!("{:?}", status);
            updates.push("status = ?");
            params.push(Box::new(status_str.clone()));
            // Published => approved；Draft => pending；其他不改
            if status_str == "Published" {
                updates.push("review_status = ?");
                params.push(Box::new("approved".to_string()));
            } else if status_str == "Draft" {
                updates.push("review_status = ?");
                params.push(Box::new("pending".to_string()));
            }
        }

        if let Some(is_pinned) = req.is_pinned {
            updates.push("is_pinned = ?");
            params.push(Box::new(is_pinned));
            pin_changed = Some(is_pinned != old_pinned);
        }

        if let Some(is_featured) = req.is_featured {
            updates.push("is_featured = ?");
            params.push(Box::new(is_featured));
            feat_changed = Some(is_featured != old_featured);
        }

        if !updates.is_empty() {
            updates.push("updated_at = ?");
            params.push(Box::new(Utc::now()));

            let sql = format!("UPDATE posts SET {} WHERE id = ?", updates.join(", "));
            params.push(Box::new(post_id));
            let mut stmt = conn.prepare(&sql)?;
            stmt.execute(rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())))?;
        }

        // 处理标签更新（列 + 关联表）
        if let Some(tags) = req.tags {
            let json = serde_json::to_string(&tags).unwrap_or("[]".to_string());
            let mut stmt2 = conn.prepare("UPDATE posts SET tags = ? WHERE id = ?")?;
            stmt2.execute(params![json, post_id])?;
            self.update_post_tags(post_id, &tags).await?;
        }

        // 通知作者：被置顶/加精
        if pin_changed == Some(true) || feat_changed == Some(true) {
            if let Some(notify) = &self.notifier {
                let user_repo = UserRepository::new(&self.db_path).map_err(|e| rusqlite::Error::InvalidPath(std::path::PathBuf::from(e.to_string())))?;
                // 查作者ID
                let (author_id, title): (i32, String) = conn.query_row(
                    "SELECT author_id, title FROM posts WHERE id = ?",
                    params![post_id],
                    |r| Ok((r.get(0)?, r.get(1)?))
                )?;
                let link = format!("/post/{}", post_id);
                let mut parts = Vec::new();
                if pin_changed == Some(true) { parts.push("置顶"); }
                if feat_changed == Some(true) { parts.push("加精"); }
                let msg = format!("您的帖子《{}》已被{}", title, parts.join("、"));
                let _ = notify.notify(author_id, "帖子状态更新", &msg, Some(&link), Some("PostFlagChanged"), Some("Post"), Some(post_id)).await;
            }
        }

        Ok(true)
    }

    // 新增：检查用户是否点赞了指定帖子
    pub async fn is_post_liked_by_user(&self, user_id: i32, post_id: i32) -> SqliteResult<bool> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS post_likes (user_id INTEGER NOT NULL, post_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, post_id))",
            [],
        )?;
        let exists: Result<i32, _> = conn.query_row(
            "SELECT 1 FROM post_likes WHERE user_id = ? AND post_id = ? LIMIT 1",
            params![user_id, post_id],
            |r| r.get(0),
        );
        Ok(exists.is_ok())
    }

    // 获取帖子列表
    pub async fn get_posts(&self, query: PostQueryParams) -> SqliteResult<PostListResponse> {
        let conn = Connection::open(&self.db_path)?;
        
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);
        let offset = (page - 1) * page_size;

        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(category_id) = query.category_id {
            conditions.push("category_id = ?");
            params.push(Box::new(category_id));
        }

        if let Some(author_id) = query.author_id {
            conditions.push("author_id = ?");
            params.push(Box::new(author_id));
        }

        if let Some(status) = query.status {
            // 检查是否是审核状态查询
            if status == "pending" || status == "approved" || status == "rejected" {
                conditions.push("review_status = ?");
                params.push(Box::new(status));
            } else {
                conditions.push("status = ?");
                params.push(Box::new(status));
            }
        }

        // 公开查询时如果未传 review_status 则默认只显示 approved
        // 这里通过 query.status 的语义保留原有逻辑；审核过滤在 API 层控制

        if let Some(search) = query.search {
            conditions.push("(title LIKE ? OR content LIKE ?)");
            let search_pattern = format!("%{}%", search);
            params.push(Box::new(search_pattern.clone()));
            params.push(Box::new(search_pattern));
        }

        if let Some(is_pinned) = query.is_pinned {
            conditions.push("is_pinned = ?");
            params.push(Box::new(is_pinned));
        }

        if let Some(is_featured) = query.is_featured {
            conditions.push("is_featured = ?");
            params.push(Box::new(is_featured));
        }

        let where_clause = if conditions.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 获取总数
        let count_sql = format!("SELECT COUNT(*) FROM posts {}", where_clause);
        let total: i64 = if params.is_empty() {
            conn.query_row(&count_sql, [], |row| row.get(0))?
        } else {
            let mut stmt = conn.prepare(&count_sql)?;
            stmt.query_row(rusqlite::params_from_iter(params.iter().map(|p| p.as_ref())), |row| row.get(0))?
        };

        // 获取帖子列表（包含审核字段）
        let sql = format!(
            "SELECT id, title, content, author_id, author_name, category_id, status, view_count, like_count, comment_count, is_pinned, is_featured, created_at, updated_at, review_status, review_comment, reviewer_id, reviewed_at, images, code_snippet, tags FROM posts {} ORDER BY is_pinned DESC, is_featured DESC, created_at DESC LIMIT ? OFFSET ?",
            where_clause
        );

        let mut stmt = conn.prepare(&sql)?;
        let mut params_with_limit = params;
        params_with_limit.push(Box::new(page_size as i32));
        params_with_limit.push(Box::new(offset as i32));

        let posts = stmt.query_map(
            rusqlite::params_from_iter(params_with_limit.iter().map(|p| p.as_ref())),
            |row| {
                Ok(Post {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    author_id: row.get(3)?,
                    author_name: row.get(4)?,
                    category_id: row.get(5)?,
                    status: self.parse_post_status(&row.get::<_, String>(6)?),
                    view_count: row.get(7)?,
                    like_count: row.get(8)?,
                    comment_count: row.get(9)?,
                    is_pinned: row.get(10)?,
                    is_featured: row.get(11)?,
                    created_at: parse_timestamp(row.get::<_, String>(12)?),
                    updated_at: parse_timestamp(row.get::<_, String>(13)?),
                    review_status: row.get(14).ok(),
                    review_comment: row.get(15).ok(),
                    reviewer_id: row.get(16).ok(),
                    reviewed_at: row.get::<_, Option<String>>(17).ok().flatten().map(parse_timestamp),
                    // 新增字段
                    images: {
                        if let Ok(json_str) = row.get::<_, String>(18) {
                            serde_json::from_str(&json_str).ok()
                        } else {
                            None
                        }
                    },
                    code_snippet: row.get(19).ok(),
                    tags: {
                        if let Ok(json_str) = row.get::<_, String>(20) {
                            serde_json::from_str(&json_str).ok()
                        } else {
                            None
                        }
                    },
                })
            }
        )?.collect::<Result<Vec<_>, _>>()?;

        Ok(PostListResponse {
            list: posts,
            total,
            page: page as i32,
            size: page_size as i32,
        })
    }

    // 获取单个帖子
    pub async fn get_post(&self, post_id: i32) -> SqliteResult<Option<Post>> {
        let conn = Connection::open(&self.db_path)?;
        
        let sql = "SELECT id, title, content, author_id, author_name, category_id, status, view_count, like_count, comment_count, is_pinned, is_featured, created_at, updated_at, review_status, review_comment, reviewer_id, reviewed_at, images, code_snippet, tags FROM posts WHERE id = ?";
        
        let result = conn.query_row(sql, params![post_id], |row| {
            Ok(Post {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                author_id: row.get(3)?,
                author_name: row.get(4)?,
                category_id: row.get(5)?,
                status: self.parse_post_status(&row.get::<_, String>(6)?),
                view_count: row.get(7)?,
                like_count: row.get(8)?,
                comment_count: row.get(9)?,
                is_pinned: row.get(10)?,
                is_featured: row.get(11)?,
                created_at: parse_timestamp(row.get::<_, String>(12)?),
                updated_at: parse_timestamp(row.get::<_, String>(13)?),
                review_status: row.get(14).ok(),
                review_comment: row.get(15).ok(),
                reviewer_id: row.get(16).ok(),
                reviewed_at: row.get::<_, Option<String>>(17).ok().flatten().map(parse_timestamp),
                // 新增字段
                images: {
                    if let Ok(json_str) = row.get::<_, String>(18) {
                        serde_json::from_str(&json_str).ok()
                    } else {
                        None
                    }
                },
                code_snippet: row.get(19).ok(),
                tags: {
                    if let Ok(json_str) = row.get::<_, String>(20) {
                        serde_json::from_str(&json_str).ok()
                    } else {
                        None
                    }
                },
            })
        });

        match result {
            Ok(post) => Ok(Some(post)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    // 删除帖子
    pub async fn delete_post(&self, post_id: i32) -> SqliteResult<bool> {
        let conn = Connection::open(&self.db_path)?;
        
        let result = conn.execute("DELETE FROM posts WHERE id = ?", params![post_id])?;
        Ok(result > 0)
    }

    // 增加浏览量
    pub async fn increment_view_count(&self, post_id: i32) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("UPDATE posts SET view_count = view_count + 1 WHERE id = ?", params![post_id])?;
        Ok(())
    }

    // 添加标签到帖子
    pub async fn add_tags_to_post(&self, post_id: i32, tag_names: &[String]) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        
        for tag_name in tag_names {
            // 查找或创建标签
            let tag_id = self.get_or_create_tag(&conn, tag_name)?;
            
            // 添加帖子标签关联
            conn.execute(
                "INSERT OR IGNORE INTO post_tags (post_id, tag_id, created_at) VALUES (?, ?, ?)",
                params![post_id, tag_id, Utc::now()]
            )?;
        }
        
        Ok(())
    }

    // 更新帖子标签
    pub async fn update_post_tags(&self, post_id: i32, tag_names: &[String]) -> SqliteResult<()> {
        let conn = Connection::open(&self.db_path)?;
        
        // 删除现有标签关联
        conn.execute("DELETE FROM post_tags WHERE post_id = ?", params![post_id])?;
        
        // 添加新标签关联
        self.add_tags_to_post(post_id, tag_names).await?;
        
        Ok(())
    }

    // 获取帖子的标签
    pub async fn get_post_tags(&self, post_id: i32) -> SqliteResult<Vec<Tag>> {
        let conn = Connection::open(&self.db_path)?;
        
        let sql = "
            SELECT t.id, t.name, t.description, t.color, t.use_count, t.created_at, t.updated_at
            FROM tags t
            INNER JOIN post_tags pt ON t.id = pt.tag_id
            WHERE pt.post_id = ?
            ORDER BY t.name
        ";
        
        let tags = conn.prepare(sql)?
            .query_map(params![post_id], |row| {
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

    // 获取或创建标签
    fn get_or_create_tag(&self, conn: &Connection, tag_name: &str) -> SqliteResult<i32> {
        // 先查找现有标签
        let result = conn.query_row(
            "SELECT id FROM tags WHERE name = ?",
            params![tag_name],
            |row| row.get(0)
        );

        match result {
            Ok(tag_id) => Ok(tag_id),
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                // 创建新标签
                let tag_id = conn.execute(
                    "INSERT INTO tags (name, use_count, created_at, updated_at) VALUES (?, ?, ?, ?)",
                    params![tag_name, 1, Utc::now(), Utc::now()]
                )?;
                Ok(tag_id as i32)
            }
            Err(e) => Err(e),
        }
    }

    // 解析帖子状态
    fn parse_post_status(&self, status: &str) -> crate::models::PostStatus {
        match status {
            "Draft" => crate::models::PostStatus::Draft,
            "Published" => crate::models::PostStatus::Published,
            "Archived" => crate::models::PostStatus::Archived,
            "Deleted" => crate::models::PostStatus::Deleted,
            _ => crate::models::PostStatus::Draft,
        }
    }

    pub async fn like_post(&self, user_id: i32, post_id: i32) -> SqliteResult<i32> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS post_likes (user_id INTEGER NOT NULL, post_id INTEGER NOT NULL, created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP), PRIMARY KEY (user_id, post_id))",
            [],
        )?;
        conn.execute(
            "INSERT OR IGNORE INTO post_likes (user_id, post_id) VALUES (?, ?)",
            params![user_id, post_id],
        )?;
        conn.execute(
            "UPDATE posts SET like_count = COALESCE(like_count,0) + 1 WHERE id = ? AND EXISTS(SELECT 1 FROM post_likes WHERE user_id = ? AND post_id = ?)",
            params![post_id, user_id, post_id],
        )?;
        let cnt: i32 = conn.query_row("SELECT COUNT(*) FROM post_likes WHERE post_id = ?", params![post_id], |r| r.get(0))?;
        Ok(cnt)
    }

    pub async fn unlike_post(&self, user_id: i32, post_id: i32) -> SqliteResult<i32> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "DELETE FROM post_likes WHERE user_id = ? AND post_id = ?",
            params![user_id, post_id],
        )?;
        let cnt: i32 = conn.query_row("SELECT COUNT(*) FROM post_likes WHERE post_id = ?", params![post_id], |r| r.get(0))?;
        conn.execute("UPDATE posts SET like_count = ? WHERE id = ?", params![cnt, post_id])?;
        Ok(cnt)
    }

    pub async fn get_user_liked_posts(&self, user_id: i32, page: i32, page_size: i32) -> SqliteResult<(Vec<Post>, i64)> {
        let conn = Connection::open(&self.db_path)?;
        let total: i64 = conn.query_row(
            "SELECT COUNT(*) FROM post_likes WHERE user_id = ?",
            params![user_id],
            |r| r.get(0),
        )?;
        let offset = (page - 1).max(0) * page_size.max(1);
        let sql = "SELECT p.id, p.title, p.content, p.author_id, p.author_name, p.category_id, p.status, p.view_count, p.like_count, p.comment_count, p.is_pinned, p.is_featured, p.created_at, p.updated_at, p.review_status, p.review_comment, p.reviewer_id, p.reviewed_at, p.images, p.code_snippet, p.tags FROM posts p JOIN post_likes pl ON pl.post_id = p.id WHERE pl.user_id = ? ORDER BY pl.created_at DESC LIMIT ? OFFSET ?";
        let mut stmt = conn.prepare(sql)?;
        let posts = stmt
            .query_map(params![user_id, page_size, offset], |row| {
                Ok(Post {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    author_id: row.get(3)?,
                    author_name: row.get(4)?,
                    category_id: row.get(5)?,
                    status: self.parse_post_status(&row.get::<_, String>(6)?),
                    view_count: row.get(7)?,
                    like_count: row.get(8)?,
                    comment_count: row.get(9)?,
                    is_pinned: row.get(10)?,
                    is_featured: row.get(11)?,
                    created_at: parse_timestamp(row.get::<_, String>(12)?),
                    updated_at: parse_timestamp(row.get::<_, String>(13)?),
                    review_status: row.get(14).ok(),
                    review_comment: row.get(15).ok(),
                    reviewer_id: row.get(16).ok(),
                    reviewed_at: row.get::<_, Option<String>>(17).ok().flatten().map(parse_timestamp),
                    // 新增字段
                    images: {
                        if let Ok(json_str) = row.get::<_, String>(18) {
                            serde_json::from_str(&json_str).ok()
                        } else {
                            None
                        }
                    },
                    code_snippet: row.get(19).ok(),
                    tags: {
                        if let Ok(json_str) = row.get::<_, String>(20) {
                            serde_json::from_str(&json_str).ok()
                        } else {
                            None
                        }
                    },
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok((posts, total))
    }
} 

// 辅助函数：尽量解析日期时间字符串，优先 RFC3339，失败则按常见 SQLite 默认格式再失败返回当前时间
fn parse_timestamp(ts: String) -> DateTime<Utc> {
    // 尝试 RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(&ts) {
        return dt.with_timezone(&Utc);
    }

    // 尝试 "YYYY-MM-DD HH:MM:SS" 格式（SQLite CURRENT_TIMESTAMP 默认格式）
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(&ts, "%Y-%m-%d %H:%M:%S") {
        return DateTime::<Utc>::from_utc(naive, Utc);
    }

    // 尝试带毫秒的格式
    if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(&ts, "%Y-%m-%d %H:%M:%S%.f") {
        return DateTime::<Utc>::from_utc(naive, Utc);
    }

    // 解析失败，返回当前时间，避免 panic
    log::warn!("无法解析日期时间字符串: {}，已使用当前时间代替", ts);
    Utc::now()
} 