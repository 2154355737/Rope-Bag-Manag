# 绳包管理系统 - 主题使用规范指南

## 📋 概述

本文档为开发者提供主题系统的正确使用规范，确保整个应用的UI一致性和可维护性。

## 🚨 重要原则

### 1. 禁止重复定义CSS变量
❌ **错误做法**：
```css
/* 在组件中重新定义全局变量 */
:global(:root) {
  --brand-color: #409EFF;
  --text-primary: #303133;
}
```

✅ **正确做法**：
```css
/* 直接使用全局主题变量 */
.my-component {
  color: var(--brand-color);
  background: var(--text-primary);
}
```

### 2. 使用正确的深色模式选择器
❌ **错误做法**：
```css
:global(:root.dark) {
  /* 深色模式样式 */
}
```

✅ **正确做法**：
```css
html.dark .my-component {
  /* 深色模式样式 */
}
```

### 3. 避免硬编码颜色值
❌ **错误做法**：
```css
.my-component {
  background: rgba(255, 215, 0, 0.9);  /* 硬编码金色 */
  color: white;  /* 硬编码白色 */
}
```

✅ **正确做法**：
```css
.my-component {
  background: var(--warning-color);
  color: var(--text-inverse);
}
```

## 🎨 主题变量使用指南

### 品牌色系
```css
/* 主品牌色 */
color: var(--brand-color);

/* 品牌色变体 */
background: var(--brand-color-light);    /* 浅色 */
border: 1px solid var(--brand-color-dark); /* 深色 */
```

### 语义化颜色
```css
/* 成功状态 */
color: var(--success-color);
background: var(--success-color-light);

/* 警告状态 */
color: var(--warning-color);
background: var(--warning-color-light);

/* 危险状态 */
color: var(--danger-color);
background: var(--danger-color-light);

/* 信息状态 */
color: var(--info-color);
background: var(--info-color-light);
```

### 文字颜色
```css
/* 主要文字 */
color: var(--text-primary);

/* 次要文字 */
color: var(--text-secondary);

/* 辅助文字 */
color: var(--text-tertiary);

/* 禁用文字 */
color: var(--text-disabled);

/* 反色文字（用于深色背景） */
color: var(--text-inverse);
```

### 背景颜色
```css
/* 主要背景 */
background: var(--bg-primary);

/* 次要背景 */
background: var(--bg-secondary);

/* 卡片背景 */
background: var(--bg-card);

/* 悬停背景 */
background: var(--bg-hover);

/* 毛玻璃背景 */
background: var(--bg-glass);
backdrop-filter: var(--glass-backdrop);
```

### 边框颜色
```css
/* 浅色边框 */
border: 1px solid var(--border-light);

/* 中等边框 */
border: 1px solid var(--border-medium);

/* 深色边框 */
border: 1px solid var(--border-strong);

/* 焦点边框 */
border: 1px solid var(--border-focus);
```

### 阴影系统
```css
/* 小阴影 */
box-shadow: var(--shadow-sm);

/* 中等阴影 */
box-shadow: var(--shadow-md);

/* 大阴影 */
box-shadow: var(--shadow-lg);

/* 特大阴影 */
box-shadow: var(--shadow-xl);

/* 毛玻璃阴影 */
box-shadow: var(--shadow-glass);
```

### 间距系统
```css
/* 使用8pt网格系统 */
padding: var(--space-4);    /* 16px */
margin: var(--space-6);     /* 24px */
gap: var(--space-2);        /* 8px */
```

### 圆角系统
```css
/* 小圆角 */
border-radius: var(--radius-sm);   /* 6px */

/* 中等圆角 */
border-radius: var(--radius-md);   /* 8px */

/* 大圆角 */
border-radius: var(--radius-lg);   /* 12px */

/* 特大圆角 */
border-radius: var(--radius-xl);   /* 16px */
```

## 🔧 组件样式最佳实践

### 1. 组件样式文件组织
```css
/* 推荐的文件结构 */
src/
├── assets/
│   ├── theme-variables.css     /* 全局主题变量 */
│   └── component-styles.css    /* 组件样式 */
├── components/
│   └── MyComponent.vue
```

### 2. Vue组件中的样式引用
```vue
<template>
  <div class="my-component">
    <!-- 组件内容 -->
  </div>
</template>

<style scoped>
/* 引入外部样式文件 */
@import '@/assets/component-styles.css';

/* 组件特有样式 */
.my-component {
  /* 使用主题变量 */
  background: var(--bg-card);
  color: var(--text-primary);
}
</style>
```

### 3. 深色模式适配
```css
/* 基础样式 */
.my-component {
  background: var(--bg-primary);
  color: var(--text-primary);
}

/* 深色模式适配 */
html.dark .my-component {
  background: var(--bg-primary);
  color: var(--text-primary);
}
```

## 🎯 常见问题解决

### 问题1: 主题切换不生效
**原因**: 使用了硬编码颜色值
**解决**: 替换为主题变量

### 问题2: 深色模式样式不显示
**原因**: 使用了错误的选择器
**解决**: 使用 `html.dark` 选择器

### 问题3: 样式冲突
**原因**: 重复定义CSS变量
**解决**: 移除重复定义，使用全局变量

### 问题4: 组件样式过大
**原因**: 样式代码写在Vue文件中
**解决**: 分离到独立的CSS文件

## 📝 代码审查清单

在提交代码前，请检查：

- [ ] 是否使用了硬编码颜色值
- [ ] 是否重复定义了CSS变量
- [ ] 深色模式选择器是否正确
- [ ] 是否使用了语义化的主题变量
- [ ] 组件样式是否分离到独立文件
- [ ] 是否遵循了8pt网格系统
- [ ] 是否使用了正确的圆角和阴影

## 🔄 主题系统更新

当需要更新主题时：

1. 修改 `src/assets/theme-variables.css`
2. 更新主题文档
3. 通知团队成员
4. 进行全面的视觉测试

## 📚 相关文档

- [主题系统使用指南](./THEME_SYSTEM_GUIDE.md)
- [主题重构总结](./THEME_REFACTOR_SUMMARY.md)
- [Element Plus 主题覆盖](./src/assets/element-plus-dark.css)

---

**记住**: 主题系统是整个应用的视觉基础，正确使用主题变量是保证UI一致性的关键！ 