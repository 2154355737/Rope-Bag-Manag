# 绳包管理器后端 API 对接文档

## 📋 目录

- [基础信息](#基础信息)
- [认证系统](#认证系统)
- [用户管理](#用户管理)
- [资源包管理](#资源包管理)
- [评论系统](#评论系统)
- [分类管理](#分类管理)
- [订阅系统](#订阅系统)
- [管理员功能](#管理员功能)
- [系统监控](#系统监控)
- [错误处理](#错误处理)
- [使用示例](#使用示例)
- [常见问题解答](#常见问题解答-faq)

---

## 🔧 基础信息

### 服务器配置
| 配置项 | 值 |
|--------|-----|
| 基础URL | `http://127.0.0.1:15201` |
| API版本 | v1 |
| API前缀 | `/api/v1` |
| 端口 | 15201 |

### 通用响应格式
```json
{
  "code": 0,           // 状态码，0表示成功
  "message": "success", // 响应消息
  "data": {}           // 响应数据(可选)
}
```

### 认证方式
系统支持两种认证方式，按优先级顺序：

#### 1. Authorization 头部认证（推荐）
```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

#### 2. Cookie 认证
```http
Cookie: auth_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### 权限等级说明
| 角色 | 英文标识 | 权限说明 |
|------|----------|----------|
| 普通用户 | user | 基础功能使用权限 |
| 元老 | elder | 拥有部分管理权限，可审核资源 |
| 版主 | moderator | 拥有内容管理权限 |
| 管理员 | admin | 拥有所有系统权限 |

### 认证参数详细说明

#### 需要登录认证的接口
**参数要求**:
- **必须**: 有效的JWT Token
- **提交方式**: 
  - Header: `Authorization: Bearer <token>`
  - 或 Cookie: `auth_token=<token>`

#### 需要管理员权限的接口
**参数要求**:
- **必须**: 有效的JWT Token
- **角色限制**: Token中的role字段必须为 `admin`
- **提交方式**: 
  - Header: `Authorization: Bearer <token>`
  - 或 Cookie: `auth_token=<token>`

#### 需要元老或管理员权限的接口
**参数要求**:
- **必须**: 有效的JWT Token  
- **角色限制**: Token中的role字段必须为 `elder` 或 `admin`
- **提交方式**: 
  - Header: `Authorization: Bearer <token>`
  - 或 Cookie: `auth_token=<token>`

#### 需要资源所有者或管理员权限的接口
**参数要求**:
- **必须**: 有效的JWT Token
- **权限判断**: 
  - Token中的user_id与资源所有者ID一致
  - 或 Token中的role字段为 `admin`
- **提交方式**: 
  - Header: `Authorization: Bearer <token>`
  - 或 Cookie: `auth_token=<token>`

### 状态码说明
| 状态码 | 说明 |
|--------|------|
| 0 | 成功 |
| 400 | 请求参数错误 |
| 401 | 未认证 |
| 403 | 权限不足 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

---

## 🏥 系统健康检查

### 健康检查
检查服务器运行状态

**接口信息**
- **路径**: `GET /health`
- **认证**: 无需认证

**请求示例**
```bash
curl -X GET http://127.0.0.1:15201/health
```

**响应示例**
```json
{
  "status": "ok",
  "message": "绳包管理器后端服务运行正常",
  "timestamp": "2025-07-01T10:00:00Z"
}
```

---

## 🔐 认证系统

### 1. 用户登录
使用用户名和密码登录系统

**接口信息**
- **路径**: `POST /api/v1/auth/login`
- **认证**: 无需认证

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| username | string | ✅ | 用户名 |
| password | string | ✅ | 密码 |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'
```

**响应示例**
```json
{
  "code": 0,
  "message": "登录成功",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 1,
      "username": "testuser",
      "email": "test@example.com",
      "nickname": "测试用户",
      "role": "user",
      "star": 0,
      "ban_status": "normal",
      "created_at": "2023-12-01T10:00:00Z",
      "is_admin": false
    }
  }
}
```

### 2. 邮箱登录
使用邮箱和验证码登录

**接口信息**
- **路径**: `POST /api/v1/auth/login-by-email`
- **认证**: 无需认证

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| email | string | ✅ | 邮箱地址 |
| code | string | ✅ | 验证码 |

### 3. 用户注册
注册新用户账号

**接口信息**
- **路径**: `POST /api/v1/auth/register`
- **认证**: 无需认证

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| username | string | ✅ | 用户名 |
| email | string | ✅ | 邮箱地址 |
| password | string | ✅ | 密码 |
| nickname | string | ❌ | 昵称 |
| qq_number | string | ❌ | QQ号 |
| verification_code | string | ✅ | 邮箱验证码 |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "email": "newuser@example.com",
    "password": "password123",
    "nickname": "新用户",
    "verification_code": "123456"
  }'
```

### 4. 获取用户信息
获取当前登录用户的详细信息

**接口信息**
- **路径**: `GET /api/v1/auth/user-info`
- **认证**: 需要Token

**请求示例**
```bash
curl -X GET http://127.0.0.1:15201/api/v1/auth/user-info \
  -H "Authorization: Bearer <token>"
```

### 5. 验证认证状态
验证当前Token是否有效

**接口信息**
- **路径**: `GET /api/v1/auth/verify`
- **认证**: 需要Token

### 6. 退出登录
清除用户登录状态

**接口信息**
- **路径**: `POST /api/v1/auth/logout`
- **认证**: 需要Token

### 7. 邮箱验证相关

#### 发送注册验证码
注册新用户前需要先验证邮箱地址

**接口信息**
- **路径**: `POST /api/v1/auth/send-register-code`
- **认证**: 无需认证
- **限制**: 同一邮箱每分钟最多发送1次，每天最多发送10次

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| email | string | ✅ | 邮箱地址，必须是有效的邮箱格式 |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/send-register-code \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@example.com"
  }'
```

**成功响应**
```json
{
  "code": 0,
  "message": "验证码已发送到您的邮箱，请查收",
  "data": {
    "email": "newuser@example.com",
    "expires_in": 300,
    "next_send_time": 60
  }
}
```

**错误响应示例**
```json
// 邮箱格式错误
{
  "code": 400,
  "message": "邮箱格式不正确"
}

// 发送过于频繁
{
  "code": 429,
  "message": "发送过于频繁，请60秒后再试"
}

// 邮箱已被注册
{
  "code": 409,
  "message": "该邮箱已被注册"
}
```

#### 发送登录验证码
用于邮箱登录时获取验证码

**接口信息**
- **路径**: `POST /api/v1/auth/send-login-code`
- **认证**: 无需认证
- **限制**: 同一邮箱每分钟最多发送1次

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| email | string | ✅ | 已注册的邮箱地址 |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/send-login-code \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com"
  }'
```

**成功响应**
```json
{
  "code": 0,
  "message": "登录验证码已发送",
  "data": {
    "email": "user@example.com",
    "expires_in": 300
  }
}
```

#### 验证验证码
验证用户输入的邮箱验证码是否正确

**接口信息**
- **路径**: `POST /api/v1/auth/verify-code`
- **认证**: 无需认证
- **限制**: 验证码5分钟内有效，最多尝试5次

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| email | string | ✅ | 邮箱地址 |
| code | string | ✅ | 6位数字验证码 |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/verify-code \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "code": "123456"
  }'
```

**成功响应**
```json
{
  "code": 0,
  "message": "验证码验证成功",
  "data": {
    "verified": true,
    "email": "user@example.com"
  }
}
```

**错误响应示例**
```json
// 验证码错误
{
  "code": 400,
  "message": "验证码错误"
}

// 验证码已过期
{
  "code": 410,
  "message": "验证码已过期，请重新获取"
}

// 尝试次数过多
{
  "code": 429,
  "message": "验证失败次数过多，请重新获取验证码"
}
```

### 8. 密码重置

#### 请求重置密码
忘记密码时，通过邮箱获取重置链接

**接口信息**
- **路径**: `POST /api/v1/auth/reset-request`
- **认证**: 无需认证
- **限制**: 同一邮箱每小时最多请求3次

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| email | string | ✅ | 已注册的邮箱地址 |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/reset-request \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com"
  }'
```

**成功响应**
```json
{
  "code": 0,
  "message": "密码重置邮件已发送，请查收邮箱",
  "data": {
    "email": "user@example.com",
    "expires_in": 1800,
    "sent_at": "2023-12-01T10:00:00Z"
  }
}
```

**错误响应示例**
```json
// 邮箱未注册
{
  "code": 404,
  "message": "该邮箱尚未注册"
}

// 请求过于频繁
{
  "code": 429,
  "message": "请求过于频繁，请1小时后再试"
}
```

#### 重置密码
使用邮件中的重置令牌设置新密码

**接口信息**
- **路径**: `POST /api/v1/auth/reset-password`
- **认证**: 无需认证
- **限制**: 重置令牌30分钟内有效，只能使用一次

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| email | string | ✅ | 邮箱地址 |
| token | string | ✅ | 邮件中的重置令牌 |
| new_password | string | ✅ | 新密码(至少8位，包含字母和数字) |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/reset-password \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "new_password": "newPassword123"
  }'
```

**成功响应**
```json
{
  "code": 0,
  "message": "密码重置成功",
  "data": {
    "email": "user@example.com",
    "reset_at": "2023-12-01T10:30:00Z"
  }
}
```

**错误响应示例**
```json
// 重置令牌无效
{
  "code": 400,
  "message": "重置令牌无效或已过期"
}

// 密码格式错误
{
  "code": 400,
  "message": "密码至少8位，且包含字母和数字"
}

// 令牌已使用
{
  "code": 410,
  "message": "重置令牌已使用，请重新申请"
}
```

---

## 👥 用户管理

### 1. 获取用户列表
获取系统中所有用户的列表

**接口信息**
- **路径**: `GET /api/v1/users`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**响应示例**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "list": [
      {
        "id": 1,
        "username": "user1",
        "email": "user1@example.com",
        "nickname": "用户1",
        "role": "user",
        "star": 5,
        "ban_status": "normal",
        "login_count": 10,
        "upload_count": 3,
        "download_count": 15,
        "created_at": "2023-12-01T10:00:00Z",
        "last_login": "2023-12-01T15:30:00Z",
        "is_admin": false
      }
    ],
    "total": 100,
    "page": 1,
    "pageSize": 20,
    "totalPages": 5
  }
}
```

### 2. 获取单个用户信息
根据用户ID获取用户详细信息

**接口信息**
- **路径**: `GET /api/v1/users/{id}`
- **认证**: 无需认证

**路径参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| id | integer | 用户ID |

### 3. 个人资料管理

#### 获取当前用户资料
**接口信息**
- **路径**: `GET /api/v1/users/profile`
- **认证**: 需要Token

#### 更新当前用户资料
**接口信息**
- **路径**: `PUT /api/v1/users/profile`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| nickname | string | ❌ | 昵称 |
| email | string | ❌ | 邮箱 |
| qq_number | string | ❌ | QQ号 |
| avatar_url | string | ❌ | 头像URL |

### 4. 用户内容管理

#### 获取我的资源
**接口信息**
- **路径**: `GET /api/v1/users/my-resources`
- **认证**: 需要Token

#### 获取我的评论
**接口信息**
- **路径**: `GET /api/v1/users/my-comments`
- **认证**: 需要Token

### 5. 密码管理

#### 修改密码
**接口信息**
- **路径**: `POST /api/v1/users/change-password`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| old_password | string | ✅ | 旧密码 |
| new_password | string | ✅ | 新密码 |

### 6. 管理员用户操作

#### 更新用户信息
**接口信息**
- **路径**: `PUT /api/v1/users/{id}`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

#### 删除用户
**接口信息**
- **路径**: `DELETE /api/v1/users/{id}`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

#### 批量删除用户
**接口信息**
- **路径**: `DELETE /api/v1/users/batch`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| ids | array | ✅ | 用户ID数组 |

---

## 📦 资源包管理

### 1. 获取资源包列表
获取系统中的资源包列表，支持分页和筛选

**接口信息**
- **路径**: `GET /api/v1/packages`
- **认证**: 无需认证

**查询参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| page | integer | ❌ | 页码(默认1) |
| page_size | integer | ❌ | 每页大小(默认20) |
| category_id | integer | ❌ | 分类ID |
| search | string | ❌ | 搜索关键词 |
| status | string | ❌ | 状态筛选 |

**请求示例**
```bash
curl -X GET "http://127.0.0.1:15201/api/v1/packages?page=1&page_size=10&category_id=1&search=测试"
```

**响应示例**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "list": [
      {
        "id": 1,
        "name": "测试包",
        "author": "testuser",
        "version": "1.0.0",
        "description": "这是一个测试包",
        "file_url": "/uploads/test.zip",
        "file_size": 1024000,
        "download_count": 50,
        "like_count": 10,
        "favorite_count": 5,
        "category_id": 1,
        "status": "Active",
        "created_at": "2023-12-01T10:00:00Z",
        "updated_at": "2023-12-01T10:00:00Z",
        "reviewer_id": null,
        "reviewed_at": null,
        "review_comment": null
      }
    ],
    "total": 100,
    "page": 1,
    "size": 10
  }
}
```

### 2. 获取单个资源包
根据ID获取资源包详细信息

**接口信息**
- **路径**: `GET /api/v1/packages/{id}`
- **认证**: 无需认证

**路径参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| id | integer | 资源包ID |

### 3. 资源提交

#### 用户提交资源
普通用户提交资源，自动设置为待审核状态

**接口信息**
- **路径**: `POST /api/v1/packages/user-submit`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| title | string | ✅ | 资源标题 |
| description | string | ❌ | 描述 |
| category | string | ❌ | 分类 |
| tags | array | ❌ | 标签数组 |
| file_url | string | ✅ | 文件URL |
| cover_url | string | ❌ | 封面URL |

#### 管理员创建资源包
管理员直接创建资源包，可设置任意状态

**接口信息**
- **路径**: `POST /api/v1/packages/admin-create`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | ✅ | 包名 |
| author | string | ✅ | 作者 |
| version | string | ❌ | 版本 |
| description | string | ❌ | 描述 |
| category_id | integer | ❌ | 分类ID |
| file_url | string | ❌ | 文件URL |

### 4. 资源管理

#### 更新资源包信息
**接口信息**
- **路径**: `PUT /api/v1/packages/{id}`
- **认证**: 需要权限

#### 删除资源包
**接口信息**
- **路径**: `DELETE /api/v1/packages/{id}`
- **认证**: 需要权限

### 5. 文件操作

#### 下载资源包
**接口信息**
- **路径**: `GET /api/v1/packages/{id}/download`
- **认证**: 无需认证

#### 上传资源包文件
**接口信息**
- **路径**: `POST /api/v1/packages/{id}/upload`
- **认证**: 需要权限
- **Content-Type**: `multipart/form-data`

### 6. 审核系统

#### 获取待审核资源
**接口信息**
- **路径**: `GET /api/v1/packages/pending`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

#### 审核资源
**接口信息**
- **路径**: `POST /api/v1/packages/{id}/review`
- **认证**: 需要管理员或元老权限
- **权限要求**: JWT Token中role字段必须为 `admin` 或 `elder`
- **请求头**: `Authorization: Bearer <token>` 或 Cookie: `auth_token=<token>`

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| status | string | ✅ | 审核状态(approved/rejected) |
| comment | string | ❌ | 审核备注 |

### 7. 其他功能

#### 获取资源包分类
**接口信息**
- **路径**: `GET /api/v1/packages/categories`
- **认证**: 无需认证

#### 获取资源包评论
**接口信息**
- **路径**: `GET /api/v1/packages/{id}/comments`
- **认证**: 无需认证

---

## 💬 评论系统

### 1. 评论列表管理

#### 获取所有评论
管理员查看系统中所有评论

**接口信息**
- **路径**: `GET /api/v1/comments`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**查询参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| page | integer | ❌ | 页码 |
| size | integer | ❌ | 每页大小 |
| status | string | ❌ | 状态筛选 |
| target_type | string | ❌ | 目标类型 |
| target_id | integer | ❌ | 目标ID |
| user_id | integer | ❌ | 用户ID |
| start_date | string | ❌ | 开始日期 |
| end_date | string | ❌ | 结束日期 |
| search | string | ❌ | 搜索关键词 |

#### 获取单个评论
**接口信息**
- **路径**: `GET /api/v1/comments/{id}`
- **认证**: 无需认证

### 2. 评论操作

#### 创建评论
**接口信息**
- **路径**: `POST /api/v1/comments`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| content | string | ✅ | 评论内容 |
| target_type | string | ✅ | 目标类型(Package/User/System) |
| target_id | integer | ✅ | 目标ID |
| parent_id | integer | ❌ | 父评论ID(用于回复) |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/comments \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "这个包很有用！",
    "target_type": "Package",
    "target_id": 1
  }'
```

#### 更新评论
**接口信息**
- **路径**: `PUT /api/v1/comments/{id}`
- **认证**: 需要权限

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| content | string | ❌ | 评论内容 |
| status | string | ❌ | 状态 |

#### 删除评论
**接口信息**
- **路径**: `DELETE /api/v1/comments/{id}`
- **认证**: 需要权限

### 3. 评论回复

#### 获取评论回复
**接口信息**
- **路径**: `GET /api/v1/comments/{id}/replies`
- **认证**: 无需认证

#### 回复评论
**接口信息**
- **路径**: `POST /api/v1/comments/{id}/reply`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| content | string | ✅ | 回复内容 |

### 4. 评论互动

#### 点赞评论
**接口信息**
- **路径**: `POST /api/v1/comments/{id}/like`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| like | boolean | ✅ | true为点赞，false为取消点赞 |

#### 点踩评论
**接口信息**
- **路径**: `POST /api/v1/comments/{id}/dislike`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| dislike | boolean | ✅ | true为点踩，false为取消点踩 |

#### 置顶评论
**接口信息**
- **路径**: `POST /api/v1/comments/{id}/pin`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| pinned | boolean | ✅ | true为置顶，false为取消置顶 |

### 5. 批量操作

#### 批量更新评论状态
**接口信息**
- **路径**: `PUT /api/v1/comments/batch-status`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| ids | array | ✅ | 评论ID数组 |
| status | string | ✅ | 新状态 |

#### 批量删除评论
**接口信息**
- **路径**: `DELETE /api/v1/comments/batch`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| ids | array | ✅ | 评论ID数组 |

---

## 📁 分类管理

### 1. 获取所有分类
获取系统中的所有资源分类

**接口信息**
- **路径**: `GET /api/v1/categories`
- **认证**: 无需认证

**响应示例**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "list": [
      {
        "id": 1,
        "name": "工具类",
        "description": "各种实用工具",
        "enabled": true,
        "subscription_locked": false,
        "created_at": "2023-12-01T10:00:00Z",
        "updated_at": "2023-12-01T10:00:00Z",
        "count": 25
      }
    ]
  }
}
```

### 2. 获取单个分类
根据ID获取分类详细信息

**接口信息**
- **路径**: `GET /api/v1/categories/{id}`
- **认证**: 无需认证

**路径参数**
| 参数 | 类型 | 说明 |
|------|------|------|
| id | integer | 分类ID |

### 3. 创建分类
创建新的资源分类

**接口信息**
- **路径**: `POST /api/v1/categories`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| name | string | ✅ | 分类名称 |
| description | string | ❌ | 描述 |
| enabled | boolean | ❌ | 是否启用 |
| subscription_locked | boolean | ❌ | 是否锁定订阅 |

### 4. 更新分类
更新分类信息

**接口信息**
- **路径**: `PUT /api/v1/categories/{id}`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

### 5. 删除分类
删除指定分类

**接口信息**
- **路径**: `DELETE /api/v1/categories/{id}`
- **认证**: 需要管理员权限
- **权限要求**: JWT Token中role字段必须为 `admin`
- **请求头**: `Authorization: Bearer <admin_token>` 或 Cookie: `auth_token=<admin_token>`

---

## 🔔 订阅系统

### 1. 获取用户订阅
获取当前用户的分类订阅状态

**接口信息**
- **路径**: `GET /api/v1/subscriptions`
- **认证**: 需要Token

**响应示例**
```json
{
  "code": 0,
  "data": [
    {
      "category_id": 1,
      "enabled": true
    },
    {
      "category_id": 2,
      "enabled": false
    }
  ]
}
```

### 2. 设置订阅
设置用户对特定分类的订阅状态

**接口信息**
- **路径**: `POST /api/v1/subscriptions/set`
- **认证**: 需要Token

**请求参数**
| 参数 | 类型 | 必填 | 说明 |
|------|------|------|------|
| category_id | integer | ✅ | 分类ID |
| enabled | boolean | ✅ | 是否启用订阅 |

**请求示例**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/subscriptions/set \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "enabled": true
  }'
```

---

## 🛠 管理员功能

### 1. 系统统计

#### 获取系统统计信息
**接口信息**
- **路径**: `GET /api/v1/admin/stats`
- **认证**: 需要管理员权限

#### 获取用户统计
**接口信息**
- **路径**: `GET /api/v1/admin/user-stats`
- **认证**: 需要管理员权限

### 2. 数据备份

#### 创建备份
**接口信息**
- **路径**: `POST /api/v1/admin/backup`
- **认证**: 需要管理员权限

#### 获取备份列表
**接口信息**
- **路径**: `GET /api/v1/admin/backups`
- **认证**: 需要管理员权限

#### 获取备份统计
**接口信息**
- **路径**: `GET /api/v1/admin/backup/stats`
- **认证**: 需要管理员权限

#### 批量删除备份
**接口信息**
- **路径**: `POST /api/v1/admin/backup/batch-delete`
- **认证**: 需要管理员权限

#### 获取备份详情
**接口信息**
- **路径**: `GET /api/v1/admin/backup/{backup_id}`
- **认证**: 需要管理员权限

#### 删除备份
**接口信息**
- **路径**: `DELETE /api/v1/admin/backup/{backup_id}`
- **认证**: 需要管理员权限

#### 下载备份
**接口信息**
- **路径**: `GET /api/v1/admin/backup/{backup_id}/download`
- **认证**: 需要管理员权限

#### 恢复备份
**接口信息**
- **路径**: `POST /api/v1/admin/backup/{backup_id}/restore`
- **认证**: 需要管理员权限

#### 备份计划管理
**获取备份计划**
- **路径**: `GET /api/v1/admin/backup-schedule`
- **认证**: 需要管理员权限

**更新备份计划**
- **路径**: `POST /api/v1/admin/backup-schedule`
- **认证**: 需要管理员权限

### 3. 公告管理

#### 获取公告列表
**接口信息**
- **路径**: `GET /api/v1/admin/announcements`
- **认证**: 需要管理员权限

#### 创建公告
**接口信息**
- **路径**: `POST /api/v1/admin/announcements`
- **认证**: 需要管理员权限

#### 获取单个公告
**接口信息**
- **路径**: `GET /api/v1/admin/announcements/{id}`
- **认证**: 需要管理员权限

#### 更新公告
**接口信息**
- **路径**: `PUT /api/v1/admin/announcements/{id}`
- **认证**: 需要管理员权限

#### 删除公告
**接口信息**
- **路径**: `DELETE /api/v1/admin/announcements/{id}`
- **认证**: 需要管理员权限

#### 批量更新公告状态
**接口信息**
- **路径**: `PUT /api/v1/admin/announcements/batch-status`
- **认证**: 需要管理员权限

#### 批量删除公告
**接口信息**
- **路径**: `POST /api/v1/admin/announcements/batch-delete`
- **认证**: 需要管理员权限

### 4. 系统设置

#### 主题设置
**获取主题设置**
- **路径**: `GET /api/v1/admin/theme-settings`
- **认证**: 需要管理员权限

**更新主题设置**
- **路径**: `POST /api/v1/admin/theme-settings`
- **认证**: 需要管理员权限

#### 系统设置
**获取系统设置**
- **路径**: `GET /api/v1/admin/settings`
- **认证**: 需要管理员权限

**更新系统设置**
- **路径**: `POST /api/v1/admin/settings`
- **认证**: 需要管理员权限

**重置系统设置**
- **路径**: `POST /api/v1/admin/settings/reset`
- **认证**: 需要管理员权限

#### 邮件设置
**获取邮件设置**
- **路径**: `GET /api/v1/admin/mail-settings`
- **认证**: 需要管理员权限

**更新邮件设置**
- **路径**: `POST /api/v1/admin/mail-settings`
- **认证**: 需要管理员权限

**发送测试邮件**
- **路径**: `POST /api/v1/admin/send-test-email`
- **认证**: 需要管理员权限

#### 社区设置
**获取社区设置**
- **路径**: `GET /api/v1/admin/community-settings`
- **认证**: 需要管理员权限

**更新社区设置**
- **路径**: `POST /api/v1/admin/community-settings`
- **认证**: 需要管理员权限

### 5. 公开接口

#### 获取公开的公告
**接口信息**
- **路径**: `GET /api/v1/announcements`
- **认证**: 无需认证

---

## 📊 系统监控

### 1. 用户行为记录

#### 获取用户行为记录
**接口信息**
- **路径**: `GET /api/v1/user-actions`
- **认证**: 需要管理员权限

#### 记录用户行为
**接口信息**
- **路径**: `POST /api/v1/user-actions`
- **认证**: 需要Token

#### 获取行为统计
**接口信息**
- **路径**: `GET /api/v1/user-actions/stats`
- **认证**: 需要管理员权限

#### 批量删除行为记录
**接口信息**
- **路径**: `DELETE /api/v1/user-actions/batch`
- **认证**: 需要管理员权限

#### 获取单个行为记录
**接口信息**
- **路径**: `GET /api/v1/user-actions/{id}`
- **认证**: 需要管理员权限

#### 删除行为记录
**接口信息**
- **路径**: `DELETE /api/v1/user-actions/{id}`
- **认证**: 需要管理员权限

### 2. 系统日志

#### 获取系统日志
**接口信息**
- **路径**: `GET /api/v1/admin/logs`
- **认证**: 需要管理员权限

### 3. 社区功能

#### 获取社区评论
**接口信息**
- **路径**: `GET /api/v1/community/comments`
- **认证**: 无需认证

#### 创建社区评论
**接口信息**
- **路径**: `POST /api/v1/community/comments`
- **认证**: 需要Token

---

## ⚠️ 错误处理

### 常见错误响应格式

#### Token缺失
```json
{
  "code": 401,
  "message": "需要登录认证"
}
```

#### Token无效
```json
{
  "code": 401,
  "message": "认证Token无效"
}
```

#### 账户被禁
```json
{
  "code": 403,
  "message": "账户已被封禁或暂停"
}
```

#### 权限不足
```json
{
  "code": 403,
  "message": "权限不足"
}
```

#### 需要管理员权限
```json
{
  "code": 403,
  "message": "需要管理员权限"
}
```

#### 资源不存在
```json
{
  "code": 404,
  "message": "用户不存在"
}
```

#### 参数错误
```json
{
  "code": 400,
  "message": "参数验证失败: 用户名不能为空"
}
```

#### 服务器错误
```json
{
  "code": 500,
  "message": "数据库连接失败"
}
```

### 错误处理建议

1. **检查HTTP状态码**: 首先检查HTTP状态码确定请求是否到达服务器
2. **解析响应体**: 从响应体的`code`和`message`字段获取详细错误信息
3. **重试机制**: 对于5xx错误可以实施重试机制
4. **用户友好提示**: 将技术错误信息转换为用户友好的提示
5. **日志记录**: 记录详细的错误信息用于调试

---

## 💡 使用示例

### 管理员权限获取和使用流程

#### 1. 管理员登录获取Token
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin_user",
    "password": "admin_password"
  }'
```

**响应示例**:
```json
{
  "code": 0,
  "message": "登录成功",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6ImFkbWluIiwicm9sZSI6ImFkbWluIiwiZXhwIjoxNzAwMDAwMDAwfQ.signature",
    "user": {
      "id": 1,
      "username": "admin_user",
      "role": "admin",
      "is_admin": true
    }
  }
}
```

#### 2. 使用管理员Token访问管理接口
```bash
# 获取所有用户（需要管理员权限）
curl -X GET http://127.0.0.1:15201/api/v1/users \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6ImFkbWluIiwicm9sZSI6ImFkbWluIiwiZXhwIjoxNzAwMDAwMDAwfQ.signature"

# 或者使用Cookie方式
curl -X GET http://127.0.0.1:15201/api/v1/users \
  -H "Cookie: auth_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxLCJ1c2VybmFtZSI6ImFkbWluIiwicm9sZSI6ImFkbWluIiwiZXhwIjoxNzAwMDAwMDAwfQ.signature"
```

#### 3. JWT Token解析说明
管理员Token包含以下关键信息：
```json
{
  "user_id": 1,          // 用户ID
  "username": "admin",   // 用户名
  "role": "admin",       // 角色（关键字段）
  "exp": 1700000000      // 过期时间戳
}
```

**重要**: 系统会验证Token中的 `role` 字段来确定用户权限。

### 📧 邮箱验证完整流程

#### 1. 用户注册流程（带邮箱验证）

**步骤1: 发送注册验证码**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/send-register-code \
  -H "Content-Type: application/json" \
  -d '{"email": "newuser@example.com"}'
```

**步骤2: 用户检查邮箱，获取6位验证码**
邮件内容示例：
```
您的注册验证码是：123456
验证码5分钟内有效，请勿泄露给他人。
```

**步骤3: 验证邮箱验证码（可选步骤，用于前端验证）**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/verify-code \
  -H "Content-Type: application/json" \
  -d '{
    "email": "newuser@example.com",
    "code": "123456"
  }'
