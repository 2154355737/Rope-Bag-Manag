<template>
  <div class="resource-detail">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      :title="resource?.name || '资源详情'"
      left-arrow
      @click-left="onBack"
      fixed
    />
    
    <div class="detail-content" :style="{ paddingTop: '46px' }">
      <!-- 加载骨架屏 -->
      <van-skeleton title :row="12" :loading="loading" v-if="loading" />
      
      <template v-else-if="resource">
        <!-- 资源封面 -->
        <div class="resource-cover" v-if="resource.cover">
          <van-image :src="resource.cover" fit="cover" />
        </div>
        
        <!-- 资源信息 -->
        <div class="detail-card">
          <h1 class="resource-title">{{ resource.name }}</h1>
          <div class="resource-meta">
            <span class="author">作者：{{ resource.author }}</span>
            <span class="version" v-if="resource.version">版本：{{ resource.version }}</span>
            <span class="date">发布时间：{{ formatDate(resource.created_at) }}</span>
          </div>
          
          <div class="resource-tags" v-if="resource.tags && resource.tags.length">
            <van-tag
              v-for="tag in resource.tags"
              :key="tag"
              plain
              round
              size="medium"
              type="primary"
              class="mr-2"
            >
              {{ tag }}
            </van-tag>
          </div>
          
          <div class="resource-stats">
            <div class="stat-item">
              <van-icon name="down" />
              <span>{{ resource.download_count || 0 }}</span>
            </div>
            <div class="stat-item like-click" @click="toggleLike">
              <van-icon :name="liked ? 'like' : 'like-o'" :color="liked ? '#ee0a24' : undefined" />
              <span>{{ likeCount }}</span>
            </div>
            <div class="stat-item">
              <van-icon name="comment-o" />
              <span>{{ resource.comment_count || 0 }}</span>
            </div>
          </div>
          
          <div class="resource-actions actions-row">
            <van-button type="primary" block round @click="downloadResource" :loading="downloading">
              立即下载
            </van-button>
            <van-button class="ml8" :type="liked ? 'danger' : 'default'" plain round icon="like-o" @click="toggleLike">
              {{ liked ? '已赞' : '点赞' }}（{{ likeCount }}）
            </van-button>
          </div>
        </div>
        
        <!-- 资源描述 -->
        <div class="detail-card">
          <div class="card-title">资源介绍</div>
          <div class="resource-description" v-if="resource.description">
            {{ resource.description }}
          </div>
          <div class="resource-description" v-else>
            该资源暂无详细介绍...
          </div>
        </div>
        
        <!-- 资源评论 -->
        <div class="detail-card">
          <div class="card-title">
            评论 ({{ comments.length }})
            <div class="comment-action" @click="showCommentInput = true">
              <van-icon name="edit" />
              <span>写评论</span>
            </div>
          </div>
          
          <div class="comment-list" v-if="comments.length > 0">
            <div class="comment-item" v-for="comment in comments" :key="comment.id">
              <div class="comment-header">
                <div class="comment-author">
                  <img :src="comment.author_avatar || '/img/default-avatar.jpg'" alt="头像" class="author-avatar" />
                  <div>
                    <div class="author-name">{{ comment.author_name }}</div>
                    <div class="comment-time">{{ formatDate(comment.created_at) }}</div>
                  </div>
                </div>
              </div>
              <div class="comment-content">
                {{ comment.content }}
              </div>
              <div class="comment-actions">
                <div class="comment-action" @click="likeComment(comment)">
                  <van-icon name="like-o" :class="{ 'liked': comment.isLiked }" />
                  <span>{{ comment.likes || 0 }}</span>
                </div>
                <div class="comment-action" @click="replyComment(comment)">
                  <van-icon name="comment-o" />
                  <span>回复</span>
                </div>
              </div>
            </div>
          </div>
          
          <div class="no-comments" v-else>
            暂无评论，快来发表第一条评论吧！
          </div>
        </div>
        
        <!-- 相关推荐 -->
        <div class="detail-card">
          <div class="card-title">相关推荐</div>
          <resource-list 
            :resources="relatedResources" 
            :loading="relatedLoading"
            :finished="true"
            :showCover="true"
            :emptyText="'暂无相关推荐'"
          />
        </div>
      </template>
      
      <template v-else>
        <div class="resource-not-found">
          <van-empty image="error" description="资源不存在或已被删除" />
        </div>
      </template>
    </div>
    
    <!-- 评论输入框 -->
    <van-action-sheet v-model:show="showCommentInput" title="发表评论">
      <div class="comment-form">
        <van-field
          v-model="commentContent"
          rows="3"
          autosize
          type="textarea"
          placeholder="请输入评论内容"
          maxlength="500"
          show-word-limit
        />
        <div class="comment-form-actions">
          <van-button plain @click="showCommentInput = false">取消</van-button>
          <van-button type="primary" :disabled="!commentContent.trim()" @click="submitComment">发表</van-button>
        </div>
      </div>
    </van-action-sheet>
    
    <!-- 底部Tab栏 -->
    <tab-bar />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { showToast, showDialog } from 'vant';
