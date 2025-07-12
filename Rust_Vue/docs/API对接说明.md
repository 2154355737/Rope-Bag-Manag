# API对接说明

## 概述

本文档详细说明了前端与后端API的对接情况，包括接口映射、数据格式转换、功能实现等。

## 后端API接口

### 用户认证相关
- `GET /api/login` - 用户登录
- `GET /api/register` - 用户注册
- `GET /api/user-info` - 获取用户信息
- `GET /api/sign-in` - 用户签到
- `GET /api/change-password` - 修改密码
- `GET /api/change-nickname` - 修改昵称
- `GET /api/nicknames` - 获取用户昵称列表

### 资源管理相关
- `GET /api/get-data-db` - 获取所有资源
- `GET /api/add-rope-package` - 添加资源
- `GET /api/update-rope-package` - 更新资源
- `GET /api/delete-rope-package` - 删除资源
- `GET /api/download-rope-package` - 下载资源统计

### 管理员相关
- `GET /api/get-users-db` - 获取所有用户
- `GET /api/admin/user-info` - 获取用户详细信息
- `GET /api/admin/set-user` - 设置用户信息
- `GET /api/admin/set-star` - 设置用户星级
- `GET /api/admin/ban-user` - 封禁/解封用户
- `GET /api/admin/add-rope-package` - 管理员添加资源
- `GET /api/admin/update-rope-package` - 管理员更新资源
- `GET /api/admin/delete-rope-package` - 管理员删除资源
- `GET /api/set-admin` - 设置管理员权限

### 统计相关
- `GET /api/stats/api-counts` - 获取API调用统计
- `GET /api/stats/downloads` - 获取下载统计
- `GET /api/stats/api-calls` - 获取API调用记录
- `GET /api/stats/api-performance` - 获取API性能统计
- `GET /api/stats/recent-calls` - 获取最近调用记录
- `GET /api/dashboard` - 获取仪表盘数据

### 日志相关
- `GET /api/logs/entries` - 获取日志条目
- `GET /api/logs/stats` - 获取日志统计
- `GET /api/logs/clear` - 清空日志

## 前端API接口

### 核心API文件
- `src/api/index.ts` - 主要API接口
- `src/api/community.ts` - 社区相关API接口
- `src/api/cache.ts` - 缓存管理

### 接口映射

#### 用户认证
```typescript
// 前端接口
export const login = async (username: string, password: string)
export const register = async (username: string, password: string, nickname: string)
export const getUserInfo = async (username: string)
export const signIn = async (username: string)
export const changePassword = async (username: string, oldPassword: string, newPassword: string)
export const changeNickname = async (username: string, nickname: string)

// 对应后端接口
GET /api/login?username=xxx&password=xxx
GET /api/register?username=xxx&password=xxx&nickname=xxx
GET /api/user-info?username=xxx
GET /api/sign-in?username=xxx
GET /api/change-password?username=xxx&old_password=xxx&new_password=xxx
GET /api/change-nickname?username=xxx&nickname=xxx
```

#### 资源管理
```typescript
// 前端接口
export const getPackages = async ()
export const addPackage = async (data: PackageData)
export const updatePackage = async (data: PackageData)
export const deletePackage = async (id: number, username: string, admin_password?: string)
export const downloadPackage = async (id: number)

// 对应后端接口
GET /api/get-data-db?username=xxx
GET /api/add-rope-package?name=xxx&author=xxx&version=xxx&desc=xxx&url=xxx&username=xxx
GET /api/update-rope-package?id=xxx&name=xxx&author=xxx&version=xxx&desc=xxx&url=xxx&username=xxx
GET /api/delete-rope-package?id=xxx&username=xxx&admin_password=xxx
GET /api/download-rope-package?id=xxx
```

#### 管理员功能
```typescript
// 前端接口
export const getUsers = async ()
export const adminUserInfo = async (username: string, admin_username: string, admin_password: string)
export const adminSetUser = async (data: AdminUserData)
export const adminSetStar = async (target: string, star: number, admin_username: string, admin_password: string)
export const adminBanUser = async (target: string, banned: boolean, admin_username: string, admin_password: string)
export const setAdmin = async (target: string, is_admin: boolean, admin_username: string, admin_password: string)

// 对应后端接口
GET /api/get-users-db
GET /api/admin/user-info?username=xxx&admin_username=xxx&admin_password=xxx
GET /api/admin/set-user?target=xxx&nickname=xxx&password=xxx&admin_username=xxx&admin_password=xxx
GET /api/admin/set-star?target=xxx&star=xxx&admin_username=xxx&admin_password=xxx
GET /api/admin/ban-user?target=xxx&banned=xxx&admin_username=xxx&admin_password=xxx
GET /api/set-admin?target=xxx&is_admin=xxx&admin_username=xxx&admin_password=xxx
```

