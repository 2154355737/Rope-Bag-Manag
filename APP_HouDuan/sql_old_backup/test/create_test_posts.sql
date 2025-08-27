-- 创建一些测试帖子数据用于测试帖子审核功能

-- 插入测试帖子（待审核状态）
INSERT OR IGNORE INTO posts (
    title, content, author_id, category_id, status, 
    view_count, like_count, comment_count, is_pinned, is_featured,
    created_at, updated_at, review_status
) VALUES 
('第一个测试帖子', '这是第一个测试帖子的内容，用于测试帖子审核功能。', 1, 1, 'Published', 0, 0, 0, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'pending'),
('Vue.js 开发经验分享', '分享一些 Vue.js 开发的最佳实践和经验技巧。包括组件设计、状态管理、性能优化等方面的内容。', 1, 1, 'Published', 5, 2, 1, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'pending'),
('Rust 后端开发指南', '详细介绍如何使用 Rust 进行后端开发，包括 Web 框架选择、数据库操作、API 设计等。', 1, 2, 'Published', 10, 3, 2, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'pending'),
('前端性能优化技巧', '总结一些前端性能优化的实用技巧，包括代码分割、懒加载、缓存策略等。', 1, 1, 'Published', 8, 1, 0, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'pending'),
('数据库设计最佳实践', '分享数据库设计的一些最佳实践，包括表结构设计、索引优化、查询性能调优等。', 1, 2, 'Published', 12, 4, 3, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 'pending');

-- 显示插入的测试数据
SELECT COUNT(*) as '插入的测试帖子数量' FROM posts WHERE review_status = 'pending';

-- 显示所有待审核的帖子
SELECT id, title, author_id, status, review_status, created_at 
FROM posts 
WHERE review_status = 'pending' 
ORDER BY created_at DESC; 