```

**步骤4: 完成用户注册**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "email": "newuser@example.com", 
    "password": "password123",
    "nickname": "新用户",
    "verification_code": "123456"
  }'
```

#### 2. 邮箱登录流程

**步骤1: 发送登录验证码**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/send-login-code \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com"}'
```

**步骤2: 使用验证码登录**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/login-by-email \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "code": "123456"
  }'
```

#### 3. 密码重置流程

**步骤1: 申请密码重置**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/reset-request \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com"}'
```

**步骤2: 用户收到重置邮件**
邮件内容示例：
```
点击以下链接重置密码：
https://yoursite.com/reset-password?token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...&email=user@example.com

链接30分钟内有效，如非本人操作请忽略此邮件。
```

**步骤3: 使用重置令牌设置新密码**
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/reset-password \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "new_password": "newPassword123"
  }'
```

#### 4. 前端JavaScript示例

**注册流程前端实现**
```javascript
class EmailVerification {
  constructor(baseURL = 'http://127.0.0.1:15201/api/v1') {
    this.baseURL = baseURL;
  }

  // 发送注册验证码
  async sendRegisterCode(email) {
    try {
      const response = await fetch(`${this.baseURL}/auth/send-register-code`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ email })
      });
      
      const result = await response.json();
      
      if (result.code === 0) {
        console.log('验证码发送成功');
        return {
          success: true,
          message: '验证码已发送到您的邮箱',
          expiresIn: result.data.expires_in,
          nextSendTime: result.data.next_send_time
        };
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      console.error('发送验证码失败:', error);
      return {
        success: false,
        message: error.message || '发送失败，请重试'
      };
    }
  }

  // 验证验证码
  async verifyCode(email, code) {
    try {
      const response = await fetch(`${this.baseURL}/auth/verify-code`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ email, code })
      });
      
      const result = await response.json();
      
      if (result.code === 0) {
        return { success: true, message: '验证成功' };
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      return {
        success: false,
        message: error.message || '验证失败'
      };
    }
  }

  // 注册用户
  async register(userData) {
    try {
      const response = await fetch(`${this.baseURL}/auth/register`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(userData)
      });
      
      const result = await response.json();
      
      if (result.code === 0) {
        return {
          success: true,
          message: '注册成功',
          user: result.data.user,
          token: result.data.token
        };
      } else {
        throw new Error(result.message);
      }
    } catch (error) {
      return {
        success: false,
        message: error.message || '注册失败'
      };
    }
  }
}

// 使用示例
const emailVerify = new EmailVerification();

async function handleRegistration() {
  const email = 'newuser@example.com';
  const username = 'newuser';
  const password = 'password123';
  
  // 1. 发送验证码
  const sendResult = await emailVerify.sendRegisterCode(email);
  if (!sendResult.success) {
    alert(sendResult.message);
    return;
  }
  
  // 2. 等待用户输入验证码
  const code = prompt('请输入邮箱收到的验证码：');
  
  // 3. 验证验证码（可选）
  const verifyResult = await emailVerify.verifyCode(email, code);
  if (!verifyResult.success) {
    alert(verifyResult.message);
    return;
  }
  
  // 4. 注册用户
  const registerResult = await emailVerify.register({
    username,
    email,
    password,
    nickname: '新用户',
    verification_code: code
  });
  
  if (registerResult.success) {
    alert('注册成功！');
    // 保存Token
    localStorage.setItem('auth_token', registerResult.token);
  } else {
    alert(registerResult.message);
  }
}
```

