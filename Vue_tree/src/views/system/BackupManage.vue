<template>
  <PageWrapper title="备份管理" content="数据库备份列表">
    <Card :loading="loading" class="!mb-4">
      <Space>
        <Button type="primary" @click="handleCreate('Manual')">创建手动备份</Button>
      </Space>
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
  import { ref, onMounted } from 'vue'
  import { Card, Table, Button, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getBackups, createBackup, deleteBackup, type BackupInfo } from '/@/api/dashboard'
  import { useMessage } from '/@/hooks/web/useMessage'

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
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 