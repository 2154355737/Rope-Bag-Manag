# SQL文件使用指南

## 📁 新的SQL目录结构

```
sql/
├── init.sql                    # 📖 完整数据库初始化脚本
├── database_manager.sql        # 🔧 数据库维护和管理工具
├── schema/                     # 📋 表结构定义
│   ├── 001_core_tables.sql     # 👥 用户、资源、社区核心表
│   ├── 002_system_tables.sql   # ⚙️ 系统、安全、日志管理表
│   └── 003_indexes.sql         # 🚀 性能优化索引定义
├── migrations/                 # 🔄 数据库版本迁移
│   └── 001_add_missing_columns.sql
├── seeds/                      # 🌱 初始数据和默认配置
│   └── 001_default_data.sql
└── patches/                    # 🩹 未来补丁脚本目录
```

## 🚀 快速开始

### 全新安装
```bash
# 1. 启动项目，会自动执行数据库初始化
cargo run

# 数据库会自动创建：
# - 所有必要的表结构
# - 性能优化索引  
# - 默认系统配置
# - 管理员账户 (admin/admin123)
# - 基础分类和标签
```

### 现有项目升级
```bash
# 项目会自动检测并应用迁移
cargo run

# 迁移脚本会：
# - 检查缺失的表和列
# - 安全地添加新功能
# - 保留所有现有数据
```

## 📖 核心SQL文件详解

### 1. init.sql - 主初始化脚本
**用途**: 新数据库的完整初始化  
**包含**: 
- 所有表结构定义
- 核心索引创建
- 基础默认数据
- 系统配置设置

**自动执行**: ✅ 在 `main.rs` 中自动调用

### 2. database_manager.sql - 数据库管理工具
**用途**: 日常维护和优化  
**功能**:
- 数据库完整性检查
- 性能优化 (VACUUM, ANALYZE)
- 清理过期数据
- 数据库统计信息
- 备份建议

**执行方式**: 
```bash
# 项目启动时自动执行维护任务
cargo run
```

### 3. migrations/001_add_missing_columns.sql - 表结构迁移
**用途**: 为现有数据库添加新功能  
**安全性**: ✅ 只添加，不删除  
**包含**:
- 新表创建 (如果不存在)
- 新列添加 (如果不存在)
- 新索引创建

## 🏗️ 表结构设计

### 核心业务表 (001_core_tables.sql)

#### 用户系统
- **users**: 用户基本信息、权限、统计
- **email_verifications**: 邮箱验证码管理

#### 资源管理  
- **categories**: 资源分类
- **packages**: 绳包资源主表
- **package_tags**: 资源标签关联

#### 社区功能
- **posts**: 社区帖子
- **comments**: 评论系统 (支持嵌套)
- **comment_likes/dislikes**: 评论点赞系统

#### 标签系统
- **tags**: 标签主表
- **post_tags**: 帖子标签关联

#### 通知订阅
- **subscriptions**: 用户分类订阅
- **notifications**: 系统通知

### 系统管理表 (002_system_tables.sql)

#### 系统配置
- **system_settings**: 系统参数配置
- **system_logs**: 系统日志记录
- **user_actions**: 用户行为日志
- **resource_records**: 资源操作记录

#### 安全管理
- **ip_bans**: IP封禁管理
- **download_security_logs**: 下载安全日志  
- **security_actions**: 安全操作记录

#### 内容管理
- **announcements**: 系统公告
- **banners**: 横幅广告
- **homepage_settings**: 首页配置

#### 邮件系统
- **mail_settings**: SMTP配置

#### 内容过滤
- **forbidden_words**: 违禁词管理

### 性能优化 (003_indexes.sql)
- 🔍 搜索优化索引
- 🔗 外键关联索引  
- 📊 统计查询索引
- 🚀 复合查询索引

## 🌱 默认数据详解 (seeds/001_default_data.sql)

### 系统配置
- 网站基本信息 (名称、描述、Logo)
- 功能开关 (注册、评论、上传等)
- 主题配置 (颜色、字体、语言)
- 文件上传限制
- 安全策略配置

