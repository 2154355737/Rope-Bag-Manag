/**
 * 页面预加载服务
 * 确保启动页结束时目标页面和数据都已经准备好
 */

class PreloaderService {
  constructor() {
    this.cache = new Map();
    this.loadingPromises = new Map();
  }

  /**
   * 预加载页面组件和数据
   * @param {string} path - 目标路径
   * @param {Object} router - Vue Router实例
   * @param {Object} userStore - 用户状态store
   */
  async preloadPage(path, router, userStore) {
    // 避免重复加载
    if (this.loadingPromises.has(path)) {
      return this.loadingPromises.get(path);
    }

    const loadingPromise = this._performPreload(path, router, userStore);
    this.loadingPromises.set(path, loadingPromise);

    try {
      const result = await loadingPromise;
      this.cache.set(path, result);
      return result;
    } catch (error) {
      console.warn('预加载失败:', path, error);
      return null;
    } finally {
      this.loadingPromises.delete(path);
    }
  }

  /**
   * 执行实际的预加载逻辑
   */
  async _performPreload(path, router, userStore) {
    const tasks = [];

    // 1. 预加载路由组件
    tasks.push(this._preloadRouteComponent(path, router));

    // 2. 根据路径预加载对应的数据
    if (path === '/' || path === '/home') {
      tasks.push(this._preloadHomeData());
    } else if (path.startsWith('/category')) {
      tasks.push(this._preloadCategoryData(path));
    } else if (path.startsWith('/resource/')) {
      const resourceId = path.split('/')[2];
      tasks.push(this._preloadResourceData(resourceId));
    } else if (path.startsWith('/post/')) {
      const postId = path.split('/')[2];
      tasks.push(this._preloadPostData(postId));
    } else if (path === '/community') {
      tasks.push(this._preloadCommunityData());
    }

    // 3. 确保用户状态已加载
    if (!userStore.isChecked) {
      tasks.push(userStore.checkAuth().catch(() => {}));
    }

    // 并行执行所有预加载任务
    const results = await Promise.allSettled(tasks);
    
    return {
      path,
      success: results.filter(r => r.status === 'fulfilled').length,
      failed: results.filter(r => r.status === 'rejected').length,
      timestamp: Date.now()
    };
  }

  /**
   * 预加载路由组件
   */
  async _preloadRouteComponent(path, router) {
    try {
      const route = router.resolve(path);
      if (route.matched.length > 0) {
        const routeRecord = route.matched[route.matched.length - 1];
        if (routeRecord.components?.default) {
          if (typeof routeRecord.components.default === 'function') {
            // 懒加载组件
            await routeRecord.components.default();
          }
        }
      }
    } catch (error) {
      console.warn('预加载路由组件失败:', path, error);
    }
  }

  /**
   * 预加载首页数据
   */
  async _preloadHomeData() {
    try {
      const { get } = await import('../utils/request');
      
      // 并行加载首页所需的数据
      const homeDataTasks = [
        get('/packages', { page: 1, page_size: 10, sort_by: 'download_count' }).catch(() => null),
        get('/posts', { page: 1, page_size: 5 }).catch(() => null),
        get('/categories').catch(() => null),
        get('/public/banners').catch(() => null)
      ];

      await Promise.allSettled(homeDataTasks);
    } catch (error) {
      console.warn('预加载首页数据失败:', error);
    }
  }

  /**
   * 预加载分类数据
   */
  async _preloadCategoryData(path) {
    try {
      const { get } = await import('../utils/request');
      const categoryId = path.split('/')[2];
      
      if (categoryId && categoryId !== 'undefined') {
        await Promise.allSettled([
          get(`/categories/${categoryId}`).catch(() => null),
          get('/packages', { category_id: categoryId, page: 1, page_size: 10 }).catch(() => null)
        ]);
      } else {
        await get('/categories').catch(() => null);
      }
    } catch (error) {
      console.warn('预加载分类数据失败:', error);
    }
  }

  /**
   * 预加载资源详情数据
   */
  async _preloadResourceData(resourceId) {
    try {
      const { get } = await import('../utils/request');
      
      if (resourceId && resourceId !== 'undefined') {
        await Promise.allSettled([
          get(`/packages/${resourceId}`).catch(() => null),
          get(`/packages/${resourceId}/comments`, { page: 1, page_size: 10 }).catch(() => null)
        ]);
      }
    } catch (error) {
      console.warn('预加载资源数据失败:', error);
    }
  }

  /**
   * 预加载帖子数据
   */
  async _preloadPostData(postId) {
    try {
      const { postApi } = await import('../api/post');
      
      if (postId && postId !== 'undefined') {
        await Promise.allSettled([
          postApi.getPostDetail(postId).catch(() => null),
          postApi.getPostComments(postId, 1, 10).catch(() => null)
        ]);
      }
    } catch (error) {
      console.warn('预加载帖子数据失败:', error);
    }
  }

  /**
   * 预加载社区数据
   */
  async _preloadCommunityData() {
    try {
      const { postApi } = await import('../api/post');
      
      await Promise.allSettled([
        postApi.getPosts({ page: 1, page_size: 10 }).catch(() => null),
        postApi.getFeaturedPosts(5).catch(() => null)
      ]);
    } catch (error) {
      console.warn('预加载社区数据失败:', error);
    }
  }

  /**
   * 清理缓存
   */
  clearCache() {
    this.cache.clear();
    this.loadingPromises.clear();
  }

  /**
   * 获取预加载统计信息
   */
  getStats() {
    return {
      cacheSize: this.cache.size,
      loadingCount: this.loadingPromises.size,
      cachedPaths: Array.from(this.cache.keys())
    };
  }
}

// 创建全局实例
const preloader = new PreloaderService();

export default preloader; 