# 绳包管理器 API 文档

## 📋 API 概述

绳包管理器提供完整的 RESTful API，支持用户管理、资源管理、评论系统等功能。所有API都遵循统一的响应格式和错误处理机制。

### 🔗 基础信息

- **Base URL**: `http://127.0.0.1:15201/api`
- **API Version**: v1
- **Content-Type**: `application/json`
- **Authorization**: `Bearer <JWT_TOKEN>`

### 📊 响应格式

#### 成功响应
```json
{
  "code": 0,
  "message": "success",
  "data": {
    // 响应数据
  }
}
```

#### 错误响应
```json
{
  "code": 1001,
  "message": "用户名已存在",
  "data": null
}
```

### 🔐 认证方式

大多数API需要JWT认证，在请求头中包含：
```
Authorization: Bearer <your_jwt_token>
```

## 🚀 认证相关 API

### 用户注册

**POST** `/v1/auth/register`

注册新用户账户。

#### 请求参数
```json
{
  "username": "testuser",
  "password": "password123",
  "email": "test@example.com",
  "nickname": "测试用户",
  "qq_number": "123456789",
  "verification_code": "123456"
}
```

#### 响应数据
```json
{
  "code": 0,
  "message": "注册成功",
  "data": {
    "user": {
      "id": 1,
      "username": "testuser",
      "email": "test@example.com",
      "role": "user",
      "status": 1,
      "created_at": "2024-01-20T10:00:00Z"
    },
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
  }
}
```

### 用户登录

**POST** `/v1/auth/login`

用户登录获取访问令牌。

#### 请求参数
```json
{
  "username": "testuser",
  "password": "password123"
}
```

#### 响应数据
```json
{
  "code": 0,
  "message": "登录成功",
  "data": {
    "user": {
      "id": 1,
      "username": "testuser",
      "email": "test@example.com",
      "role": "user",
      "status": 1
    },
    "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
  }
}
```

### 邮箱验证码登录

**POST** `/v1/auth/login-by-email`

使用邮箱验证码登录。

#### 请求参数
```json
{
  "email": "test@example.com",
  "code": "123456"
}
```

### 获取用户信息

**GET** `/v1/auth/user-info`

获取当前登录用户信息。

**需要认证**: ✅

#### 响应数据
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "id": 1,
    "username": "testuser",
    "email": "test@example.com",
    "nickname": "测试用户",
    "role": "user",
    "status": 1,
    "created_at": "2024-01-20T10:00:00Z",
    "updated_at": "2024-01-20T10:00:00Z"
  }
}
```

### 发送注册验证码

**POST** `/v1/auth/send-register-code`

发送注册验证码到指定邮箱。

#### 请求参数
```json
{
  "email": "test@example.com"
}
```

### 用户退出

**POST** `/v1/auth/logout`

退出登录，清除服务器端会话。

**需要认证**: ✅

## 👥 用户管理 API

### 获取用户列表

**GET** `/v1/users`

获取用户列表，支持分页和过滤。

**需要认证**: ✅  
**需要权限**: `admin`

#### 查询参数
- `page` (int): 页码，默认 1
- `page_size` (int): 每页数量，默认 20
- `role` (string): 角色过滤
- `status` (string): 状态过滤
- `search` (string): 搜索关键词

#### 示例请求
```
GET /v1/users?page=1&page_size=10&role=user&search=test
```

#### 响应数据
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "list": [
      {
        "id": 1,
        "username": "testuser",
        "email": "test@example.com",
        "nickname": "测试用户",
        "role": "user",
        "status": 1,
        "created_at": "2024-01-20T10:00:00Z"
      }
    ],
    "total": 100,
    "page": 1,
    "pageSize": 10,
    "totalPages": 10
  }
}
```

### 获取单个用户

**GET** `/v1/users/{id}`

获取指定用户的详细信息。

**需要认证**: ✅  
**需要权限**: `admin` 或自己的用户ID

#### 路径参数
- `id` (int): 用户ID

### 更新用户信息

**PUT** `/v1/users/{id}`

更新用户信息。

**需要认证**: ✅  
**需要权限**: `admin` 或自己的用户ID

#### 请求参数
```json
{
  "nickname": "新昵称",
  "qq_number": "987654321",
  "role": "elder",
  "ban_status": "normal"
}
```

### 删除用户

**DELETE** `/v1/users/{id}`

删除指定用户。

**需要认证**: ✅  
**需要权限**: `admin`

### 批量删除用户

**DELETE** `/v1/users/batch`

批量删除用户。

**需要认证**: ✅  
**需要权限**: `admin`

#### 请求参数
```json
{
  "usernames": ["user1", "user2", "user3"]
}
```

## 📦 资源包管理 API

### 获取资源包列表

**GET** `/v1/packages`

获取资源包列表，支持分页、搜索和过滤。

#### 查询参数
- `page` (int): 页码，默认 1
- `page_size` (int): 每页数量，默认 20
- `category` (string): 分类过滤
- `status` (string): 状态过滤
- `search` (string): 搜索关键词
- `author` (string): 作者过滤

