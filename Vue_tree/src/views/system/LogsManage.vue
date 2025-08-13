<template>
  <PageWrapper title="系统日志" content="系统运行日志列表">
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
  import { ref, onMounted } from 'vue'
  import { Card, Table, Button, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getSystemLogs, type SystemLog } from '/@/api/dashboard'

  const loading = ref(true)
  const dataSource = ref<SystemLog[]>([])

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
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 