# 前端API需求表

## 📋 概述
本文档列出了前端所有功能模块需要的API接口，用于清理硬编码数据并实现完整的前后端对接。

## 🔍 前端功能模块分析

### 1. 用户认证模块
- **登录页面** (`Login.vue`)
- **用户仪表盘** (`UserDashboard.vue`)
- **用户管理** (`UserManage.vue`)

### 2. 系统管理模块
- **主仪表盘** (`Dashboard.vue`)
- **统计信息** (`Stats.vue`)
- **日志查看** (`LogView.vue`)
- **系统设置** (`ThemeSettings.vue`)

### 3. 资源管理模块
- **绳包管理** (`PackageManage.vue`)
- **资源记录** (`ResourceRecord.vue`)
- **社区首页** (`CommunityHome.vue`)

### 4. 内容管理模块
- **评论管理** (`CommentManage.vue`)
- **公告管理** (`AnnouncementManage.vue`)

### 5. 系统维护模块
- **数据备份** (`BackupManage.vue`)
- **用户行为记录** (`UserActions.vue`)
- **用户行为日志** (`UserActionLog.vue`)

## 🚀 API接口需求表

### 1. 用户认证相关API

#### 1.1 用户登录
```typescript
POST /api/v1/auth/login
Request: { username: string, password: string }
Response: { code: number, message: string, data: { user: User, token: string } }
```

#### 1.2 用户注册
```typescript
POST /api/v1/auth/register
Request: { username: string, password: string, nickname: string }
Response: { code: number, message: string, data: { user: User } }
```

#### 1.3 获取用户信息
```typescript
GET /api/v1/auth/user-info
Request: { username: string }
Response: { code: number, message: string, data: User }
```

#### 1.4 用户签到
```typescript
POST /api/v1/auth/sign-in
Request: { username: string }
Response: { code: number, message: string, data: { sign_in_date: string, points: number } }
```

#### 1.5 修改密码
```typescript
PUT /api/v1/auth/change-password
Request: { username: string, old_password: string, new_password: string }
Response: { code: number, message: string }
```

#### 1.6 修改昵称
```typescript
PUT /api/v1/auth/change-nickname
Request: { username: string, nickname: string }
Response: { code: number, message: string }
```

#### 1.7 获取所有昵称
```typescript
GET /api/v1/auth/nicknames
Response: { code: number, message: string, data: string[] }
```

### 2. 用户管理相关API

#### 2.1 获取用户列表
```typescript
GET /api/v1/users
Request: { admin_username: string, admin_password: string }
Response: { code: number, message: string, data: { users: User[] } }
```

#### 2.2 获取单个用户信息
```typescript
GET /api/v1/users/{username}
Request: { admin_username: string, admin_password: string }
Response: { code: number, message: string, data: User }
```

#### 2.3 创建用户
```typescript
POST /api/v1/users
Request: { 
  username: string, 
  password: string, 
  role?: string, 
  star?: number,
  qq_number?: string,
  avatar_url?: string,
  admin_username: string,
  admin_password: string
}
Response: { code: number, message: string, data: User }
```

#### 2.4 更新用户信息
```typescript
PUT /api/v1/users/{username}
Request: { 
  star?: number,
  role?: string,
  ban_status?: string,
  ban_reason?: string,
  qq_number?: string,
  avatar_url?: string,
  is_admin?: boolean,
  admin_username: string,
  admin_password: string
}
Response: { code: number, message: string }
```

#### 2.5 删除用户
```typescript
DELETE /api/v1/users/{username}
Request: { admin_username: string, admin_password: string }
Response: { code: number, message: string }
```

#### 2.6 批量删除用户
```typescript
DELETE /api/v1/users/batch
Request: { usernames: string[], admin_username: string, admin_password: string }
Response: { code: number, message: string }
```

#### 2.7 设置用户角色
```typescript
PUT /api/v1/users/{username}/role
Request: { role: string, admin_username: string, admin_password: string }
Response: { code: number, message: string }
```

#### 2.8 设置用户星级
```typescript
PUT /api/v1/users/{username}/star
Request: { star: number, admin_username: string, admin_password: string }
Response: { code: number, message: string }
```

#### 2.9 封禁/解封用户
```typescript
PUT /api/v1/users/{username}/ban
Request: { banned: boolean, admin_username: string, admin_password: string }
Response: { code: number, message: string }
```

### 3. 绳包管理相关API

#### 3.1 获取绳包列表
```typescript
GET /api/v1/packages
Request: { username: string }
Response: { code: number, message: string, data: Package[] }
```

#### 3.2 获取单个绳包
```typescript
GET /api/v1/packages/{id}
Response: { code: number, message: string, data: Package }
```