#### 示例请求
```
GET /v1/packages?page=1&page_size=10&category=rope&status=approved&search=绳结
```

#### 响应数据
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "list": [
      {
        "id": 1,
        "name": "基础绳结包",
        "description": "包含基础绳结的资源包",
        "author": "testuser",
        "version": "1.0.0",
        "category": "rope",
        "status": "approved",
        "file_path": "/uploads/packages/basic_knots.zip",
        "download_count": 100,
        "created_at": "2024-01-20T10:00:00Z",
        "updated_at": "2024-01-20T10:00:00Z"
      }
    ],
    "total": 50,
    "page": 1,
    "pageSize": 10,
    "totalPages": 5
  }
}
```

### 获取资源包详情

**GET** `/v1/packages/{id}`

获取指定资源包的详细信息。

#### 路径参数
- `id` (int): 资源包ID

#### 响应数据
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "id": 1,
    "name": "基础绳结包",
    "description": "包含基础绳结的资源包",
    "author": "testuser",
    "version": "1.0.0",
    "category": "rope",
    "status": "approved",
    "file_path": "/uploads/packages/basic_knots.zip",
    "file_size": 2048576,
    "download_count": 100,
    "rating": 4.5,
    "created_at": "2024-01-20T10:00:00Z",
    "updated_at": "2024-01-20T10:00:00Z"
  }
}
```

### 创建资源包

**POST** `/v1/packages`

创建新的资源包。

**需要认证**: ✅

#### 请求参数 (multipart/form-data)
- `name` (string): 资源包名称
- `description` (string): 描述
- `category` (string): 分类
- `version` (string): 版本号
- `file` (file): 资源包文件

#### 响应数据
```json
{
  "code": 0,
  "message": "资源包创建成功",
  "data": {
    "id": 1,
    "name": "基础绳结包",
    "status": "pending"
  }
}
```

### 更新资源包

**PUT** `/v1/packages/{id}`

更新资源包信息。

**需要认证**: ✅  
**需要权限**: 资源包作者或 `admin`

#### 请求参数
```json
{
  "name": "更新的资源包名称",
  "description": "更新的描述",
  "version": "1.1.0"
}
```

### 删除资源包

**DELETE** `/v1/packages/{id}`

删除指定资源包。

**需要认证**: ✅  
**需要权限**: 资源包作者或 `admin`

### 下载资源包

**GET** `/v1/packages/{id}/download`

下载指定资源包文件。

#### 响应
- Content-Type: `application/octet-stream`
- Content-Disposition: `attachment; filename="package.zip"`

### 审核资源包

**POST** `/v1/packages/{id}/review`

审核资源包（批准或拒绝）。

**需要认证**: ✅  
**需要权限**: `admin` 或 `elder`

#### 请求参数
```json
{
  "action": "approve",
  "comment": "审核通过"
}
```

## 💬 评论系统 API

### 获取资源评论

**GET** `/v1/resources/{resource_id}/comments`

获取指定资源的评论列表。

#### 查询参数
- `page` (int): 页码，默认 1
- `page_size` (int): 每页数量，默认 20
- `status` (string): 状态过滤

#### 响应数据
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "list": [
      {
        "id": 1,
        "user_id": 1,
        "username": "testuser",
        "content": "很好的资源包！",
        "status": "active",
        "likes": 5,
        "replies": [],
        "created_at": "2024-01-20T10:00:00Z"
      }
    ],
    "total": 10,
    "page": 1,
    "page_size": 20
  }
}
```

### 创建评论

**POST** `/v1/comments`

创建新评论。

**需要认证**: ✅

#### 请求参数
```json
{
  "resource_id": 1,
  "content": "这是一个很好的资源包！",
  "parent_id": null
}
```

### 更新评论

**PUT** `/v1/comments/{id}`

更新评论内容。

**需要认证**: ✅  
**需要权限**: 评论作者或 `admin`

#### 请求参数
```json
{
  "content": "更新后的评论内容"
}
```

### 删除评论

**DELETE** `/v1/comments/{id}`

删除指定评论。

**需要认证**: ✅  
**需要权限**: 评论作者或 `admin`

### 点赞评论

**POST** `/v1/comments/{id}/like`

点赞或取消点赞评论。

**需要认证**: ✅

#### 请求参数
```json
{
  "like": true
}
```

## 📂 分类管理 API

### 获取分类列表

**GET** `/v1/categories`

获取所有分类列表。

#### 响应数据
```json
{
  "code": 0,
  "message": "success",
  "data": [
    {
      "id": 1,
      "name": "绳结技巧",
      "description": "各种绳结技巧和教程",
      "icon": "rope",
      "color": "#ff6b6b",
      "sort": 1,
      "enabled": true,
      "package_count": 25
    }
  ]
}
```

### 创建分类

**POST** `/v1/categories`

创建新分类。

**需要认证**: ✅  
**需要权限**: `admin`

#### 请求参数
```json
{
  "name": "新分类",
  "description": "分类描述",
  "icon": "icon-name",
  "color": "#ff6b6b",
  "sort": 1,
  "enabled": true
}
```

### 更新分类

**PUT** `/v1/categories/{id}`

更新分类信息。

**需要认证**: ✅  
**需要权限**: `admin`

### 删除分类

**DELETE** `/v1/categories/{id}`

删除指定分类。

**需要认证**: ✅  
**需要权限**: `admin`

## 📊 统计信息 API

### 获取系统统计

**GET** `/v1/admin/stats`

获取系统整体统计信息。

**需要认证**: ✅  
**需要权限**: `admin`

#### 响应数据
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "users": {
      "total": 1000,
      "active": 800,
      "new_today": 10
    },
    "packages": {
      "total": 500,
      "approved": 450,
      "pending": 30,
      "downloads_today": 100
    },
    "comments": {
      "total": 2000,
      "active": 1800,
      "new_today": 50
    }
  }
}
```