import { resourceApi } from '../api/resource';
import { useUserStore } from '../store/user';
import ResourceList from '../components/ResourceList.vue';
import TabBar from '../components/TabBar.vue';
import dayjs from 'dayjs';
import { get, post, del } from '../utils/request';

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

// 获取路由参数中的资源ID
const resourceId = ref(Number(route.params.id) || 0);

// 资源详情
const resource = ref(null);
const loading = ref(false);

// 资源评论
const comments = ref([]);
const commentLoading = ref(false);

// 相关推荐
const relatedResources = ref([]);
const relatedLoading = ref(false);

// 评论相关
const showCommentInput = ref(false);
const commentContent = ref('');
const replyTo = ref(null);

// 下载状态
const downloading = ref(false);

// 点赞状态
const liked = ref(false);
const likeCount = ref(0);

// 返回上一页
const onBack = () => {
  router.back();
};

// 格式化日期
const formatDate = (date) => {
  if (!date) return '';
  return dayjs(date).format('YYYY-MM-DD HH:mm');
};

// 获取资源详情
const getResourceDetail = async () => {
  loading.value = true;
  try {
    const res = await resourceApi.getResourceDetail(resourceId.value);
    resource.value = res.data;
    likeCount.value = resource.value?.like_count || 0;
    // 资源详情已包含 tags 字段，无需额外请求
    
    // 增加浏览量
    // 这里可以调用一个增加浏览量的接口
  } catch (error) {
    console.error('获取资源详情失败', error);
    showToast('获取资源详情失败');
  } finally {
    loading.value = false;
  }
};

const toggleLike = async () => {
  if (!userStore.isLoggedIn) { showToast('请先登录'); return; }
  try {
    if (!liked.value) {
      const res = await post(`/packages/${resourceId.value}/like`, {});
      likeCount.value = res?.data?.like_count ?? (likeCount.value + 1);
      liked.value = true;
    } else {
      const res = await del(`/packages/${resourceId.value}/like`);
      likeCount.value = res?.data?.like_count ?? Math.max(0, likeCount.value - 1);
      liked.value = false;
    }
  } catch (e) {
    showToast('操作失败');
  }
};

// 已移除独立的标签请求逻辑，标签随资源详情返回

// 获取资源评论
const getResourceComments = async () => {
  commentLoading.value = true;
  try {
    const res = await resourceApi.getResourceComments(resourceId.value);
    comments.value = res.data.list || [];
  } catch (error) {
    console.error('获取评论失败', error);
  } finally {
    commentLoading.value = false;
  }
};

// 获取相关推荐
const getRelatedResources = async () => {
  if (!resource.value) return;
  
  relatedLoading.value = true;
  try {
    // 根据分类获取相关资源
    if (resource.value.category_id) {
      const res = await resourceApi.getResourcesByCategory(resource.value.category_id, 1, 5);
      // 过滤掉当前资源
      relatedResources.value = (res.data.list || []).filter(item => item.id !== resourceId.value);
    }
  } catch (error) {
    console.error('获取相关资源失败', error);
  } finally {
    relatedLoading.value = false;
  }
};