#### 3.3 创建绳包
```typescript
POST /api/v1/packages
Request: { 
  name: string,
  author: string,
  version: string,
  desc: string,
  url: string,
  username: string,
  admin_password?: string
}
Response: { code: number, message: string, data: Package }
```

#### 3.4 更新绳包
```typescript
PUT /api/v1/packages/{id}
Request: { 
  name: string,
  author: string,
  version: string,
  desc: string,
  url: string,
  username: string,
  admin_password?: string
}
Response: { code: number, message: string }
```

#### 3.5 删除绳包
```typescript
DELETE /api/v1/packages/{id}
Request: { username: string, admin_password?: string }
Response: { code: number, message: string }
```

#### 3.6 下载绳包
```typescript
GET /api/v1/packages/{id}/download
Response: File download
```

### 4. 分类管理相关API

#### 4.1 获取分类列表
```typescript
GET /api/v1/categories
Request: { admin_username: string, admin_password: string }
Response: { code: number, message: string, data: Category[] }
```

#### 4.2 创建分类
```typescript
POST /api/v1/categories
Request: { 
  name: string,
  description: string,
  enabled: boolean,
  admin_username: string,
  admin_password: string
}
Response: { code: number, message: string, data: Category }
```

#### 4.3 更新分类
```typescript
PUT /api/v1/categories/{id}
Request: { 
  name: string,
  description: string,
  enabled: boolean,
  admin_username: string,
  admin_password: string
}
Response: { code: number, message: string }
```

#### 4.4 删除分类
```typescript
DELETE /api/v1/categories/{id}
Request: { admin_username: string, admin_password: string }
Response: { code: number, message: string }
```

### 5. 评论管理相关API

#### 5.1 获取评论列表
```typescript
GET /api/v1/comments
Response: { code: number, message: string, data: Comment[] }
```

#### 5.2 创建评论
```typescript
POST /api/v1/comments
Request: { 
  content: string,
  author: string,
  package_id: number,
  parent_id?: number
}
Response: { code: number, message: string, data: Comment }
```

#### 5.3 更新评论
```typescript
PUT /api/v1/comments/{id}
Request: { content: string, status?: string }
Response: { code: number, message: string }
```

#### 5.4 删除评论
```typescript
DELETE /api/v1/comments/{id}
Response: { code: number, message: string }
```

### 6. 公告管理相关API

#### 6.1 获取公告列表
```typescript
GET /api/v1/announcements
Response: { code: number, message: string, data: Announcement[] }
```

#### 6.2 创建公告
```typescript
POST /api/v1/announcements
Request: { 
  title: string,
  content: string,
  author: string,
  priority: string,
  active: boolean
}
Response: { code: number, message: string, data: Announcement }
```

#### 6.3 更新公告
```typescript
PUT /api/v1/announcements/{id}
Request: { 
  title: string,
  content: string,
  priority: string,
  active: boolean
}
Response: { code: number, message: string }
```

#### 6.4 删除公告
```typescript
DELETE /api/v1/announcements/{id}
Response: { code: number, message: string }
```

### 7. 备份管理相关API

#### 7.1 获取备份列表
```typescript
GET /api/v1/backup-records
Response: { code: number, message: string, data: BackupRecord[] }
```

#### 7.2 创建备份
```typescript
POST /api/v1/backup-records
Request: { 
  name: string,
  description: string,
  type: string
}
Response: { code: number, message: string, data: BackupRecord }
```

#### 7.3 下载备份
```typescript
GET /api/v1/backup-records/{id}/download
Response: File download
```

#### 7.4 恢复备份
```typescript
POST /api/v1/backup-records/{id}/restore
Response: { code: number, message: string }
```

#### 7.5 删除备份
```typescript
DELETE /api/v1/backup-records/{id}
Response: { code: number, message: string }
```

### 8. 用户行为记录相关API

#### 8.1 获取用户行为列表
```typescript
GET /api/v1/user-actions
Response: { code: number, message: string, data: UserAction[] }
```

#### 8.2 创建用户行为记录
```typescript
POST /api/v1/user-actions
Request: { 
  user_id: string,
  action_type: string,
  description: string,
  ip_address?: string,
  user_agent?: string
}
Response: { code: number, message: string, data: UserAction }
```

#### 8.3 删除用户行为记录
```typescript
DELETE /api/v1/user-actions/{id}
Response: { code: number, message: string }
```

### 9. 资源记录相关API

#### 9.1 获取资源记录列表
```typescript
GET /api/v1/resource-records
Response: { code: number, message: string, data: ResourceRecord[] }
```

#### 9.2 创建资源记录
```typescript
POST /api/v1/resource-records
Request: { 
  name: string,
  type: string,
  size: number,
  url: string,
  description: string
}
Response: { code: number, message: string, data: ResourceRecord }
```

