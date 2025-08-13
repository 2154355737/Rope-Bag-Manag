<template>
  <PageWrapper title="内容审核中心" content="审核资源与帖子状态">
    <Card class="!mb-4">
      <template #title>待审概览</template>
      <div ref="modChartRef" style="width: 100%; height: 220px"></div>
    </Card>
    <div class="md:flex md:space-x-4">
      <Card class="md:w-1/2 w-full !mb-4" :loading="loadingPackages">
        <template #title>资源待审</template>
        <Table :columns="packageColumns" :data-source="packages" row-key="id" :pagination="{ pageSize: 20 }">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'status'">
              <Tag :color="packageStatusColor(record.status)">{{ record.status }}</Tag>
            </template>
            <template v-else-if="column.key === 'action'">
              <Space>
                <Button type="link" @click="() => approvePackage(record)">通过</Button>
                <Button type="link" danger @click="() => rejectPackage(record)">驳回</Button>
              </Space>
            </template>
          </template>
        </Table>
      </Card>

      <Card class="md:w-1/2 w-full !mb-4" :loading="loadingPosts">
        <template #title>帖子审核</template>
        <Table :columns="postColumns" :data-source="posts" row-key="id" :pagination="{ pageSize: 20 }">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'status'">
              <Tag :color="postStatusColor(record.status)">{{ record.status }}</Tag>
            </template>
            <template v-else-if="column.key === 'action'">
              <Space>
                <Button type="link" @click="() => publishPost(record)">发布</Button>
                <Button type="link" @click="() => archivePost(record)">归档</Button>
                <Button type="link" danger @click="() => removePost(record)">删除</Button>
              </Space>
            </template>
          </template>
        </Table>
      </Card>
    </div>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, nextTick } from 'vue'
  import { Card, Table, Button, Space, Tag } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getPackages, reviewPackage, type PackageItem } from '/@/api/packages'
  import { getPosts, updatePost, deletePost, type PostItem } from '/@/api/posts'
  import * as echarts from 'echarts'

  const loadingPackages = ref(true)
  const loadingPosts = ref(true)
  const packages = ref<PackageItem[]>([])
  const posts = ref<PostItem[]>([])

  const modChartRef = ref<HTMLDivElement | null>(null)
  let modChart: echarts.ECharts | null = null
  const renderModChart = () => {
    if (!modChartRef.value) return
    if (!modChart) modChart = echarts.init(modChartRef.value as HTMLDivElement)
    const labels = ['资源待审', '帖子草稿']
    const values = [packages.value.length, posts.value.length]
    modChart.setOption({
      tooltip: { trigger: 'axis' },
      grid: { left: 40, right: 10, top: 10, bottom: 30 },
      xAxis: { type: 'category', data: labels },
      yAxis: { type: 'value' },
      series: [{ type: 'bar', data: values }],
    })
  }

  const packageColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '标题', dataIndex: 'name', key: 'name' },
    { title: '作者', dataIndex: 'author', key: 'author' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '操作', key: 'action', width: 160 },
  ]

  const postColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '标题', dataIndex: 'title', key: 'title' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '操作', key: 'action', width: 200 },
  ]

  const packageStatusColor = (s: string) => ({ Active: 'success', Pending: 'warning', Rejected: 'error', Inactive: 'default', Deleted: 'default' }[s] || 'default')
  const postStatusColor = (s: string) => ({ Published: 'success', Draft: 'warning', Archived: 'default', Deleted: 'default' }[s] || 'default')

  const fetchPackages = async () => {
    try {
      loadingPackages.value = true
      const res = await getPackages({ page: 1, page_size: 20, status: 'Pending' })
      packages.value = res.list || []
    } finally {
      loadingPackages.value = false
    }
  }

  const fetchPosts = async () => {
    try {
      loadingPosts.value = true
      // 这里将草稿视为待审核，也可按需调整筛选条件
      const res = await getPosts({ page: 1, page_size: 20, status: 'Draft' })
      posts.value = res.list || []
    } finally {
      loadingPosts.value = false
    }
  }

  const approvePackage = async (row: PackageItem) => {
    await reviewPackage(row.id, { status: 'Active', review_comment: '' })
    await fetchPackages()
    await nextTick(); renderModChart()
  }

  const rejectPackage = async (row: PackageItem) => {
    await reviewPackage(row.id, { status: 'Rejected', review_comment: '' })
    await fetchPackages()
    await nextTick(); renderModChart()
  }

  const publishPost = async (row: PostItem) => {
    await updatePost(row.id, { status: 'Published' as any })
    await fetchPosts()
    await nextTick(); renderModChart()
  }

  const archivePost = async (row: PostItem) => {
    await updatePost(row.id, { status: 'Archived' as any })
    await fetchPosts()
    await nextTick(); renderModChart()
  }

  const removePost = async (row: PostItem) => {
    await deletePost(row.id)
    await fetchPosts()
    await nextTick(); renderModChart()
  }

  onMounted(async () => {
    await Promise.all([fetchPackages(), fetchPosts()])
    await nextTick(); renderModChart()
  })
</script> 