// 仓储实现模块
// 此模块包含所有数据访问层的实现

use sqlx::{Pool, Sqlite};
use tracing::instrument;
use serde::Serialize;
use sqlx::Row;

#[derive(Debug)]
pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn find_by_username_or_email(&self, ue: &str) -> sqlx::Result<Option<crate::core::domain::user::User>> {
        let row = sqlx::query_as::<_, crate::core::domain::user::User>(
            "SELECT id, username, email, password_hash, nickname, avatar_url, bio, role, status, settings, stats, created_at, updated_at FROM users WHERE username = ? OR email = ? LIMIT 1"
        )
        .bind(ue)
        .bind(ue)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: i64) -> sqlx::Result<Option<crate::core::domain::user::User>> {
        let row = sqlx::query_as::<_, crate::core::domain::user::User>(
            "SELECT id, username, email, password_hash, nickname, avatar_url, bio, role, status, settings, stats, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    #[instrument(skip(self))]
    pub async fn exists_username(&self, username: &str) -> sqlx::Result<bool> {
        let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(1) FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        Ok(cnt > 0)
    }

    #[instrument(skip(self))]
    pub async fn exists_email(&self, email: &str) -> sqlx::Result<bool> {
        let (cnt,): (i64,) = sqlx::query_as("SELECT COUNT(1) FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(&self.pool)
            .await?;
        Ok(cnt > 0)
    }

    #[instrument(skip(self))]
    pub async fn insert_user(&self, username: &str, email: &str, password_hash: &str, nickname: Option<&str>) -> sqlx::Result<i64> {
        let rec = sqlx::query(
            "INSERT INTO users (username, email, password_hash, nickname, role, status, settings, stats) VALUES (?, ?, ?, ?, 'user', 'active', '{}', '{}')"
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(nickname)
        .execute(&self.pool)
        .await?;
        Ok(rec.last_insert_rowid())
    }

    #[instrument(skip(self))]
    pub async fn update_profile(&self, id: i64, nickname: Option<&str>, avatar_url: Option<&str>, bio: Option<&str>, settings: Option<serde_json::Value>) -> sqlx::Result<u64> {
        let mut q = sqlx::QueryBuilder::<sqlx::Sqlite>::new(
            "UPDATE users SET updated_at = CURRENT_TIMESTAMP"
        );
        if nickname.is_some() { q.push(", nickname = ").push_bind(nickname); }
        if avatar_url.is_some() { q.push(", avatar_url = ").push_bind(avatar_url); }
        if bio.is_some() { q.push(", bio = ").push_bind(bio); }
        if settings.is_some() { q.push(", settings = ").push_bind(settings); }
        q.push(" WHERE id = ").push_bind(id);
        let res = q.build().execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    #[instrument(skip(self))]
    pub async fn stats_of(&self, user_id: i64) -> sqlx::Result<(i64, i64, i64, i64)> {
        let (packages_count,): (i64,) = sqlx::query_as("SELECT COUNT(1) FROM packages WHERE author_id = ?")
            .bind(user_id).fetch_one(&self.pool).await?;
        let (comments_count,): (i64,) = sqlx::query_as("SELECT COUNT(1) FROM comments WHERE user_id = ?")
            .bind(user_id).fetch_one(&self.pool).await?;
        let (likes_received,): (i64,) = sqlx::query_as("SELECT COALESCE(SUM(cnt),0) FROM (
            SELECT like_count as cnt FROM packages WHERE author_id = ?
            UNION ALL
            SELECT like_count FROM posts WHERE author_id = ?
        ) t")
            .bind(user_id).bind(user_id).fetch_one(&self.pool).await?;
        let (downloads_total,): (i64,) = sqlx::query_as("SELECT COALESCE(SUM(download_count),0) FROM packages WHERE author_id = ?")
            .bind(user_id).fetch_one(&self.pool).await?;
        Ok((packages_count, comments_count, likes_received, downloads_total))
    }
}

#[derive(Debug)]
pub struct PackageRepository {
    pool: Pool<Sqlite>,
}

impl PackageRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct CategoryRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct CategoryItem {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i64,
    pub enabled: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl CategoryRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_all(&self) -> sqlx::Result<Vec<CategoryItem>> {
        let items = sqlx::query_as::<_, CategoryItem>(
            "SELECT id, name, description, icon, sort_order, enabled, created_at, updated_at FROM categories ORDER BY sort_order ASC, id ASC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(items)
    }

    #[instrument(skip(self))]
    pub async fn counts_map(&self) -> sqlx::Result<std::collections::HashMap<i64, i64>> {
        let rows = sqlx::query(
            "SELECT category_id, COUNT(1) as cnt FROM packages WHERE category_id IS NOT NULL GROUP BY category_id"
        )
        .fetch_all(&self.pool)
        .await?;
        let mut map = std::collections::HashMap::new();
        for r in rows {
            let cat_id: i64 = r.try_get::<i64, _>("category_id").unwrap_or(0);
            let cnt: i64 = r.try_get::<i64, _>("cnt").unwrap_or(0);
            map.insert(cat_id, cnt);
        }
        Ok(map)
    }
}

#[derive(Debug)]
pub struct CommentRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct CommentItem {
    pub id: i64,
    pub user_id: i64,
    pub target_type: String,
    pub target_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
    pub likes_count: i64,
    pub helpful_count: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CommentRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self {
            pool: pool.clone(),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn list_by_target(&self, target_type: &str, target_id: i64, page: i64, page_size: i64) -> sqlx::Result<crate::shared::types::pagination::PaginatedResult<CommentItem>> {
        let offset = (page - 1) * page_size;
        let total_row = sqlx::query("SELECT COUNT(1) as cnt FROM comments WHERE target_type = ? AND target_id = ? AND status != 'deleted'")
            .bind(target_type).bind(target_id)
            .fetch_one(&self.pool).await?;
        let total = total_row.try_get::<i64, _>("cnt").unwrap_or(0);
        let items = sqlx::query_as::<_, CommentItem>(
            "SELECT id, user_id, target_type, target_id, content, parent_id, likes_count, helpful_count, created_at FROM comments WHERE target_type = ? AND target_id = ? AND status != 'deleted' ORDER BY created_at DESC LIMIT ? OFFSET ?"
        ).bind(target_type).bind(target_id).bind(page_size).bind(offset)
        .fetch_all(&self.pool).await?;
        Ok(crate::shared::types::pagination::PaginatedResult { data: items, total, page, page_size })
    }

    #[instrument(skip(self))]
    pub async fn create(&self, user_id: i64, target_type: &str, target_id: i64, content: &str, parent_id: Option<i64>) -> sqlx::Result<CommentItem> {
        let rec = sqlx::query_as::<_, CommentItem>(
            "INSERT INTO comments (user_id, target_type, target_id, content, parent_id, status, likes_count, dislikes_count, is_pinned) VALUES (?, ?, ?, ?, ?, 'active', 0, 0, 0) RETURNING id, user_id, target_type, target_id, content, parent_id, likes_count, helpful_count, created_at"
        )
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .bind(content)
        .bind(parent_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(rec)
    }

    #[instrument(skip(self))]
    pub async fn find_by_id(&self, id: i64) -> sqlx::Result<Option<crate::core::domain::comment::Comment>> {
        let row = sqlx::query_as::<_, crate::core::domain::comment::Comment>(
            "SELECT id, user_id, target_type, target_id, content, parent_id, status, likes_count, dislikes_count, is_pinned, helpful_count, is_helpful, created_at, updated_at FROM comments WHERE id = ?"
        ).bind(id).fetch_optional(&self.pool).await?;
        Ok(row)
    }

    #[instrument(skip(self))]
    pub async fn update_like_count(&self, id: i64, count: i64) -> sqlx::Result<u64> {
        let res = sqlx::query("UPDATE comments SET likes_count = ? WHERE id = ?")
            .bind(count).bind(id).execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    #[instrument(skip(self))]
    pub async fn mark_helpful(&self, id: i64) -> sqlx::Result<u64> {
        let res = sqlx::query("UPDATE comments SET helpful_count = COALESCE(helpful_count,0) + 1, is_helpful = 1 WHERE id = ?")
            .bind(id).execute(&self.pool).await?;
        Ok(res.rows_affected())
    }
}

#[derive(Debug)]
pub struct NotificationRepository {
    pool: Pool<Sqlite>,
}

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct NotificationItem {
    pub id: i64,
    pub user_id: i64,
    pub r#type: String,
    pub title: String,
    pub content: Option<String>,
    pub data: Option<String>,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl NotificationRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }

    #[instrument(skip(self))]
    pub async fn health_check(&self) -> sqlx::Result<()> {
        sqlx::query("SELECT 1").execute(&self.pool).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn unread_count(&self, user_id: i64) -> sqlx::Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(1) FROM notifications WHERE user_id = ? AND read_at IS NULL"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    #[instrument(skip(self))]
    pub async fn list(&self, user_id: i64, page: i64, page_size: i64, unread_only: bool) -> sqlx::Result<crate::shared::types::pagination::PaginatedResult<NotificationItem>> {
        let offset = (page - 1) * page_size;
        let (count_sql, list_sql) = if unread_only {
            (
                "SELECT COUNT(1) as cnt FROM notifications WHERE user_id = ? AND read_at IS NULL",
                "SELECT id, user_id, type, title, content, CAST(data AS TEXT) as data, read_at, created_at FROM notifications WHERE user_id = ? AND read_at IS NULL ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
        } else {
            (
                "SELECT COUNT(1) as cnt FROM notifications WHERE user_id = ?",
                "SELECT id, user_id, type, title, content, CAST(data AS TEXT) as data, read_at, created_at FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
        };
        let row = sqlx::query(count_sql)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        let total = row.try_get::<i64, _>("cnt").unwrap_or(0);
        let items = sqlx::query_as::<_, NotificationItem>(list_sql)
            .bind(user_id)
            .bind(page_size)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(crate::shared::types::pagination::PaginatedResult { data: items, total, page, page_size })
    }

    #[instrument(skip(self))]
    pub async fn mark_read(&self, user_id: i64, id: i64) -> sqlx::Result<u64> {
        let res = sqlx::query("UPDATE notifications SET read_at = CURRENT_TIMESTAMP WHERE id = ? AND user_id = ? AND read_at IS NULL")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    #[instrument(skip(self))]
    pub async fn mark_all_read(&self, user_id: i64) -> sqlx::Result<u64> {
        let res = sqlx::query("UPDATE notifications SET read_at = CURRENT_TIMESTAMP WHERE user_id = ? AND read_at IS NULL")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    #[instrument(skip(self))]
    pub async fn delete_read(&self, user_id: i64) -> sqlx::Result<u64> {
        let res = sqlx::query("DELETE FROM notifications WHERE user_id = ? AND read_at IS NOT NULL")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    #[instrument(skip(self))]
    pub async fn delete_by_id(&self, user_id: i64, id: i64) -> sqlx::Result<u64> {
        let res = sqlx::query("DELETE FROM notifications WHERE id = ? AND user_id = ?")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}

// =============== 新增：邮件设置与验证码仓储 ===============
#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct MailSettings {
    pub id: i64,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub from_address: Option<String>,
    pub from_name: Option<String>,
    pub enabled: i64,
}

#[derive(Debug)]
pub struct MailSettingsRepository { pool: Pool<Sqlite>, }
impl MailSettingsRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self { Self { pool: pool.clone() } }

    #[instrument(skip(self))]
    pub async fn get(&self) -> sqlx::Result<Option<MailSettings>> {
        let row = sqlx::query_as::<_, MailSettings>(
            "SELECT id, smtp_host, smtp_port, username, password, from_address, from_name, COALESCE(enabled,0) as enabled FROM mail_settings WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    #[instrument(skip(self))]
    pub async fn upsert(&self,
        smtp_host: Option<&str>, smtp_port: Option<i64>, username: Option<&str>, password: Option<&str>,
        from_address: Option<&str>, from_name: Option<&str>, enabled: Option<bool>
    ) -> sqlx::Result<u64> {
        let enabled_i = enabled.unwrap_or(false) as i64;
        let res = sqlx::query(
            "INSERT INTO mail_settings (id, smtp_host, smtp_port, username, password, from_address, from_name, enabled, updated_at) \
             VALUES (1, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP) \
             ON CONFLICT(id) DO UPDATE SET \
             smtp_host=excluded.smtp_host, smtp_port=excluded.smtp_port, username=excluded.username, password=excluded.password, \
             from_address=excluded.from_address, from_name=excluded.from_name, enabled=excluded.enabled, updated_at=CURRENT_TIMESTAMP"
        )
        .bind(smtp_host)
        .bind(smtp_port)
        .bind(username)
        .bind(password)
        .bind(from_address)
        .bind(from_name)
        .bind(enabled_i)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct EmailVerification {
    pub id: i64,
    pub email: String,
    pub code: String,
    #[sqlx(rename = "type")]
    pub r#type: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub used_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug)]
pub struct EmailVerificationRepository { pool: Pool<Sqlite>, }
impl EmailVerificationRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self { Self { pool: pool.clone() } }

    #[instrument(skip(self))]
    pub async fn create_code(&self, email: &str, code: &str, typ: &str, expires_at: chrono::DateTime<chrono::Utc>) -> sqlx::Result<i64> {
        let res = sqlx::query(
            "INSERT INTO email_verifications (email, code, type, expires_at) VALUES (?, ?, ?, ?)"
        )
        .bind(email)
        .bind(code)
        .bind(typ)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;
        Ok(res.last_insert_rowid())
    }

    #[instrument(skip(self))]
    pub async fn find_valid(&self, email: &str, code: &str, typ: &str) -> sqlx::Result<Option<EmailVerification>> {
        let row = sqlx::query_as::<_, EmailVerification>(
            "SELECT id, email, code, type, expires_at, used_at FROM email_verifications WHERE email = ? AND code = ? AND type = ? AND used_at IS NULL AND expires_at > CURRENT_TIMESTAMP ORDER BY id DESC LIMIT 1"
        )
        .bind(email)
        .bind(code)
        .bind(typ)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    #[instrument(skip(self))]
    pub async fn mark_used(&self, id: i64) -> sqlx::Result<u64> {
        let res = sqlx::query("UPDATE email_verifications SET used_at = CURRENT_TIMESTAMP WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}

// =============== 新增：点赞与收藏仓储实现 ===============
use async_trait::async_trait;
use crate::shared::errors::{AppError, AppResult};
use crate::shared::types::pagination::PaginatedResult;
use crate::core::ports::repositories::{LikeRepository, BookmarkRepository, UserLike, UserBookmark};

#[derive(Debug, Clone)]
pub struct SqlxLikeRepository {
    pool: Pool<Sqlite>,
}

impl SqlxLikeRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl LikeRepository for SqlxLikeRepository {
    async fn add_like(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO likes (user_id, target_type, target_id) VALUES (?, ?, ?)"
        )
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    async fn remove_like(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()> {
        sqlx::query(
            "DELETE FROM likes WHERE user_id = ? AND target_type = ? AND target_id = ?"
        )
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    async fn is_liked(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<bool> {
        let exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(1) FROM likes WHERE user_id = ? AND target_type = ? AND target_id = ?"
        )
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(exists.0 > 0)
    }

    async fn get_like_count(&self, target_type: &str, target_id: i64) -> AppResult<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(1) FROM likes WHERE target_type = ? AND target_id = ?"
        )
        .bind(target_type)
        .bind(target_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(count.0)
    }

    async fn find_user_likes(&self, user_id: i64, target_type: Option<&str>, page: i64, page_size: i64) -> AppResult<PaginatedResult<UserLike>> {
        let offset = (page - 1) * page_size;

        let (sql_count, sql_list) = if let Some(tt) = target_type {
            (
                "SELECT COUNT(1) as cnt FROM likes WHERE user_id = ? AND target_type = ?",
                "SELECT id, user_id, target_type, target_id, created_at FROM likes WHERE user_id = ? AND target_type = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
        } else {
            (
                "SELECT COUNT(1) as cnt FROM likes WHERE user_id = ?",
                "SELECT id, user_id, target_type, target_id, created_at FROM likes WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
        };

        let total: i64 = if let Some(tt) = target_type {
            let row = sqlx::query(sql_count)
                .bind(user_id)
                .bind(tt)
                .fetch_one(&self.pool)
                .await
                .map_err(AppError::Database)?;
            row.try_get::<i64, _>("cnt").unwrap_or(0)
        } else {
            let row = sqlx::query(sql_count)
                .bind(user_id)
                .fetch_one(&self.pool)
                .await
                .map_err(AppError::Database)?;
            row.try_get::<i64, _>("cnt").unwrap_or(0)
        };

        let items: Vec<UserLike> = if let Some(tt) = target_type {
            sqlx::query_as::<_, UserLike>(sql_list)
                .bind(user_id)
                .bind(tt)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::Database)?
        } else {
            sqlx::query_as::<_, UserLike>(sql_list)
                .bind(user_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::Database)?
        };

        Ok(PaginatedResult { data: items, total, page, page_size })
    }
}

#[derive(Debug, Clone)]
pub struct SqlxBookmarkRepository {
    pool: Pool<Sqlite>,
}

impl SqlxBookmarkRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl BookmarkRepository for SqlxBookmarkRepository {
    async fn add_bookmark(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()> {
        sqlx::query(
            "INSERT OR IGNORE INTO favorites (user_id, target_type, target_id) VALUES (?, ?, ?)"
        )
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    async fn remove_bookmark(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<()> {
        sqlx::query(
            "DELETE FROM favorites WHERE user_id = ? AND target_type = ? AND target_id = ?"
        )
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .execute(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(())
    }

    async fn is_bookmarked(&self, user_id: i64, target_type: &str, target_id: i64) -> AppResult<bool> {
        let exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(1) FROM favorites WHERE user_id = ? AND target_type = ? AND target_id = ?"
        )
        .bind(user_id)
        .bind(target_type)
        .bind(target_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(exists.0 > 0)
    }

    async fn find_user_bookmarks(&self, user_id: i64, target_type: Option<&str>, page: i64, page_size: i64) -> AppResult<PaginatedResult<UserBookmark>> {
        let offset = (page - 1) * page_size;

        let (sql_count, sql_list) = if let Some(tt) = target_type {
            (
                "SELECT COUNT(1) as cnt FROM favorites WHERE user_id = ? AND target_type = ?",
                "SELECT id, user_id, target_type, target_id, created_at FROM favorites WHERE user_id = ? AND target_type = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
        } else {
            (
                "SELECT COUNT(1) as cnt FROM favorites WHERE user_id = ?",
                "SELECT id, user_id, target_type, target_id, created_at FROM favorites WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            )
        };

        let total: i64 = if let Some(tt) = target_type {
            let row = sqlx::query(sql_count)
                .bind(user_id)
                .bind(tt)
                .fetch_one(&self.pool)
                .await
                .map_err(AppError::Database)?;
            row.try_get::<i64, _>("cnt").unwrap_or(0)
        } else {
            let row = sqlx::query(sql_count)
                .bind(user_id)
                .fetch_one(&self.pool)
                .await
                .map_err(AppError::Database)?;
            row.try_get::<i64, _>("cnt").unwrap_or(0)
        };

        let items: Vec<UserBookmark> = if let Some(tt) = target_type {
            sqlx::query_as::<_, UserBookmark>(sql_list)
                .bind(user_id)
                .bind(tt)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::Database)?
        } else {
            sqlx::query_as::<_, UserBookmark>(sql_list)
                .bind(user_id)
                .bind(page_size)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(AppError::Database)?
        };

        Ok(PaginatedResult { data: items, total, page, page_size })
    }
} 

use crate::core::domain::post as domain_post;
use crate::core::domain::post::{Post, PostStatus, PostSearchParams, CreatePostData};

#[derive(Debug, Clone)]
pub struct SqlxPostRepository {
    pool: Pool<Sqlite>,
}

impl SqlxPostRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl crate::core::ports::repositories::PostRepository for SqlxPostRepository {
    async fn create(&self, data: CreatePostData) -> AppResult<Post> {
        let status_str = match data.status {
            PostStatus::Draft => "draft",
            PostStatus::Published => "published",
            PostStatus::Archived => "archived",
            PostStatus::Banned => "banned",
        };

        let rec = sqlx::query_as::<_, Post>(
            "INSERT INTO posts (title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet) \
             VALUES (?, ?, ?, 0, 0, 0, ?, 0, 0, ?, ?, ?) RETURNING id, title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet, created_at, updated_at"
        )
        .bind(data.title)
        .bind(data.content)
        .bind(data.author_id)
        .bind(status_str)
        .bind(data.tags)
        .bind(data.images)
        .bind(data.code_snippet)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(rec)
    }

    async fn find_by_id(&self, id: i64) -> AppResult<Option<Post>> {
        let row = sqlx::query_as::<_, Post>(
            "SELECT id, title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet, created_at, updated_at FROM posts WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row)
    }

    async fn update(&self, id: i64, data: domain_post::UpdatePostRequest) -> AppResult<Post> {
        // 简化：仅支持部分字段更新
        let rec = sqlx::query_as::<_, Post>(
            "UPDATE posts SET title = COALESCE(?, title), content = COALESCE(?, content), tags = COALESCE(?, tags), images = COALESCE(?, images), code_snippet = COALESCE(?, code_snippet), updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id, title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet, created_at, updated_at"
        )
        .bind(data.title.map(|v| v))
        .bind(data.content.map(|v| v))
        .bind(data.tags.map(|t| serde_json::to_string(&t).unwrap_or_default()))
        .bind(data.images.map(|i| serde_json::to_string(&i).unwrap_or_default()))
        .bind(data.code_snippet)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(rec)
    }

    async fn update_status(&self, id: i64, status: PostStatus) -> AppResult<Post> {
        let status_str = match status {
            PostStatus::Draft => "draft",
            PostStatus::Published => "published",
            PostStatus::Archived => "archived",
            PostStatus::Banned => "banned",
        };
        let rec = sqlx::query_as::<_, Post>(
            "UPDATE posts SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? RETURNING id, title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet, created_at, updated_at"
        )
        .bind(status_str)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(rec)
    }

    async fn search(&self, params: PostSearchParams) -> AppResult<PaginatedResult<Post>> {
        let mut conditions: Vec<String> = Vec::new();
        let mut bind_values: Vec<String> = Vec::new();

        if let Some(q) = params.query {
            if !q.is_empty() {
                conditions.push("(title LIKE ? OR content LIKE ?)".to_string());
                let pattern = format!("%{}%", q);
                bind_values.push(pattern.clone());
                bind_values.push(pattern);
            }
        }
        if let Some(tag) = params.tag {
            if !tag.is_empty() {
                conditions.push("(tags LIKE ?)".to_string());
                bind_values.push(format!("%{}%", tag));
            }
        }
        if let Some(is_featured) = params.is_featured {
            if is_featured {
                conditions.push("is_featured = 1".to_string());
            }
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(1) as cnt FROM posts{}", where_clause);
        let list_sql = format!("SELECT id, title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet, created_at, updated_at FROM posts{} ORDER BY created_at DESC LIMIT ? OFFSET ?", where_clause);

        // 计算分页
        let page = params.page.max(1);
        let page_size = params.page_size.min(100).max(1);
        let offset = (page - 1) * page_size;

        // 执行计数
        let total_row = sqlx::query(&count_sql)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;
        let total: i64 = total_row.try_get::<i64, _>("cnt").unwrap_or(0);

        // 执行列表查询
        let mut query = sqlx::query_as::<_, Post>(&list_sql);
        query = query.bind(page_size).bind(offset);
        let items = query
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(PaginatedResult { data: items, total, page, page_size })
    }

    async fn find_by_author(&self, author_id: i64, page: i64, page_size: i64) -> AppResult<PaginatedResult<Post>> {
        let offset = (page - 1) * page_size;
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(1) FROM posts WHERE author_id = ?"
        )
        .bind(author_id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        let items: Vec<Post> = sqlx::query_as(
            "SELECT id, title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet, created_at, updated_at FROM posts WHERE author_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(author_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(PaginatedResult { data: items, total: total.0, page, page_size })
    }

    async fn find_popular(&self, limit: i64) -> AppResult<Vec<Post>> {
        let items: Vec<Post> = sqlx::query_as(
            "SELECT id, title, content, author_id, like_count, view_count, comment_count, status, is_featured, is_pinned, tags, images, code_snippet, created_at, updated_at FROM posts ORDER BY like_count DESC, view_count DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(items)
    }

    async fn increment_view_count(&self, id: i64) -> AppResult<()> {
        sqlx::query("UPDATE posts SET view_count = view_count + 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    async fn update_like_count(&self, id: i64, count: i64) -> AppResult<()> {
        sqlx::query("UPDATE posts SET like_count = ? WHERE id = ?")
            .bind(count)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    async fn delete(&self, id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
} 

use crate::core::domain::resource as domain_resource;
use crate::core::domain::resource::{Resource, ResourceStatus, ResourceSearchParams, CreateResourceData};

#[derive(Debug, Clone)]
pub struct SqlxResourceRepository {
    pool: Pool<Sqlite>,
}

impl SqlxResourceRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl crate::core::ports::repositories::ResourceRepository for SqlxResourceRepository {
    async fn create(&self, data: CreateResourceData) -> AppResult<Resource> {
        let status_str = match data.status {
            ResourceStatus::Draft => "draft",
            ResourceStatus::Published => "published",
            ResourceStatus::Archived => "archived",
            ResourceStatus::Banned => "banned",
        };
        let rec = sqlx::query_as::<_, Resource>(
            "INSERT INTO packages (name, slug, author_id, version, description, readme, category_id, tags, metadata, stats, status, published_at, file_path, file_size, download_count, like_count, view_count, comment_count, rating, is_featured, is_pinned, requirements, screenshots) \
             VALUES (?, ?, ?, ?, ?, NULL, ?, ?, '{}', '{}', ?, CURRENT_TIMESTAMP, ?, ?, 0, 0, 0, 0, 0.0, 0, 0, ?, ?) \
             RETURNING id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at"
        )
        .bind(&data.name)
        .bind(&data.name) // slug 简化复用 name
        .bind(data.author_id)
        .bind(&data.version)
        .bind(&data.description)
        .bind(data.category_id)
        .bind(&data.tags)
        .bind(status_str)
        .bind(&data.file_path)
        .bind(data.file_size)
        .bind(&data.requirements)
        .bind(&data.screenshots)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(rec)
    }

    async fn find_by_id(&self, id: i64) -> AppResult<Option<Resource>> {
        let row = sqlx::query_as::<_, Resource>(
            "SELECT id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at FROM packages WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row)
    }

    async fn find_by_name(&self, name: &str) -> AppResult<Option<Resource>> {
        let row = sqlx::query_as::<_, Resource>(
            "SELECT id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at FROM packages WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(row)
    }

    async fn exists_by_name(&self, name: &str) -> AppResult<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(1) FROM packages WHERE name = ?")
            .bind(name)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(count.0 > 0)
    }

    async fn update(&self, id: i64, data: domain_resource::UpdateResourceRequest) -> AppResult<Resource> {
        let rec = sqlx::query_as::<_, Resource>(
            "UPDATE packages SET name = COALESCE(?, name), description = COALESCE(?, description), version = COALESCE(?, version), category_id = COALESCE(?, category_id), tags = COALESCE(?, tags), screenshots = COALESCE(?, screenshots), updated_at = CURRENT_TIMESTAMP WHERE id = ? \
             RETURNING id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at"
        )
        .bind(data.title)
        .bind(data.description)
        .bind(data.version)
        .bind(data.category_id)
        .bind(data.tags.map(|t| serde_json::to_string(&t).unwrap_or_default()))
        .bind(data.screenshots.map(|s| serde_json::to_string(&s).unwrap_or_default()))
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(rec)
    }

    async fn update_status(&self, id: i64, status: ResourceStatus) -> AppResult<Resource> {
        let status_str = match status {
            ResourceStatus::Draft => "draft",
            ResourceStatus::Published => "published",
            ResourceStatus::Archived => "archived",
            ResourceStatus::Banned => "banned",
        };
        let rec = sqlx::query_as::<_, Resource>(
            "UPDATE packages SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? \
             RETURNING id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at"
        )
        .bind(status_str)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(rec)
    }

    async fn search(&self, params: ResourceSearchParams) -> AppResult<PaginatedResult<Resource>> {
        let mut conditions: Vec<&str> = Vec::new();
        if let Some(q) = params.query { if !q.is_empty() { conditions.push("(name LIKE ? OR description LIKE ?)"); } }
        if let Some(cat) = params.category_id { if cat > 0 { conditions.push("category_id = ?"); } }
        if let Some(is_featured) = params.is_featured { if is_featured { conditions.push("is_featured = 1"); } }
        let where_clause = if conditions.is_empty() { String::from("") } else { format!(" WHERE {}", conditions.join(" AND ")) };

        let count_sql = format!("SELECT COUNT(1) as cnt FROM packages{}", where_clause);
        let list_sql = format!("SELECT id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at FROM packages{} ORDER BY created_at DESC LIMIT ? OFFSET ?", where_clause);

        let page = params.page.max(1);
        let page_size = params.page_size.min(100).max(1);
        let offset = (page - 1) * page_size;

        let total: (i64,) = sqlx::query_as::<_, (i64,)>(count_sql.as_str())
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;

        let items: Vec<Resource> = sqlx::query_as::<_, Resource>(list_sql.as_str())
            .bind(page_size)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(PaginatedResult { data: items, total: total.0, page, page_size })
    }

    async fn find_by_author(&self, author_id: i64, page: i64, page_size: i64) -> AppResult<PaginatedResult<Resource>> {
        let offset = (page - 1) * page_size;
        let total: (i64,) = sqlx::query_as("SELECT COUNT(1) FROM packages WHERE author_id = ?")
            .bind(author_id)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::Database)?;
        let items: Vec<Resource> = sqlx::query_as(
            "SELECT id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at FROM packages WHERE author_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(author_id)
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(PaginatedResult { data: items, total: total.0, page, page_size })
    }

    async fn find_popular(&self, limit: i64) -> AppResult<Vec<Resource>> {
        let items: Vec<Resource> = sqlx::query_as(
            "SELECT id, name as title, name as name, description, version, author_id, COALESCE(category_id, 0) as category_id, COALESCE(file_path, '') as file_path, COALESCE(file_size, 0) as file_size, COALESCE(download_count, 0) as download_count, COALESCE(like_count, 0) as like_count, COALESCE(view_count, 0) as view_count, COALESCE(comment_count, 0) as comment_count, COALESCE(rating, 0.0) as rating, status, COALESCE(is_featured, 0) as is_featured, COALESCE(is_pinned, 0) as is_pinned, requirements, tags, screenshots, created_at, updated_at FROM packages ORDER BY download_count DESC, like_count DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;
        Ok(items)
    }

    async fn increment_download_count(&self, id: i64) -> AppResult<()> {
        sqlx::query("UPDATE packages SET download_count = download_count + 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    async fn increment_view_count(&self, id: i64) -> AppResult<()> {
        sqlx::query("UPDATE packages SET view_count = view_count + 1 WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    async fn update_like_count(&self, id: i64, count: i64) -> AppResult<()> {
        sqlx::query("UPDATE packages SET like_count = ? WHERE id = ?")
            .bind(count)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    async fn delete(&self, id: i64) -> AppResult<()> {
        sqlx::query("DELETE FROM packages WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
} 

#[derive(Debug, sqlx::FromRow, Clone, Serialize)]
pub struct StorageFileItem {
    pub id: i64,
    pub user_id: Option<i64>,
    pub sha256: String,
    pub size: i64,
    pub mime: Option<String>,
    pub original_name: Option<String>,
    pub ext: Option<String>,
    pub relative_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct StorageFileRepository { pool: Pool<Sqlite> }
impl StorageFileRepository {
    pub fn new(pool: &Pool<Sqlite>) -> Self { Self { pool: pool.clone() } }

    #[instrument(skip(self))]
    pub async fn find_by_sha256(&self, sha256: &str) -> sqlx::Result<Option<StorageFileItem>> {
        let row = sqlx::query_as::<_, StorageFileItem>(
            "SELECT id, user_id, sha256, size, mime, original_name, ext, relative_path, created_at FROM storage_files WHERE sha256 = ?"
        ).bind(sha256).fetch_optional(&self.pool).await?;
        Ok(row)
    }

    #[instrument(skip(self))]
    pub async fn insert(&self, user_id: Option<i64>, sha256: &str, size: i64, mime: Option<&str>, original_name: Option<&str>, ext: Option<&str>, relative_path: &str) -> sqlx::Result<i64> {
        let rec = sqlx::query(
            "INSERT OR IGNORE INTO storage_files (user_id, sha256, size, mime, original_name, ext, relative_path) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(user_id)
        .bind(sha256)
        .bind(size)
        .bind(mime)
        .bind(original_name)
        .bind(ext)
        .bind(relative_path)
        .execute(&self.pool).await?;
        Ok(rec.last_insert_rowid())
    }
} 