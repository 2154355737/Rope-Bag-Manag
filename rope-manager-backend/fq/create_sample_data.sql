-- 创建示例社区动态数据

-- 插入一些示例用户行为记录
INSERT OR IGNORE INTO user_actions (id, user_id, action_type, target_type, target_id, details, ip_address, user_agent, created_at) VALUES 
(1, 1, '上传资源', 'Package', 1, '上传了新的开发工具包', '127.0.0.1', 'Mozilla/5.0', datetime('now', '-2 hours')),
(2, 1, '发布帖子', 'Post', 1, '分享了关于React开发的经验', '127.0.0.1', 'Mozilla/5.0', datetime('now', '-1 hour')),
(3, 1, '评论资源', 'Package', 2, '对Vue组件库进行了评价', '127.0.0.1', 'Mozilla/5.0', datetime('now', '-30 minutes')),
(4, 1, '点赞帖子', 'Post', 2, '点赞了技术分享帖子', '127.0.0.1', 'Mozilla/5.0', datetime('now', '-15 minutes')),
(5, 1, '收藏资源', 'Package', 3, '收藏了实用工具集', '127.0.0.1', 'Mozilla/5.0', datetime('now', '-10 minutes')),
(6, 1, '更新资源', 'Package', 1, '更新了开发工具包到v2.0', '127.0.0.1', 'Mozilla/5.0', datetime('now', '-5 minutes'));