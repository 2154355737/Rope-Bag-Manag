import { get, post } from '@/utils/request';

export const notificationApi = {
  list: (page = 1, size = 20) => get('/notifications', { page, size }),
  unreadCount: () => get('/notifications/unread-count'),
  markRead: (id) => post(`/notifications/${id}/read`, {}),
  // 新增：一键已读
  markAllRead: () => post('/notifications/mark-all-read', {}),
};

export default { notificationApi }; 