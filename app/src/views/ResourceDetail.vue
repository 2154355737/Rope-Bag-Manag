<template>
  <div class="resource-detail">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      :title="resource?.name || '资源详情'"
      left-arrow
      @click-left="onBack"
      fixed
    />
    
    <div class="detail-content">
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
              <span>{{ resource.comment_count || 0 }}</span>
            </div>
          </div>
          
          <div class="resource-actions actions-row">
            <van-button type="primary" block round @click="downloadResource" :loading="downloading">
              立即下载
            </van-button>
            <van-button 
              class="ml8" 
              :type="liked ? 'danger' : 'default'" 
              plain 
              round 
              icon="like-o" 
              :loading="isLikeProcessing"
              @click="toggleLike"
            >
              {{ isLikeProcessing ? '处理中...' : (liked ? '已赞' : '点赞') }}（{{ likeCount }}）
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
            评论 ({{ totalComments }})
            <div class="comment-action" @click="showCommentInput = true">
              <van-icon name="edit" />
              <span>写评论</span>
            </div>
          </div>
          
          <!-- 评论控制栏 -->
          <div class="comment-controls-bar" v-if="comments.length > 0">
            <div class="comment-sort">
              <span class="sort-label">排序：</span>
              <van-dropdown-menu>
                <van-dropdown-item v-model="sortType" :options="sortOptions" @change="onSortChange" />
              </van-dropdown-menu>
            </div>
            <div class="comment-info">
              <span>{{ (currentPage - 1) * pageSize + 1 }}-{{ Math.min(currentPage * pageSize, totalComments) }} / {{ totalComments }}</span>
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
                <!-- 回复格式显示：回复 @用户名: 内容 -->
                <div v-if="comment.quoted_content" class="reply-format">
                  <div class="reply-text">
                    回复 @{{ comment.reply_to_user }}：<span v-if="comment.content">{{ comment.content }}</span>
                  </div>
                  <div class="original-message">{{ comment.quoted_content }}</div>
                </div>
                <!-- 普通评论 -->
                <div v-else class="comment-text">{{ comment.content }}</div>
              </div>
              <div class="comment-actions">
                <div class="comment-action" @click="likeComment(comment)" :class="{ 'processing': getCommentLikeState(comment).processing }">
                  <van-icon name="like-o" :class="{ 'liked': getCommentLikeState(comment).liked }" />
                  <span>{{ getCommentLikeState(comment).likeCount }}</span>
                </div>
                <div class="comment-action" @click="replyComment(comment)">
                  <van-icon name="comment-o" />
                  <span>回复</span>
                </div>
              </div>
              
              <!-- 回复评论列表 -->
              <div class="reply-list" v-if="comment.replies && comment.replies.length > 0">
                <div class="reply-item" v-for="reply in comment.replies" :key="reply.id">
                  <div class="reply-header">
                    <img :src="reply.author_avatar || '/img/default-avatar.jpg'" alt="头像" class="reply-avatar" />
                    <div class="reply-author">
                      <span class="reply-author-name">{{ reply.author_name }}</span>
                      <span class="reply-time">{{ formatDate(reply.created_at) }}</span>
                    </div>
                  </div>
                  <div class="reply-content">
                    <!-- 回复格式显示：回复 @用户名: 内容 -->
                    <div v-if="reply.quoted_content" class="reply-format">
                      <div class="reply-text">
                        回复 @{{ reply.reply_to_user }}：<span v-if="reply.content">{{ reply.content }}</span>
                      </div>
                      <div class="original-message">{{ reply.quoted_content }}</div>
                    </div>
                    <!-- 普通回复 -->
                    <div v-else class="reply-text">{{ reply.content }}</div>
                  </div>
                  <div class="reply-actions">
                    <div class="comment-action" @click="likeComment(reply)" :class="{ 'processing': getCommentLikeState(reply).processing }">
                      <van-icon name="like-o" :class="{ 'liked': getCommentLikeState(reply).liked }" />
                      <span>{{ getCommentLikeState(reply).likeCount }}</span>
                    </div>
                    <div class="comment-action" @click="replyComment(reply, comment)">
                      <van-icon name="comment-o" />
                      <span>回复</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- 分页控制 -->
            <div class="comment-pagination" v-if="totalPages > 1">
              <van-pagination
                v-model="currentPage"
                :total-items="totalComments"
                :items-per-page="pageSize"
                :force-ellipses="true"
                @change="onPageChange"
              />
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
    <van-action-sheet v-model:show="showCommentInput" :title="quotedMessage ? '回复评论' : '发表评论'">
      <div class="comment-form">
        <!-- 引用原消息显示 -->
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
        />
        <div class="comment-form-actions">
          <van-button plain @click="cancelComment">取消</van-button>
          <van-button type="primary" :disabled="!commentContent.trim()" @click="submitComment">发表</van-button>
        </div>
      </div>
    </van-action-sheet>
    
  </div>