**错误处理最佳实践**
```javascript
class EmailErrorHandler {
  static handleEmailError(error) {
    const errorMessages = {
      400: '请求参数错误，请检查邮箱格式',
      404: '邮箱未注册',
      409: '邮箱已被注册',
      410: '验证码已过期，请重新获取',
      429: '操作过于频繁，请稍后再试',
      500: '服务器错误，请联系客服'
    };
    
    return errorMessages[error.code] || error.message || '未知错误';
  }
  
  static showUserFriendlyMessage(error) {
    const message = this.handleEmailError(error);
    
    // 不同错误类型的用户提示
    if (error.code === 429) {
      return `${message}\n建议：请等待一段时间再重新发送`;
    } else if (error.code === 410) {
      return `${message}\n建议：点击重新发送验证码`;
    } else if (error.code === 409) {
      return `${message}\n建议：使用其他邮箱或直接登录`;
    }
    
    return message;
  }
}

// 使用示例
try {
  const result = await emailVerify.sendRegisterCode(email);
  if (!result.success) {
    const friendlyMessage = EmailErrorHandler.showUserFriendlyMessage({
      code: result.code,
      message: result.message
    });
    alert(friendlyMessage);
  }
} catch (error) {
  console.error('请求失败:', error);
  alert('网络连接失败，请检查网络后重试');
}
```