// 下载资源
const downloadResource = async () => {
  if (!userStore.isLoggedIn) {
    showDialog({
      title: '提示',
      message: '请先登录再下载资源',
      confirmButtonText: '去登录',
      cancelButtonText: '取消',
    }).then(() => {
      router.push({
        path: '/login',
        query: { redirect: `/resource/${resourceId.value}` }
      });
    });
    return;
  }
  
  downloading.value = true;
  try {
    const res = await resourceApi.downloadResource(resourceId.value);
    if (res.data && res.data.url) {
      // 打开下载链接
      window.open(res.data.url, '_blank');
      showToast('开始下载');
    } else {
      showToast('下载失败，请重试');
    }
  } catch (error) {
    console.error('下载失败', error);
    showToast('下载失败，请重试');
  } finally {
    downloading.value = false;
  }
};

// 点赞评论
const likeComment = (comment) => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    return;
  }
  
  // TODO: 调用点赞接口
  showToast('点赞成功');
  comment.likes = (comment.likes || 0) + 1;
  comment.isLiked = true;
};

// 回复评论
const replyComment = (comment) => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    return;
  }
  
  replyTo.value = comment;
  commentContent.value = `回复 @${comment.author_name}：`;
  showCommentInput.value = true;
};

// 提交评论
const submitComment = async () => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    return;
  }
  
  if (!commentContent.value.trim()) {
    showToast('评论内容不能为空');
    return;
  }
  
  try {
    await resourceApi.addComment(resourceId.value, commentContent.value);
    showToast('评论成功');
    commentContent.value = '';
    showCommentInput.value = false;
    
    // 重新获取评论
    await getResourceComments();
  } catch (error) {
    console.error('提交评论失败', error);
    showToast('提交评论失败');
  }
};

// 页面加载
onMounted(() => {
  getResourceDetail();
  getResourceComments();
  getRelatedResources();
});
</script>

<style scoped>
.resource-detail {
  background-color: var(--background-color);
  min-height: 100vh;
  padding-bottom: 50px;
}

.detail-content {
  padding-bottom: 16px;
}

.resource-cover {
  width: 100%;
  height: 240px;
  overflow: hidden;
}

.resource-cover .van-image {
  width: 100%;
  height: 100%;
}

.detail-card {
  background-color: #fff;
  margin-bottom: 12px;
  padding: 16px;
}

.resource-title {
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 12px;
  color: var(--text-color);
}

.resource-meta {
  display: flex;
  flex-wrap: wrap;
  font-size: 14px;
  color: var(--text-color-light);
  margin-bottom: 12px;
}

.resource-meta span {
  margin-right: 12px;
  margin-bottom: 8px;
}

.resource-tags {
  margin-bottom: 16px;
}

.resource-stats {
  display: flex;
  margin-bottom: 16px;
}

.stat-item {
  display: flex;
  align-items: center;
  margin-right: 16px;
  color: var(--text-color-light);
}

.stat-item .van-icon {
  margin-right: 4px;
}

.resource-actions {
  margin-top: 16px;
}

.card-title {
  font-size: 16px;
  font-weight: 500;
  margin-bottom: 12px;
  color: var(--text-color);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.resource-description {
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-color);
  white-space: pre-wrap;
}

.comment-list {
  margin-top: 12px;
}

.comment-item {
  padding: 12px 0;
  border-bottom: 1px solid var(--border-color);
}

.comment-item:last-child {
  border-bottom: none;
}

.comment-header {
  margin-bottom: 8px;
}

.comment-author {
  display: flex;
  align-items: center;
}

.author-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  margin-right: 8px;
  object-fit: cover;
}

.author-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-color);
}

.comment-time {
  font-size: 12px;
  color: var(--text-color-lighter);
}

.comment-content {
  font-size: 14px;
  color: var(--text-color);
  line-height: 1.6;
  margin-bottom: 8px;
}

.comment-actions {
  display: flex;
  font-size: 12px;
  color: var(--text-color-light);
}

.comment-action {
  display: flex;
  align-items: center;
  margin-right: 16px;
  cursor: pointer;
}

.comment-action .van-icon {
  margin-right: 4px;
}

.comment-action .liked {
  color: #ee0a24;
}

.no-comments {
  text-align: center;
  padding: 16px 0;
  color: var(--text-color-light);
}

.comment-form {
  padding: 16px;
}

.comment-form-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
  gap: 12px;
}

.resource-not-found {
  padding: 32px 16px;
  text-align: center;
}

.mr-2 {
  margin-right: 8px;
}
.actions-row { display: flex; align-items: center; gap: 8px; }
.like-click { cursor: pointer; }
.ml8 { margin-left: 8px; }
</style> 