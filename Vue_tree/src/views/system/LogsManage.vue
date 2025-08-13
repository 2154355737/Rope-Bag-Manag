<template>
  <PageWrapper title="系统日志" content="系统运行日志列表">
    <Card :loading="loading" class="!mb-4">
      <div ref="logChartRef" style="width: 100%; height: 220px"></div>
    </Card>
    <Card :loading="loading">
      <Table
        :columns="columns"
        :data-source="dataSource"
        :pagination="{ pageSize: 20 }"
        row-key="id"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'action'">
            <Space>
              <Button type="link" danger @click="() => handleDelete(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, nextTick } from 'vue'
  import { Card, Table, Button, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getSystemLogs, type SystemLog } from '/@/api/dashboard'
  import * as echarts from 'echarts'

  const loading = ref(true)
  const dataSource = ref<SystemLog[]>([])

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
    { title: '级别', dataIndex: 'level', key: 'level' },
    { title: '模块', dataIndex: 'module', key: 'module' },
    { title: '内容', dataIndex: 'message', key: 'message' },
    { title: '时间', dataIndex: 'timestamp', key: 'timestamp' },
    { title: '操作', key: 'action', width: 120 },
  ]

  const handleDelete = async (_record: SystemLog) => {
    // OpenAPI 未提供删除日志接口，这里仅做前端占位（可根据后端后续补充）
    // await deleteSystemLog(_record.id)
  }

  const fetchList = async () => {
    try {
      const res = await getSystemLogs({ page: 1, page_size: 20 })
      dataSource.value = res.list || []
      await nextTick()
      renderLogChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 