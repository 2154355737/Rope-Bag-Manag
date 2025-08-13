<template>
  <PageWrapper title="存储配置" content="AList 存储状态与工具">
    <Card class="!mb-4" :loading="loadingStats">
      <Space>
        <Button @click="fetchStats">刷新统计</Button>
        <Button @click="handleCleanup">清理孤立文件</Button>
        <input type="file" @change="onFileChange" />
        <Button type="primary" @click="handleUpload" :disabled="!fileRef">上传测试文件</Button>
      </Space>
      <div class="mt-4 text-secondary">容量/文件数等统计：{{ statsText }}</div>
    </Card>

    <Card :loading="loadingFiles">
      <Table :columns="columns" :data-source="files" row-key="file_path">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'action'">
            <Space>
              <Button type="link" @click="() => getDownload(record)">下载链接</Button>
              <Button type="link" danger @click="() => removeFile(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, computed, onMounted } from 'vue'
  import { Card, Table, Button, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { defHttp } from '/@/utils/http/axios'
  import { useMessage } from '/@/hooks/web/useMessage'

  const { createMessage } = useMessage()

  const loadingStats = ref(true)
  const loadingFiles = ref(true)
  const stats = ref<any>(null)
  const files = ref<any[]>([])
  const fileRef = ref<File | null>(null)

  const columns = [
    { title: '路径', dataIndex: 'file_path', key: 'file_path' },
    { title: '大小(bytes)', dataIndex: 'file_size', key: 'file_size' },
    { title: '操作', key: 'action', width: 160 },
  ]

  const statsText = computed(() => (stats.value ? JSON.stringify(stats.value) : '...'))

  const onFileChange = (e: any) => {
    const files = e?.target?.files
    fileRef.value = files && files[0] ? files[0] : null
  }

  const handleUpload = async () => {
    if (!fileRef.value) return
    const form = new FormData()
    form.append('file', fileRef.value)
    await defHttp.post({ url: '/api/v1/storage/upload', data: form }, { isTransformResponse: false })
    createMessage.success('上传成功')
    await fetchFiles()
  }

  const getDownload = async (record: any) => {
    const res = await defHttp.post<any>({ url: '/api/v1/storage/download-link', data: { file_path: record.file_path } })
    const url = res?.data?.download_url || res?.download_url
    if (url) {
      await navigator.clipboard.writeText(url)
      createMessage.success('下载链接已复制')
    }
  }

  const removeFile = async (record: any) => {
    await defHttp.post({ url: '/api/v1/storage/delete', data: { file_path: record.file_path } })
    await fetchFiles()
  }

  const handleCleanup = async () => {
    await defHttp.post({ url: '/api/v1/storage/cleanup' })
    createMessage.success('清理任务已执行')
    await fetchFiles()
  }

  const fetchFiles = async () => {
    try {
      loadingFiles.value = true
      const res = await defHttp.get<any>({ url: '/api/v1/storage/files' })
      files.value = res?.data || res || []
    } finally {
      loadingFiles.value = false
    }
  }

  const fetchStats = async () => {
    try {
      loadingStats.value = true
      const res = await defHttp.get<any>({ url: '/api/v1/storage/stats' })
      stats.value = res?.data || res || null
    } finally {
      loadingStats.value = false
    }
  }

  onMounted(async () => {
    await Promise.all([fetchStats(), fetchFiles()])
  })
</script> 