</template>

<script setup>
 import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
 import { useRouter, useRoute, onBeforeRouteUpdate } from 'vue-router';
 import { showToast, showDialog } from 'vant';
 import { resourceApi } from '../api/resource'
import { likeComment as likeCommentApi, checkCommentLikeStatus } from '../api/comment';
 import { useUserStore } from '../store/user';
 import ResourceList from '../components/ResourceList.vue';
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
const quotedMessage = ref(null); // 引用的原消息

// 评论点赞状态管理
const commentLikeStates = ref(new Map()); // commentId -> { liked: boolean, likeCount: number, processing: boolean }

// 获取评论的点赞状态
const getCommentLikeState = (comment) => {
  const state = commentLikeStates.value.get(comment.id);
  if (state) {
    return {
      liked: state.liked,
      likeCount: state.likeCount,
      processing: state.processing
    };
  }
  return {
    liked: comment.isLiked || false,
    likeCount: comment.likes || 0,
    processing: false
  };
};

// 评论分页和排序
const currentPage = ref(1); // 当前页码
const pageSize = ref(5); // 每页评论数量
const totalComments = ref(0); // 评论总数
const sortType = ref('latest'); // 排序类型
const allComments = ref([]); // 所有评论数据

// 排序选项
const sortOptions = [
  { text: '最新发布', value: 'latest' },
  { text: '最早发布', value: 'earliest' },
  { text: '最多点赞', value: 'most_liked' },
  { text: '最少点赞', value: 'least_liked' }
];

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

// 计算属性：总页数
const totalPages = computed(() => {
  return Math.ceil(totalComments.value / pageSize.value);
});

// 排序变化处理
const onSortChange = (value) => {
  sortType.value = value;
  currentPage.value = 1; // 重置到第一页
  updateDisplayedComments();
};

// 页码变化处理
const onPageChange = (page) => {
  currentPage.value = page;
  updateDisplayedComments();
  
  // 滚动到评论区域顶部
  const commentElement = document.querySelector('.comment-list');
  if (commentElement) {
    commentElement.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }
};

// 获取资源详情
const getResourceDetail = async () => {
  loading.value = true;
  try {
    const res = await resourceApi.getResourceDetail(resourceId.value);
    resource.value = res.data;
    likeCount.value = resource.value?.like_count || 0;
    
    // 检查用户点赞状态
    if (userStore.isLoggedIn) {
      await checkLikeStatus();
    } else {
      liked.value = false;
    }
  } catch (error) {
    console.error('获取资源详情失败', error);
    showToast('获取资源详情失败');
  } finally {
    loading.value = false;
  }
};

