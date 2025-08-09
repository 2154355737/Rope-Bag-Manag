# 标签使用次数统计功能

## 概述
这个功能可以统计和更新标签的真实使用次数，基于帖子和资源包中的标签关联来计算。

## 功能特性

### 1. 命令行工具
使用独立的命令行工具来统计和更新标签使用次数：

```bash
# 运行标签使用次数统计
cargo run --bin update_tag_counts
```

**功能说明：**
- 重置所有标签使用次数为 0
- 统计帖子中的标签使用情况（基于 `post_tags` 表）
- 统计资源包中的标签使用情况（基于 `package_tags` 表，如果存在）
- 更新所有标签的 `use_count` 字段
- 提供详细的统计报告

### 2. API 端点

#### 2.1 更新标签使用次数
```http
POST /api/v1/tags/update-counts
Authorization: Bearer <admin_token>
```

**功能：** 触发标签使用次数的重新计算和更新
**权限：** 需要管理员权限
**响应：**
```json
{
  "code": 0,
  "message": "标签使用次数更新成功",
  "msg": "标签使用次数更新成功"
}
```

#### 2.2 获取标签使用统计
```http
GET /api/v1/tags/stats
```

**功能：** 获取标签使用的统计信息
**权限：** 无需认证
**响应：**
```json
{
  "code": 0,
  "message": "success",
  "data": {
    "total_tags": 10,
    "used_tags": 5,
    "unused_tags": 5,
    "total_usage": 45,
    "post_tag_usage": 30,
    "package_tag_usage": 15
  }
}
```

## 数据库结构

### 相关表
1. **tags** - 标签主表
   - `use_count`: 标签使用次数（自动计算）
   
2. **post_tags** - 帖子标签关联表
   - `post_id`: 帖子ID
   - `tag_id`: 标签ID
   
3. **package_tags** - 资源包标签关联表（可选）
   - `package_id`: 资源包ID
   - `tag_id`: 标签ID

### 计算规则
```sql
use_count = (
    SELECT COUNT(*) FROM post_tags WHERE tag_id = tags.id
) + (
    SELECT COUNT(*) FROM package_tags WHERE tag_id = tags.id  -- 如果表存在
)
```

## 使用建议

### 1. 定期更新
建议定期运行标签使用次数统计，可以通过以下方式：

**方式一：手动运行**
```bash
cd rope-manager-backend
cargo run --bin update_tag_counts
```

**方式二：API 调用**
```bash
curl -X POST http://localhost:8080/api/v1/tags/update-counts \
  -H "Authorization: Bearer <admin_token>"
```

### 2. 监控统计
通过统计API监控标签使用情况：

```bash
curl http://localhost:8080/api/v1/tags/stats
```

### 3. 集成到系统
可以将标签使用次数更新集成到以下场景：
- 用户发布新帖子时
- 用户上传新资源包时
- 定时任务（每日/每周更新一次）

## 错误处理

### 表不存在
如果 `package_tags` 表不存在，系统会自动跳过相关统计，只基于 `post_tags` 计算使用次数。

### 权限错误
API 调用需要管理员权限，非管理员用户调用会返回 403 错误。

## 输出示例

运行命令行工具时的典型输出：
```
开始统计标签真实使用次数...
✓ 已重置所有标签使用次数为0
标签 ID 1 在帖子中使用了 5 次
标签 ID 2 在帖子中使用了 3 次
⚠️ package_tags 表不存在，跳过资源包标签统计
✓ 已更新所有标签的使用次数

=== 统计结果 ===
帖子标签使用总次数: 8
资源包标签使用总次数: 0
标签使用总次数: 8

=== 标签使用次数排行榜 ===
1. [ID:1] Vue - 总计: 5 次 (帖子: 5 次, 资源: 0 次)
2. [ID:2] Rust - 总计: 3 次 (帖子: 3 次, 资源: 0 次)
...

✅ 标签使用次数统计完成！
```

## 注意事项

1. **数据一致性**：更新操作会重置所有标签的使用次数，然后重新计算
2. **性能影响**：对于大量数据，统计过程可能需要一些时间
3. **权限控制**：API 更新功能仅限管理员使用，确保数据安全
4. **向后兼容**：代码会自动检测表是否存在，兼容不同的数据库结构 