## 数据格式转换

### 资源数据格式
```typescript
// 后端数据格式
{
  "数据库配置": {
    "数据库名称": "结绳绳包数据库",
    "数据库版本": "1.0.0",
    "数据库项目": 10,
    "数据库更新时间": "20240101"
  },
  "绳包列表": [
    {
      "id": 1,
      "绳包名称": "Vue3 完整开发教程",
      "作者": "张老师",
      "版本": "1.0.0",
      "简介": "从零开始学习Vue3...",
      "项目直链": "https://example.com",
      "下载次数": 1250,
      "上架时间": "2024-01-15"
    }
  ]
}

// 前端处理后的格式
{
  id: 1,
  绳包名称: "Vue3 完整开发教程",
  作者: "张老师",
  版本: "1.0.0",
  简介: "从零开始学习Vue3...",
  项目直链: "https://example.com",
  下载次数: 1250,
  上架时间: "2024-01-15"
}
```

### 用户数据格式
```typescript
// 后端数据格式
{
  "admin": {
    "username": "admin",
    "password": "admin123",
    "nickname": "管理员",
    "star": 5,
    "banned": false,
    "sign_days": 30,
    "sign_total": 30,
    "last_sign": "2024-01-01",
    "is_admin": true
  }
}

// 前端处理后的格式
{
  id: "admin",
  username: "admin",
  nickname: "管理员",
  star: 5,
  banned: false,
  sign_days: 30,
  sign_total: 30,
  last_sign: "2024-01-01",
  is_admin: true
}
```

## 社区功能对接

### 资源社区
- 使用 `getPackages()` 获取资源列表
- 使用 `addPackage()` 创建新资源
- 使用 `downloadPackage()` 统计下载
- 支持搜索、分类、排序功能

### 用户管理
- 使用 `getUsers()` 获取用户列表
- 使用 `adminSetUser()` 更新用户信息
- 使用 `adminSetStar()` 设置用户星级
- 使用 `adminBanUser()` 封禁/解封用户

### 分类管理
- 基于资源数据动态生成分类
- 按作者分组作为分类依据
- 支持分类统计和筛选

## 缓存策略

### 缓存时间
- 用户数据：60秒
- 资源数据：30秒
- 统计数据：20-30秒
- 日志数据：10-15秒

### 缓存键
- `getUsers` - 用户列表缓存
- `getPackages` - 资源列表缓存
- `getStats` - 统计数据缓存
- `getLogs` - 日志数据缓存

## 错误处理

### 网络错误
- 连接失败时显示"Service unavailable"
- 超时错误自动重试
- 服务器错误显示具体错误信息

### 业务错误
- 登录失败：用户名或密码错误
- 权限不足：需要管理员权限
- 资源不存在：ID无效
- 操作失败：具体错误信息

## 安全措施

### 认证机制
- 登录状态存储在localStorage
- 敏感操作需要管理员密码
- 用户权限验证

### 限流保护
- API调用频率限制
- 全局请求限制
- 单用户请求限制

## 扩展建议

### 后端扩展
1. 添加分类管理API
2. 支持文件上传功能
3. 添加评论系统
4. 实现点赞收藏功能
5. 添加搜索API

### 前端扩展
1. 完善错误处理
2. 添加加载状态
3. 优化用户体验
4. 增加数据验证
5. 实现离线缓存

## 部署配置

### 开发环境
- 前端：`npm run dev`
- 后端：`cargo run`
- 代理配置：Vite代理到后端API

### 生产环境
- 前端：`npm run build`
- 后端：编译为可执行文件
- 静态文件服务：Nginx
- API服务：后端独立运行

## 测试建议

### 功能测试
1. 用户登录注册
2. 资源上传下载
3. 管理员操作
4. 数据统计查看
5. 日志查看清理

### 性能测试
1. 大量数据加载
2. 并发请求处理
3. 缓存效果验证
4. 网络异常处理

### 兼容性测试
1. 不同浏览器测试
2. 移动端适配
3. 响应式布局
4. 主题切换功能 