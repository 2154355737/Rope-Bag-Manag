<template>
  <div class="post-detail">
    <van-nav-bar :title="post?.title || '帖子详情'" left-arrow @click-left="onBack" fixed />

    <div class="detail-content">
      <van-skeleton title :row="12" :loading="loading" v-if="loading" />

      <template v-else-if="post">
        <!-- 头部信息卡片 -->
        <div class="detail-card">
          <h1 class="post-title">{{ post.title }}</h1>
          <div class="post-meta">
            <span class="author">作者：{{ post.author_name || post.author_id }}</span>
            <span class="date">发布时间：{{ formatDate(post.created_at) }}</span>
            <van-tag v-if="post.is_featured" type="primary" size="small" class="ml8">精选</van-tag>
          </div>

          <div class="post-tags" v-if="post.tags && post.tags.length">
            <van-tag
              v-for="t in post.tags"
              :key="t"
              plain
              round
              size="medium"
              type="primary"
              class="mr6"
            >
              # {{ t }}
            </van-tag>
          </div>

          <div class="post-stats">
            <div class="stat-item">
              <van-icon name="eye-o" />
              <span>{{ post.view_count || 0 }}</span>
            </div>
            <div class="stat-item like-click" @click="toggleLike" :class="{ 'processing': isLikeProcessing }">
              <van-icon 
                :name="isLikeProcessing ? 'loading' : (liked ? 'like' : 'like-o')" 
                :color="liked ? '#ee0a24' : undefined" 
                :class="{ 'van-loading__spinner': isLikeProcessing }"
              />
              <span>{{ likeCount }}</span>
            </div>
            <div class="stat-item">
              <van-icon name="comment-o" />
              <span>{{ commentsTotal || 0 }}</span>
            </div>
          </div>

          <div class="actions-row">
            <van-button 
              :type="liked ? 'danger' : 'primary'" 
              block 
              round 
              :icon="liked ? 'like' : 'like-o'"
              :loading="isLikeProcessing"
              @click="toggleLike"
            >
              {{ isLikeProcessing ? '处理中...' : (liked ? '已赞' : '点赞支持') }}（{{ likeCount }}）
            </van-button>
          </div>
        </div>

        <!-- 正文内容 -->
        <div class="detail-card">
          <div class="card-title">帖子内容</div>
          <div class="content-box" v-html="renderedContent"></div>
        </div>

        <!-- 评论区域 -->
        <div class="detail-card">
          <div class="card-title">
            评论 ({{ commentsTotal }})
            <div class="comment-action" @click="showCommentInput = true">
              <van-icon name="edit" />
              <span>写评论</span>
            </div>
          </div>

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
                <div v-if="c.quoted_content" class="quoted-message">{{ c.quoted_content }}</div>
                <div class="comment-text">{{ c.content }}</div>
              </div>
              <div class="comment-actions">
                <van-tag size="small" type="primary" v-if="c.pinned">置顶</van-tag>
                <van-button v-if="canPin" size="mini" type="default" @click="togglePin(c)" class="ml8">{{ c.pinned ? '取消置顶' : '置顶' }}</van-button>
                <div class="comment-action ml8" @click="replyComment(c)">
                  <van-icon name="comment-o" />
                  <span>回复</span>
                </div>
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
                    <div v-if="reply.quoted_content" class="quoted-message">{{ reply.quoted_content }}</div>
                    <div class="reply-text">{{ reply.content }}</div>
                  </div>
                  <div class="reply-actions">
                    <van-tag size="small" type="primary" v-if="reply.pinned">置顶</van-tag>
                    <div class="comment-action" @click="replyComment(reply, c)">
                      <van-icon name="comment-o" />
                      <span>回复</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="no-comments">暂无评论</div>
        </div>

        <!-- 评论输入框 -->
        <van-action-sheet v-model:show="showCommentInput" title="发表评论" :safe-area-inset-bottom="true">
          <div class="comment-form" :style="{ paddingBottom: keyboardPadding + 'px' }">
            <div v-if="quotedMessage" class="quoted-original">
              <div class="quoted-header">
                <van-icon name="chat-o" class="quoted-header-icon" />
                <span>回复 @{{ quotedMessage.author }} 的评论</span>
                <van-icon name="cross" class="close-quote" @click="clearQuote" />
              </div>
              <div class="quoted-original-content">{{ quotedMessage.content }}</div>
            </div>
            <van-field
              v-model="commentContent"
              rows="3"
              autosize
              type="textarea"
              placeholder="请输入评论内容"
              maxlength="500"
              show-word-limit
              @focus="onFieldFocus"
              @blur="onFieldBlur"
            />
            <div class="comment-form-actions">
              <van-button plain @click="cancelComment">取消</van-button>
              <van-button type="primary" :disabled="!commentContent.trim()" @click="submitComment">发表</van-button>
            </div>
          </div>
        </van-action-sheet>
      </template>

      <template v-else>
        <div class="post-not-found">
          <van-empty image="error" description="帖子不存在或已被删除" />
        </div>
      </template>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue';
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

