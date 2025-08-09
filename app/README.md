# 资源社区APP

这是一个基于Vue 3 + Vite + Vant UI开发的资源社区移动端应用，为用户提供资源分享、下载和交流的平台。

## 功能特点

- 资源浏览：支持分类浏览、搜索和筛选
- 资源下载：支持资源查看和下载
- 用户系统：支持注册、登录和个人中心
- 评论系统：支持资源评论和交流
- 收藏功能：支持收藏喜欢的资源
- 移动端优化：针对移动设备进行UI/UX优化

## 技术栈

- **前端框架**：Vue 3
- **构建工具**：Vite
- **UI组件**：Vant UI
- **状态管理**：Pinia
- **路由管理**：Vue Router
- **HTTP请求**：Axios

## 项目结构

```
app/
├── public/             # 静态资源
│   └── img/            # 图片资源
│       ├── banner/     # 轮播图
│       ├── categories/ # 分类图标
│       └── default/    # 默认图片
├── src/
│   ├── api/            # API请求接口
│   │   └── resource.js # 资源相关API
│   ├── assets/         # 项目资源文件
│   │   └── styles/     # 样式文件
│   │       └── main.css # 全局样式
│   ├── components/     # 通用组件
│   │   ├── BannerSwiper.vue # 轮播图组件
│   │   ├── CategoryList.vue # 分类列表组件
│   │   ├── ResourceCard.vue # 资源卡片组件
│   │   ├── ResourceList.vue # 资源列表组件
│   │   └── TabBar.vue       # 底部导航栏组件
│   ├── router/         # 路由配置
│   │   └── index.js    # 路由定义
│   ├── store/          # Pinia状态管理
│   │   └── user.js     # 用户状态
│   ├── utils/          # 工具函数
│   │   ├── request.js  # 请求工具
│   │   └── apiClient.js # API客户端
│   ├── views/          # 页面组件
│   │   ├── Category.vue       # 分类页
│   │   ├── Home.vue           # 首页
│   │   ├── Login.vue          # 登录页
│   │   ├── NotFound.vue       # 404页面
│   │   ├── Profile.vue        # 个人中心
│   │   ├── Register.vue       # 注册页
│   │   ├── ResourceDetail.vue # 资源详情页
│   │   ├── Search.vue         # 搜索页
│   │   ├── SubmitResource.vue # 资源提交页
│   │   ├── UserComments.vue   # 用户评论页
│   │   ├── UserFavorites.vue  # 用户收藏页
│   │   └── UserResources.vue  # 用户资源页
│   ├── App.vue         # 根组件
│   └── main.js         # 入口文件
├── index.html          # HTML模板
├── package.json        # 项目依赖
└── vite.config.js      # Vite配置
```

## 页面说明

- **首页**：展示精选资源、最新资源、热门资源，以及分类导航
- **分类页**：按类别浏览资源
- **资源详情页**：查看资源详细信息，下载资源，发表评论
- **搜索页**：搜索资源
- **个人中心**：用户信息管理、我的资源、我的评论、我的收藏等功能
- **登录/注册页**：用户登录和注册

## 如何运行

### 初始准备

1. 确保后端服务（rope-manager-backend）已经启动
2. 在资源目录下创建必要的默认图片：
   ```
   app/public/img/default/default-cover.jpg
   app/public/img/default/default-avatar.jpg
   app/public/img/logo.png
   ```

### 开发环境

1. 安装依赖

```bash
cd app
npm install
```

2. 启动开发服务器

```bash
npm run dev
```

3. 在浏览器中访问 `http://localhost:5173`

### 或者使用快速启动脚本

```bash
cd app
start_dev.bat
```

### 构建生产版本

```bash
cd app
npm run build
```

### 预览生产版本

```bash
cd app
npm run preview
```

## API对接

本项目后端API基于绳包管理器后端服务（rope-manager-backend）。API详情请参考`/rope-manager-backend/openapi.yaml`文档。

## 已知问题

1. 部分API可能需要根据实际后端接口调整
2. 需要手动创建默认图片文件
3. 如果出现SASS依赖问题，可以：
   - 安装sass依赖：`npm install -D sass`
   - 或者使用CSS替代SCSS (当前方案)

## 后续优化方向

- 添加暗色模式支持
- 优化图片懒加载和缓存
- 添加离线浏览功能
- 完善用户体验细节
- 添加更多交互动画效果 