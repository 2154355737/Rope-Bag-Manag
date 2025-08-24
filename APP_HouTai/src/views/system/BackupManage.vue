<template>
  <PageWrapper title="备份管理" content="数据库备份列表">
    <Card :loading="loading" class="!mb-4">
      <Space>
        <Button type="primary" @click="handleCreate('Manual')">创建手动备份</Button>
      </Space>
      <div ref="bkChartRef" style="width: 100%; height: 220px" class="mt-2"></div>
    </Card>
    <Card :loading="loading">
      <Table
        :columns="columns"
        :data-source="dataSource"
        :pagination="{ pageSize: 10 }"
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
  import { getBackups, createBackup, deleteBackup, type BackupInfo } from '/@/api/dashboard'
  import { useMessage } from '/@/hooks/web/useMessage'
  import * as echarts from 'echarts'

  const { createMessage } = useMessage()

  const loading = ref(true)
  const dataSource = ref<BackupInfo[]>([])

  const columns = [
    { title: '文件名', dataIndex: 'filename', key: 'filename' },
    { title: '类型', dataIndex: 'backup_type', key: 'backup_type' },
    { title: '大小', dataIndex: 'file_size', key: 'file_size' },
    { title: '时间', dataIndex: 'backup_time', key: 'backup_time' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '操作', key: 'action', width: 120 },
  ]

  const bkChartRef = ref<HTMLDivElement | null>(null)
  let bkChart: echarts.ECharts | null = null
  const renderBkChart = () => {
    if (!bkChartRef.value) return
    if (!bkChart) bkChart = echarts.init(bkChartRef.value as HTMLDivElement)
    const typeCount: Record<string, number> = {}
    dataSource.value.forEach((b: any) => {
      const t = b.backup_type || 'Unknown'
      typeCount[t] = (typeCount[t] || 0) + 1
    })
    const data = Object.keys(typeCount).map((k) => ({ name: k, value: typeCount[k] }))
    bkChart.setOption({ tooltip: { trigger: 'item' }, legend: { bottom: 0 }, series: [{ type: 'pie', radius: '60%', data }] })
  }

  const handleCreate = async (backup_type: 'Manual' | 'Auto' | 'Scheduled') => {
    await createBackup({ backup_type })
    createMessage.success('备份已创建')
    fetchList()
  }

  const handleDelete = async (record: BackupInfo) => {
    await deleteBackup(record.id)
    createMessage.success('删除成功')
    fetchList()
  }

  const fetchList = async () => {
    try {
      const res = await getBackups({ page: 1, page_size: 10 })
      dataSource.value = res.list || []
      await nextTick()
      renderBkChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 