### 完整的用户注册和登录流程

#### 1. 传统用户名密码注册
```bash
# 如果不使用邮箱验证，可以直接注册（需要先获取验证码）
curl -X POST http://127.0.0.1:15201/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "email": "newuser@example.com", 
    "password": "password123",
    "nickname": "新用户",
    "verification_code": "123456"
  }'
```

#### 2. 用户名密码登录
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "newuser",
    "password": "password123"
  }'
```

#### 3. 使用Token访问受保护的接口
```bash
curl -X GET http://127.0.0.1:15201/api/v1/users/profile \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### 资源包管理流程

#### 1. 获取资源包列表
```bash
curl -X GET "http://127.0.0.1:15201/api/v1/packages?page=1&page_size=10"
```

#### 2. 用户提交资源
```bash
curl -X POST http://127.0.0.1:15201/api/v1/packages/user-submit \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "我的工具包",
    "description": "一个很有用的工具包",
    "category": "工具类",
    "file_url": "/uploads/mytool.zip"
  }'
```

#### 3. 管理员审核资源
```bash
curl -X POST http://127.0.0.1:15201/api/v1/packages/1/review \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "approved",
    "comment": "资源质量良好，已通过审核"
  }'
```

### 评论系统使用流程

#### 1. 给资源包添加评论
```bash
curl -X POST http://127.0.0.1:15201/api/v1/comments \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "这个包很有用！",
    "target_type": "Package",
    "target_id": 1
  }'
```

