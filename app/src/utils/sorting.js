/**
 * 资源排序工具函数
 * 统一的排序逻辑：置顶 > 精华 > 其他条件
 */

/**
 * 对资源列表进行排序
 * @param {Array} resources - 资源列表
 * @param {Object} options - 排序选项
 * @param {string} options.sortBy - 排序字段 ('created_at', 'download_count', 'like_count')
 * @param {string} options.sortOrder - 排序顺序 ('desc', 'asc')
 * @returns {Array} 排序后的资源列表
 */
export function sortResources(resources, options = {}) {
  const { sortBy = 'created_at', sortOrder = 'desc' } = options;
  
  return [...resources].sort((a, b) => {
    // 1. 置顶资源优先
    if (a.is_pinned && !b.is_pinned) return -1;
    if (!a.is_pinned && b.is_pinned) return 1;
    
    // 2. 精华资源次之
    if (a.is_featured && !b.is_featured) return -1;
    if (!a.is_featured && b.is_featured) return 1;
    
    // 3. 在相同优先级内按指定条件排序
    let result = 0;
    
    switch (sortBy) {
      case 'created_at':
        result = new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
        break;
      case 'download_count':
        result = (b.download_count || 0) - (a.download_count || 0);
        break;
      case 'like_count':
        result = (b.like_count || 0) - (a.like_count || 0);
        break;
      case 'updated_at':
        result = new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime();
        break;
      default:
        result = 0;
    }
    
    // 如果是升序，反转结果
    return sortOrder === 'asc' ? -result : result;
  });
}

/**
 * 对资源列表进行默认排序（置顶 > 精华 > 创建时间）
 * @param {Array} resources - 资源列表
 * @returns {Array} 排序后的资源列表
 */
export function sortResourcesDefault(resources) {
  return sortResources(resources, { sortBy: 'created_at', sortOrder: 'desc' });
} 