<template>
  <div class="user-resources-page">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      title="我的资源"
      left-arrow
      @click-left="onBack"
      fixed
      placeholder
    />
    
    <div class="content page-with-fixed-navbar">
      <!-- 操作栏 -->
      <div class="action-bar">
        <van-button 
          type="primary" 
          size="small" 
          icon="plus" 
          to="/submit"
        >
          上传资源
        </van-button>
        <van-dropdown-menu>
          <van-dropdown-item v-model="filterValue" :options="filterOptions" />
          <van-dropdown-item v-model="sortValue" :options="sortOptions" />
        </van-dropdown-menu>
      </div>
      
      <!-- 资源列表 -->
      <div class="resource-list-container">
        <resource-list
          :resources="resources"
          :loading="loading"
          :finished="finished"
          :emptyText="'暂无上传资源'"
          :showActions="true"
          @load-more="loadMore"
          @more="onCardMore"
          @resource-click="openResource"
        />
      </div>
      
      <!-- 加载更多 -->
      <div class="load-more" v-if="resources.length > 0 && !finished">
        <van-button 
          plain 
          size="small" 
          @click="loadMore" 
          :loading="loadingMore"
        >
          加载更多
        </van-button>
      </div>
      
      <!-- 批量操作浮动按钮 -->
      <van-action-bar v-if="isEditing">
        <van-action-bar-button
          type="warning"
          text="全选"
          @click="selectAll"
        />
        <van-action-bar-button
          type="danger"
          text="删除"
          :disabled="!hasSelected"
          @click="confirmDelete"
        />
        <van-action-bar-button
          type="default"
          text="取消"
          @click="cancelEdit"
        />
      </van-action-bar>
      
      <van-button
        v-else
        type="primary"
        icon="edit"
        class="fab"
        @click="startEdit"
      />

      <!-- 单项操作单 -->
      <van-action-sheet
        v-model:show="showActionSheet"
        :actions="actionItems"
        cancel-text="取消"
        close-on-click-action
        @select="onActionSelect"
      />

      <!-- 编辑弹层 -->
      <resource-edit-sheet
        v-model="showEditSheet"
        :resource="currentResource"
        :saving="editSaving"
        @submit="submitEdit"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch, onActivated } from 'vue';
import { useRouter } from 'vue-router';
import { showToast, showDialog } from 'vant';
import { useUserStore } from '../store/user';
import { resourceApi } from '../api/resource';
import ResourceList from '../components/ResourceList.vue';
import ResourceEditSheet from '../components/ResourceEditSheet.vue';

const router = useRouter();
const userStore = useUserStore();

// 资源列表数据
const resources = ref([]);
const loading = ref(false);
const loadingMore = ref(false);
const finished = ref(false);
const page = ref(1);
const pageSize = 10;

// 筛选和排序
const filterValue = ref('all');
const filterOptions = [
  { text: '全部资源', value: 'all' },
  { text: '已审核', value: 'approved' },
  { text: '待审核', value: 'pending' },
  { text: '已拒绝', value: 'rejected' },
];

const sortValue = ref('newest');
const sortOptions = [
  { text: '最新上传', value: 'newest' },
  { text: '最多下载', value: 'downloads' },
  { text: '最多点赞', value: 'likes' },
];

// 编辑模式
const isEditing = ref(false);
const selectedResources = ref([]);

// 单项操作状态
const showActionSheet = ref(false);
const currentResource = ref(null);
const showEditSheet = ref(false);
const editSaving = ref(false);
const deleteLoading = ref(false);

const actionItems = [
  { name: '编辑', action: 'edit' },
  { name: '删除', action: 'delete', color: '#ee0a24' },
];

// 返回上一页
const onBack = () => {
  router.back();
};

// 是否有选中项
const hasSelected = computed(() => selectedResources.value.length > 0);

// 加载用户资源
const loadResources = async (isLoadMore = false) => {
  if (loading.value && !isLoadMore) return;
  if (loadingMore.value && isLoadMore) return;
  
  if (isLoadMore) {
    loadingMore.value = true;
  } else {
    loading.value = true;
  }
  
  try {
    const params = {
      page: page.value,
      page_size: pageSize,
    };
    
    // 添加筛选条件
    if (filterValue.value !== 'all') {
      params.status = filterValue.value;
    }
    
    // 添加排序条件（本地排序，参数仅用于本地）
    const res = await resourceApi.getUserResources(params);
    const newResources = res.data.list || [];

    // 前端本地排序：先按置顶和精华，再按用户选择的排序方式
    const sorted = [...newResources].sort((a, b) => {
      // 置顶资源优先
      if (a.is_pinned && !b.is_pinned) return -1;
      if (!a.is_pinned && b.is_pinned) return 1;
      
      // 精华资源次之
      if (a.is_featured && !b.is_featured) return -1;
      if (!a.is_featured && b.is_featured) return 1;
      
      // 在相同优先级内按用户选择的方式排序
      if (sortValue.value === 'downloads') {
        return (b.download_count || 0) - (a.download_count || 0);
      } else if (sortValue.value === 'likes') {
        return (b.like_count || 0) - (a.like_count || 0);
      } else {
        return new Date(b.created_at) - new Date(a.created_at);
      }
    });

    if (page.value === 1) {
      resources.value = sorted;
    } else {
      resources.value = [...resources.value, ...sorted];
    }
    
    // 判断是否加载完毕
    if (newResources.length < pageSize) {
      finished.value = true;
    } else {
      page.value += 1;
    }
  } catch (error) {
    console.error('获取用户资源失败', error);
    showToast('获取资源失败');
  } finally {
    loading.value = false;
    loadingMore.value = false;
  }
};

