# 数据库修补工具使用说明

## 概述

数据库修补工具用于安全地更新绳包管理器的数据库结构，添加新的配置管理功能所需的设置项。

## 功能特性

- ✅ 自动备份原数据库
- ✅ 安全的数据库结构迁移
- ✅ 配置项完整性验证
- ✅ 回滚功能
- ✅ 详细的操作日志

## 使用方法

### 方法一：使用批处理脚本（推荐）

```bash
# 运行修补工具
./patch_database.bat
```

脚本会提供交互式菜单：
1. **执行数据库修补** - 主要功能，修补数据库结构
2. **查看修补状态** - 显示当前数据库配置状态
3. **回滚到备份** - 恢复到修补前的状态
4. **退出** - 退出工具

### 方法二：直接使用命令行

```bash
# 编译修补工具
cargo build --bin patch_database --release

# 执行修补（默认操作）
./target/release/patch_database.exe data.db

# 查看状态
./target/release/patch_database.exe data.db status

# 回滚到备份
./target/release/patch_database.exe data.db rollback data.db.backup_20240101_120000
```

## 修补内容

此修补工具会添加以下配置项：

### 🎨 主题设置
- `primary_color` - 主要颜色
- `secondary_color` - 次要颜色  
- `dark_mode` - 深色模式
- `font_size` - 字体大小
- `language` - 系统语言

### 🔧 功能开关
- `enable_registration` - 允许用户注册
- `enable_community` - 启用社区功能
- `enable_upload` - 允许文件上传
- `enable_comments` - 启用评论功能
- `enable_qq_binding` - 启用QQ绑定

### 🏠 首页配置
- `hero_title` - 首页主标题
- `hero_subtitle` - 首页副标题
- `homepage_sections` - 主页显示模块配置
- `resources_per_page` - 每页显示资源数量
- `posts_per_page` - 每页显示帖子数量
- `default_sort` - 默认排序方式

### 📄 页脚配置
- `copyright_text` - 版权信息
- `icp_number` - ICP备案号
- `footer_show_links` - 页脚显示友情链接
- `footer_show_stats` - 页脚显示统计信息

### 🔍 SEO配置
- `seo_keywords` - SEO关键词
- `seo_description` - SEO描述
- `seo_author` - SEO作者信息

### 🌐 社区设置
- `site_title` - 网站标题
- `site_subtitle` - 网站副标题
- `site_description` - 网站描述
- `welcome_message` - 欢迎消息
- `footer_text` - 页脚文本
- `contact_email` - 联系邮箱

### ⚙️ 后端配置
- `proxy_address` - 代理地址
- `api_timeout` - API超时时间
- `max_upload_size` - 最大上传文件大小

### 💾 备份配置
- `enable_auto_backup` - 启用自动备份
- `backup_interval_hours` - 备份间隔时间
- `backup_location` - 备份文件存储路径
- `max_backup_files` - 最大备份文件数量

### 📢 全局公告配置
- `global_announcement_enabled` - 启用全局公告
- `global_announcement_title` - 全局公告标题
- `global_announcement_content` - 全局公告内容
- `global_announcement_type` - 全局公告类型
- `global_announcement_priority` - 全局公告优先级

## 安全机制

### 自动备份
- 修补前自动创建数据库备份
- 备份文件名格式：`data.db.backup_YYYYMMDD_HHMMSS`
- 支持手动回滚到任意备份点

### 事务保护
- 所有修补操作在单个事务中执行
- 任何错误都会自动回滚，保证数据完整性

### 完整性验证
- 修补后自动验证表结构
- 检查关键配置项是否正确添加
- 验证失败会报错并提供回滚建议

## 故障排除

### 常见问题

1. **找不到数据库文件**
   ```
   ❌ 错误: 未找到数据库文件 data.db
   ```
   - 确保在包含 `data.db` 的目录下运行
   - 检查数据库文件权限

2. **找不到修补脚本**
   ```
   ❌ 错误: 未找到修补脚本 sql\patch_database_settings.sql
   ```
   - 确保 `sql/` 目录存在
   - 确保修补脚本文件存在

3. **编译失败**
   ```
   ❌ 编译失败，请检查代码
   ```
   - 检查 Rust 环境是否正确安装
   - 运行 `cargo check` 检查代码问题

4. **修补失败**
   - 查看错误信息确定具体原因
   - 使用回滚功能恢复到修补前状态
   - 检查数据库文件是否被其他程序占用

### 回滚操作

如果修补出现问题，可以回滚到备份：

```bash
# 查看可用备份
dir data.db.backup_*

# 回滚到指定备份
./patch_database.bat
# 选择选项 3，然后输入备份文件名
```

## 日志记录

修补工具会在 `system_logs` 表中记录操作日志：
- 修补开始和完成时间
- 添加的配置项数量
- 任何错误或警告信息

可以通过管理后台的日志查看功能查看详细记录。

## 注意事项

⚠️ **重要提醒**：
1. 修补前请确保数据库没有被其他程序占用
2. 建议在非生产环境先测试修补过程
3. 保留好自动生成的备份文件
4. 如有疑问，请先使用状态查看功能了解当前数据库状态

## 技术细节

- **数据库**: SQLite 3
- **语言**: Rust
- **事务**: ACID 兼容
- **备份**: 文件级完整备份
- **验证**: 结构和数据完整性检查

---

如有问题，请查看系统日志或联系技术支持。 