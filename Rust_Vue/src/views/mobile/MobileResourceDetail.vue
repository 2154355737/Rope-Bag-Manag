<template>
  <MobileLayout :title="resource.name" showBack>
    <div class="resource-detail-page">
      <!-- 资源图片轮播 -->
      <div class="resource-image-gallery">
        <el-carousel v-if="resource.images && resource.images.length > 0" height="200px" indicator-position="inside">
          <el-carousel-item v-for="(image, index) in resource.images" :key="index">
            <img :src="image.url" :alt="`${resource.name} 图片${index + 1}`" class="gallery-image">
          </el-carousel-item>
        </el-carousel>
        <img v-else :src="resource.image_url || defaultImage" :alt="resource.name" class="single-image">
      </div>

      <!-- 资源基本信息 -->
      <div class="resource-info card-block">
        <h1 class="resource-title">{{ resource.name }}</h1>
        
        <div class="resource-meta">
          <div class="rating-block">
            <el-rate v-model="resource.rating" disabled text-color="#ff9900" />
            <span class="rating-value">{{ resource.rating }}</span>
            <span class="rating-count">({{ resource.rating_count || 0 }}评价)</span>
          </div>
          <div class="download-count">
            <el-icon><Download /></el-icon> {{ resource.download_count || 0 }}
          </div>
        </div>
        
        <div class="tag-list" v-if="resource.tags && resource.tags.length > 0">
          <el-tag 
            v-for="tag in resource.tags" 
            :key="tag.id" 
            size="small" 
            class="tag-item"
            @click="searchByTag(tag.id)"
          >
            {{ tag.name }}
          </el-tag>
        </div>

        <div class="resource-actions">
          <el-button type="primary" size="large" @click="downloadResource" class="action-btn download-btn">
            <el-icon><Download /></el-icon> 下载资源
          </el-button>
          <el-button plain size="large" @click="toggleFavorite" class="action-btn">
            <el-icon><Star v-if="isFavorited" /> <StarFilled v-else /></el-icon> {{ isFavorited ? '已收藏' : '收藏' }}
          </el-button>
        </div>
      </div>

      <!-- 资源详情 -->
      <div class="card-block">
        <div class="block-title">
          <h2>资源详情</h2>
        </div>
        <div class="resource-details">
          <div class="detail-item">
            <span class="detail-label">分类</span>
            <span class="detail-value">{{ resource.category?.name || '未分类' }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">作者</span>
            <span class="detail-value">{{ resource.author || '未知' }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">上传者</span>
            <span class="detail-value clickable" @click="viewUser(resource.user_id)">
              {{ resource.username || '未知' }}
            </span>
          </div>
          <div class="detail-item">
            <span class="detail-label">发布时间</span>
            <span class="detail-value">{{ formatDate(resource.created_at) }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">更新时间</span>
            <span class="detail-value">{{ formatDate(resource.updated_at) }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">文件大小</span>
            <span class="detail-value">{{ formatSize(resource.file_size) }}</span>
          </div>
        </div>
      </div>

      <!-- 资源描述 -->
      <div class="card-block">
        <div class="block-title">
          <h2>资源描述</h2>
        </div>
        <div class="resource-description" v-html="formattedDescription"></div>
      </div>

      <!-- 评论区 -->
      <div class="card-block">
        <div class="block-title">
          <h2>评论 ({{ resource.comment_count || 0 }})</h2>
          <el-button size="small" @click="showCommentInput = true" v-if="!showCommentInput">
            <el-icon><Edit /></el-icon> 写评论
          </el-button>
        </div>

        <div class="comment-input" v-if="showCommentInput">
          <el-input
            v-model="commentContent"
            type="textarea"
            :rows="3"
            placeholder="写下你的评论..."
            maxlength="500"
            show-word-limit
          />
          <div class="comment-actions">
            <el-button size="small" @click="showCommentInput = false">取消</el-button>
            <el-button type="primary" size="small" @click="submitComment" :disabled="!commentContent.trim()">发表评论</el-button>
          </div>
        </div>

        <div class="no-comments" v-if="comments.length === 0">
          暂无评论，成为第一个评论的人吧！
        </div>

        <div class="comment-list" v-else>
          <div class="comment-item" v-for="comment in comments" :key="comment.id">
            <div class="comment-header">
              <div class="comment-user" @click="viewUser(comment.user_id)">
                <img :src="comment.avatar || defaultAvatar" class="user-avatar" alt="用户头像">
                <span class="username">{{ comment.username }}</span>
              </div>
              <div class="comment-time">{{ formatDate(comment.created_at) }}</div>
            </div>
            <div class="comment-content">{{ comment.content }}</div>
            <div class="comment-actions">
              <div class="comment-action" @click="likeComment(comment)">
                <el-icon><ThumbUp /></el-icon> {{ comment.likes || 0 }}
              </div>
              <div class="comment-action" @click="replyToComment(comment)">
                <el-icon><ChatDotRound /></el-icon> 回复
              </div>
            </div>
            
            <!-- 回复列表 -->
            <div class="reply-list" v-if="comment.replies && comment.replies.length > 0">
              <div class="reply-item" v-for="reply in comment.replies" :key="reply.id">
                <div class="reply-header">
                  <div class="reply-user" @click="viewUser(reply.user_id)">
                    <img :src="reply.avatar || defaultAvatar" class="user-avatar small" alt="用户头像">
                    <span class="username">{{ reply.username }}</span>
                  </div>
                  <div class="reply-time">{{ formatDate(reply.created_at) }}</div>
                </div>
                <div class="reply-content">{{ reply.content }}</div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="load-more" v-if="hasMoreComments" @click="loadMoreComments">
          加载更多评论...
        </div>
      </div>

      <!-- 相关推荐 -->
      <div class="card-block">
        <div class="block-title">
          <h2>相关推荐</h2>
        </div>
        <div class="related-resources">
          <div 
            v-for="item in relatedResources" 
            :key="item.id"
            class="related-item"
            @click="viewResource(item.id)"
          >
            <div class="related-image">
              <img :src="item.image_url || defaultImage" :alt="item.name">
            </div>
            <div class="related-info">
              <div class="related-name">{{ item.name }}</div>
              <div class="related-rating">
                <el-rate v-model="item.rating" disabled size="small" />
                <span class="rating-count">{{ item.download_count || 0 }}下载</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- 下载确认对话框 -->
    <el-dialog v-model="downloadDialogVisible" title="下载确认" width="80%" center>
      <div class="download-dialog-content">
        <p>您确认要下载 <strong>{{ resource.name }}</strong> 吗？</p>
        <p class="file-info">文件大小：{{ formatSize(resource.file_size) }}</p>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="downloadDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="confirmDownload">确认下载</el-button>
        </span>
      </template>
    </el-dialog>
  </MobileLayout>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { ElMessage, ElMessageBox } from 'element-plus';
import MobileLayout from '@/layouts/mobile/MobileLayout.vue';
import { Download, Edit, Star, StarFilled, ThumbUp, ChatDotRound } from '@element-plus/icons-vue';
import { useApiState } from '@/composables/useApiState';
// 实际项目中引入相关API
// import { getResourceDetail, getResourceComments, downloadResource, favoriteResource, likeComment } from '@/api/resources';

// 路由和状态
const route = useRoute();
const router = useRouter();
const { loading, setLoading } = useApiState();

// 默认图片
const defaultImage = '/assets/default-resource.jpg';
const defaultAvatar = '/assets/default-avatar.jpg';

// 资源数据
const resource = ref({
  id: 0,
  name: '',
  description: '',
  image_url: '',
  images: [],
  rating: 0,
  rating_count: 0,
  download_count: 0,
  tags: [],
  category: null,
  author: '',
  user_id: 0,
  username: '',
  created_at: '',
  updated_at: '',
  file_size: 0,
  comment_count: 0
});

// 评论相关
const comments = ref([]);
const commentContent = ref('');
const showCommentInput = ref(false);
const currentPage = ref(1);
const hasMoreComments = ref(true);

// 收藏状态
const isFavorited = ref(false);

// 相关推荐
const relatedResources = ref([]);

// 下载确认
const downloadDialogVisible = ref(false);

// 格式化后的描述
const formattedDescription = computed(() => {
  return resource.value.description?.replace(/\n/g, '<br>') || '暂无描述';
});

// 初始化加载数据
onMounted(async () => {
  const resourceId = parseInt(route.params.id);
  if (!resourceId) {
    ElMessage.error('资源ID无效');
    router.push('/mobile/home');
    return;
  }
  
  await fetchResourceDetail(resourceId);
  await fetchComments(resourceId);
  await fetchRelatedResources(resourceId);
});

// 获取资源详情
const fetchResourceDetail = async (id) => {
  setLoading(true);
  try {
    // 实际项目中使用API
    // const response = await getResourceDetail(id);
    // resource.value = response.data;
    // isFavorited.value = response.data.is_favorited;
    
    // 模拟数据
    resource.value = {
      id: id,
      name: '示例资源',
      description: '这是一个详细的资源描述，包含了资源的特点、使用方法等信息。\n\n支持多段落显示。',
      image_url: '',
      images: [
        { url: '' },
        { url: '' }
      ],
      rating: 4.5,
      rating_count: 120,
      download_count: 3500,
      tags: [
        { id: 1, name: '标签1' },
        { id: 2, name: '标签2' },
        { id: 3, name: '标签3' }
      ],
      category: { id: 1, name: '设计资源' },
      author: '资源作者',
      user_id: 100,
      username: '上传用户',
      created_at: '2023-07-01T12:00:00Z',
      updated_at: '2023-07-15T16:30:00Z',
      file_size: 15728640, // 15MB
      comment_count: 25
    };
    isFavorited.value = false;
  } catch (error) {
    console.error('获取资源详情失败:', error);
    ElMessage.error('获取资源详情失败');
  } finally {
    setLoading(false);
  }
};

// 获取评论
const fetchComments = async (resourceId, page = 1) => {
  try {
    // 实际项目中使用API
    // const response = await getResourceComments(resourceId, { page, page_size: 10 });
    // if (page === 1) {
    //   comments.value = response.data.list;
    // } else {
    //   comments.value = [...comments.value, ...response.data.list];
    // }
    // hasMoreComments.value = response.data.total > page * 10;
    
    // 模拟数据
    const mockComments = [
      {
        id: 1,
        user_id: 101,
        username: '评论用户1',
        avatar: '',
        content: '这个资源非常棒，感谢分享！',
        created_at: '2023-07-20T10:15:00Z',
        likes: 12,
        replies: [
          {
            id: 101,
            user_id: 102,
            username: '回复用户1',
            avatar: '',
            content: '我也觉得很好用',
            created_at: '2023-07-20T11:30:00Z'
          }
        ]
      },
      {
        id: 2,
        user_id: 103,
        username: '评论用户2',
        avatar: '',
        content: '请问这个资源可以商用吗？',
        created_at: '2023-07-19T09:45:00Z',
        likes: 3,
        replies: []
      }
    ];
    
    if (page === 1) {
      comments.value = mockComments;
    } else {
      comments.value = [...comments.value, ...mockComments];
    }
    hasMoreComments.value = page < 3; // 模拟有3页数据
    currentPage.value = page;
  } catch (error) {
    console.error('获取评论失败:', error);
    ElMessage.error('获取评论失败');
  }
};

// 获取相关推荐
const fetchRelatedResources = async (resourceId) => {
  try {
    // 实际项目中使用API
    // const response = await getRelatedResources(resourceId);
    // relatedResources.value = response.data.list;
    
    // 模拟数据
    relatedResources.value = [
      { id: 101, name: '相关资源1', image_url: '', rating: 4.2, download_count: 2300 },
      { id: 102, name: '相关资源2', image_url: '', rating: 4.7, download_count: 1800 },
      { id: 103, name: '相关资源3', image_url: '', rating: 3.9, download_count: 950 }
    ];
  } catch (error) {
    console.error('获取相关推荐失败:', error);
    // 不显示错误提示，因为这不是核心功能
  }
};

// 加载更多评论
const loadMoreComments = async () => {
  const nextPage = currentPage.value + 1;
  await fetchComments(resource.value.id, nextPage);
};

// 提交评论
const submitComment = async () => {
  if (!commentContent.value.trim()) return;
  
  try {
    // 实际项目中使用API
    // await addComment(resource.value.id, commentContent.value);
    // 刷新评论列表
    // await fetchComments(resource.value.id, 1);
    
    // 模拟添加评论
    const newComment = {
      id: Date.now(),
      user_id: 999, // 假设当前用户ID
      username: '当前用户',
      avatar: '',
      content: commentContent.value,
      created_at: new Date().toISOString(),
      likes: 0,
      replies: []
    };
    
    comments.value.unshift(newComment);
    resource.value.comment_count = (resource.value.comment_count || 0) + 1;
    
    commentContent.value = '';
    showCommentInput.value = false;
    ElMessage.success('评论发表成功');
  } catch (error) {
    console.error('发表评论失败:', error);
    ElMessage.error('发表评论失败');
  }
};

// 回复评论
const replyToComment = (comment) => {
  ElMessageBox.prompt(`回复 ${comment.username}:`, '回复评论', {
    confirmButtonText: '提交',
    cancelButtonText: '取消',
    inputPlaceholder: '请输入回复内容'
  }).then(({ value }) => {
    if (value.trim()) {
      // 实际项目中使用API
      // await replyComment(resource.value.id, comment.id, value);
      // 刷新评论
      // await fetchComments(resource.value.id, currentPage.value);
      
      // 模拟添加回复
      if (!comment.replies) comment.replies = [];
      const newReply = {
        id: Date.now(),
        user_id: 999, // 假设当前用户ID
        username: '当前用户',
        avatar: '',
        content: value,
        created_at: new Date().toISOString()
      };
      comment.replies.push(newReply);
      
      ElMessage.success('回复成功');
    }
  }).catch(() => {});
};

// 点赞评论
const likeComment = async (comment) => {
  try {
    // 实际项目中使用API
    // await likeComment(comment.id);
    
    // 模拟点赞
    comment.likes = (comment.likes || 0) + 1;
    ElMessage.success('点赞成功');
  } catch (error) {
    console.error('点赞失败:', error);
    ElMessage.error('点赞失败');
  }
};

// 下载资源
const downloadResource = () => {
  downloadDialogVisible.value = true;
};

// 确认下载
const confirmDownload = async () => {
  try {
    // 实际项目中使用API
    // const response = await downloadResource(resource.value.id);
    // 处理下载链接
    // window.location.href = response.data.download_url;
    
    downloadDialogVisible.value = false;
    ElMessage.success('开始下载资源');
    
    // 更新下载计数
    resource.value.download_count = (resource.value.download_count || 0) + 1;
  } catch (error) {
    console.error('下载资源失败:', error);
    ElMessage.error('下载资源失败');
  }
};

// 收藏/取消收藏
const toggleFavorite = async () => {
  try {
    // 实际项目中使用API
    // await favoriteResource(resource.value.id, !isFavorited.value);
    
    isFavorited.value = !isFavorited.value;
    ElMessage.success(isFavorited.value ? '收藏成功' : '已取消收藏');
  } catch (error) {
    console.error('操作失败:', error);
    ElMessage.error('操作失败');
  }
};

// 查看标签相关资源
const searchByTag = (tagId) => {
  router.push(`/mobile/resources?tag=${tagId}`);
};

// 查看用户
const viewUser = (userId) => {
  router.push(`/mobile/user/${userId}`);
};

// 查看资源
const viewResource = (id) => {
  router.push(`/mobile/resource/${id}`);
};

// 格式化日期
const formatDate = (dateString) => {
  if (!dateString) return '未知时间';
  const date = new Date(dateString);
  return date.toLocaleDateString('zh-CN', { 
    year: 'numeric', 
    month: '2-digit', 
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  });
};

// 格式化文件大小
const formatSize = (bytes) => {
  if (!bytes || bytes === 0) return '未知大小';
  
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = bytes;
  let unitIndex = 0;
  
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024;
    unitIndex++;
  }
  
  return `${size.toFixed(2)} ${units[unitIndex]}`;
};
</script>

<style scoped>
.resource-detail-page {
  padding-bottom: 20px;
}

.resource-image-gallery {
  margin: -15px -15px 15px -15px;
  background-color: #f5f5f5;
}

.gallery-image, .single-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
  background-color: #f5f5f5;
}

.single-image {
  height: 200px;
}

.card-block {
  background-color: #fff;
  border-radius: 10px;
  padding: 15px;
  margin-bottom: 15px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.resource-info {
  margin-top: -5px;
}

.resource-title {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 10px;
}

.resource-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.rating-block {
  display: flex;
  align-items: center;
}

.rating-value {
  margin: 0 5px;
  font-weight: 600;
}

.rating-count {
  color: var(--el-text-color-secondary);
  font-size: 0.9em;
}

.download-count {
  display: flex;
  align-items: center;
  gap: 5px;
  color: var(--el-text-color-secondary);
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 15px;
}

.resource-actions {
  display: flex;
  gap: 10px;
}

.action-btn {
  flex: 1;
}

.download-btn {
  flex: 2;
}

.block-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.block-title h2 {
  font-size: 16px;
  font-weight: 600;
  margin: 0;
}

.resource-details {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.detail-item {
  display: flex;
  align-items: center;
  line-height: 1.5;
}

.detail-label {
  width: 80px;
  color: var(--el-text-color-secondary);
}

.detail-value {
  flex: 1;
}

.clickable {
  color: var(--el-color-primary);
  cursor: pointer;
}

.resource-description {
  line-height: 1.6;
  white-space: pre-wrap;
}

.comment-input {
  margin-bottom: 15px;
}

.comment-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 10px;
  gap: 10px;
}

.no-comments {
  text-align: center;
  padding: 20px 0;
  color: var(--el-text-color-secondary);
}

.comment-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.comment-item {
  border-bottom: 1px solid var(--el-border-color-lighter);
  padding-bottom: 15px;
}

.comment-item:last-child {
  border-bottom: none;
}

.comment-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.comment-user {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.user-avatar {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  object-fit: cover;
}

.user-avatar.small {
  width: 24px;
  height: 24px;
}

.username {
  font-weight: 500;
}

.comment-time {
  font-size: 0.8em;
  color: var(--el-text-color-secondary);
}

.comment-content {
  line-height: 1.5;
  margin-bottom: 8px;
}

.comment-actions {
  display: flex;
  gap: 15px;
  color: var(--el-text-color-regular);
}

.comment-action {
  display: flex;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  font-size: 0.9em;
}

.reply-list {
  margin-top: 10px;
  margin-left: 30px;
  padding-left: 10px;
  border-left: 2px solid var(--el-border-color-lighter);
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.reply-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 5px;
}

.reply-time {
  font-size: 0.8em;
  color: var(--el-text-color-secondary);
}

.reply-content {
  line-height: 1.5;
  font-size: 0.95em;
}

.load-more {
  text-align: center;
  color: var(--el-color-primary);
  padding: 10px 0;
  cursor: pointer;
}

.related-resources {
  display: flex;
  overflow-x: auto;
  gap: 15px;
  padding-bottom: 5px;
  scroll-snap-type: x mandatory;
  scrollbar-width: none; /* Firefox */
}

.related-resources::-webkit-scrollbar {
  display: none; /* Chrome, Safari, Opera */
}

.related-item {
  min-width: 120px;
  width: 120px;
  scroll-snap-align: start;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  background-color: #fff;
}

.related-image {
  width: 100%;
  height: 120px;
  overflow: hidden;
}

.related-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.related-info {
  padding: 8px;
}

.related-name {
  font-size: 0.9em;
  font-weight: 500;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.related-rating {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 0.8em;
}

.file-info {
  color: var(--el-text-color-secondary);
}

@media (prefers-color-scheme: dark) {
  .card-block {
    background-color: var(--el-bg-color);
  }
  
  .resource-image-gallery {
    background-color: var(--el-bg-color-page);
  }
  
  .related-item {
    background-color: var(--el-bg-color);
  }
}
</style> 