// 检查用户点赞状态
const checkLikeStatus = async () => {
  if (!userStore.isLoggedIn || !resourceId.value) {
    liked.value = false;
    return;
  }
  
  try {
    console.log('🔍 检查点赞状态，用户ID:', userStore.userId, '资源ID:', resourceId.value);
    
    // 尝试调用点赞状态检查接口
    const res = await resourceApi.checkLikeStatus(resourceId.value);
    const isLiked = res?.data?.liked || res?.data?.is_liked || false;
    liked.value = isLiked;
    console.log('✅ 从点赞状态接口获取:', isLiked);
  } catch (error) {
    // 如果接口不存在或出错，尝试从用户行为记录中获取
    console.log('🔄 点赞状态接口不可用，尝试从用户行为记录获取:', error.message);
    
    try {
      const userActions = await get('/user-actions', {
        page: 1,
        page_size: 100,
        user_id: userStore.userId,
        action_type: 'Like',
        target_type: 'Package',
        target_id: resourceId.value
      });
      const hasLiked = (userActions?.data?.actions?.length || 0) > 0;
      liked.value = hasLiked;
      console.log('✅ 从用户行为记录获取点赞状态:', hasLiked);
    } catch (fallbackError) {
      // 最终回退：默认为未点赞
      console.log('⚠️ 用户行为记录也无法获取，默认为未点赞');
      liked.value = false;
    }
  }
};

// 防止重复点击的标记
const isLikeProcessing = ref(false);

const toggleLike = async () => {
  if (!userStore.isLoggedIn) { 
    showToast('请先登录'); 
    return; 
  }
  
  // 防止重复点击
  if (isLikeProcessing.value) {
    console.log('🔄 点赞操作进行中，忽略重复点击');
    return;
  }
  
  isLikeProcessing.value = true;
  const originalLiked = liked.value;
  const originalCount = likeCount.value;
  
  try {
    if (!liked.value) {
      // 乐观更新UI
      liked.value = true;
      likeCount.value = likeCount.value + 1;
      
      console.log('👍 执行点赞操作');
      const res = await resourceApi.likeResource(resourceId.value);
      
      // 从服务器响应更新实际数据
      if (res?.data?.like_count !== undefined) {
        likeCount.value = res.data.like_count;
      }
      console.log('✅ 点赞成功，当前点赞数:', likeCount.value);
      showToast('点赞成功');
    } else {
      // 乐观更新UI
      liked.value = false;
      likeCount.value = Math.max(0, likeCount.value - 1);
      
      console.log('👎 执行取消点赞操作');
      const res = await resourceApi.unlikeResource(resourceId.value);
      
      // 从服务器响应更新实际数据
      if (res?.data?.like_count !== undefined) {
        likeCount.value = res.data.like_count;
      }
      console.log('✅ 取消点赞成功，当前点赞数:', likeCount.value);
      showToast('已取消点赞');
    }
  } catch (error) {
    // 错误时回滚UI状态
    liked.value = originalLiked;
    likeCount.value = originalCount;
    
    console.error('点赞操作失败:', error);
    showToast('操作失败，请稍后重试');
    
    // 重新检查状态以确保同步
    setTimeout(() => checkLikeStatus(), 1000);
  } finally {
    // 无论成功或失败都要释放处理标记
    isLikeProcessing.value = false;
  }
};

// 已移除独立的标签请求逻辑，标签随资源详情返回

