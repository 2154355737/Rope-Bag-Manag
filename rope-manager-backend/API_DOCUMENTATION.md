# 绳包管理系统 API 文档

## 概述

本文档描述了绳包管理系统的所有API端点，包括认证、用户管理、包管理、管理员功能等。

## 基础信息

- **基础URL**: `http://127.0.0.1:15201`
- **API版本**: v1
- **认证方式**: JWT Token
- **数据格式**: JSON

## 通用响应格式

所有API都使用统一的响应格式：

```json
{
  "code": 0,           // 0表示成功，其他值表示错误
  "message": "success", // 响应消息
  "data": {}           // 响应数据
}
```

## 认证相关 API

### 用户登录
- **URL**: `POST /api/v1/auth/login`
- **描述**: 用户登录获取JWT token
- **请求体**:
```json
{
  "username": "admin",
  "password": "admin123"
}
```
- **响应**:
```json
{
  "code": 0,
  "message": "登录成功",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": 1,
      "username": "admin",
      "role": "admin"
    }
  }
}
```

### 用户注册
- **URL**: `POST /api/v1/auth/register`
- **描述**: 新用户注册
- **请求体**:
```json
{
  "username": "newuser",
  "password": "password123",
  "email": "user@example.com"
}
```

### 获取用户信息
- **URL**: `GET /api/v1/auth/user-info`
- **描述**: 获取当前登录用户信息
- **认证**: 需要JWT token

## 用户管理 API

### 获取用户列表
- **URL**: `GET /api/v1/users`
- **描述**: 获取所有用户列表
- **权限**: 需要管理员权限

### 获取单个用户
- **URL**: `GET /api/v1/users/{id}`
- **描述**: 根据ID获取用户信息

### 更新用户
- **URL**: `PUT /api/v1/users/{id}`
- **描述**: 更新用户信息
- **请求体**:
```json
{
  "role": "admin",
  "status": 1,
  "nickname": "新昵称"
}
```

### 删除用户
- **URL**: `DELETE /api/v1/users/{id}`
- **描述**: 删除用户
- **权限**: 需要管理员权限

### 获取当前用户资料
- **URL**: `GET /api/v1/users/profile`
- **描述**: 获取当前登录用户的资料

### 更新当前用户资料
- **URL**: `PUT /api/v1/users/profile`
- **描述**: 更新当前登录用户的资料

### 获取我的资源
- **URL**: `GET /api/v1/users/my-resources`
- **描述**: 获取当前用户上传的资源

### 获取我的评论
- **URL**: `GET /api/v1/users/my-comments`
- **描述**: 获取当前用户的评论

### 修改密码
- **URL**: `POST /api/v1/users/change-password`
- **描述**: 修改当前用户密码
- **请求体**:
```json
{
  "old_password": "oldpass",
  "new_password": "newpass"
}
```

## 包管理 API

### 获取包列表
- **URL**: `GET /api/v1/packages`
- **描述**: 获取所有绳包列表

### 获取单个包
- **URL**: `GET /api/v1/packages/{id}`
- **描述**: 根据ID获取包信息

### 创建包
- **URL**: `POST /api/v1/packages`
- **描述**: 创建新的绳包
- **请求体**:
```json
{
  "name": "包名称",
  "author": "作者",
  "version": "1.0.0",
  "description": "描述",
  "category_id": 1
}
```

### 更新包
- **URL**: `PUT /api/v1/packages/{id}`
- **描述**: 更新包信息
- **权限**: 需要管理员或作者权限

### 删除包
- **URL**: `DELETE /api/v1/packages/{id}`
- **描述**: 删除包
- **权限**: 需要管理员或作者权限

### 下载包
- **URL**: `GET /api/v1/packages/{id}/download`
- **描述**: 下载包文件

### 上传包文件
- **URL**: `POST /api/v1/packages/{id}/upload`
- **描述**: 上传包文件
- **权限**: 需要管理员或作者权限

### 获取包分类
- **URL**: `GET /api/v1/packages/categories`
- **描述**: 获取所有包分类

## 管理员 API