#### 9.3 删除资源记录
```typescript
DELETE /api/v1/resource-records/{id}
Response: { code: number, message: string }
```

### 10. 统计相关API

#### 10.1 获取仪表盘数据
```typescript
GET /api/v1/dashboard
Response: { 
  code: number, 
  message: string, 
  data: {
    total_users: number,
    active_users: number,
    total_packages: number,
    available_packages: number,
    total_logs: number,
    today_logs: number,
    system_status: string,
    uptime: string,
    cpu_usage: number,
    memory_usage: number
  }
}
```

#### 10.2 获取统计数据
```typescript
GET /api/v1/stats/api-counts
Response: { code: number, message: string, data: StatsData }
```

#### 10.3 获取日志统计
```typescript
GET /api/v1/logs/stats
Response: { code: number, message: string, data: LogStats }
```

#### 10.4 获取日志条目
```typescript
GET /api/v1/logs/entries
Response: { code: number, message: string, data: LogEntry[] }
```

#### 10.5 清除日志
```typescript
DELETE /api/v1/logs/clear
Response: { code: number, message: string }
```

### 11. 设置相关API

#### 11.1 获取设置
```typescript
GET /api/v1/settings
Response: { code: number, message: string, data: Settings }
```

#### 11.2 更新设置
```typescript
PUT /api/v1/settings
Request: Settings
Response: { code: number, message: string }
```

#### 11.3 检查功能
```typescript
GET /api/v1/check-feature
Request: { feature: string }
Response: { code: number, message: string, data: { enabled: boolean } }
```

### 12. 社区相关API

#### 12.1 获取社区资源列表
```typescript
GET /api/v1/community/resources
Response: { code: number, message: string, data: Resource[] }
```

#### 12.2 获取热门资源
```typescript
GET /api/v1/community/hot
Response: { code: number, message: string, data: Resource[] }
```

#### 12.3 获取资源详情
```typescript
GET /api/v1/community/resources/{id}
Response: { code: number, message: string, data: Resource }
```

## 📊 数据模型定义

### User 用户模型
```typescript
interface User {
  id: number;
  username: string;
  nickname: string;
  role: string;
  star: number;
  online_status: string;
  ban_status: string;
  ban_reason?: string;
  qq_number?: string;
  avatar_url?: string;
  is_admin: boolean;
  created_at: string;
  updated_at: string;
}
```

### Package 绳包模型
```typescript
interface Package {
  id: number;
  name: string;
  author: string;
  version: string;
  desc: string;
  url: string;
  category: string;
  status: string;
  downloads: number;
  created_at: string;
  updated_at: string;
}
```

### Category 分类模型
```typescript
interface Category {
  id: number;
  name: string;
  description: string;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}
```

### Comment 评论模型
```typescript
interface Comment {
  id: number;
  content: string;
  author: string;
  package_id: number;
  parent_id?: number;
  status: string;
  created_at: string;
  updated_at: string;
}
```

### Announcement 公告模型
```typescript
interface Announcement {
  id: number;
  title: string;
  content: string;
  author: string;
  priority: string;
  active: boolean;
  created_at: string;
  updated_at: string;
}
```

### BackupRecord 备份记录模型
```typescript
interface BackupRecord {
  id: number;
  name: string;
  description: string;
  type: string;
  size: number;
  status: string;
  created_at: string;
  updated_at: string;
}
```

### UserAction 用户行为模型
```typescript
interface UserAction {
  id: number;
  user_id: string;
  action_type: string;
  description: string;
  ip_address?: string;
  user_agent?: string;
  created_at: string;
}
```

### ResourceRecord 资源记录模型
```typescript
interface ResourceRecord {
  id: number;
  name: string;
  type: string;
  size: number;
  url: string;
  description: string;
  created_at: string;
  updated_at: string;
}
```

## 🔧 实现优先级

### 高优先级 (P0)
1. 用户认证相关API
2. 用户管理相关API
3. 绳包管理相关API
4. 仪表盘数据API

### 中优先级 (P1)
1. 分类管理相关API
2. 评论管理相关API
3. 公告管理相关API
4. 统计相关API

### 低优先级 (P2)
1. 备份管理相关API
2. 用户行为记录相关API
3. 资源记录相关API
4. 设置相关API
5. 社区相关API

## 📝 注意事项

1. **统一响应格式**: 所有API都应返回统一的响应格式
2. **错误处理**: 完善的错误处理机制
3. **权限控制**: 管理员API需要验证管理员权限
4. **数据验证**: 请求参数的数据验证
5. **缓存策略**: 合理使用缓存提高性能
6. **日志记录**: 重要操作的日志记录
7. **API版本控制**: 使用v1版本控制
8. **文档更新**: 及时更新API文档 