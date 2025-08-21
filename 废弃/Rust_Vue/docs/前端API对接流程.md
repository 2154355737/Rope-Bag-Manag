# 前端API对接流程 - SQL.js模拟后端系统

## 概述

本项目使用SQL.js在前端模拟完整的后端数据库系统，所有API调用都通过本地SQLite数据库进行，无需真实的后端服务器。

## 系统架构

```
前端Vue应用
    ↓
API客户端 (apiClient)
    ↓
SQL.js数据库 (sqlite.ts)
    ↓
本地SQLite数据库文件
```

## 数据库初始化

系统启动时会自动初始化SQLite数据库，创建以下表结构：

- `user` - 用户表
- `resource` - 资源表
- `comment` - 评论表
- `action_log` - 用户行为日志表
- `resource_record` - 资源操作记录表
- `backup` - 备份记录表
- `announcement` - 公告表
- `log` - 系统日志表
- `settings` - 系统设置表
- `category` - 分类表

## API使用示例

### 1. 用户管理

```typescript
import { apiClient } from '@/utils/apiClient'

// 用户登录
const loginResult = await apiClient.user.login('admin', 'admin123')
if (loginResult.code === 0) {
  apiClient.utils.setCurrentUser(loginResult.data)
}

// 获取用户列表
const usersResult = await apiClient.user.getUsers({
  page: 1,
  pageSize: 10,
  search: 'admin'
})

// 创建用户
await apiClient.user.createUser({
  username: 'newuser',
  password: 'password123',
  role: 'user',
  nickname: '新用户'
})
```

### 2. 资源管理

```typescript
// 获取资源列表
const resourcesResult = await apiClient.community.getResources({
  page: 1,
  pageSize: 12,
  category: '游戏',
  search: '示例'
})

// 创建资源
await apiClient.community.createResource({
  title: '新资源',
  description: '资源描述',
  category: '工具',
  tags: ['实用', '工具'],
  status: 'active',
  file: fileInput.files[0],
  cover: coverInput.files[0]
})

// 记录资源操作
await apiClient.utils.logResourceAction(resourceId, 'download')
```

### 3. 评论管理

```typescript
// 获取资源评论
const commentsResult = await apiClient.comment.getComments(resourceId, {
  page: 1,
  pageSize: 10
})

// 创建评论
await apiClient.comment.createComment({
  content: '这是一条评论',
  resource_id: resourceId
})
```

### 4. 公告管理

```typescript
// 获取公告列表
const announcementsResult = await apiClient.announcement.getAnnouncements({
  page: 1,
  pageSize: 10,
  type: 'info'
})

// 创建公告
await apiClient.announcement.createAnnouncement({
  title: '系统公告',
  content: '公告内容',
  type: 'info',
  priority: 1
})
```

### 5. 系统设置

```typescript
// 获取系统设置
const settingsResult = await apiClient.settings.getSettings()

// 更新设置
await apiClient.settings.updateSettings({
  theme: {
    community_theme: 'dark',
    admin_theme: 'light'
  },
  feature_flags: {
    enable_registration: true,
    enable_community: true
  }
})
```

### 6. 备份管理

```typescript
// 创建备份
await apiClient.backup.createBackup('backup-2024-01-01.sqlite')

// 获取备份列表
const backupsResult = await apiClient.backup.getBackups({
  page: 1,
  pageSize: 10
})
```

### 7. 用户行为记录

```typescript
// 记录用户行为
await apiClient.utils.logAction('login', '用户登录')

// 获取行为记录
const actionsResult = await apiClient.userAction.getUserActions({
  page: 1,
  pageSize: 10,
  action: 'login'
})
```

## 权限控制

系统内置权限控制机制：

```typescript
// 检查用户权限
if (apiClient.utils.hasPermission('manage_users')) {
  // 可以管理用户
}

if (apiClient.utils.hasPermission('create_resources')) {
  // 可以创建资源
}
```

权限级别：
- `admin` - 管理员，拥有所有权限
- `moderator` - 版主，拥有部分管理权限
- `user` - 普通用户，拥有基本操作权限

## 数据库导出导入

```typescript
import { databaseExport } from '@/utils/sqliteExport'

// 导出数据库
await databaseExport.exportToFile('database.sqlite')

// 导出为JSON
await databaseExport.exportJSONToFile('database.json')

// 获取数据库信息
const info = await databaseExport.getDatabaseInfo()
```

## 错误处理

所有API调用都返回统一的响应格式：

```typescript
interface ApiResponse<T = any> {
  code: number    // 0表示成功，非0表示失败
  msg: string     // 响应消息
  data?: T        // 响应数据
}
```

错误处理示例：

```typescript
try {
  const result = await apiClient.user.login('admin', 'wrong_password')
  if (result.code === 0) {
    // 登录成功
    console.log('登录成功:', result.data)
  } else {
    // 登录失败
    console.error('登录失败:', result.msg)
  }
} catch (error) {
  console.error('API调用异常:', error)
}
```

## 开发注意事项

1. **数据库初始化**: 确保在应用启动时调用 `initDB()`
2. **用户状态管理**: 使用 `apiClient.utils` 管理用户登录状态
3. **权限检查**: 在关键操作前检查用户权限
4. **错误处理**: 所有API调用都要进行错误处理
5. **数据持久化**: 数据库数据存储在内存中，页面刷新会重置

## 迁移到真实后端

当需要迁移到真实后端时，只需要：

1. 修改API文件中的实现，将SQL.js调用替换为HTTP请求
2. 保持API接口不变，确保前端代码无需修改
3. 使用 `databaseExport.exportToJSON()` 导出数据
4. 在真实后端中导入数据

这种设计确保了前端代码的稳定性和可维护性。 