#### 2. 回复评论
```bash
curl -X POST http://127.0.0.1:15201/api/v1/comments/1/reply \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "content": "感谢你的反馈！"
  }'
```

#### 3. 点赞评论
```bash
curl -X POST http://127.0.0.1:15201/api/v1/comments/1/like \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"like": true}'
```

### 分类订阅流程

#### 1. 获取所有分类
```bash
curl -X GET http://127.0.0.1:15201/api/v1/categories
```

#### 2. 订阅特定分类
```bash
curl -X POST http://127.0.0.1:15201/api/v1/subscriptions/set \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "enabled": true
  }'
```

#### 3. 查看我的订阅
```bash
curl -X GET http://127.0.0.1:15201/api/v1/subscriptions \
  -H "Authorization: Bearer <token>"
```

---

## 📝 注意事项

### 1. 认证相关
- **Token获取**: 通过登录接口获取JWT Token
- **Token提交**: 在请求头中添加 `Authorization: Bearer <token>` 或使用Cookie `auth_token=<token>`
- **Token有效期**: 24小时，过期后需要重新登录
- **权限验证**: 系统根据Token中的 `role` 字段判断用户权限
- **优先级**: Authorization头部认证优先于Cookie认证

### 2. 权限体系
- **普通用户(user)**: 基础功能使用权限
- **元老(elder)**: 拥有部分管理权限
- **版主(moderator)**: 拥有内容管理权限  
- **管理员(admin)**: 拥有所有权限

