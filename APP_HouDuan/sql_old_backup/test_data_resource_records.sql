-- 插入测试资源记录数据
INSERT INTO resource_records (resource_id, resource_type, user_id, action, ip_address, old_data, new_data, timestamp, created_at)
VALUES 
-- 包操作记录
(1, 'Package', 1, 'Create', '127.0.0.1', NULL, '{"name":"测试绳包1","type":"教程"}', strftime('%s','now'), datetime('now')),
(2, 'Package', 1, 'Update', '127.0.0.1', '{"name":"测试绳包2"}', '{"name":"更新的绳包2","description":"这是一个更新的描述"}', strftime('%s','now'), datetime('now')),
(3, 'Package', 1, 'Download', '127.0.0.1', NULL, NULL, strftime('%s','now'), datetime('now')),
(1, 'Package', 2, 'Download', '127.0.0.1', NULL, NULL, strftime('%s','now'), datetime('now')),
-- 用户操作记录
(2, 'User', 1, 'Create', '127.0.0.1', NULL, '{"username":"testuser","email":"test@example.com"}', strftime('%s','now'), datetime('now')),
(2, 'User', 1, 'Update', '127.0.0.1', '{"status":"inactive"}', '{"status":"active"}', strftime('%s','now'), datetime('now')),
-- 评论操作记录
(1, 'Comment', 2, 'Create', '127.0.0.1', NULL, '{"content":"这是一条测试评论"}', strftime('%s','now'), datetime('now')),
(1, 'Comment', 2, 'Update', '127.0.0.1', '{"content":"这是一条测试评论"}', '{"content":"这是一条更新后的评论"}', strftime('%s','now'), datetime('now')),
-- 分类操作记录
(1, 'Category', 1, 'Create', '127.0.0.1', NULL, '{"name":"新分类"}', strftime('%s','now'), datetime('now')),
(1, 'Category', 1, 'Delete', '127.0.0.1', '{"name":"新分类"}', NULL, strftime('%s','now'), datetime('now'));

-- 插入一些历史记录，用于测试图表展示
INSERT INTO resource_records (resource_id, resource_type, user_id, action, ip_address, timestamp, created_at)
VALUES 
(1, 'Package', 1, 'Create', '127.0.0.1', strftime('%s', date('now', '-7 days')), datetime('now', '-7 days')),
(2, 'Package', 1, 'Create', '127.0.0.1', strftime('%s', date('now', '-6 days')), datetime('now', '-6 days')),
(3, 'Package', 1, 'Update', '127.0.0.1', strftime('%s', date('now', '-5 days')), datetime('now', '-5 days')),
(4, 'Package', 1, 'Download', '127.0.0.1', strftime('%s', date('now', '-4 days')), datetime('now', '-4 days')),
(5, 'Package', 1, 'Download', '127.0.0.1', strftime('%s', date('now', '-3 days')), datetime('now', '-3 days')),
(6, 'Package', 1, 'Delete', '127.0.0.1', strftime('%s', date('now', '-2 days')), datetime('now', '-2 days')),
(7, 'Package', 1, 'Create', '127.0.0.1', strftime('%s', date('now', '-1 days')), datetime('now', '-1 days')); 