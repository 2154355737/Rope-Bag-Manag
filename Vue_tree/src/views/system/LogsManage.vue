<template>
  <PageWrapper title="系统日志" content="系统运行日志列表">
    <Card class="!mb-4">
      <Form layout="inline" @submit.prevent>
        <FormItem label="级别">
          <Select v-model:value="filters.level" style="width: 140px" :options="levelOptions" allowClear />
        </FormItem>
        <FormItem label="关键字">
          <Input v-model:value="filters.search" placeholder="message/details" style="width: 220px" allowClear />
        </FormItem>
        <FormItem label="时间范围">
          <RangePicker v-model:value="filters.range" show-time style="width: 360px" />
        </FormItem>
        <FormItem>
          <Space>
            <Button type="primary" @click="onSearch">查询</Button>
            <Button @click="onReset">重置</Button>
            <Button danger @click="onClear" :loading="opLoading">清空日志</Button>
          </Space>
        </FormItem>
      </Form>
    </Card>

    <Card :loading="loading" class="!mb-4">
      <div ref="logChartRef" style="width: 100%; height: 220px"></div>
    </Card>
    <Card :loading="loading">
      <Table
        :columns="columns"
        :data-source="dataSource"
        :pagination="pagination"
        row-key="id"
        :row-selection="rowSelection"
        @change="onTableChange"
      >
        <template #title>
          <Space>
            <Button danger :disabled="!selectedRowKeys.length" @click="onBatchDelete" :loading="opLoading">批量删除</Button>
          </Space>
        </template>
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'action'">
            <Space>
              <Button type="link" danger @click="() => handleDelete(record)" :loading="opLoading">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, nextTick, computed } from 'vue'
  import { Card, Table, Button, Space, Form, Input, Select, DatePicker, message } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getSystemLogs, type SystemLog, deleteLog, batchDeleteLogs, clearLogs } from '/@/api/dashboard'
  import * as echarts from 'echarts'
  import dayjs, { Dayjs } from 'dayjs'

  const { RangePicker } = DatePicker
  const FormItem = Form.Item

  const loading = ref(true)
  const opLoading = ref(false)
  const dataSource = ref<SystemLog[]>([])

  const levelOptions = [
    { label: 'ALL', value: '' },
    { label: 'INFO', value: 'INFO' },
    { label: 'WARN', value: 'WARN' },
    { label: 'ERROR', value: 'ERROR' },
  ]

  const filters = ref<{ level?: string; search?: string; range?: [Dayjs, Dayjs] | undefined }>({ level: '', search: '', range: undefined })

  const pagination = ref<{ current: number; pageSize: number; total: number }>({ current: 1, pageSize: 20, total: 0 })

  const logChartRef = ref<HTMLDivElement | null>(null)
  let logChart: echarts.ECharts | null = null
  const renderLogChart = () => {
    if (!logChartRef.value) return
    if (!logChart) logChart = echarts.init(logChartRef.value as HTMLDivElement)
    const levelCount: Record<string, number> = {}
    dataSource.value.forEach((l: any) => { const lvl = l.level || 'INFO'; levelCount[lvl] = (levelCount[lvl] || 0) + 1 })
    const labels = Object.keys(levelCount)
    const values = labels.map((k) => levelCount[k])
    logChart.setOption({
      tooltip: { trigger: 'axis' },
      grid: { left: 40, right: 10, top: 10, bottom: 30 },
      xAxis: { type: 'category', data: labels },
      yAxis: { type: 'value' },
      series: [{ type: 'bar', data: values }],
    })
  }

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '级别', dataIndex: 'level', key: 'level', width: 100 },
    { title: '内容', dataIndex: 'message', key: 'message' },
    { title: '详情', dataIndex: 'details', key: 'details' },
    { title: '时间', dataIndex: 'timestamp', key: 'timestamp', width: 200 },
    { title: '操作', key: 'action', width: 120 },
  ]

  const selectedRowKeys = ref<number[]>([])
  const rowSelection = computed(() => ({
    selectedRowKeys: selectedRowKeys.value,
    onChange: (keys: any[]) => (selectedRowKeys.value = keys as number[]),
  }))

  const handleDelete = async (record: SystemLog) => {
    opLoading.value = true
    try {
      await deleteLog(record.id)
      message.success('已删除')
      await fetchList()
    } finally {
      opLoading.value = false
    }
  }

  const onBatchDelete = async () => {
    if (!selectedRowKeys.value.length) return
    opLoading.value = true
    try {
      await batchDeleteLogs(selectedRowKeys.value)
      message.success('已批量删除')
      selectedRowKeys.value = []
      await fetchList()
    } finally {
      opLoading.value = false
    }
  }

  const onClear = async () => {
    opLoading.value = true
    try {
      await clearLogs()
      message.success('日志已清空')
      selectedRowKeys.value = []
      await fetchList(true)
    } finally {
      opLoading.value = false
    }
  }

  const buildTimeParams = () => {
    const range = filters.value.range
    if (!range || range.length !== 2) return {}
    const [start, end] = range
    // 使用本地时间格式 YYYY-MM-DD HH:mm:ss，便于 SQLite 文本比较
    const start_time = dayjs(start).format('YYYY-MM-DD HH:mm:ss')
    const end_time = dayjs(end).format('YYYY-MM-DD HH:mm:ss')
    return { start_time, end_time }
  }

  const fetchList = async (resetPage = false) => {
    loading.value = true
    try {
      const params: any = {
        page: resetPage ? 1 : pagination.value.current,
        page_size: pagination.value.pageSize,
      }
      if (filters.value.level) params.level = filters.value.level
      if (filters.value.search) params.search = filters.value.search
      Object.assign(params, buildTimeParams())

      const res = await getSystemLogs(params)
      const data = (res as any) || {}
      const list = data.list || []
      const total = data.total || 0
      dataSource.value = list
      pagination.value = { ...pagination.value, current: params.page, total }
      await nextTick()
      renderLogChart()
    } finally {
      loading.value = false
    }
  }

  const onSearch = () => fetchList(true)
  const onReset = () => {
    filters.value = { level: '', search: '', range: undefined }
    fetchList(true)
  }
  const onTableChange = (p: any) => {
    pagination.value.current = p.current
    pagination.value.pageSize = p.pageSize
    fetchList()
  }

  onMounted(() => fetchList(true))
</script> 