### 3. 分页机制
- 列表接口通常支持分页参数 `page` 和 `page_size`
- 默认页码从1开始，默认每页20条记录
- 响应中包含 `total`、`page`、`pageSize`、`totalPages` 等分页信息

### 4. 状态码规范
- HTTP状态码用于网络层错误判断
- 业务逻辑状态统一使用响应体中的 `code` 字段
- `code: 0` 表示成功，其他值表示各种错误情况

### 5. 时间格式
- 所有时间字段使用ISO 8601格式
- 统一使用UTC时间
- 格式示例: `2023-12-01T10:00:00Z`

### 6. 文件处理
- 文件上传接口使用 `multipart/form-data` 格式
- 支持的文件类型: zip, rar, 7z, tar, gz
- 单文件大小限制: 10MB
- 上传的文件存储在 `/uploads` 目录下

### 7. 邮件功能
- 需要在配置文件中正确配置SMTP信息
- 支持注册验证、登录验证、密码重置等邮件功能
- 当前配置使用163邮箱作为SMTP服务器

### 8. CORS配置
- 服务器已配置CORS支持
- 允许的源: `http://localhost:5173`, `http://127.0.0.1:5173`, `http://localhost:3000`, `http://127.0.0.1:3000`
- 支持的方法: GET, POST, PUT, DELETE, OPTIONS
- 支持Cookie跨域传输