// 处理评论数据，为回复评论添加引用信息
const processCommentsWithQuotes = (commentList) => {
  // 创建所有评论的映射表，用于查找被回复的原评论
  const allCommentsByUser = new Map();
  
  // 第一遍：收集所有评论（包括主评论和回复）
  commentList.forEach(comment => {
    if (!allCommentsByUser.has(comment.author_name)) {
      allCommentsByUser.set(comment.author_name, []);
    }
    allCommentsByUser.get(comment.author_name).push(comment);
    
    // 同时收集回复评论
    if (comment.replies && comment.replies.length > 0) {
      comment.replies.forEach(reply => {
        if (!allCommentsByUser.has(reply.author_name)) {
          allCommentsByUser.set(reply.author_name, []);
        }
        allCommentsByUser.get(reply.author_name).push(reply);
      });
    }
  });
  
  // 第二遍：处理回复评论，添加引用信息
  return commentList.map(comment => {
    const processedComment = { ...comment };
    
    // 检查主评论是否是回复格式
    const replyPattern = /^回复\s+@([^：:]+)[：:]\s*(.*)/;
    const match = comment.content.match(replyPattern);
    
    if (match) {
      const replyToUser = match[1].trim();
      const actualContent = match[2].trim();
      
      // 查找被回复用户的最近评论
      const targetUserComments = allCommentsByUser.get(replyToUser) || [];
      const targetComment = targetUserComments
        .filter(c => new Date(c.created_at) < new Date(comment.created_at))
        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))[0];
      
      if (targetComment) {
        // 清理目标评论内容（如果目标评论也是回复格式，则提取纯内容）
        let cleanTargetContent = targetComment.content;
        const targetMatch = targetComment.content.match(replyPattern);
        if (targetMatch) {
          cleanTargetContent = targetMatch[2].trim();
        }
        
        processedComment.quoted_content = cleanTargetContent;
        processedComment.quoted_time = targetComment.created_at;
        processedComment.reply_to_user = replyToUser;
        
        // 更新评论内容，如果没有实际内容则标记为纯回复
        if (actualContent) {
          processedComment.content = actualContent;
        } else {
          processedComment.content = ''; // 纯回复，无额外内容
          processedComment.is_pure_reply = true;
        }
      }
    }
    
    // 处理回复列表
    if (comment.replies && comment.replies.length > 0) {
      processedComment.replies = comment.replies.map(reply => {
        const processedReply = { ...reply };
        const replyMatch = reply.content.match(replyPattern);
        
        if (replyMatch) {
          const replyToUser = replyMatch[1].trim();
          const actualContent = replyMatch[2].trim();
          
          // 查找被回复的内容
          let targetContent = null;
          let targetTime = null;
          
                     // 先查看是否回复主评论作者
           if (replyToUser === comment.author_name) {
             // 清理主评论内容
             let cleanMainContent = comment.content;
             const mainMatch = comment.content.match(replyPattern);
             if (mainMatch) {
               cleanMainContent = mainMatch[2].trim();
             }
             targetContent = cleanMainContent;
             targetTime = comment.created_at;
           } else {
             // 查找同层回复中的目标
             const targetReply = comment.replies
               .filter(r => r.author_name === replyToUser && new Date(r.created_at) < new Date(reply.created_at))
               .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))[0];
             
             if (targetReply) {
               // 清理目标回复内容
               let cleanReplyContent = targetReply.content;
               const replyContentMatch = targetReply.content.match(replyPattern);
               if (replyContentMatch) {
                 cleanReplyContent = replyContentMatch[2].trim();
               }
               targetContent = cleanReplyContent;
               targetTime = targetReply.created_at;
             }
           }
           
           if (targetContent) {
             processedReply.quoted_content = targetContent;
             processedReply.quoted_time = targetTime;
             processedReply.reply_to_user = replyToUser;
             
                         // 更新回复内容，如果没有实际内容则标记为纯回复
            if (actualContent) {
              processedReply.content = actualContent;
            } else {
              processedReply.content = ''; // 纯回复，无额外内容
              processedReply.is_pure_reply = true;
            }
           }
        }
        
        return processedReply;
      });
    }
    
    return processedComment;
  });
};