### 获取统计数据
- **URL**: `GET /api/v1/admin/stats`
- **描述**: 获取系统统计数据
- **权限**: 需要管理员权限
- **响应**:
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "total_users": 100,
    "total_packages": 50,
    "total_comments": 200,
    "active_users": 30,
    "new_users_today": 5,
    "new_packages_today": 2,
    "system_status": "normal",
    "uptime": 86400
  }
}
```

### 获取系统日志
- **URL**: `GET /api/v1/admin/logs`
- **描述**: 获取系统日志
- **权限**: 需要管理员权限

### 获取用户行为记录
- **URL**: `GET /api/v1/admin/user-actions`
- **描述**: 获取用户行为记录
- **权限**: 需要管理员权限

### 创建备份
- **URL**: `POST /api/v1/admin/backup`
- **描述**: 创建数据库备份
- **权限**: 需要管理员权限

### 获取备份列表
- **URL**: `GET /api/v1/admin/backups`
- **描述**: 获取所有备份文件列表
- **权限**: 需要管理员权限

### 下载备份
- **URL**: `GET /api/v1/admin/backup/{backup_id}/download`
- **描述**: 下载指定备份文件
- **权限**: 需要管理员权限

### 删除备份
- **URL**: `DELETE /api/v1/admin/backup/{backup_id}`
- **描述**: 删除指定备份文件
- **权限**: 需要管理员权限

### 获取公告列表
- **URL**: `GET /api/v1/admin/announcements`
- **描述**: 获取所有公告
- **权限**: 需要管理员权限

### 创建公告
- **URL**: `POST /api/v1/admin/announcements`
- **描述**: 创建新公告
- **权限**: 需要管理员权限
- **请求体**:
```json
{
  "title": "公告标题",
  "content": "公告内容",
  "priority": 1
}
```

### 更新公告
- **URL**: `PUT /api/v1/admin/announcements/{id}`
- **描述**: 更新公告
- **权限**: 需要管理员权限

### 删除公告
- **URL**: `DELETE /api/v1/admin/announcements/{id}`
- **描述**: 删除公告
- **权限**: 需要管理员权限

### 获取主题设置
- **URL**: `GET /api/v1/admin/theme-settings`
- **描述**: 获取系统主题设置
- **权限**: 需要管理员权限

### 更新主题设置
- **URL**: `PUT /api/v1/admin/theme-settings`
- **描述**: 更新系统主题设置
- **权限**: 需要管理员权限
- **请求体**:
```json
{
  "primary_color": "#409EFF",
  "secondary_color": "#67C23A",
  "dark_mode": false
}
```

### 获取资源记录
- **URL**: `GET /api/v1/admin/resource-records`
- **描述**: 获取资源记录统计
- **权限**: 需要管理员权限

## 社区 API

### 获取评论
- **URL**: `GET /api/v1/community/comments/{package_id}`
- **描述**: 获取指定包的评论列表

### 创建评论
- **URL**: `POST /api/v1/community/comments/{package_id}`
- **描述**: 为指定包创建评论
- **认证**: 需要登录
- **请求体**:
```json
{
  "content": "评论内容"
}
```

## 错误码说明

| 错误码 | 说明 |
|--------|------|
| 0 | 成功 |
| 400 | 请求参数错误 |
| 401 | 未认证 |
| 403 | 权限不足 |
| 404 | 资源不存在 |
| 500 | 服务器内部错误 |

## 权限说明

### 用户角色
- **admin**: 管理员，拥有所有权限
- **elder**: 元老，可以管理资源
- **user**: 普通用户，基本功能
- **guest**: 访客，只读权限

### 权限要求
- 用户管理相关API需要管理员权限
- 包管理相关API需要管理员或作者权限
- 评论功能需要登录
- 管理员功能需要管理员权限

## 使用示例

### 登录获取token
```bash
curl -X POST http://127.0.0.1:15201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'
```

### 使用token访问API
```bash
curl -X GET http://127.0.0.1:15201/api/v1/admin/stats \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

### 上传文件
```bash
curl -X POST http://127.0.0.1:15201/api/v1/packages/1/upload \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -F "file=@package.zip"
```

## 注意事项

1. 所有需要认证的API都需要在请求头中包含JWT token
2. 文件上传使用multipart/form-data格式
3. 分页参数使用page和pageSize
4. 时间格式使用ISO 8601标准
5. 文件大小限制为100MB
6. 支持的图片格式：jpg, png, gif
7. 支持的包文件格式：zip, rar, 7z 