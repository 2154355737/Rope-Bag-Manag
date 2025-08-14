<template>
  <PageWrapper title="评论管理" content="评论列表与状态管理">
    <Card class="!mb-4">
      <Space>
        <Input v-model:value="search" placeholder="搜索内容/用户ID" style="width: 240px" />
        <Select v-model:value="status" :options="statusOptions" style="width: 160px" />
        <Button type="primary" @click="fetchList">搜索</Button>
        <Button danger :disabled="!selectedRowKeys.length" @click="batchDelete">批量删除</Button>
        <Button :disabled="!selectedRowKeys.length" @click="() => batchStatus('Hidden')">设为隐藏</Button>
        <Button :disabled="!selectedRowKeys.length" @click="() => batchStatus('Active')">设为显示</Button>
      </Space>
      <div ref="cmtChartRef" style="width: 100%; height: 220px" class="mt-2"></div>
    </Card>
    <Card :loading="loading">
      <Table :columns="columns" :data-source="dataSource" :row-selection="rowSelection" row-key="id" :pagination="{ pageSize: 20 }">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'status'">
            <Tag :color="statusColor(record.status)">{{ record.status }}</Tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <Space>
              <Button type="link" @click="() => setStatus(record, 'Hidden')">隐藏</Button>
              <Button type="link" @click="() => setStatus(record, 'Active')">显示</Button>
              <Button type="link" danger @click="() => remove(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, computed, onMounted, nextTick } from 'vue'
  import { Card, Table, Button, Space, Input, Select, Tag } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getComments, deleteComment, deleteCommentsBatch, updateCommentsStatus, type CommentItem } from '/@/api/comments'
  import * as echarts from 'echarts'

  const loading = ref(true)
  const dataSource = ref<CommentItem[]>([])
  const search = ref('')
  const status = ref<string | undefined>(undefined)
  const selectedRowKeys = ref<number[]>([])

  const statusOptions = [
    { label: '全部', value: undefined },
    { label: '显示', value: 'Active' },
    { label: '隐藏', value: 'Hidden' },
    { label: '删除', value: 'Deleted' },
  ]

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '用户', dataIndex: 'user_id', key: 'user_id' },
    { title: '目标类型', dataIndex: 'target_type', key: 'target_type' },
    { title: '目标ID', dataIndex: 'target_id', key: 'target_id' },
    { title: '内容', dataIndex: 'content', key: 'content' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '时间', dataIndex: 'created_at', key: 'created_at' },
    { title: '操作', key: 'action', width: 220 },
  ]

  const statusColor = (s: string) => ({ Active: 'success', Hidden: 'warning', Deleted: 'default' }[s] || 'default')

  const rowSelection = computed(() => ({
    selectedRowKeys: selectedRowKeys.value,
    onChange: (keys: any[]) => (selectedRowKeys.value = keys as number[]),
  }))

  const cmtChartRef = ref<HTMLDivElement | null>(null)
  let cmtChart: echarts.ECharts | null = null
  const renderChart = () => {
    if (!cmtChartRef.value) return
    if (!cmtChart) cmtChart = echarts.init(cmtChartRef.value as HTMLDivElement)
    const map: Record<string, number> = { Active: 0, Hidden: 0, Deleted: 0 }
    dataSource.value.forEach((c) => { map[c.status] = (map[c.status] || 0) + 1 })
    cmtChart.setOption({
      tooltip: { trigger: 'item' },
      legend: { bottom: 0 },
      series: [{ type: 'pie', radius: '60%', data: Object.keys(map).map((k) => ({ name: k, value: map[k] })) }],
    })
  }

  const setStatus = async (row: CommentItem, s: 'Active' | 'Hidden' | 'Deleted') => {
    await updateCommentsStatus([row.id], s)
    await fetchList()
  }

  const remove = async (row: CommentItem) => {
    await deleteComment(row.id)
    await fetchList()
  }

  const batchStatus = async (s: 'Active' | 'Hidden' | 'Deleted') => {
    await updateCommentsStatus(selectedRowKeys.value, s)
    selectedRowKeys.value = []
    await fetchList()
  }

  const batchDelete = async () => {
    await deleteCommentsBatch(selectedRowKeys.value)
    selectedRowKeys.value = []
    await fetchList()
  }

  const fetchList = async () => {
    try {
      loading.value = true
      const res = await getComments({ page: 1, size: 20, status: status.value, search: search.value || undefined })
      dataSource.value = res.list || []
      await nextTick(); renderChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 