### 9. 数据库
- 使用SQLite数据库
- 数据库文件: `data.db`
- 支持自动初始化和迁移

### 10. 安全考虑
- 密码使用bcrypt加密存储
- JWT Token包含用户基本信息
- 敏感操作需要相应权限验证
- 支持用户状态管理(正常/暂停/封禁)

---

## ❓ 常见问题解答 (FAQ)

### Q1: 如何获取管理员权限？
**A**: 管理员权限需要通过以下步骤获取：
1. 使用管理员账号登录 (`role: admin`)
2. 获取包含管理员角色的JWT Token
3. 在请求中携带该Token

### Q2: 如何判断Token是否有管理员权限？
**A**: 解码JWT Token，检查payload中的 `role` 字段：
```json
{
  "user_id": 1,
  "username": "admin",
  "role": "admin",  // 这里必须是 "admin"
  "exp": 1700000000
}
```

### Q3: 为什么提示"需要管理员权限"？
**A**: 可能的原因：
- Token中的 `role` 字段不是 `admin`
- Token已过期或无效
- 未携带Authorization头部或Cookie
- 账户被封禁或暂停

### Q4: Authorization头部和Cookie哪个优先？
**A**: Authorization头部优先级更高，如果两者都存在，系统会优先使用Authorization头部的Token。

