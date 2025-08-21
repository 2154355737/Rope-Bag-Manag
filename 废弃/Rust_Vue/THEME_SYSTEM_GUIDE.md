# 绳包管理系统 - 主题系统使用指南

## 📖 概述

本项目采用统一的主题系统，支持多种颜色主题和深色/浅色模式切换。所有主题变量都定义在 `src/assets/theme-variables.css` 中，确保整个应用的视觉一致性。

## 🎨 主题架构

### 核心设计原则

1. **统一管理**: 所有主题变量集中在一个文件中管理
2. **语义化命名**: 使用语义化的变量名，便于理解和维护
3. **完整适配**: 所有主题都支持深色模式
4. **无障碍支持**: 支持高对比度模式和减少动画模式
5. **响应式设计**: 内置响应式断点系统

### 主题变量结构

```
theme-variables.css
├── 基础设计令牌 (Design Tokens)
│   ├── 颜色调色板 (50-900 色阶)
│   ├── 语义化主题变量
│   ├── 背景色系统
│   ├── 文字色系统
│   ├── 边框色系统
│   ├── 阴影系统
│   ├── 圆角系统
│   ├── 间距系统
│   ├── 字体系统
│   ├── 过渡动画系统
│   └── 层级系统
├── 深色模式主题
├── 彩色主题变体 (蓝色、绿色、紫色、橙色、红色)
├── Element Plus 组件主题覆盖
└── 无障碍支持
```

## 🌈 可用主题

### 主题类型

- **light**: 默认浅色主题
- **dark**: 深色主题
- **blue**: 蓝色主题
- **green**: 绿色主题
- **purple**: 紫色主题
- **orange**: 橙色主题
- **red**: 红色主题
- **auto**: 自动跟随系统主题

### 主题应用方式

主题通过在 `html` 元素上添加对应的类名来应用：

```html
<!-- 浅色主题 -->
<html class="light">

<!-- 深色主题 -->
<html class="dark">

<!-- 蓝色深色主题 -->
<html class="blue dark">
```

## 🎯 使用方法

### 1. 基础颜色使用

```css
.my-component {
  /* 品牌色 */
  color: var(--brand-color);
  background: var(--brand-color-light);
  border: 1px solid var(--brand-color-dark);
  
  /* 语义化颜色 */
  color: var(--success-color);
  color: var(--warning-color);
  color: var(--danger-color);
  color: var(--info-color);
}
```

### 2. 背景色系统

```css
.my-component {
  /* 主要背景 */
  background: var(--bg-primary);
  
  /* 次要背景 */
  background: var(--bg-secondary);
  
  /* 卡片背景 */
  background: var(--bg-card);
  
  /* 悬停状态 */
  background: var(--bg-hover);
  
  /* 毛玻璃效果 */
  background: var(--bg-glass);
  backdrop-filter: var(--glass-backdrop);
}
```

### 3. 文字色系统

```css
.my-component {
  /* 主要文字 */
  color: var(--text-primary);
  
  /* 次要文字 */
  color: var(--text-secondary);
  
  /* 三级文字 */
  color: var(--text-tertiary);
  
  /* 禁用状态 */
  color: var(--text-disabled);
  
  /* 反色文字 */
  color: var(--text-inverse);
}
```

### 4. 边框和阴影

```css
.my-component {
  /* 边框 */
  border: 1px solid var(--border-color);
  border: 1px solid var(--border-color-light);
  border: 1px solid var(--border-color-strong);
  
  /* 聚焦边框 */
  border-color: var(--border-color-focus);
  
  /* 阴影 */
  box-shadow: var(--shadow-sm);
  box-shadow: var(--shadow-md);
  box-shadow: var(--shadow-lg);
  box-shadow: var(--shadow-xl);
}
```

### 5. 间距和圆角

```css
.my-component {
  /* 间距 */
  padding: var(--space-4);
  margin: var(--space-2) var(--space-4);
  gap: var(--space-3);
  
  /* 圆角 */
  border-radius: var(--radius-md);
  border-radius: var(--radius-lg);
  border-radius: var(--radius-xl);
}
```

### 6. 字体系统

```css
.my-component {
  /* 字体大小 */
  font-size: var(--font-size-sm);
  font-size: var(--font-size-base);
  font-size: var(--font-size-lg);
  
  /* 字体粗细 */
  font-weight: var(--font-weight-normal);
  font-weight: var(--font-weight-medium);
  font-weight: var(--font-weight-semibold);
  
  /* 行高 */
  line-height: var(--line-height-normal);
  line-height: var(--line-height-tight);
}
```

### 7. 过渡动画

```css
.my-component {
  /* 过渡动画 */
  transition: var(--transition-fast);
  transition: var(--transition-normal);
  transition: var(--transition-slow);
  
  /* 或者使用综合过渡 */
  transition: var(--transition-all);
}
```

