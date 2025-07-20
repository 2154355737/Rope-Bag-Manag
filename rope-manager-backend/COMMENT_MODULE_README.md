# 评论管理模块使用指南

## 🔍 模块概述

评论管理模块用于管理用户对各种资源（如绳包、用户等）的评论，包括评论的创建、查询、修改、删除、回复、点赞、点踩等功能。本模块支持批量操作和状态管理，适用于管理员和普通用户。

## 🛠️ 技术实现

### 后端实现

- **API路由**: `/api/v1/comments`系列接口
- **服务层**: `CommentService`处理评论业务逻辑
- **数据访问**: `CommentRepository`处理数据库操作
- **数据模型**: `Comment`及相关请求/响应模型

### 前端实现

- **管理界面**: `CommentManage.vue`提供评论管理UI
- **API封装**: `comments.ts`提供前端API调用封装
- **组件复用**: 可在资源详情页中集成评论组件

## 💡 功能特性

- **基础功能**: 评论创建、查询、修改、删除
- **高级功能**: 评论回复、点赞、点踩
- **管理功能**: 批量操作、状态筛选、审核处理
- **扩展功能**: 评论通知、敏感词过滤

## 📋 API接口列表

| 路径 | 方法 | 描述 | 权限 |
|------|------|------|------|
| `/comments` | GET | 获取评论列表 | 管理员 |
| `/comments` | POST | 创建新评论 | 登录用户 |
| `/comments/{id}` | GET | 获取单个评论 | 所有用户 |
| `/comments/{id}` | PUT | 更新评论 | 作者/管理员 |
| `/comments/{id}` | DELETE | 删除评论 | 作者/管理员 |
| `/comments/{id}/replies` | GET | 获取评论回复 | 所有用户 |
| `/comments/{id}/reply` | POST | 回复评论 | 登录用户 |
| `/comments/batch/status` | PUT | 批量更新状态 | 管理员 |
| `/comments/batch` | DELETE | 批量删除评论 | 管理员 |
| `/comments/{id}/like` | POST | 点赞评论 | 登录用户 |
| `/comments/{id}/dislike` | POST | 点踩评论 | 登录用户 |
| `/packages/{id}/comments` | GET | 获取包评论 | 所有用户 |
| `/users/{id}/comments` | GET | 获取用户评论 | 用户本人/管理员 |

## 🚀 使用方法

### 管理员操作

1. **查看评论列表**
   - 导航到管理面板 → 评论管理
   - 使用筛选条件过滤评论
   - 分页浏览评论列表

2. **批量操作**
   - 选择多个评论
   - 使用批量操作按钮进行处理

### 开发者集成

1. **前端集成**
   ```typescript
   // 引入评论API
   import { commentApi } from '@/api/comments';
   
   // 获取评论列表
   const getComments = async (targetId) => {
     const response = await commentApi.getPackageComments(targetId);
     return response.data.data;
   };
   
   // 创建评论
   const createComment = async (content, targetId) => {
     await commentApi.createComment({
       content,
       target_type: 'Package',
       target_id: targetId
     });
   };
   ```

2. **后端扩展**
   ```rust
   // 添加自定义评论过滤
   async fn get_filtered_comments(&self, filter: CustomFilter) -> Result<Vec<Comment>> {
       // 实现自定义过滤逻辑
   }
   ```

## ⚠️ 常见问题

1. **评论不显示?**
   - 检查评论状态是否为"Active"
   - 确认当前用户有查看权限

2. **点赞/点踩失败?**
   - 确认用户已登录
   - 检查网络连接

3. **批量操作无效?**
   - 确认已选择评论
   - 检查用户权限

## 📝 开发计划

- [ ] 添加评论举报功能
- [ ] 实现敏感词过滤
- [ ] 添加评论热度排序
- [ ] 集成通知系统

---

✅ 本模块已完成基础功能开发，可投入使用。如有问题，请联系技术支持。 