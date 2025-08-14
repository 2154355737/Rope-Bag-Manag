<template>
  <PageWrapper title="帖子管理" content="帖子列表与编辑">
    <Card class="!mb-4">
      <div ref="postChartRef" style="width: 100%; height: 260px"></div>
      <Space class="!mt-2">
        <Input v-model:value="search" placeholder="搜索标题关键词" style="width: 240px" />
        <Select v-model:value="statusFilter" :options="statusOptionsSelect" style="width: 140px" placeholder="状态" allowClear @change="fetchList" />
        <Button type="primary" @click="fetchList">搜索</Button>
        <Button @click="handleCreate">新建帖子</Button>
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
              <Button type="link" @click="handleEdit(record)">编辑</Button>
              <Button type="link" @click="() => publishPost(record)" v-if="record.status !== 'Published'">发布</Button>
              <Button type="link" @click="() => archivePost(record)" v-if="record.status !== 'Archived'">归档</Button>
              <Button type="link" danger @click="() => handleDelete(record)">删除</Button>
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
        <FormItem label="置顶">
          <Switch v-model:checked="formState.is_pinned" />
        </FormItem>
        <FormItem label="精选">
          <Switch v-model:checked="formState.is_featured" />
        </FormItem>
        <FormItem label="状态">
          <Select v-model:value="formState.status" :options="statusOptions" />
        </FormItem>
      </Form>
    </Modal>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, nextTick } from 'vue'
  import { Card, Table, Modal, Form, Input, Select, Switch, Button, Tag, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getPosts, createPost, updatePost, deletePost, type PostItem } from '/@/api/posts'
  import { getCategories } from '/@/api/dashboard'
  import * as echarts from 'echarts'

  const FormItem = Form.Item

  const loading = ref(true)
  const dataSource = ref<PostItem[]>([])
  const categoryOptions = ref<any[]>([])
  const search = ref('')
  const statusFilter = ref<string | undefined>(undefined)

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
    { title: '访问量', dataIndex: 'view_count', key: 'view_count' },
    { title: '点赞量', dataIndex: 'like_count', key: 'like_count' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '置顶', dataIndex: 'is_pinned', key: 'is_pinned' },
    { title: '精选', dataIndex: 'is_featured', key: 'is_featured' },
    { title: '操作', key: 'action', width: 200 },
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
    formState.value = { title: '', content: '', category_id: undefined, tags: [], is_pinned: false, is_featured: false, status: 'Draft' }
    editOpen.value = true
  }

  const handleEdit = (record: PostItem) => {
    currentId.value = record.id
    formState.value = { ...record, tags: (record as any).tags || [] }
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
      const [list, cats] = await Promise.all([
        getPosts(params),
        getCategories(),
      ])
      dataSource.value = list.list || []
      categoryOptions.value = (cats || []).map((c: any) => ({ label: c.name, value: c.id }))
      await nextTick()
      renderPostChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 