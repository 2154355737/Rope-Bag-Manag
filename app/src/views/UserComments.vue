<template>
  <div class="user-comments-page">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      title="我的评论"
      left-arrow
      @click-left="onBack"
      fixed
      placeholder
    />
    
    <div class="content page-with-fixed-navbar">
      <!-- 评论列表 -->
      <van-list
        v-model:loading="loading"
        :finished="finished"
        finished-text="没有更多评论了"
        @load="loadComments"
      >
        <div v-if="comments.length > 0">
          <div class="comment-item" v-for="comment in comments" :key="comment.id">
            <div class="comment-resource">
              <div class="resource-info" @click="goToResource(comment.resource_id)">
                <div class="resource-title ellipsis">{{ comment.resource_name }}</div>
                <div class="comment-time">{{ formatDate(comment.created_at) }}</div>
              </div>
            </div>
            
            <div class="comment-content">{{ comment.content }}</div>
            
            <div class="comment-actions">
              <div class="comment-action" @click="likeComment(comment)">
                <van-icon name="like-o" :class="{ 'liked': comment.isLiked }" />
                <span>{{ comment.like_count || 0 }}</span>
              </div>
              <div class="comment-action" @click="editComment(comment)">
                <van-icon name="edit" />
                <span>编辑</span>
              </div>
              <div class="comment-action" @click="confirmDeleteComment(comment)">
                <van-icon name="delete" />
                <span>删除</span>
              </div>
            </div>
          </div>
        </div>
        
        <div v-else-if="!loading" class="empty-comments">
          <van-empty description="暂无评论" />
        </div>
      </van-list>
    </div>
    
    <!-- 编辑评论弹窗 -->
    <van-action-sheet v-model:show="showEditPopup" title="编辑评论">
      <div class="edit-form">
        <van-field
          v-model="editForm.content"
          rows="3"
          type="textarea"
          maxlength="500"
          placeholder="请输入评论内容"
          show-word-limit
        />
        <div class="edit-actions">
          <van-button plain @click="cancelEdit">取消</van-button>
          <van-button 
            type="primary" 
            @click="submitEdit"
            :disabled="!editForm.content.trim()"
            :loading="submitting"
          >
            保存
          </van-button>
        </div>
      </div>
    </van-action-sheet>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { showToast, showDialog } from 'vant';
import { useUserStore } from '../store/user';
import dayjs from 'dayjs';
import { get, put, del } from '../utils/request';

const router = useRouter();
const userStore = useUserStore();

// 评论数据
const comments = ref([]);
const loading = ref(false);
const finished = ref(false);
const page = ref(1);
const pageSize = 10;

// 编辑相关
const showEditPopup = ref(false);
const editForm = ref({
  id: null,
  content: ''
});
const submitting = ref(false);
const currentComment = ref(null);

// 格式化日期
const formatDate = (date) => {
  if (!date) return '';
  return dayjs(date).format('YYYY-MM-DD HH:mm');
};

// 返回上一页
const onBack = () => {
  router.back();
};

// 跳转到资源详情页
const goToResource = (resourceId) => {
  router.push(`/resource/${resourceId}`);
};

// 加载评论
const loadComments = async () => {
  if (loading.value) return;
  
  loading.value = true;
  try {
    // 这里应该调用获取用户评论的接口
    const data = await get('/users/my-comments', { page: page.value, page_size: pageSize });
    
    if (data.code === 0) {
      const newComments = data.data.list || [];
      
      if (page.value === 1) {
        comments.value = newComments;
      } else {
        comments.value = [...comments.value, ...newComments];
      }
      
      // 判断是否加载完毕
      if (newComments.length < pageSize) {
        finished.value = true;
      } else {
        page.value += 1;
      }
    } else {
      showToast(data.message || '获取评论失败');
      finished.value = true;
    }
  } catch (error) {
    console.error('获取评论失败', error);
    showToast('获取评论失败');
    finished.value = true;
  } finally {
    loading.value = false;
  }
};

// 点赞评论
const likeComment = (comment) => {
  // TODO: 调用点赞接口
  showToast('点赞成功');
  comment.like_count = (comment.like_count || 0) + 1;
  comment.isLiked = true;
};

// 编辑评论
const editComment = (comment) => {
  currentComment.value = comment;
  editForm.value.id = comment.id;
  editForm.value.content = comment.content;
  showEditPopup.value = true;
};

// 取消编辑
const cancelEdit = () => {
  showEditPopup.value = false;
  currentComment.value = null;
  editForm.value = {
    id: null,
    content: ''
  };
};

// 提交编辑
const submitEdit = async () => {
  if (!editForm.value.content.trim()) {
    showToast('评论内容不能为空');
    return;
  }
  
  if (!currentComment.value) return;
  
  submitting.value = true;
  try {
    // 这里应该调用更新评论的接口
    const data = await put(`/comments/${editForm.value.id}`, {
        content: editForm.value.content
    });
    if (data.code === 0) {
      showToast('修改成功');
      
      // 更新本地数据
      const index = comments.value.findIndex(item => item.id === editForm.value.id);
      if (index !== -1) {
        comments.value[index].content = editForm.value.content;
      }
      
      cancelEdit();
    } else {
      showToast(data.message || '修改失败');
    }
  } catch (error) {
    console.error('修改评论失败', error);
    showToast('修改失败');
  } finally {
    submitting.value = false;
  }
};

// 确认删除评论
const confirmDeleteComment = (comment) => {
  showDialog({
    title: '确认删除',
    message: '确定要删除此评论吗？',
    showCancelButton: true
  }).then((action) => {
    if (action === 'confirm') {
      deleteComment(comment.id);
    }
  });
};

// 删除评论
const deleteComment = async (commentId) => {
  try {
    // 这里应该调用删除评论的接口
    const data = await del(`/comments/${commentId}`);
    if (data.code === 0) {
      showToast('删除成功');
      
      // 更新本地数据
      comments.value = comments.value.filter(item => item.id !== commentId);
    } else {
      showToast(data.message || '删除失败');
    }
  } catch (error) {
    console.error('删除评论失败', error);
    showToast('删除失败');
  }
};

// 页面加载时检查登录状态并加载评论
onMounted(() => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    router.replace({
      path: '/login',
      query: { redirect: '/my-comments' }
    });
    return;
  }
  
  loadComments();
});
</script>

<style scoped>
.user-comments-page {
  min-height: 100vh;
  background-color: var(--background-color);
}

.content {
  padding: 12px;
}

.page-with-fixed-navbar {
  padding-top: 8px !important;
}

.comment-item {
  background-color: #fff;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 12px;
}

.comment-resource {
  display: flex;
  margin-bottom: 12px;
}

.resource-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.resource-title {
  font-size: 16px;
  font-weight: 500;
  color: var(--text-color);
}

.comment-time {
  font-size: 12px;
  color: var(--text-color-lighter);
}

.comment-content {
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-color);
  margin-bottom: 12px;
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

.empty-comments {
  padding: 20px 0;
}

.edit-form {
  padding: 16px;
}

.edit-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 16px;
  gap: 12px;
}
</style> 