// 批量检查评论点赞状态
const checkCommentsLikeStatus = async (comments) => {
  if (!userStore.isLoggedIn || !comments.length) return;
  
  try {
    // 为每个评论检查点赞状态
    const promises = comments.map(async (comment) => {
      try {
        const response = await checkCommentLikeStatus(comment.id);
        if (response.code === 200) {
          comment.isLiked = response.data.liked;
          commentLikeStates.value.set(comment.id, {
            liked: response.data.liked,
            likeCount: comment.likes || 0,
            processing: false
          });
        }
      } catch (error) {
        console.error(`检查评论${comment.id}点赞状态失败:`, error);
      }
    });
    
    await Promise.all(promises);
  } catch (error) {
    console.error('批量检查评论点赞状态失败:', error);
  }
};

// 获取资源评论
const getResourceComments = async () => {
  commentLoading.value = true;
  try {
    // 获取所有评论数据（不分页）
    const params = {
      page: 1,
      pageSize: 1000 // 获取大量数据，避免分页限制
    };
    
    console.log('评论请求参数:', params); // 调试日志
    
    const res = await resourceApi.getResourceComments(resourceId.value, params);
    if (res.data) {
      let commentList = res.data.list || res.data || [];
      
      // 处理评论数据，为回复评论添加引用信息
      const processedComments = processCommentsWithQuotes(commentList);
      
      // 存储所有评论
      allComments.value = processedComments;
      totalComments.value = processedComments.length;
      
      // 应用排序和分页
      updateDisplayedComments();
      
      // 检查评论点赞状态
      await checkCommentsLikeStatus(processedComments);
    } else {
      allComments.value = [];
      comments.value = [];
      totalComments.value = 0;
    }
  } catch (error) {
    console.error('获取评论失败', error);
    showToast('加载评论失败');
    // 设置默认值避免界面错误
    allComments.value = [];
    comments.value = [];
    totalComments.value = 0;
  } finally {
    commentLoading.value = false;
  }
};

// 更新显示的评论（排序+分页）
const updateDisplayedComments = () => {
  // 先排序
  let sortedComments = sortComments(allComments.value, sortType.value);
  
  // 再分页
  const startIndex = (currentPage.value - 1) * pageSize.value;
  const endIndex = startIndex + pageSize.value;
  comments.value = sortedComments.slice(startIndex, endIndex);
  
  console.log(`显示评论: 第${currentPage.value}页, ${startIndex}-${endIndex}, 共${sortedComments.length}条`);
};

// 前端排序评论（临时方案）
const sortComments = (commentList, sortType) => {
  const sorted = [...commentList];
  
  switch (sortType) {
    case 'latest':
      return sorted.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
    case 'earliest':
      return sorted.sort((a, b) => new Date(a.created_at) - new Date(b.created_at));
    case 'most_liked':
      return sorted.sort((a, b) => (b.likes || 0) - (a.likes || 0));
    case 'least_liked':
      return sorted.sort((a, b) => (a.likes || 0) - (b.likes || 0));
    default:
      return sorted;
  }
};



// 获取相关推荐
const getRelatedResources = async () => {
  if (!resource.value) return;
  relatedLoading.value = true;
  try {
    if (resource.value.category_id) {
      const res = await resourceApi.getResourcesByCategory(resource.value.category_id, 1, 5);
      relatedResources.value = (res.data.list || []).filter(item => item.id !== resourceId.value);
    }
  } catch (error) {
    console.error('获取相关资源失败', error);
  } finally {
    relatedLoading.value = false;
  }
};

// 统一重载方法
const reloadAll = async (id) => {
  const newId = Number(id ?? route.params.id);
  if (!Number.isFinite(newId)) return;
  resourceId.value = newId;
  await getResourceDetail();
  await getResourceComments();
  await getRelatedResources();
};

// 页面加载
onMounted(() => {
  reloadAll(resourceId.value);
  
  // 监听页面可见性变化，用户回到页面时重新检查点赞状态
  const handleVisibilityChange = () => {
    if (!document.hidden && userStore.isLoggedIn) {
      console.log('🔄 页面重新可见，检查点赞状态');
      checkLikeStatus();
    }
  };
  
  document.addEventListener('visibilitychange', handleVisibilityChange);
  
  // 清理事件监听器
  onUnmounted(() => {
    document.removeEventListener('visibilitychange', handleVisibilityChange);
  });
});