### Q5: 如何在代码中使用管理员权限？
**A**: 
```javascript
// 使用fetch示例
fetch('/api/v1/users', {
  method: 'GET',
  headers: {
    'Authorization': `Bearer ${adminToken}`,
    'Content-Type': 'application/json'
  }
})

// 使用axios示例
axios.get('/api/v1/users', {
  headers: {
    'Authorization': `Bearer ${adminToken}`
  }
})

// 使用Cookie方式
document.cookie = `auth_token=${adminToken}; path=/`;
```

### Q6: 元老(elder)权限和管理员有什么区别？
**A**: 
- **管理员(admin)**: 拥有所有系统权限
- **元老(elder)**: 拥有部分管理权限，主要是资源审核权限
- **版主(moderator)**: 拥有内容管理权限
- **普通用户(user)**: 基础功能使用权限

### Q7: Token过期了怎么办？
**A**: Token过期后需要重新登录获取新的Token，或者实现Token刷新机制。

### Q8: 邮箱验证码多久过期？
**A**: 
- **注册验证码**: 5分钟内有效
- **登录验证码**: 5分钟内有效  
- **密码重置令牌**: 30分钟内有效
- **发送频率限制**: 每分钟最多1次，注册验证码每天最多10次

### Q9: 为什么收不到验证码邮件？
**A**: 可能的原因和解决方案：
1. **检查垃圾邮件箱**: 验证码邮件可能被误判为垃圾邮件
2. **邮箱地址错误**: 确认邮箱地址拼写正确
3. **网络延迟**: 等待1-2分钟，邮件可能有延迟
4. **邮箱服务商限制**: 某些邮箱服务商可能拒收或延迟接收
5. **发送频率限制**: 检查是否超过发送频率限制

### Q10: 如何在前端实现验证码倒计时？
**A**: 
```javascript
class CountdownTimer {
  constructor(duration = 60) {
    this.duration = duration;
    this.timer = null;
    this.isRunning = false;
  }
  
  start(callback) {
    if (this.isRunning) return;
    
    this.isRunning = true;
    let remaining = this.duration;
    
    const tick = () => {
      callback(remaining);
      remaining--;
      
      if (remaining < 0) {
        this.stop();
        callback(0, true); // 倒计时结束
      }
    };
    
    tick(); // 立即执行一次
    this.timer = setInterval(tick, 1000);
  }
  
  stop() {
    if (this.timer) {
      clearInterval(this.timer);
      this.timer = null;
    }
    this.isRunning = false;
  }
}

// 使用示例
const countdown = new CountdownTimer(60);
const button = document.getElementById('send-code-btn');

function sendVerificationCode() {
  // 发送验证码逻辑...
  
  // 开始倒计时
  countdown.start((remaining, finished) => {
    if (finished) {
      button.textContent = '发送验证码';
      button.disabled = false;
    } else {
      button.textContent = `${remaining}秒后重发`;
      button.disabled = true;
    }
  });
}
```

### Q11: 如何处理邮件发送失败的情况？
**A**: 
```javascript
async function sendEmailWithRetry(email, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      const result = await sendRegisterCode(email);
      if (result.success) {
        return result;
      }
      
      // 如果是频率限制，不需要重试
      if (result.code === 429) {
        throw new Error(result.message);
      }
      
    } catch (error) {
      if (i === maxRetries - 1) {
        throw error; // 最后一次重试失败
      }
      
      // 等待后重试
      await new Promise(resolve => setTimeout(resolve, 1000 * (i + 1)));
    }
  }
}
```

### Q12: 如何验证邮箱格式？
**A**: 
```javascript
function validateEmail(email) {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  const commonDomains = ['gmail.com', 'yahoo.com', '163.com', 'qq.com', 'hotmail.com'];
  
  // 基本格式验证
  if (!emailRegex.test(email)) {
    return { valid: false, message: '邮箱格式不正确' };
  }
  
  // 长度检查
  if (email.length > 254) {
    return { valid: false, message: '邮箱地址过长' };
  }
  
  // 域名检查（可选）
  const domain = email.split('@')[1];
  if (commonDomains.includes(domain.toLowerCase())) {
    return { valid: true, message: '邮箱格式正确' };
  }
  
  return { valid: true, message: '邮箱格式正确', warning: '请确认邮箱地址可以正常接收邮件' };
}

// 使用示例
const validation = validateEmail('user@example.com');
if (!validation.valid) {
  alert(validation.message);
}
```

---

## 📞 技术支持

如有技术问题或需要帮助，请通过以下方式联系：

- **项目地址**: 查看项目源码了解更多实现细节
- **配置文件**: `config.toml` - 服务器配置
- **日志文件**: `logs/app.log` - 应用运行日志
- **数据库**: `data.db` - SQLite数据库文件

---

*文档最后更新时间: 2023-12-01* 