// 加载更多
const loadMore = () => {
  if (finished.value) return;
  loadResources(true);
};

// 开始编辑
const startEdit = () => {
  isEditing.value = true;
};

// 取消编辑
const cancelEdit = () => {
  isEditing.value = false;
  selectedResources.value = [];
};

// 全选
const selectAll = () => {
  if (selectedResources.value.length === resources.value.length) {
    // 如果已经全选，则取消全选
    selectedResources.value = [];
  } else {
    // 全选
    selectedResources.value = resources.value.map(item => item.id);
  }
};

// 确认删除（批量）
const confirmDelete = () => {
  if (!hasSelected.value) return;
  
  showDialog({
    title: '确认删除',
    message: `确定要删除选中的 ${selectedResources.value.length} 个资源吗？此操作不可恢复。`,
    showCancelButton: true,
  }).then((action) => {
    if (action === 'confirm') {
      deleteResources();
    }
  });
};

// 删除资源（批量）
const deleteResources = async () => {
  try {
    // 调用批量删除接口
    await Promise.all(
      selectedResources.value.map(id => resourceApi.deleteResource(id))
    );
    
    showToast('删除成功');
    
    // 重新加载资源
    page.value = 1;
    finished.value = false;
    loadResources();
    
    // 退出编辑模式
    cancelEdit();
  } catch (error) {
    console.error('删除资源失败', error);
    showToast('删除失败');
  }
};

// 卡片更多
const onCardMore = (resource) => {
  currentResource.value = resource;
  showActionSheet.value = true;
};

const onActionSelect = (action) => {
  if (!currentResource.value) return;
  if (action.action === 'edit') {
    showActionSheet.value = false;
    showEditSheet.value = true;
  } else if (action.action === 'delete') {
    showActionSheet.value = false;
    confirmDeleteOne();
  }
};

const confirmDeleteOne = () => {
  if (!currentResource.value) return;
  showDialog({
    title: '确认删除',
    message: `确定要删除「${currentResource.value.name}」吗？此操作不可恢复。`,
    showCancelButton: true,
  }).then((action) => {
    if (action === 'confirm') {
      deleteOne();
    }
  });
};

const deleteOne = async () => {
  if (!currentResource.value) return;
  try {
    deleteLoading.value = true;
    await resourceApi.deleteResource(currentResource.value.id);
    showToast('删除成功');
    // 重新加载第一页
    page.value = 1;
    finished.value = false;
    await loadResources();
  } catch (e) {
    console.error('删除失败', e);
    showToast('删除失败');
  } finally {
    deleteLoading.value = false;
  }
};

const submitEdit = async (form) => {
  if (!currentResource.value) return;
  try {
    editSaving.value = true;
    const payload = {
      name: form.name,
      description: form.description,
      category_id: form.category_id || undefined,
      tags: Array.isArray(form.tags) ? form.tags : undefined,
      file_url: form.file_url || undefined,
    };
    // 先更新基础字段
    const res = await resourceApi.updateResource(currentResource.value.id, payload);
    if (res.code !== 0) throw new Error(res.message || '保存失败');

    // 如果选择了文件，则调用上传接口覆盖文件
    if (form.__file) {
      const fd = new FormData();
      fd.append('file', form.__file);
      const up = await resourceApi.uploadFile(currentResource.value.id, fd);
      if (up.code === 0) {
        // 覆盖前端的 file_url
        const path = up.data?.file_path || up.data || '';
        if (path) {
          payload.file_url = path;
        }
      } else {
        throw new Error(up.message || '文件上传失败');
      }
    }

    showToast('保存成功');
    showEditSheet.value = false;
    // 更新本地列表项
    const idx = resources.value.findIndex(r => r.id === currentResource.value.id);
    if (idx !== -1) {
      // 合并更新（优先使用后端回包，其次使用本地payload）
      const merged = { ...resources.value[idx], ...res.data };
      if (payload.tags) merged.tags = payload.tags;
      if (payload.category_id !== undefined) merged.category_id = payload.category_id;
      if (payload.file_url) merged.file_url = payload.file_url;
      resources.value[idx] = merged;
    }
  } catch (e) {
    console.error('保存失败', e);
    showToast(e?.message || '保存失败');
  } finally {
    editSaving.value = false;
  }
};

// 打开资源详情
const openResource = (resource) => {
  if (!resource || !resource.id) return;
  router.push(`/resource/${resource.id}`);
};

// 监听筛选和排序变化
watch([filterValue, sortValue], () => {
  page.value = 1;
  finished.value = false;
  loadResources();
});

// 用户切换时重载
watch(() => userStore.userId, async (n, o) => {
  page.value = 1;
  finished.value = false;
  resources.value = [];
  if (!userStore.isLoggedIn) {
    router.replace({ path: '/login', query: { redirect: '/my-resources' } });
    return;
  }
  await loadResources();
});

onActivated(() => {
  // 返回该页面时，如需可选择刷新
  // if (resources.value.length === 0) loadResources();
});

// 页面加载时获取资源
onMounted(() => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    router.replace({
      path: '/login',
      query: { redirect: '/my-resources' }
    });
    return;
  }
  loadResources();
});
</script>

<style scoped>
.user-resources-page {
  min-height: 100vh;
  background-color: var(--background-color);
}

.content {
  padding: 16px;
}
.page-with-fixed-navbar { padding-top: 8px !important; }

.action-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.resource-list-container {
  margin-bottom: 20px;
}

.load-more {
  text-align: center;
  margin-top: 16px;
}

.fab {
  position: fixed;
  right: 16px;
  bottom: 80px;
  z-index: 1000;
  border-radius: 24px;
}
</style> 