// 路由参数变化时刷新（同组件复用场景）
watch(() => route.params.id, (newId, oldId) => {
  if (newId !== oldId) reloadAll(Number(newId));
});

// 监听用户登录状态变化，重新检查点赞状态
watch(() => userStore.isLoggedIn, (newLoginStatus, oldLoginStatus) => {
  if (newLoginStatus && !oldLoginStatus) {
    // 用户刚登录，检查点赞状态
    console.log('🔄 用户登录状态变化，重新检查点赞状态');
    checkLikeStatus();
  } else if (!newLoginStatus && oldLoginStatus) {
    // 用户退出登录，重置点赞状态
    liked.value = false;
  }
});

onBeforeRouteUpdate((to, from, next) => {
  // 先刷新，再进入
  reloadAll(Number(to.params.id)).then(() => next());
});

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
    if (res.code === 0 && res.data) {
      // 直接使用返回的下载链接
      const downloadUrl = typeof res.data === 'string' ? res.data : res.data.url;
      window.open(downloadUrl, '_blank');
      showToast('开始下载');
    } else {
      showToast(res.message || '下载失败，请重试');
    }
  } catch (error) {
    console.error('下载失败', error);
    showToast('下载失败，请重试');
  } finally {
    downloading.value = false;
  }
};

// 点赞评论
const likeComment = async (comment) => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    return;
  }
  
  const commentId = comment.id;
  const currentState = commentLikeStates.value.get(commentId) || {
    liked: comment.isLiked || false,
    likeCount: comment.likes || 0,
    processing: false
  };
  
  if (currentState.processing) {
    return; // 防止重复点击
  }
  
  // 乐观更新UI
  const newLiked = !currentState.liked;
  const newLikeCount = newLiked ? currentState.likeCount + 1 : currentState.likeCount - 1;
  
  commentLikeStates.value.set(commentId, {
    liked: newLiked,
    likeCount: Math.max(0, newLikeCount),
    processing: true
  });
  
  try {
    const response = await likeCommentApi(commentId, newLiked);
    
    // 更新实际的点赞数
    commentLikeStates.value.set(commentId, {
      liked: newLiked,
      likeCount: response.data || Math.max(0, newLikeCount),
      processing: false
    });
    
    // 更新评论对象
    comment.isLiked = newLiked;
    comment.likes = response.data || Math.max(0, newLikeCount);
    
  } catch (error) {
    console.error('评论点赞失败:', error);
    
    // 回滚UI状态
    commentLikeStates.value.set(commentId, {
      liked: currentState.liked,
      likeCount: currentState.likeCount,
      processing: false
    });
    
    comment.isLiked = currentState.liked;
    comment.likes = currentState.likeCount;
    
    showToast('操作失败，请重试');
  }
};

// 回复评论
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

// 清除引用
const clearQuote = () => {
  quotedMessage.value = null;
  replyTo.value = null;
  commentContent.value = '';
};

// 取消评论
const cancelComment = () => {
  showCommentInput.value = false;
  clearQuote();
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
    clearQuote(); // 清除引用和内容
    showCommentInput.value = false;
    
    // 重新获取评论，重置到第一页显示最新评论
    currentPage.value = 1;
    sortType.value = 'latest'; // 切换到最新排序以看到刚发布的评论
    await getResourceComments();
  } catch (error) {
    console.error('提交评论失败', error);
    showToast('提交评论失败');
  }
};
</script>

<style scoped>
.resource-detail {
  background-color: var(--background-color);
  min-height: 100vh;
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
  padding: 16px;
  margin-bottom: 16px;
  background: #ffffff;
  border-radius: 12px;
  border: 1px solid #e8eaed;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  position: relative;
}

