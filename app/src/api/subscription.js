import { get, post } from '@/utils/request';

export const subscriptionApi = {
  // 获取当前用户的分类订阅 map: { [categoryId]: true|false }
  getUserSubscriptions: () => get('/subscriptions'),
  // 设置订阅
  setSubscription: (categoryId, enabled) => post('/subscriptions/set', { category_id: categoryId, enabled }),
};

export default { subscriptionApi }; 