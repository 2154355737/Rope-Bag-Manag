<template>
  <PageWrapper title="帖子管理" content="帖子列表与编辑">
    <Card class="!mb-4">
      <div ref="postChartRef" style="width: 100%; height: 260px"></div>
      <Space class="!mt-2">
        <Input v-model:value="search" placeholder="搜索标题关键词" style="width: 240px" />
        <Select v-model:value="statusFilter" :options="statusOptionsSelect" style="width: 140px" placeholder="状态" allowClear @change="fetchList" />
        <Button type="primary" @click="fetchList">搜索</Button>
        <Button v-if="canCreatePost" @click="handleCreate">新建帖子</Button>
      </Space>
    </Card>
    <Card :loading="loading">
      <Table :columns="columns" :data-source="dataSource" :pagination="{ pageSize: 20 }" row-key="id">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'status'">
            <Tag :color="statusColor(record.status)">{{ record.status }}</Tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <Space>
              <Button v-if="canEditPost(record)" type="link" @click="handleEdit(record)">编辑</Button>
              <Button v-if="canPublishPost(record)" type="link" @click="() => publishPost(record)">发布</Button>
              <Button v-if="canArchivePost(record)" type="link" @click="() => archivePost(record)">归档</Button>
              <Button v-if="canDeletePost(record)" type="link" danger @click="() => handleDelete(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>

    <Modal v-model:visible="editOpen" :title="currentId ? '编辑帖子' : '新建帖子'" :confirm-loading="submitLoading" @ok="handleSubmit" @cancel="() => (editOpen = false)" destroy-on-close width="720px">
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 16 }">
        <FormItem label="标题">
          <Input v-model:value="formState.title" placeholder="请输入标题" />
        </FormItem>
        <FormItem label="内容">
          <Input.TextArea v-model:value="formState.content" :rows="6" placeholder="请输入内容" />
        </FormItem>
        <FormItem label="分类">
          <Select v-model:value="formState.category_id" :options="categoryOptions" placeholder="请选择分类" />
        </FormItem>
        <FormItem label="标签">
          <Select v-model:value="formState.tags" mode="tags" placeholder="输入后回车添加标签" />
        </FormItem>
        <FormItem v-if="isAdminUser" label="作者">
          <Select v-model:value="formState.author_id" :options="userOptions" placeholder="选择作者" />
        </FormItem>
        <FormItem v-if="isAdminUser" label="置顶">
          <Switch v-model:checked="formState.is_pinned" />
        </FormItem>
        <FormItem v-if="isAdminUser" label="精选">
          <Switch v-model:checked="formState.is_featured" />
        </FormItem>
        <FormItem v-if="isAdminUser" label="状态">
          <Select v-model:value="formState.status" :options="statusOptions" />
        </FormItem>
      </Form>
    </Modal>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, nextTick, computed } from 'vue'
  import { Card, Table, Modal, Form, Input, Select, Switch, Button, Tag, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getPosts, createPost, updatePost, deletePost, type PostItem } from '/@/api/posts'
  import { getCategories } from '/@/api/dashboard'
  import { getUsers, type UserItem } from '/@/api/users'
  import { useUserStore } from '/@/store/modules/user'
  import * as echarts from 'echarts'

  const FormItem = Form.Item
  const userStore = useUserStore()

  const loading = ref(true)
  const dataSource = ref<PostItem[]>([])
  const categoryOptions = ref<any[]>([])
  const userOptions = ref<{ label: string; value: number }[]>([])
  const search = ref('')
  const statusFilter = ref<string | undefined>(undefined)

  // 权限计算
  const currentUser = computed(() => userStore.getUserInfo)
  const isAdminUser = computed(() => {
    const userInfo = currentUser.value as any
    const role = userInfo?.role || userInfo?.roles?.[0]?.value
    return role === 'admin' || role === 'elder'
  })

  // 权限检查函数
  const canCreatePost = computed(() => {
    return !!currentUser.value?.userId // 已登录用户都可以创建帖子
  })

  const canEditPost = (record: PostItem) => {
    if (!currentUser.value?.userId) return false
    return isAdminUser.value || record.author_id === currentUser.value.userId
  }

  const canDeletePost = (record: PostItem) => {
    if (!currentUser.value?.userId) return false
    return isAdminUser.value || record.author_id === currentUser.value.userId
  }

  const canPublishPost = (record: PostItem) => {
    if (!isAdminUser.value) return false
    return record.status !== 'Published'
  }

  const canArchivePost = (record: PostItem) => {
    if (!isAdminUser.value) return false
    return record.status !== 'Archived'
  }

  const statusOptionsSelect = [
    { label: '全部', value: '' },
    { label: '发布', value: 'Published' },
    { label: '草稿', value: 'Draft' },
    { label: '归档', value: 'Archived' },
  ]

  const postChartRef = ref<HTMLDivElement | null>(null)
  let postChart: echarts.ECharts | null = null

  const renderPostChart = () => {
    if (!postChartRef.value) return
    if (!postChart) postChart = echarts.init(postChartRef.value as HTMLDivElement)
    const labels = dataSource.value.map((p) => p.title)
    const views = dataSource.value.map((p: any) => p.view_count || 0)
    const likes = dataSource.value.map((p: any) => p.like_count || 0)
    postChart.setOption({
      tooltip: { trigger: 'axis' },
      legend: { bottom: 0, data: ['访问量', '点赞量'] },
      grid: { left: 40, right: 10, top: 10, bottom: 50 },
      xAxis: { type: 'category', data: labels },
      yAxis: { type: 'value' },
      series: [
        { name: '访问量', type: 'bar', data: views },
        { name: '点赞量', type: 'bar', data: likes },
      ],
    })
  }

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '标题', dataIndex: 'title', key: 'title' },
    { title: '作者ID', dataIndex: 'author_id', key: 'author_id', width: 100 },
    { title: '访问量', dataIndex: 'view_count', key: 'view_count', width: 100 },
    { title: '点赞量', dataIndex: 'like_count', key: 'like_count', width: 100 },
    { title: '状态', dataIndex: 'status', key: 'status', width: 100 },
    { title: '置顶', dataIndex: 'is_pinned', key: 'is_pinned', width: 80 },
    { title: '精选', dataIndex: 'is_featured', key: 'is_featured', width: 80 },
    { title: '操作', key: 'action', width: 250 },
  ]

  const statusOptions = [
    { label: '草稿', value: 'Draft' },
    { label: '发布', value: 'Published' },
    { label: '归档', value: 'Archived' },
  ]

  const statusColor = (s: string) => ({ Published: 'success', Draft: 'warning', Archived: 'default', Deleted: 'default' }[s] || 'default')

  const editOpen = ref(false)
  const submitLoading = ref(false)
  const currentId = ref<number | null>(null)
  const formState = ref<any>({ title: '', content: '', category_id: undefined, tags: [], is_pinned: false, is_featured: false, status: 'Draft' })

  const handleCreate = () => {
    currentId.value = null
    formState.value = { title: '', content: '', category_id: undefined, tags: [], author_id: undefined, is_pinned: false, is_featured: false, status: 'Draft' }
    editOpen.value = true
  }

  const handleEdit = (record: PostItem) => {
    currentId.value = record.id
    formState.value = { ...record, tags: (record as any).tags || [], author_id: record.author_id }
    editOpen.value = true
  }

  const publishPost = async (record: PostItem) => {
    await updatePost(record.id, { status: 'Published' as any })
    await fetchList()
  }

  const archivePost = async (record: PostItem) => {
    await updatePost(record.id, { status: 'Archived' as any })
    await fetchList()
  }

  const handleDelete = async (record: PostItem) => {
    await deletePost(record.id)
    await fetchList()
  }

  const handleSubmit = async () => {
    submitLoading.value = true
    try {
      if (currentId.value) {
        await updatePost(currentId.value, { ...formState.value })
      } else {
        await createPost({ ...formState.value })
      }
      editOpen.value = false
      await fetchList()
    } finally {
      submitLoading.value = false
    }
  }

  const fetchList = async () => {
    try {
      loading.value = true
      const params: any = { page: 1, page_size: 20, search: search.value || undefined }
      if (statusFilter.value) params.status = statusFilter.value
      const [list, cats, users] = await Promise.all([
        getPosts(params),
        getCategories(),
        isAdminUser.value ? getUsers({ page: 1, page_size: 100 }) : Promise.resolve({ list: [] }),
      ])
      dataSource.value = list.list || []
      categoryOptions.value = (cats || []).map((c: any) => ({ label: c.name, value: c.id }))
      userOptions.value = (users.list || []).map((u: UserItem) => ({ label: u.username + (u.nickname ? ` (${u.nickname})` : ''), value: u.id }))
      await nextTick()
      renderPostChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 