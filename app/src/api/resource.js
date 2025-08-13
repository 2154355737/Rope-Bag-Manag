import { get, post, put, del } from '../utils/request';

// 资源包接口
export const resourceApi = {
  // 获取资源列表
  getResources: (params) => {
    const queryParams = {
      page: params?.page || 1,
      page_size: params?.pageSize || 10,
      ...params
    };
    return get('/packages', queryParams);
  },
  
  // 获取资源详情
  getResourceDetail: (id) => {
    return get(`/packages/${id}`);
  },
  
  // 获取热门资源
  getHotResources: (limit = 5) => {
    return get('/packages', { page_size: limit, sort_by: 'download_count', sort_order: 'desc' });
  },
  
  // 获取最新资源
  getLatestResources: (limit = 5) => {
    return get('/packages', { page_size: limit, sort_by: 'created_at', sort_order: 'desc' });
  },
  
  // 搜索资源
  searchResources: (keyword, page = 1, pageSize = 10) => {
    return get('/packages', { search: keyword, page, page_size: pageSize });
  },
  
  // 按分类获取资源
  getResourcesByCategory: (categoryId, page = 1, pageSize = 10) => {
    return get('/packages', { category_id: categoryId, page, page_size: pageSize });
  },
  
  // 获取我的资源
  getUserResources: (paramsOrPage = 1, pageSize) => {
    // 支持两种调用方式：
    // 1) 对象参数：{ page, pageSize | page_size }
    // 2) 位置参数：(page, pageSize)
    let page;
    let finalPageSize;

    if (typeof paramsOrPage === 'object' && paramsOrPage !== null) {
      const { page: p = 1, pageSize: camelSize, page_size: snakeSize } = paramsOrPage;
      page = p;
      finalPageSize = camelSize ?? snakeSize ?? 10;
    } else {
      page = typeof paramsOrPage === 'number' ? paramsOrPage : 1;
      finalPageSize = typeof pageSize === 'number' ? pageSize : 10;
    }

    return get('/users/my-resources', { page, pageSize: finalPageSize });
  },
  
  // 创建资源
  createResource: (data) => {
    return post('/packages/user-submit', data);
  },
  
  // 更新资源
  updateResource: (id, data) => {
    return put(`/packages/${id}`, data);
  },
  
  // 删除资源
  deleteResource: (id) => {
    return del(`/packages/${id}`);
  },
  
  // 下载资源
  downloadResource: (id) => {
    return get(`/packages/${id}/download`);
  },
  
  // 上传资源文件
  uploadResourceFile: (file) => {
    const formData = new FormData();
    formData.append('file', file);
    return post('/upload', formData);
  },
  
  // 为指定资源上传文件
  uploadFile: (resourceId, formData) => {
    return post(`/packages/${resourceId}/upload`, formData);
  },
  
  // 获取资源评论
  getResourceComments: (resourceId, params = {}) => {
    const queryParams = {
      page: params.page || 1,
      page_size: params.pageSize || 10
    };
    return get(`/packages/${resourceId}/comments`, queryParams);
  },
  
  // 添加评论
  addComment: (resourceId, content) => {
    return post(`/comments`, {
      content,
      target_id: resourceId,
      target_type: 'Package'
    });
  },
  
  // 点赞/取消点赞资源
  likeResource: (id) => {
    return post(`/packages/${id}/like`, {});
  },
  
  unlikeResource: (id) => {
    return del(`/packages/${id}/like`);
  },
  
  // 检查资源点赞状态
  checkLikeStatus: (id) => {
    return get(`/packages/${id}/like-status`);
  },
  

};

// 分类接口
export const categoryApi = {
  // 获取所有分类
  getAllCategories: () => {
    return get('/categories');
  },
  
  // 获取分类详情
  getCategoryDetail: (id) => {
    return get(`/categories/${id}`);
  }
};

// 标签接口
export const tagApi = {
  // 获取所有标签
  getAllTags: (params) => {
    return get('/tags', params);
  },
  
  // 获取热门标签
  getHotTags: (limit = 20) => {
    return get('/tags/popular', { limit });
  }
};

// 添加banner相关API
export const bannerApi = {
  // 获取活动轮播图
  getBanners: () => {
    return get('/public/banners');
  }
};

export default {
  resourceApi,
  categoryApi,
  tagApi,
  bannerApi
}; 