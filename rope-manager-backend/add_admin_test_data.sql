-- 为admin用户添加测试数据

-- 添加一些资源（packages）
INSERT INTO packages (name, description, author, version, tags, category_id, status, created_at) VALUES 
('结绳语言开发工具包', '一套完整的结绳语言开发工具，包含编译器、调试器和IDE插件', 'admin', '1.0.0', '开发工具,编译器,IDE', 1, 'published', datetime('now', '-10 days')),
('移动端UI组件库', '专为结绳语言设计的移动端UI组件库，支持iOS和Android', 'admin', '2.1.0', 'UI组件,移动开发,界面设计', 2, 'published', datetime('now', '-8 days')),
('数据可视化图表库', '强大的数据可视化工具，支持多种图表类型和交互功能', 'admin', '1.5.2', '数据可视化,图表,统计', 3, 'published', datetime('now', '-5 days')),
('网络请求框架', '轻量级的网络请求库，支持HTTP/HTTPS、WebSocket等协议', 'admin', '3.2.1', '网络,HTTP,WebSocket', 1, 'published', datetime('now', '-3 days')),
('加密工具集', '提供各种加密算法的实现，包括AES、RSA、MD5等', 'admin', '1.8.0', '加密,安全,算法', 4, 'published', datetime('now', '-1 days'));

-- 添加一些帖子（posts）
INSERT INTO posts (title, content, author_id, category_id, status, views, likes, created_at) VALUES 
('结绳语言最佳实践分享', '经过多年的开发经验，我总结了一些结绳语言的最佳实践...', 1, 1, 'published', 1250, 89, datetime('now', '-12 days')),
('如何优化结绳应用性能', '性能优化是每个开发者都需要关注的话题，本文将分享一些实用的优化技巧...', 1, 2, 'published', 980, 67, datetime('now', '-9 days')),
('结绳社区发展规划', '作为结绳社区的创始人，我想与大家分享一下未来的发展规划...', 1, 3, 'published', 2100, 156, datetime('now', '-6 days')),
('新手入门指南', '欢迎来到结绳语言的世界！本文将帮助新手快速入门...', 1, 1, 'published', 1800, 123, datetime('now', '-4 days')),
('社区活动预告', '下个月我们将举办结绳语言开发者大会，欢迎大家参加...', 1, 4, 'published', 750, 45, datetime('now', '-2 days'));

-- 添加一些评论（comments）
INSERT INTO comments (post_id, user_id, content, likes, created_at) VALUES 
(1, 1, '感谢大家的支持！如果有任何问题欢迎随时交流。', 25, datetime('now', '-11 days')),
(2, 1, '性能优化确实很重要，后续我会分享更多相关内容。', 18, datetime('now', '-8 days')),
(3, 1, '大家的建议都很有价值，我会认真考虑的。', 32, datetime('now', '-5 days')),
(4, 1, '希望这个指南能帮到新手朋友们！', 21, datetime('now', '-3 days')),
(5, 1, '期待与大家在大会上见面！', 15, datetime('now', '-1 days'));

-- 更新用户统计数据
UPDATE users SET 
    upload_count = 5,
    login_count = 45,
    last_login = datetime('now', '-1 hours')
WHERE username = 'admin';

-- 添加一些包的点赞记录
INSERT INTO package_likes (package_id, user_id, created_at) VALUES 
(1, 1, datetime('now', '-9 days')),
(2, 1, datetime('now', '-7 days')),
(3, 1, datetime('now', '-4 days'));

-- 添加一些帖子的点赞记录  
INSERT INTO post_likes (post_id, user_id, created_at) VALUES
(1, 1, datetime('now', '-11 days')),
(2, 1, datetime('now', '-8 days')),
(3, 1, datetime('now', '-5 days'));

-- 添加一些包的浏览记录
INSERT INTO package_views (package_id, user_id, created_at) VALUES
(1, 1, datetime('now', '-9 days')),
(2, 1, datetime('now', '-7 days')),
(3, 1, datetime('now', '-4 days')),
(4, 1, datetime('now', '-2 days')),
(5, 1, datetime('now', '-1 days'));

-- 添加一些帖子的浏览记录
INSERT INTO post_views (post_id, user_id, created_at) VALUES
(1, 1, datetime('now', '-11 days')),
(2, 1, datetime('now', '-8 days')),
(3, 1, datetime('now', '-5 days')),
(4, 1, datetime('now', '-3 days')),
(5, 1, datetime('now', '-1 days')); 