import axios from 'axios';
import { get, post, put, del } from './request';

// 统一响应格式
export class ApiResponse {
  constructor(code, message, data) {
    this.code = code;
    this.message = message;
    this.data = data;
  }
}

// API客户端
export const api = {
  get: async (url, params = {}) => {
    try {
      const response = await get(url, params);
      return response;
    } catch (error) {
      console.error(`GET ${url} failed:`, error);
      throw error;
    }
  },
  
  post: async (url, data = {}) => {
    try {
      const response = await post(url, data);
      return response;
    } catch (error) {
      console.error(`POST ${url} failed:`, error);
      throw error;
    }
  },
  
  put: async (url, data = {}) => {
    try {
      const response = await put(url, data);
      return response;
    } catch (error) {
      console.error(`PUT ${url} failed:`, error);
      throw error;
    }
  },
  
  delete: async (url, params = {}) => {
    try {
      const response = await del(url, params);
      return response;
    } catch (error) {
      console.error(`DELETE ${url} failed:`, error);
      throw error;
    }
  },
  
  // 上传文件
  upload: async (url, formData) => {
    try {
      const response = await post(url, formData);
      return response;
    } catch (error) {
      console.error(`UPLOAD ${url} failed:`, error);
      throw error;
    }
  }
};

// 对接具体的后端API
export const backendApi = {
  // 健康检查
  healthCheck: () => api.get('/health'),
  
  // 用户相关
  getUserProfile: () => api.get('/users/profile'),
  
  // 资源相关
  getResources: (params) => api.get('/packages', params),
  getResourceDetail: (id) => api.get(`/packages/${id}`),
  
  // 分类相关
  getCategories: () => api.get('/categories'),
  
  // 标签相关
  getTags: (params) => api.get('/tags', params),
  getPopularTags: (limit = 20) => api.get('/tags/popular', { limit }),
  
  // 评论相关
  getComments: (resourceId) => api.get(`/packages/${resourceId}/comments`),
  addComment: (data) => api.post('/comments', data),
  
  // 统计相关
  getStats: () => api.get('/admin/stats'),
};

export default api; 