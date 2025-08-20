## 结绳社区 App 后端对接 API 需求（API 接口）

> 本文档基于现有前端模拟数据与交互点整理，作为后端对接需求说明。字段与分页等约定以此为准，后端可结合实际再细化。

### 通用约定
- **接口前缀**: /api/v1
- **鉴权**: Bearer Token（除登录/注册/找回密码外），Header: `Authorization: Bearer <token>`
- **分页**: `page`(默认1), `pageSize`(默认10)，返回 `total`/`items`/`hasMore`
- **时间**: ISO 8601 字符串（如需本地化由前端处理）
- **ID 类型**: 数字型（前端当前均为 number）

### 1. 鉴权与账号
- 登录（账号/邮箱）
  - POST /auth/login
  - Body: { usernameOrEmail, password, remember? }
  - Resp: { token, user: UserProfileSummary }
- 注册
  - POST /auth/register
  - Body: { username, email, password, qq?, agreeTerms, agreePrivacy }
  - Resp: { userId, token }
- 忘记密码（发送重置邮件）
  - POST /auth/forgot-password
  - Body: { email }
  - Resp: { success: boolean }
- 重置密码（按后端实际实现）
  - POST /auth/reset-password
  - Body: { token, newPassword }
  - Resp: { success: boolean }

### 2. 主页/信息流（帖子/资源/公告聚合）
- 获取信息流（支持类型筛选与置顶优先）
  - GET /feed
  - Query: { type? in [post, resource, announcement], page?, pageSize? }
  - Resp: { items: ContentCard[], total, hasMore }

### 3. 搜索
- 热门搜索
  - GET /search/trending
  - Resp: { keywords: string[] }
- 联想/建议
  - GET /search/suggest?query=xxx
  - Resp: { suggestions: string[] }
- 综合搜索
  - GET /search?query=xxx&type?=post|resource|announcement&page?&pageSize?
  - Resp: { items: ContentCard[], total, hasMore }

### 4. 帖子（Post）
- 列表
  - GET /posts?page&pageSize&tag?
  - Resp: { items: PostSummary[], total, hasMore }
- 详情
  - GET /posts/{postId}
  - Resp: PostDetail
- 创建
  - POST /posts
  - Body: { title, content(markdown), tags: string[], images?: string[], codeSnippet?: string }
  - Resp: { id, status: 'reviewing'|'published'|'draft' }
- 更新
  - PUT /posts/{postId}
  - Body: 同创建
- 删除
  - DELETE /posts/{postId}
- 状态变更（下架/提交审核等）
  - PATCH /posts/{postId}/status
  - Body: { status: 'draft'|'reviewing'|'published'|'rejected' }
- 互动
  - POST /posts/{postId}/like (幂等切换)
  - POST /posts/{postId}/bookmark (幂等切换)
  - POST /posts/{postId}/share
  - POST /posts/{postId}/report

### 5. 资源（Resource）
- 列表
  - GET /resources?page&pageSize&category?&tag?
  - Resp: { items: ResourceSummary[], total, hasMore }
- 详情
  - GET /resources/{resourceId}
  - Resp: ResourceDetail
- 创建（文件/截图建议走直传，先拿上传凭证）
  - POST /uploads/presign
  - Body: { filename, contentType, scope: 'resource-file'|'resource-screenshot'|'post-image' }
  - Resp: { uploadUrl, publicUrl, headers? }
  - POST /resources
  - Body: { title, content(markdown), tags, version, category, files: string[], screenshots?: string[], requirements?: string[] }
- 更新/删除/状态变更（同帖子）
  - PUT /resources/{id}
  - DELETE /resources/{id}
  - PATCH /resources/{id}/status
- 下载计数与校验
  - POST /resources/{id}/download
  - Resp: { url }（直链或受控下载地址）
- 互动
  - POST /resources/{id}/like | /bookmark | /share | /report

### 6. 公告（Announcement）
- 列表
  - GET /announcements?page&pageSize&tag?
- 详情
  - GET /announcements/{id}
- 互动
  - POST /announcements/{id}/like | /bookmark | /share | /report
- 附件下载
  - GET /announcements/{id}/attachments （或随详情返回）
  - GET /announcements/{id}/attachments/{fileId}/download

### 7. 评论与回复（通用于帖子/资源/公告）
- 列表（分页+总数）
  - GET /{type}/{id}/comments?page&pageSize
  - Resp: { items: Comment[], total, hasMore }
- 发表评论
  - POST /{type}/{id}/comments
  - Body: { content, rating? (资源评价) }
- 回复评论
  - POST /comments/{commentId}/replies
  - Body: { content }
- 点赞/举报评论
  - POST /comments/{commentId}/like (幂等切换)
  - POST /comments/{commentId}/report
- 有用投票（资源评价）
  - POST /comments/{commentId}/helpful (幂等切换)

### 8. 推荐（相关推荐）
- 获取相关推荐（基于当前内容与标签）
  - GET /recommendations?currentItemId=xxx&type=post|resource|announcement&tags=tag1,tag2&limit=6
  - Resp: { items: RecommendedItem[] }

### 9. 分类与发现
- 一级分类（用于资源/学习）
  - GET /categories
  - Resp: { items: Category[] }
- 分类下资源
  - GET /categories/{id}/resources?page&pageSize

### 10. 我的（个人中心/内容）
- 我的资料
  - GET /me
  - Resp: UserProfile
  - PUT /me
  - Body: { name?, bio?, avatar?, email?, location?, website?, skills?: string[] }
- 头像上传：参考上传凭证流程
- 我的统计/周报
  - GET /me/stats
  - Resp: { posts, resources, views, likes }
  - GET /me/weekly-report
  - Resp: WeeklyReport
- 我的内容
  - GET /me/resources?page&pageSize
  - GET /me/posts?page&pageSize
  - GET /me/comments?page&pageSize
  - PATCH /me/{type}/{id}/status
  - DELETE /me/{type}/{id}

### 11. 消息/会话（列表页）
- 会话列表
  - GET /conversations?page&pageSize
  - Resp: { items: Conversation[], total, hasMore }
- 删除会话
  - DELETE /conversations/{id}
- 置顶/取消置顶
  - PATCH /conversations/{id}/pin { pinned: boolean }

### 12. 通知
- 未读数量
  - GET /notifications/count
  - Resp: { count: number }
- 列表/已读
  - GET /notifications?page&pageSize
  - POST /notifications/{id}/read
  - POST /notifications/read-all

### 13. 设置/偏好（本地优先，可选同步）
- 同步主题与偏好（可选）
  - GET /me/preferences
  - PUT /me/preferences { theme?, notifications? }

### 返回结构示例
```json
{
  "items": [
    {
      "id": 2,
      "type": "post",
      "title": "结绳语言学习心得分享",
      "description": "从零基础到熟练掌握...",
      "tags": ["学习心得", "经验分享"],
      "author": { "id": 11, "name": "张同学", "avatar": "https://...", "verified": false },
      "stats": { "likes": 156, "comments": 23, "views": 890, "downloads": 0, "rating": null },
      "flags": { "isTop": false, "isHot": true },
      "publishedAt": "2025-01-14T00:00:00Z"
    }
  ],
  "total": 100,
  "hasMore": true
}
```

### 错误码建议
- 400 参数错误；401 未授权；403 无权限；404 不存在；409 状态冲突；429 频率限制；500 服务器错误

### 备注
- 所有“幂等切换”接口，建议返回最新状态：{ active: boolean, count?: number }
- 评论支持“嵌套回复”，也可采用 `parentId` 扁平化返回，由前端组装树形 