### 默认分类
1. **基础绳结** - 新手入门内容
2. **进阶技巧** - 进阶学习资源
3. **工具器材** - 绳结工具介绍
4. **教学视频** - 视频教程
5. **图文教程** - 详细说明
6. **社区讨论** - 交流讨论
7. **资源分享** - 用户分享
8. **其他** - 未分类内容

### 默认标签
- **难度等级**: 新手入门、进阶技巧、专家级
- **状态标签**: 热门、精品  
- **来源标签**: 原创、转载
- **类型标签**: 视频、图文、工具

### 默认管理员
- **用户名**: admin
- **密码**: admin123 
- ⚠️ **安全提醒**: 生产环境请立即修改密码

## 🔧 开发者指南

### 添加新表
1. 在 `schema/` 目录下相应文件中添加表定义
2. 在 `003_indexes.sql` 中添加相关索引
3. 更新 `migrations/` 目录添加迁移脚本
4. 在 `seeds/` 中添加默认数据（如需要）

### 数据库迁移流程
```sql
-- 1. 创建迁移文件: migrations/002_your_feature.sql
-- 2. 添加表结构变更
CREATE TABLE IF NOT EXISTS new_table (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- 列定义
);

-- 3. 添加索引
CREATE INDEX IF NOT EXISTS idx_new_table_field ON new_table(field);

-- 4. 记录迁移完成
INSERT OR REPLACE INTO system_settings (key, value, description) VALUES 
('migration_002_completed', datetime('now'), '迁移002完成时间');
```

### 性能优化建议
1. **索引策略**: 为常用查询字段添加索引
2. **数据清理**: 定期执行 `database_manager.sql`
3. **分页查询**: 使用 LIMIT 和 OFFSET
4. **复合索引**: 为多字段查询创建复合索引

### 备份和恢复
```bash
# 数据库备份
cp data.db backup_$(date +%Y%m%d_%H%M%S).db

# 手动执行维护
sqlite3 data.db < sql/database_manager.sql

# 查看数据库信息
sqlite3 data.db "SELECT * FROM system_settings WHERE key LIKE 'database_%';"
```

## 🔐 安全注意事项

### 生产环境配置
1. **修改默认密码**: 立即更改admin账户密码
2. **JWT密钥**: 使用强密钥替换默认值
3. **SMTP配置**: 配置真实的邮件服务
4. **备份策略**: 定期备份数据库文件

### 权限管理
- **admin**: 完全权限
- **moderator**: 内容管理权限  
- **elder**: 审核权限
- **user**: 基础用户权限

### 安全功能
- IP封禁系统
- 下载频率限制
- 违禁词过滤
- 用户行为监控

## 📊 监控和诊断

### 系统状态检查
```sql
-- 检查数据库版本
SELECT value FROM system_settings WHERE key = 'database_version';

-- 查看用户统计
SELECT 
    COUNT(*) as total_users,
    COUNT(CASE WHEN is_active = 1 THEN 1 END) as active_users
FROM users;

-- 查看资源统计  
SELECT 
    COUNT(*) as total_packages,
    SUM(download_count) as total_downloads
FROM packages;
```

### 性能监控
```sql
-- 检查数据库大小
SELECT 
    page_count * page_size / 1024 / 1024 as db_size_mb
FROM pragma_page_count(), pragma_page_size();

-- 查看索引使用情况
SELECT name, rootpage FROM sqlite_master WHERE type = 'index';
```

## ❓ 常见问题

### Q: 如何重置数据库？
```bash
# 1. 停止服务
# 2. 删除数据库文件
rm data.db
# 3. 重启服务（会自动重新初始化）
cargo run
```

### Q: 如何添加新的系统配置？
```sql
INSERT INTO system_settings (key, value, description) VALUES 
('your_config_key', 'your_value', '配置说明');
```

### Q: 如何查看SQL执行日志？
项目启动时会在控制台输出SQL执行状态：
- ✅ 数据库初始化成功
- ✅ 表结构迁移成功  
- ✅ 数据库维护和优化成功

### Q: 如何处理迁移失败？
1. 查看错误日志
2. 检查SQL语法
3. 确认表结构兼容性
4. 必要时恢复备份

---

🎉 **恭喜！** 您已掌握了绳包管理器的SQL管理体系！

如有任何问题，请查看 `SQL_OPTIMIZATION_REPORT.md` 了解更多技术细节。