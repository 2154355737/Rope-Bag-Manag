<template>
  <div class="post-detail-page">
    <van-nav-bar :title="post?.title || '帖子详情'" left-arrow @click-left="onBack" fixed />
    <div class="content content-with-bar">
      <van-skeleton title :row="8" :loading="loading" />
      <template v-if="!loading && post">
        <div class="meta">
          <div class="meta-row">
            <van-icon name="user-o" />
            <span class="label">作者</span>
            <span>{{ post.author_name || post.author_id }}</span>
          </div>
          <div class="meta-row">
            <van-icon name="clock-o" />
            <span class="label">发布时间</span>
            <span>{{ formatDate(post.created_at) }}</span>
          </div>
          <div class="meta-row meta-stats">
            <div class="stat">
              <van-icon name="eye-o" />
              <span>{{ post.view_count || 0 }}</span>
            </div>
            <div class="stat like-click" @click="toggleLike">
              <van-icon :name="liked ? 'like' : 'like-o'" :color="liked ? '#ee0a24' : undefined" />
              <span>{{ post.like_count || 0 }}</span>
            </div>
            <van-tag v-if="post.is_featured" type="primary" size="small" class="ml8">精选</van-tag>
          </div>
          <div class="tags" v-if="post.tags && post.tags.length">
            <van-tag v-for="t in post.tags" :key="t" plain round type="primary" class="mr6"># {{ t }}</van-tag>
          </div>
        </div>
        <div class="content-box" v-html="renderedContent"></div>

        <div class="comments-card">
          <div class="card-title">评论 ({{ commentsTotal }})</div>
          <div v-if="comments.length > 0">
            <div class="comment-item" v-for="c in comments" :key="c.id">
              <div class="comment-header">
                <div class="author-info">
                  <img class="avatar" :src="c.author_avatar || defaultAvatar" alt="" />
                  <div class="names">
                    <div class="author">{{ c.author_name || ('用户#' + c.user_id) }}</div>
                    <div class="role" v-if="c.author_role">{{ mapRole(c.author_role) }}</div>
                  </div>
                </div>
                <div class="time">{{ formatDate(c.created_at) }}</div>
              </div>
              <div class="comment-content">
                <span v-if="c.reply_to_user" class="reply-indicator">回复 @{{ c.reply_to_user }}：</span>
                {{ c.content }}
              </div>
              <div class="comment-actions">
                <van-tag size="small" type="primary" v-if="c.pinned">置顶</van-tag>
                <van-button v-if="canPin" size="mini" type="default" @click="togglePin(c)" class="ml8">{{ c.pinned ? '取消置顶' : '置顶' }}</van-button>
              </div>
              
              <!-- 回复评论列表 -->
              <div class="reply-list" v-if="c.replies && c.replies.length > 0">
                <div class="reply-item" v-for="reply in c.replies" :key="reply.id">
                  <div class="reply-header">
                    <img :src="reply.author_avatar || defaultAvatar" alt="头像" class="reply-avatar" />
                    <div class="reply-author">
                      <span class="reply-author-name">{{ reply.author_name }}</span>
                      <span class="reply-time">{{ formatDate(reply.created_at) }}</span>
                    </div>
                  </div>
                  <div class="reply-content">
                    <span v-if="reply.reply_to_user" class="reply-indicator">回复 @{{ reply.reply_to_user }}：</span>
                    {{ reply.content }}
                  </div>
                  <div class="reply-actions">
                    <van-tag size="small" type="primary" v-if="reply.pinned">置顶</van-tag>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="no-comments">暂无评论</div>

          <div class="comment-editor">
            <van-field v-model="newComment" rows="3" type="textarea" placeholder="说点什么..." />
            <van-button type="primary" size="small" class="mt8" @click="submitComment" :loading="submitting" :disabled="!newComment.trim()">发表评论</van-button>
          </div>
        </div>
      </template>
    </div>

    <!-- Bottom Like Action Bar -->
    <van-action-bar safe-area-inset-bottom>
      <van-action-bar-icon :icon="liked ? 'like' : 'like-o'" :text="liked ? '已赞' : '点赞'" :badge="post?.like_count || 0" :color="liked ? '#ee0a24' : undefined" @click="toggleLike" />
      <van-action-bar-button type="danger" @click="toggleLike">{{ liked ? '已点赞' : '点赞支持' }}</van-action-bar-button>
    </van-action-bar>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import dayjs from 'dayjs';
import { postApi } from '../api/post';
import { useUserStore } from '../store/user';
import { showToast } from 'vant';
import { get } from '../utils/request';
import defaultAvatar from '@/assets/default-avatar.svg';

const route = useRoute();
const router = useRouter();
const userStore = useUserStore();
const id = Number(route.params.id);

