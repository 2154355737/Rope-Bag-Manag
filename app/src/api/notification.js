import { get, post } from '@/utils/request';

export const notificationApi = {
  list: (page = 1, size = 20) => get('/notifications', { page, size }),
  unreadCount: () => get('/notifications/unread-count'),
  markRead: (id) => post(`/notifications/${id}/read`, {}),
};

export default { notificationApi }; 