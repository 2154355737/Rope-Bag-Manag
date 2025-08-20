## 结绳社区 App 数据模型需求（Data Models）

> 模型来源：`src/screens/*`, `src/components/*`, `src/utils/recommendations.ts`, `src/components/comment-section.tsx` 等模拟数据与类型。

### 基础类型
- **Identifier**: number
- **ISODateTime**: string
- **URL**: string

### 用户与作者
```ts
interface AuthorSummary {
  id: number
  name: string
  avatar: URL
  verified?: boolean
  role?: string // 管理员、官方等（公告）
}

interface UserProfile {
  id: number
  name: string
  avatar: URL
  bio?: string
  level?: string // 如: Lv.3 进阶开发者
  email?: string
  location?: string
  website?: string
  skills?: string[]
  stats?: {
    posts: number
    resources: number
    views: number
    likes: number
  }
}

interface UserProfileSummary {
  id: number
  name: string
  avatar: URL
}
```

### 信息流卡片（统一展示用）
```ts
interface ContentCard {
  id: number
  type: 'post' | 'resource' | 'announcement'
  title: string
  description?: string
  tags?: string[]
  author?: AuthorSummary
  image?: URL // 列表缩略图
  stats: {
    likes?: number
    comments?: number
    views?: number
    downloads?: number
    rating?: number
  }
  flags?: { isTop?: boolean; isHot?: boolean; isPinned?: boolean }
  publishedAt: ISODateTime
}
```

### 帖子（Post）
```ts
interface PostSummary extends ContentCard {
  type: 'post'
}

interface PostDetail {
  id: number
  author: AuthorSummary
  title: string
  content: string // markdown
  images?: URL[]
  codeSnippet?: string
  tags: string[]
  likes: number
  comments: number
  views: number
  publishDate: ISODateTime
}
```

### 资源（Resource）
```ts
interface ResourceSummary extends ContentCard {
  type: 'resource'
  category?: string
  version?: string
}

interface ResourceFile {
  id?: number
  name: string
  size: string | number // 可支持字节数，前端再格式化
  type?: string // ZIP/MD 等
  url?: URL // 可选：直链
}

interface ResourceDetail {
  id: number
  title: string
  version: string
  author: AuthorSummary
  description: string // markdown
  category: string
  tags: string[]
  screenshots?: URL[]
  fileSize?: string // 汇总显示（可选）
  files: ResourceFile[]
  requirements?: string[]
  likes: number
  views: number
  rating?: number
  reviewCount?: number
  downloadCount: number
  lastUpdated?: ISODateTime
  publishDate: ISODateTime
  safetyStatus?: 'verified' | 'unknown' | 'risk'
}
```

### 公告（Announcement）
```ts
interface AnnouncementSummary extends ContentCard {
  type: 'announcement'
}

interface AnnouncementAttachment {
  id: number
  name: string
  size: string
  url: URL
}

interface AnnouncementDetail {
  id: number
  title: string
  type: 'important' | 'info' | 'warning' | 'update'
  priority: 'high' | 'medium' | 'low'
  author: AuthorSummary & { role?: string }
  content: string // markdown-like
  tags: string[]
  views: number
  likes: number
  comments: number
  isPinned?: boolean
  publishDate: ISODateTime
  effectiveDate?: ISODateTime
  expiryDate?: ISODateTime
  attachments?: AnnouncementAttachment[]
  relatedLinks?: { title: string; url: URL; description?: string }[]
}
```

### 评论（Comment）
来源：`CommentSection` 组件
```ts
interface Comment {
  id: number
  author: AuthorSummary
  content: string
  time: string // ISO 建议，但组件接受人类可读字符串
  likes: number
  isLiked?: boolean
  replies?: Comment[]
  rating?: number // 1-5，可用于资源评价
  helpful?: number // 评价“有用”票数
}
```

分页返回：
```ts
interface PaginatedResult<T> {
  items: T[]
  total: number
  hasMore: boolean
}
```

### 推荐（RecommendedItem）
来源：`src/components/related-recommendations.tsx`, `src/utils/recommendations.ts`
```ts
interface RecommendedItem {
  id: number
  type: 'post' | 'resource' | 'announcement'
  title: string
  description?: string
  author?: AuthorSummary
  image?: URL
  tags?: string[]
  stats: { likes?: number; comments?: number; views?: number; downloads?: number; rating?: number }
  time: string // 人类可读；后端也可提供 ISO 时间，由前端格式化
  isHot?: boolean
  isPinned?: boolean
}
```

### 分类（Category）
```ts
interface Category {
  id: string | number
  name: string
}
```

### 会话（Conversation）
来源：`messages-screen.tsx`
```ts
interface Conversation {
  id: number
  name: string
  avatar: URL
  lastMessage: string
  time: string // 人类可读
  unread: number
  online?: boolean
  isGroup?: boolean
  pinned?: boolean
}
```

### 上传与媒体（建议）
```ts
interface PresignRequest {
  filename: string
  contentType: string
  scope: 'resource-file' | 'resource-screenshot' | 'post-image' | 'avatar'
}

interface PresignResponse {
  uploadUrl: URL
  publicUrl: URL
  headers?: Record<string, string>
}
```

### 偏好/设置（可选服务端同步）
```ts
interface UserPreferences {
  theme?: 'light' | 'dark' | 'system'
  notifications?: boolean
  autoUpdate?: boolean
  safeArea?: {
    autoDetect: boolean
    previewMode?: boolean
    topMargin: number
    bottomMargin: number
    leftMargin: number
    rightMargin: number
  }
}
```

### 交互通用响应（建议）
```ts
interface ToggleActionResult {
  active: boolean
  count?: number
}
```

### 命名与一致性
- 所有列表均推荐返回 `PaginatedResult<T>`
- 时间字段统一使用 `publishDate`/`lastUpdated` 等具有语义的命名
- 互动统计放入 `stats`，布尔标签放入 `flags`

### 字段与组件对应关系（要点）
- 帖子详情：`PostDetail` 映射 `post-detail-screen.tsx`
- 资源详情：`ResourceDetail` 映射 `resource-detail-screen.tsx`
- 公告详情：`AnnouncementDetail` 映射 `announcement-detail-screen.tsx`
- 评论：`Comment` 与 `CommentSection` 完全一致（含分页扩展）
- 相关推荐：`RecommendedItem` 与 `related-recommendations.tsx` 对齐
- 我的内容：`PostSummary`/`ResourceSummary` + 状态 `status: 'published'|'draft'|'reviewing'|'rejected'` 