.comment-item:last-child {
  margin-bottom: 0;
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

.comment-action.processing {
  opacity: 0.6;
  pointer-events: none;
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
.like-click { 
  cursor: pointer; 
  transition: opacity 0.3s ease;
}

.like-click.processing {
  opacity: 0.6;
  cursor: not-allowed;
}

.like-click .van-loading__spinner {
  animation: van-rotate 1s linear infinite;
}
.ml8 { margin-left: 8px; }

/* 原回复样式已移除，使用新的reply-format样式 */

.comment-text, .reply-text {
  line-height: 1.6;
  color: var(--van-text-color, #323233);
}

/* 回复格式样式 */
.reply-format {
  line-height: 1.6;
}

.reply-format .reply-text {
  color: var(--van-text-color, #323233);
  margin-bottom: 4px;
}

.reply-format .original-message {
  padding: 8px 12px;
  background-color: #f7f8fa;
  border-radius: 8px;
  color: #666;
  font-size: 14px;
  line-height: 1.4;
  margin-top: 4px;
  border-left: 3px solid #ddd;
}

.reply-list {
  margin-top: 16px;
  margin-left: 16px;
  border-left: 3px solid var(--van-primary-color, #1989fa);
  padding-left: 16px;
  background: linear-gradient(90deg, rgba(25, 137, 250, 0.03) 0%, transparent 100%);
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
  background: var(--van-primary-color, #1989fa);
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
  border: 2px solid var(--van-primary-color, #1989fa);
  box-shadow: 0 2px 4px rgba(25, 137, 250, 0.2);
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

.reply-actions .comment-action {
  background: rgba(25, 137, 250, 0.1);
  padding: 4px 8px;
  border-radius: 12px;
  transition: all 0.2s ease;
}

.reply-actions .comment-action:hover {
  background: rgba(25, 137, 250, 0.2);
  transform: translateY(-1px);
}

/* 评论控制栏样式 */
.comment-controls-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  margin-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.comment-sort {
  display: flex;
  align-items: center;
  gap: 8px;
}

.sort-label {
  font-size: 14px;
  color: var(--text-color);
  font-weight: 500;
}

.comment-info {
  font-size: 12px;
  color: var(--text-color-lighter);
}

/* 分页样式 */
.comment-pagination {
  margin-top: 20px;
  padding: 16px 0;
  border-top: 1px solid var(--border-color);
  display: flex;
  justify-content: center;
}

.comment-pagination :deep(.van-pagination) {
  justify-content: center;
}

.comment-pagination :deep(.van-pagination__item) {
  margin: 0 4px;
  border-radius: 6px;
}

.comment-pagination :deep(.van-pagination__item--active) {
  background-color: var(--primary-color);
  color: white;
}

/* 评论输入框中的引用样式 */
.quoted-original {
  margin: 16px 0;
  padding: 12px;
  background: #f8f9fa;
  border-left: 4px solid #6c757d;
  border-radius: 0 8px 8px 0;
  position: relative;
}

.quoted-header {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
  gap: 6px;
  font-size: 13px;
  color: #6c757d;
  font-weight: 500;
}

.quoted-header-icon {
  font-size: 14px;
  color: #6c757d;
}

.close-quote {
  margin-left: auto;
  padding: 4px;
  cursor: pointer;
  font-size: 16px;
  color: #999;
  transition: color 0.3s ease;
}

.close-quote:hover {
  color: #ff4757;
}

.quoted-original-content {
  font-size: 14px;
  line-height: 1.4;
  color: #6c757d;
  font-style: italic;
  max-height: 80px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 4;
  -webkit-box-orient: vertical;
}

.quoted-original-content::before {
  content: '"';
  font-size: 16px;
  font-weight: bold;
  color: #999;
  margin-right: 2px;
}

.quoted-original-content::after {
  content: '"';
  font-size: 16px;
  font-weight: bold;
  color: #999;
  margin-left: 2px;
}
</style> 