# Markdown渲染器设置指南

## 1. 安装依赖

请运行以下命令安装Markdown渲染所需的依赖：

```bash
npm install react-markdown remark-gfm rehype-highlight rehype-raw highlight.js
```

## 2. 已添加的功能

### MarkdownRenderer组件特性
- ✅ 支持标准Markdown语法
- ✅ 支持GitHub Flavored Markdown (GFM)
  - 表格
  - 删除线
  - 任务列表
  - 自动链接
- ✅ 代码高亮显示
- ✅ 支持HTML内容混合渲染
- ✅ 图片点击放大功能
- ✅ 自定义样式适配暗色主题
- ✅ 自动检测内容类型（HTML/Markdown/纯文本）

### 支持的Markdown元素
- 标题 (h1-h6)
- 段落和换行
- **粗体** 和 *斜体*
- `行内代码` 和代码块
- 链接（自动在新窗口打开）
- 图片（支持点击放大）
- 列表（有序和无序）
- 引用块
- 表格
- 分隔线

## 3. 使用方式

### 在通用详情页面
组件已经自动集成到 `universal-detail-screen.tsx` 中的 `ContentRenderer`。

### 在其他组件中使用
```tsx
import MarkdownRenderer from '@/components/ui/markdown-renderer'

// 基础使用
<MarkdownRenderer content={markdownContent} />

// 带图片点击功能
<MarkdownRenderer 
  content={markdownContent}
  onImageClick={(images, index) => handleImageClick(images, index)}
/>

// 自定义样式
<MarkdownRenderer 
  content={markdownContent}
  className="custom-markdown-styles"
/>
```

## 4. 样式自定义

组件使用Tailwind CSS类，可以通过`className`属性进行自定义：

```tsx
<MarkdownRenderer 
  content={content}
  className="text-blue-600 prose-headings:text-red-500"
/>
```

## 5. 容错处理

如果依赖未安装，组件会：
- 自动降级到基础文本渲染
- 在控制台显示警告信息
- 保持基本的文本格式显示

## 6. 代码高亮主题

默认使用GitHub主题，如需更改可以：

1. 在`src/components/ui/markdown-renderer.tsx`中修改：
```tsx
// 更改为其他主题
import 'highlight.js/styles/atom-one-dark.css'
```

2. 或在CSS中自定义代码块样式

## 7. 性能优化

- 组件会自动检测内容类型，避免不必要的解析
- 支持动态导入，未安装依赖时不会影响应用启动
- 图片懒加载和点击放大功能

安装完成后重启开发服务器即可使用完整的Markdown渲染功能！ 