const commentContent = ref('');
const showCommentInput = ref(false);
const replyTo = ref(null);
const quotedMessage = ref(null);

const isLikeProcessing = ref(false);
const liked = ref(false);
const likeCount = ref(0);
const originalViewportHeight = ref(window.innerHeight || 0);
const keyboardPadding = ref(0);

const onBack = () => router.back();
const formatDate = (d) => (d ? dayjs(d).format('YYYY-MM-DD HH:mm') : '');

const renderedContent = computed(() => {
  if (!post.value?.content) return '';
  let content = post.value.content;
  content = content.replace(/```(\w*)\n([\s\S]*?)```/g, (match, language, code) => {
    return `<div class="code-block">
      <div class="code-header">${language || '代码'}</div>
      <pre class="code-content"><code>${escapeHtml(code)}</code></pre>
    </div>`;
  });
  content = content.replace(/`([^`]+)`/g, '<code class="inline-code">$1<\/code>');
  content = content.replace(/\n/g, '<br/>');
  return content;
});

const escapeHtml = (text) => {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
};

const roleMap = { admin: '管理员', elder: '元老', moderator: '版主', user: '用户' };
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
  }
};

const replyComment = (comment, parentComment = null) => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    return;
  }
  replyTo.value = comment;
  quotedMessage.value = {
    author: comment.author_name,
    content: comment.content,
    time: comment.created_at
  };
  commentContent.value = `回复 @${comment.author_name}：`;
  showCommentInput.value = true;
};

const clearQuote = () => {
  quotedMessage.value = null;
  replyTo.value = null;
  commentContent.value = '';
};

const cancelComment = () => {
  showCommentInput.value = false;
  clearQuote();
};

const toggleLike = async () => {
  if (!userStore.isLoggedIn) { 
    router.push({ path: '/login', query: { redirect: `/post/${id}` } });
    return; 
  }
  if (isLikeProcessing.value) return;
  isLikeProcessing.value = true;
  const originalLiked = liked.value;
  const originalCount = likeCount.value;

  try {
    if (!liked.value) {
      liked.value = true;
      likeCount.value = likeCount.value + 1;
      const res = await postApi.likePost(id);
      if (res?.data?.like_count !== undefined) {
        likeCount.value = res.data.like_count;
      }
      if (post.value) post.value.like_count = likeCount.value;
      showToast('点赞成功');
    } else {
      liked.value = false;
      likeCount.value = Math.max(0, likeCount.value - 1);
      const res = await postApi.unlikePost(id);
      if (res?.data?.like_count !== undefined) {
        likeCount.value = res.data.like_count;
      }
      if (post.value) post.value.like_count = likeCount.value;
      showToast('已取消点赞');
    }
  } catch (e) {
    liked.value = originalLiked;
    likeCount.value = originalCount;
    showToast('操作失败，请稍后重试');
    setTimeout(() => checkLikeStatus(), 800);
  } finally {
    isLikeProcessing.value = false;
  }
};

// 将评论解析为带引用显示的形式
const processCommentsWithQuotes = (commentList) => {
  const allCommentsByUser = new Map();
  commentList.forEach(comment => {
    if (!allCommentsByUser.has(comment.author_name)) {
      allCommentsByUser.set(comment.author_name, []);
    }
    allCommentsByUser.get(comment.author_name).push(comment);
    if (comment.replies && comment.replies.length > 0) {
      comment.replies.forEach(reply => {
        if (!allCommentsByUser.has(reply.author_name)) {
          allCommentsByUser.set(reply.author_name, []);
        }
        allCommentsByUser.get(reply.author_name).push(reply);
      });
    }
  });

  const replyPattern = /^回复\s+@([^：:]+)[：:]\s*(.*)/;

  const processOne = (c, upper = null) => {
    const processed = { ...c };
    const match = c.content && c.content.match(replyPattern);
    if (match) {
      const replyToUser = match[1].trim();
      const actualContent = match[2].trim();
      // 找目标（同用户最近一条更早的评论或上层）
      let targetContent = null;
      if (upper && replyToUser === upper.author_name) {
        targetContent = upper.content.replace(replyPattern, (m, _u, rest) => rest.trim() || upper.content);
      } else {
        const targetUserComments = allCommentsByUser.get(replyToUser) || [];
        const targetComment = targetUserComments
          .filter(item => new Date(item.created_at) < new Date(c.created_at))
          .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))[0];
        if (targetComment) {
          targetContent = targetComment.content.replace(replyPattern, (m, _u, rest) => rest.trim() || targetComment.content);
        }
      }
      if (targetContent) {
        processed.quoted_content = targetContent;
        processed.reply_to_user = replyToUser;
        processed.content = actualContent || '';
        if (!actualContent) processed.is_pure_reply = true;
      }
    }
    return processed;
  };

  return commentList.map(c => {
    const processed = processOne(c, null);
    if (c.replies && c.replies.length > 0) {
      processed.replies = c.replies.map(r => processOne(r, c));
    }
    return processed;
  });
};

const updateKeyboardPadding = () => {
  if (!showCommentInput.value) { keyboardPadding.value = 0; return; }
  const h = window.innerHeight || 0;
  const delta = Math.max(0, originalViewportHeight.value - h);
  keyboardPadding.value = delta > 0 ? delta : 0;
};

const onFieldFocus = () => {
  setTimeout(() => {
    updateKeyboardPadding();
    const el = document.querySelector('.comment-form');
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'center' });
    }
  }, 100);
};

const onFieldBlur = () => {
  keyboardPadding.value = 0;
};

const loadComments = async () => {
  try {
    const res = await postApi.getPostComments(id, 1, 1000);
    let raw = res.data?.list || res.data || [];
    const processed = processCommentsWithQuotes(raw);
    comments.value = processed;
    commentsTotal.value = processed.length;
  } catch (e) {
    comments.value = [];
    commentsTotal.value = 0;
  }
};

const submitComment = async () => {
  if (!userStore.isLoggedIn) {
    router.push({ path: '/login', query: { redirect: `/post/${id}` } });
    return;
  }
  if (!commentContent.value.trim()) return;
  try {
    await postApi.addPostComment(id, commentContent.value.trim(), replyTo.value?.id || null);
    commentContent.value = '';
    showCommentInput.value = false;
    clearQuote();
    await loadComments();
  } finally {
  }
};

const loadDetail = async () => {
  loading.value = true;
  try {
    const res = await postApi.getPostDetail(id);
    post.value = res.data;
    likeCount.value = Number(res.data?.like_count || 0);

    if (userStore.isLoggedIn) {
      await checkLikeStatus();
    } else {
      liked.value = false;
    }

    postApi.incrementView(id).catch(() => {});
    await loadComments();
  } finally {
    loading.value = false;
  }
};

const checkLikeStatus = async () => {
  try {
    const res = await postApi.checkLikeStatus(id);
    liked.value = res?.data?.liked || false;
  } catch (error) {
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
    } catch (fallbackError) {
      liked.value = false;
    }
  }
};

onMounted(() => {
  originalViewportHeight.value = window.innerHeight || 0;
  window.addEventListener('resize', updateKeyboardPadding);
  loadDetail();
});

onUnmounted(() => {
  window.removeEventListener('resize', updateKeyboardPadding);
});
</script>

<style scoped>
.detail-content { padding: 12px; }
.detail-card { background: #fff; border-radius: 12px; padding: 12px; margin-bottom: 12px; }
.post-title { font-size: 20px; font-weight: 700; margin: 0 0 8px 0; }
.post-meta { display: flex; align-items: center; gap: 12px; color: var(--text-color-light); font-size: 13px; margin-bottom: 8px; }
.post-tags { display: flex; gap: 8px; flex-wrap: wrap; margin: 8px 0; }

.post-stats { display: flex; gap: 16px; align-items: center; margin: 8px 0 0; color: var(--text-color-light); }
.stat-item { display: flex; align-items: center; gap: 6px; font-size: 14px; }
.actions-row { margin-top: 12px; display: flex; }

.card-title { font-size: 16px; font-weight: 600; margin-bottom: 10px; display: flex; align-items: center; justify-content: space-between; }
.comment-action { display: inline-flex; align-items: center; gap: 6px; color: var(--primary-color); cursor: pointer; font-weight: 500; }

.content-box { background: #fff; padding: 12px; border-radius: 8px; line-height: 1.8; }

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
.comment-text { white-space: pre-wrap; }
.quoted-message { background: #f6f7f8; border-left: 3px solid var(--primary-color); padding: 8px; border-radius: 4px; color: var(--text-color-light); margin: 6px 0; }
.comment-actions { margin-top: 6px; display: flex; align-items: center; gap: 8px; }
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

.reply-item:last-child { margin-bottom: 0; }
.reply-header { display: flex; align-items: center; margin-bottom: 10px; }
.reply-avatar { width: 28px; height: 28px; border-radius: 50%; margin-right: 10px; object-fit: cover; border: 2px solid var(--primary-color); box-shadow: 0 2px 4px rgba(79, 192, 141, 0.2); }
.reply-author { display: flex; flex-direction: column; }
.reply-author-name { font-size: 14px; font-weight: 600; color: var(--text-color); }
.reply-time { font-size: 12px; color: var(--text-color-lighter); }
.reply-content { font-size: 14px; color: var(--text-color); line-height: 1.6; margin-bottom: 10px; padding: 8px 0; }
.reply-actions { display: flex; font-size: 12px; color: var(--text-color-light); gap: 12px; align-items: center; }
.reply-actions .van-tag { background: rgba(79, 192, 141, 0.1); border: 1px solid rgba(79, 192, 141, 0.3); border-radius: 12px; }

/* 代码块样式 */
.code-block { margin: 16px 0; border-radius: 8px; overflow: hidden; border: 1px solid #e1e5e9; background-color: #f8f9fa; }
.code-header { background-color: #e9ecef; padding: 8px 12px; font-size: 12px; font-weight: 500; color: #495057; border-bottom: 1px solid #e1e5e9; }
.code-content { margin: 0; padding: 16px; background-color: #f8f9fa; overflow-x: auto; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'source-code-pro', monospace; font-size: 14px; line-height: 1.5; color: #212529; }
.code-content code { background: none; padding: 0; border: none; font-family: inherit; }
.inline-code { background-color: #f1f3f4; padding: 2px 6px; border-radius: 4px; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'source-code-pro', monospace; font-size: 13px; color: #d73a49; border: 1px solid #e1e5e9; }

/* 评论输入 ActionSheet 样式 */
.quoted-original { background: #f6f7f8; border-left: 3px solid var(--primary-color); padding: 8px; border-radius: 4px; margin-bottom: 8px; }
.quoted-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 6px; }
.quoted-header-icon { color: var(--primary-color); }
.close-quote { cursor: pointer; color: var(--text-color-lighter); }
.comment-form-actions { display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px; }
.comment-form { max-height: 70vh; overflow: auto; }
</style> 