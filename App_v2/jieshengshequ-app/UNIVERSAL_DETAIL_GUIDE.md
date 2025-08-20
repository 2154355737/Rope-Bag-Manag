# 通用详情页面使用说明

## 概述

`UniversalDetailScreen` 是一个统一的详情页面组件，用于替代之前分离的 `PostDetailScreen`、`ResourceDetailScreen` 和 `AnnouncementDetailScreen`。

## 特性

### 🎯 统一体验
- 所有类型的详情页面使用相同的布局和交互模式
- 一致的导航、操作按钮和评论系统
- 统一的文本溢出处理和响应式设计

### 🔧 智能适配
- 根据URL路径自动识别内容类型 (`/post/:id`, `/resource/:id`, `/announcement/:id`)
- 根据内容类型显示相应的字段和功能
- 智能的文本内容渲染（代码、链接、列表等）

### 📱 功能完整
- **帖子**：内容展示、代码高亮、图片显示
- **资源**：描述、截图、文件列表、系统要求、下载功能
- **公告**：公告内容、优先级、有效期

## 路由配置

```tsx
// App.tsx 中的路由配置
<Route path="post/:id" element={<UniversalDetailScreen />} />
<Route path="resource/:id" element={<UniversalDetailScreen />} />
<Route path="announcement/:id" element={<UniversalDetailScreen />} />
```

## 组件结构

### 主要组件
- `UniversalDetailScreen`: 主容器组件
- `ContentRenderer`: 内容渲染组件，根据类型显示不同内容

### 数据接口
```typescript
interface UniversalDetailItem {
  id: number
  type: 'post' | 'resource' | 'announcement'
  title: string
  author: { name: string; avatar: string; verified?: boolean }
  content?: string        // 帖子和公告内容
  description?: string    // 资源描述
  tags: string[]
  stats: { likes: number; comments: number; views: number; downloads?: number; rating?: number }
  publishDate: string
  
  // 类型特定字段
  // 资源特有: version, fileSize, downloadUrl, files, requirements, screenshots, safetyStatus
  // 帖子特有: images, code
  // 公告特有: priority, validUntil
}
```

## 智能内容渲染

### 文本处理
- **URL链接**: 自动识别并添加点击跳转
- **代码路径**: 特殊背景显示（包含 `java` 和 `.` 或 `/`）
- **特殊标记**: 橙色高亮显示（以 `★` 或 `@` 开头）
- **方法标题**: 主色调强调显示（包含"方法"）
- **代码示例**: 代码块样式（包含 `code`、`class`、`()`）
- **列表项**: 缩进显示（以 `•` 或 `-` 开头）

### 类型特定显示
- **帖子**: 内容 + 代码高亮 + 图片网格
- **资源**: 截图 + 详细描述 + 系统要求 + 文件列表 + 下载按钮
- **公告**: 公告内容 + 优先级标记 + 有效期

## 优势

### 🔄 维护性
- 单一代码库，减少重复代码
- 统一的bug修复和功能增强
- 更容易进行样式和交互的一致性改进

### 🎨 一致性
- 所有详情页面的用户体验完全一致
- 统一的文本处理和布局规则
- 相同的交互模式和反馈

### 🚀 扩展性
- 易于添加新的内容类型
- 内容渲染逻辑可重用
- 统一的数据获取和错误处理

## 迁移说明

### 从旧页面迁移
1. 路由自动重定向到新的通用页面
2. 所有现有功能保持不变
3. 用户无感知迁移

### API兼容性
- 使用现有的 `getPost`、`getResource`、`getAnnouncement` API
- 评论系统完全兼容
- 相关推荐系统已集成

## 性能优化

- 按需加载内容渲染
- 智能的图片懒加载
- 优化的文本渲染算法
- 统一的错误边界处理 