## 🔧 主题切换

### 使用主题切换器

```typescript
import { useThemeSwitcher } from '@/utils/theme'

const { currentTheme, applyTheme, themeConfigs } = useThemeSwitcher()

// 切换到深色主题
applyTheme('dark')

// 切换到蓝色主题
applyTheme('blue')

// 自动主题
applyTheme('auto')
```

### 主题切换器组件

项目中已包含 `ThemeSwitcher.vue` 组件，可直接使用：

```vue
<template>
  <ThemeSwitcher />
</template>
```

## 📱 响应式设计

### 内置断点

```css
/* 小屏幕 */
@media (max-width: 640px) {
  /* 移动端样式 */
}

/* 中等屏幕 */
@media (min-width: 768px) {
  /* 平板样式 */
}

/* 大屏幕 */
@media (min-width: 1024px) {
  /* 桌面样式 */
}
```

### 使用断点变量

```css
.my-component {
  width: 100%;
}

@media (min-width: var(--breakpoint-md)) {
  .my-component {
    width: 50%;
  }
}
```

## ♿ 无障碍支持

### 高对比度模式

系统自动适配用户的高对比度偏好：

```css
@media (prefers-contrast: high) {
  /* 高对比度样式自动应用 */
}
```

### 减少动画模式

系统自动适配用户的动画偏好：

```css
@media (prefers-reduced-motion: reduce) {
  /* 动画被自动禁用或简化 */
}
```

## 🎨 自定义主题

### 创建新主题

1. 在 `theme-variables.css` 中添加新主题：

```css
/* 自定义主题 */
html.custom {
  --brand-color: #your-color;
  --brand-color-light: #your-light-color;
  --brand-color-dark: #your-dark-color;
  /* 其他自定义变量 */
}

/* 自定义主题的深色模式 */
html.custom.dark {
  --bg-primary: #your-dark-bg;
  --text-primary: #your-dark-text;
  /* 其他深色模式变量 */
}
```

2. 在 `theme.ts` 中注册新主题：

```typescript
export const themeConfigs: Record<ThemeType, ThemeConfig> = {
  // ... 现有主题
  custom: {
    name: 'custom',
    label: '自定义主题',
    icon: '🎨',
    description: '您的自定义主题'
  }
}
```

## 🚫 注意事项

### 禁止的做法

1. **❌ 不要在组件中重复定义主题变量**
```css
/* 错误做法 */
:global(:root) {
  --text-primary: #333;
}
```

2. **❌ 不要使用硬编码颜色**
```css
/* 错误做法 */
.my-component {
  color: #333333;
  background: #ffffff;
}
```

3. **❌ 不要使用 :root.dark 选择器**
```css
/* 错误做法 */
:global(:root.dark) {
  /* 样式 */
}
```

### 推荐的做法

1. **✅ 使用语义化的主题变量**
```css
/* 正确做法 */
.my-component {
  color: var(--text-primary);
  background: var(--bg-card);
}
```

2. **✅ 使用 html.dark 选择器**
```css
/* 正确做法 */
:global(html.dark) .my-component {
  /* 深色模式特定样式 */
}
```

3. **✅ 遵循设计系统规范**
```css
/* 正确做法 */
.my-component {
  padding: var(--space-4);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  transition: var(--transition-normal);
}
```

## 🔍 调试主题

### 开发者工具

1. 在浏览器开发者工具中查看 `:root` 元素的计算样式
2. 搜索 `--` 查看所有CSS变量
3. 实时修改变量值测试效果

### 主题测试

```javascript
// 在控制台中测试主题切换
document.documentElement.className = 'dark'
document.documentElement.className = 'blue dark'
```

## 📚 最佳实践

1. **优先使用语义化变量**: 使用 `--text-primary` 而不是 `--gray-900`
2. **保持一致性**: 在整个应用中使用相同的变量命名
3. **测试所有主题**: 确保组件在所有主题下都正常显示
4. **考虑无障碍**: 确保颜色对比度符合无障碍标准
5. **响应式优先**: 使用系统提供的断点和间距变量

## 🆘 常见问题

### Q: 主题切换后颜色没有变化？
A: 检查是否使用了硬编码颜色，确保使用主题变量。

### Q: 深色模式下某些元素显示异常？
A: 检查是否使用了正确的选择器 `html.dark`，而不是 `:root.dark`。

### Q: 如何添加新的颜色变量？
A: 在 `theme-variables.css` 中的相应部分添加，并确保所有主题都有对应定义。

### Q: Element Plus 组件样式没有适配主题？
A: 检查 `theme-variables.css` 中的 Element Plus 覆盖样式是否完整。

---

## 📞 支持

如有任何问题或建议，请：
1. 查阅本文档
2. 检查 `theme-variables.css` 中的变量定义
3. 参考现有组件的实现方式
4. 联系开发团队 