const post = ref(null);
const loading = ref(false);

const comments = ref([]);
const commentsTotal = ref(0);
const newComment = ref('');
const submitting = ref(false);

const liked = ref(false);

const onBack = () => router.back();
const formatDate = (d) => (d ? dayjs(d).format('YYYY-MM-DD HH:mm') : '');

const renderedContent = computed(() => {
  if (!post.value?.content) return '';
  
  let content = post.value.content;
  
  // 处理代码块 (```)
  content = content.replace(/```(\w*)\n([\s\S]*?)```/g, (match, language, code) => {
    return `<div class="code-block">
      <div class="code-header">${language || '代码'}</div>
      <pre class="code-content"><code>${escapeHtml(code)}</code></pre>
    </div>`;
  });
  
  // 处理内联代码 (`)
  content = content.replace(/`([^`]+)`/g, '<code class="inline-code">$1</code>');
  
  // 处理换行
  content = content.replace(/\n/g, '<br/>');
  
  return content;
});

// HTML转义函数
const escapeHtml = (text) => {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
};

const roleMap = {
  admin: '管理员',
  elder: '元老',
  moderator: '版主',
  user: '用户',
};
const mapRole = (r) => roleMap[(r || '').toLowerCase()] || r || '';

const canPin = computed(() => {
  const role = (userStore.userRole || '').toLowerCase();
  return role === 'admin' || role === 'elder';
});

const togglePin = async (c) => {
  try {
    await postApi.pinComment(c.id, !c.pinned);
    c.pinned = !c.pinned;
  } catch (e) {
    // ignore
  }
};

const toggleLike = async () => {
  if (!userStore.isLoggedIn) {
    router.push({ path: '/login', query: { redirect: `/post/${id}` } });
    return;
  }
  try {
    if (!liked.value) {
      const res = await postApi.likePost(id);
      const cnt = res?.data?.like_count ?? (Number(post.value?.like_count || 0) + 1);
      if (post.value) post.value.like_count = cnt;
      liked.value = true;
    } else {
      const res = await postApi.unlikePost(id);
      const cnt = res?.data?.like_count ?? Math.max(0, Number(post.value?.like_count || 0) - 1);
      if (post.value) post.value.like_count = cnt;
      liked.value = false;
    }
  } catch (e) {
    showToast('操作失败');
  }
};

const loadComments = async () => {
  try {
    const res = await postApi.getPostComments(id, 1, 50);
    comments.value = res.data?.list || res.data || [];
    commentsTotal.value = res.data?.total ?? comments.value.length;
  } catch (e) {
    const msg = e?.message || '';
    // 403/401 统一处理：提示并显示空评论
    if (e?.response?.status === 401) {
      // 未登录可视为只读，保持空列表
      comments.value = [];
      commentsTotal.value = 0;
    } else if (e?.response?.status === 403) {
      comments.value = [];
      commentsTotal.value = 0;
    } else {
      comments.value = [];
      commentsTotal.value = 0;
    }
  }
};

const submitComment = async () => {
  if (!userStore.isLoggedIn) {
    router.push({ path: '/login', query: { redirect: `/post/${id}` } });
    return;
  }
  if (!newComment.value.trim()) return;
  submitting.value = true;
  try {
    await postApi.addPostComment(id, newComment.value.trim());
    newComment.value = '';
    await loadComments();
  } finally {
    submitting.value = false;
  }
};

const loadDetail = async () => {
  loading.value = true;
  try {
    const res = await postApi.getPostDetail(id);
    post.value = res.data;
    
    // 检查用户点赞状态
    if (userStore.isLoggedIn) {
      await checkLikeStatus();
    } else {
      liked.value = false;
    }
    
    // 上报浏览
    postApi.incrementView(id).catch(() => {});
    await loadComments();
  } finally {
    loading.value = false;
  }
};

// 检查用户点赞状态
const checkLikeStatus = async () => {
  try {
    // 尝试调用帖子点赞状态检查接口
    const res = await postApi.checkLikeStatus(id);
    liked.value = res?.data?.liked || false;
  } catch (error) {
    // 如果接口不存在或出错，尝试从用户行为记录中获取
    console.log('🔄 帖子点赞状态接口不可用，尝试从用户行为记录获取:', error.message);
    
    try {
      const userActions = await get('/user-actions', {
        page: 1,
        page_size: 100,
        user_id: userStore.userId,
        action_type: 'Like',
        target_type: 'Post',
        target_id: id
      });
      liked.value = (userActions?.data?.actions?.length || 0) > 0;
      console.log('✅ 从用户行为记录获取帖子点赞状态:', liked.value);
    } catch (fallbackError) {
      // 最终回退：默认为未点赞
      console.log('⚠️ 用户行为记录也无法获取，默认为未点赞');
      liked.value = false;
    }
  }
};

onMounted(loadDetail);
</script>

<style scoped>
.content { padding: 12px; }
.content-with-bar { padding-bottom: 12px; }
.meta { background: #fff; padding: 12px; border-radius: 8px; margin-bottom: 12px; color: var(--text-color-light); font-size: 13px; }
.meta-row { display: flex; align-items: center; gap: 8px; margin-bottom: 6px; }
.meta-row .label { color: var(--text-color-lighter); }
.meta-stats { gap: 16px; }
.meta-stats .stat { display: flex; align-items: center; gap: 4px; }
.content-box { background: #fff; padding: 12px; border-radius: 8px; line-height: 1.8; margin-bottom: 12px; }
.comments-card { background: #fff; padding: 12px; border-radius: 8px; }
.card-title { font-size: 16px; font-weight: 600; margin-bottom: 10px; }
.comment-item { 
  padding: 16px; 
  margin-bottom: 16px; 
  background: #ffffff; 
  border-radius: 12px; 
  border: 1px solid #e8eaed; 
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04); 
  position: relative; 
}
.comment-item:last-child { margin-bottom: 0; }
.comment-header { display: flex; justify-content: space-between; color: var(--text-color-light); font-size: 12px; margin-bottom: 6px; }
.author-info { display: flex; align-items: center; gap: 8px; }
.avatar { width: 28px; height: 28px; border-radius: 50%; object-fit: cover; }
.names { display: flex; flex-direction: column; line-height: 1.2; }
.role { font-size: 11px; color: var(--text-color-lighter); }
.comment-content { font-size: 14px; color: var(--text-color); line-height: 1.6; }
.comment-actions { margin-top: 6px; }
.comment-editor { margin-top: 10px; }
.mt8 { margin-top: 8px; }
.ml8 { margin-left: 8px; }
.mr6 { margin-right: 6px; }
.like-click { cursor: pointer; display: inline-flex; align-items: center; gap: 4px; }

/* 回复评论样式 */
.reply-indicator {
  color: var(--primary-color);
  font-weight: 600;
  background: rgba(79, 192, 141, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12px;
  margin-right: 4px;
}

.reply-list {
  margin-top: 16px;
  margin-left: 16px;
  border-left: 3px solid var(--primary-color);
  padding-left: 16px;
  background: linear-gradient(90deg, rgba(79, 192, 141, 0.03) 0%, transparent 100%);
  border-radius: 0 8px 8px 0;
}

.reply-item {
  padding: 16px 12px;
  margin-bottom: 8px;
  background: #fafbfc;
  border-radius: 8px;
  border: 1px solid #f0f1f3;
  position: relative;
}

.reply-item::before {
  content: '';
  position: absolute;
  left: -19px;
  top: 20px;
  width: 8px;
  height: 2px;
  background: var(--primary-color);
  border-radius: 1px;
}

.reply-item:last-child {
  margin-bottom: 0;
}

.reply-header {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

.reply-avatar {
  width: 28px;
  height: 28px;
  border-radius: 50%;
  margin-right: 10px;
  object-fit: cover;
  border: 2px solid var(--primary-color);
  box-shadow: 0 2px 4px rgba(79, 192, 141, 0.2);
}

.reply-author {
  display: flex;
  flex-direction: column;
}

.reply-author-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-color);
}

.reply-time {
  font-size: 12px;
  color: var(--text-color-lighter);
}

.reply-content {
  font-size: 14px;
  color: var(--text-color);
  line-height: 1.6;
  margin-bottom: 10px;
  padding: 8px 0;
}

.reply-actions {
  display: flex;
  font-size: 12px;
  color: var(--text-color-light);
  gap: 12px;
}

.reply-actions .van-tag {
  background: rgba(79, 192, 141, 0.1);
  border: 1px solid rgba(79, 192, 141, 0.3);
  border-radius: 12px;
}

/* 代码块样式 */
.code-block {
  margin: 16px 0;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #e1e5e9;
  background-color: #f8f9fa;
}

.code-header {
  background-color: #e9ecef;
  padding: 8px 12px;
  font-size: 12px;
  font-weight: 500;
  color: #495057;
  border-bottom: 1px solid #e1e5e9;
}

.code-content {
  margin: 0;
  padding: 16px;
  background-color: #f8f9fa;
  overflow-x: auto;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'source-code-pro', monospace;
  font-size: 14px;
  line-height: 1.5;
  color: #212529;
}

.code-content code {
  background: none;
  padding: 0;
  border: none;
  font-family: inherit;
}

.inline-code {
  background-color: #f1f3f4;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'source-code-pro', monospace;
  font-size: 13px;
  color: #d73a49;
  border: 1px solid #e1e5e9;
}
</style> 