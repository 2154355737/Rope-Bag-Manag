import { get, post, put, del } from '../utils/request';

export const postApi = {
  // 获取帖子列表
  getPosts: (params = {}) => {
    const query = {
      page: params.page || 1,
      page_size: params.page_size || params.pageSize || 10,
      ...params
    };
    return get('/posts', query);
  },

  // 获取帖子详情
  getPostDetail: (id) => get(`/posts/${id}`),

  // 获取帖子标签
  getPostTags: (id) => get(`/posts/${id}/tags`),

  // 获取精选帖子
  getFeaturedPosts: (limit = 10) => get('/posts/featured', { limit }),

  // 获取热门帖子
  getPopularPosts: (limit = 10) => get('/posts/popular', { limit }),

  // 浏览量上报
  incrementView: (id) => post(`/posts/${id}/view`, {}),

  // 创建帖子（后端会自动设置 review_status='pending'）
  createPost: (data) => post('/posts', data),

  // 更新帖子
  updatePost: (id, data) => put(`/posts/${id}`, data),

  // 删除帖子
  deletePost: (id) => del(`/posts/${id}`),

  // 获取帖子评论（改用公开接口）
  getPostComments: (postId, page = 1, pageSize = 10) => get('/public/comments', { target_type: 'Post', target_id: postId, page, size: pageSize }),

  // 发表评论到帖子
  addPostComment: (postId, content, parentId = null) => post('/comments', { target_type: 'Post', target_id: postId, content, parent_id: parentId }),

  // 置顶/取消置顶评论（管理员/长老）
  pinComment: (commentId, pinned) => put(`/comments/${commentId}/pin`, { pinned }),

  // 审核帖子（管理员/元老）
  reviewPost: (id, status, comment = '') => post(`/posts/${id}/review`, { status, comment }),

  // 点赞/取消点赞帖子
  likePost: (id) => post(`/posts/${id}/like`, {}),
  unlikePost: (id) => del(`/posts/${id}/like`),
  
  // 检查帖子点赞状态
  checkLikeStatus: (id) => get(`/posts/${id}/like-status`),

  // 公告：获取有效公告列表（公开）
  getActiveAnnouncements: () => get('/announcements/active'),
};

export default {
  postApi,
}; 