## 🛠️ 管理员 API

### 获取系统日志

**GET** `/v1/admin/logs`

获取系统日志列表。

**需要认证**: ✅  
**需要权限**: `admin`

#### 查询参数
- `page` (int): 页码
- `page_size` (int): 每页数量
- `level` (string): 日志级别 (info, warn, error)
- `search` (string): 搜索关键词

### 备份管理

**POST** `/v1/admin/backup`

创建系统备份。

**需要认证**: ✅  
**需要权限**: `admin`

#### 请求参数
```json
{
  "backup_type": "manual",
  "description": "手动备份"
}
```

### 获取备份列表

**GET** `/v1/admin/backups`

获取备份列表。

**需要认证**: ✅  
**需要权限**: `admin`

## 🔧 系统设置 API

### 获取系统设置

**GET** `/v1/admin/settings`

获取系统设置。

**需要认证**: ✅  
**需要权限**: `admin`

### 更新系统设置

**POST** `/v1/admin/settings`

更新系统设置。

**需要认证**: ✅  
**需要权限**: `admin`

#### 请求参数
```json
{
  "site_title": "绳包管理器",
  "allow_registration": true,
  "max_file_size": 10485760
}
```

## 📧 邮件相关 API

### 邮件设置

**GET** `/v1/admin/mail-settings`

获取邮件设置。

**需要认证**: ✅  
**需要权限**: `admin`

**POST** `/v1/admin/mail-settings`

更新邮件设置。

**需要认证**: ✅  
**需要权限**: `admin`

## 🔍 搜索 API

### 全局搜索

**GET** `/v1/search`

全局搜索资源包、用户等。

#### 查询参数
- `q` (string): 搜索关键词
- `type` (string): 搜索类型 (packages, users, comments)
- `page` (int): 页码
- `page_size` (int): 每页数量

## ❌ 错误代码说明

| 错误码 | 说明 |
|--------|------|
| 0 | 成功 |
| 1001 | 参数错误 |
| 1002 | 用户名已存在 |
| 1003 | 邮箱已存在 |
| 1004 | 用户名或密码错误 |
| 1005 | 验证码错误 |
| 2001 | 未授权访问 |
| 2002 | Token无效 |
| 2003 | Token已过期 |
| 2004 | 权限不足 |
| 3001 | 资源不存在 |
| 3002 | 文件上传失败 |
| 3003 | 文件类型不支持 |
| 3004 | 文件大小超限 |
| 4001 | 数据库错误 |
| 4002 | 服务器内部错误 |
| 5001 | 邮件发送失败 |

## 📝 请求示例

### 使用 curl

```bash
# 用户登录
curl -X POST http://127.0.0.1:15201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"password123"}'

# 获取用户信息
curl -X GET http://127.0.0.1:15201/api/v1/auth/user-info \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 上传资源包
curl -X POST http://127.0.0.1:15201/api/v1/packages \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -F "name=测试资源包" \
  -F "description=这是一个测试资源包" \
  -F "category=rope" \
  -F "file=@package.zip"
```

### 使用 JavaScript (Axios)

```javascript
// 设置默认配置
const api = axios.create({
  baseURL: 'http://127.0.0.1:15201/api',
  headers: {
    'Content-Type': 'application/json'
  }
});

// 添加认证拦截器
api.interceptors.request.use(config => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// 用户登录
const login = async (username, password) => {
  const response = await api.post('/v1/auth/login', {
    username,
    password
  });
  return response.data;
};

// 获取资源包列表
const getPackages = async (page = 1, pageSize = 20) => {
  const response = await api.get('/v1/packages', {
    params: { page, page_size: pageSize }
  });
  return response.data;
};
```

## 🧪 测试工具

项目提供了API测试脚本：

```bash
# 运行API测试
powershell -ExecutionPolicy Bypass -File rope-manager-backend/test_api.ps1
```

---

**API版本**: v1.0  
**最后更新**: 2024年1月20日  
**维护者**: 绳包管理器开发团队 