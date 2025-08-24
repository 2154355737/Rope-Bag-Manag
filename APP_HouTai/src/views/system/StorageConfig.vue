<template>
  <PageWrapper title="储存管理" content="AList 文件与目录浏览">
    <Card class="!mb-4" :loading="loadingStats">
      <Space>
        <Button @click="fetchStats">刷新统计</Button>
        <Button @click="handleCleanup">清理孤立文件</Button>
        <input type="file" @change="onFileChange" />
        <Button type="primary" @click="handleUpload" :disabled="!fileRef">上传测试文件</Button>
        <span v-if="uploading" class="ml-2">上传进度：</span>
        <Progress v-if="uploading" :percent="uploadPercent" style="width:180px" />
      </Space>
      <div class="mt-4 text-secondary">容量/文件数等统计：{{ statsText }}</div>
      <div ref="stChartRef" style="width: 100%; height: 220px" class="mt-2"></div>
    </Card>

    <Card :loading="loadingFiles">
      <Space class="!mb-2" wrap>
        <span>当前目录：</span>
        <Input v-model:value="currentPath" style="width: 460px" />
        <Button @click="goParent">上级</Button>
        <Button type="primary" @click="fetchFiles">进入</Button>
        <span class="text-secondary">目录：{{ dirCount }} | 文件：{{ fileCount }}</span>
      </Space>
      <Table :columns="columns" :data-source="files" row-key="__k">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'name'">
            <a v-if="isDir(record)" @click="() => enterDir(record)">{{ record.name }}</a>
            <span v-else>{{ record.name }}</span>
          </template>
          <template v-else-if="column.key === 'type'">
            <span>{{ record.is_dir ? '目录' : '文件' }}</span>
          </template>
          <template v-else-if="column.key === 'size'">
            <span>{{ record.is_dir ? '-' : record.size }}</span>
          </template>
          <template v-else-if="column.key === 'modified'">
            <span>{{ record.modified || '-' }}</span>
          </template>
          <template v-else-if="column.key === 'action'">
            <Space>
              <Button v-if="record.is_dir" type="link" @click="() => enterDir(record)">进入</Button>
              <Button v-else type="link" @click="() => getDownload(record)">下载链接</Button>
              <Button v-if="!record.is_dir" type="link" danger @click="() => removeFile(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, computed, onMounted, nextTick } from 'vue'
  import { Card, Table, Button, Space, Input, Progress } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { defHttp } from '/@/utils/http/axios'
  import { useMessage } from '/@/hooks/web/useMessage'
  import * as echarts from 'echarts'

  const { createMessage } = useMessage()

  const loadingStats = ref(true)
  const loadingFiles = ref(true)
  const stats = ref<any>(null)
  const files = ref<any[]>([])
  const fileRef = ref<File | null>(null)
  const uploading = ref(false)
  const uploadPercent = ref(0)
  const currentPath = ref<string>('/image/结绳社区')

  const columns = [
    { title: '名称', dataIndex: 'name', key: 'name' },
    { title: '类型', dataIndex: 'type', key: 'type', width: 100 },
    { title: '大小(bytes)', dataIndex: 'size', key: 'size', width: 140 },
    { title: '修改时间', dataIndex: 'modified', key: 'modified', width: 220 },
    { title: '操作', key: 'action', width: 200 },
  ]

  const statsText = computed(() => (stats.value ? JSON.stringify(stats.value) : '...'))
  const dirCount = computed(() => files.value.filter((f: any) => f.is_dir).length)
  const fileCount = computed(() => files.value.filter((f: any) => !f.is_dir).length)

  const stChartRef = ref<HTMLDivElement | null>(null)
  let stChart: echarts.ECharts | null = null
  const renderStChart = () => {
    if (!stChartRef.value) return
    if (!stChart) stChart = echarts.init(stChartRef.value as HTMLDivElement)
    const map: Record<string, number> = {}
    files.value.filter((f: any) => !f.is_dir).forEach((f: any) => {
      const m = String(f.name || '').match(/\.([a-zA-Z0-9]+)$/)
      const ext = (m && m[1] ? m[1] : 'other').toLowerCase()
      map[ext] = (map[ext] || 0) + 1
    })
    const data = Object.entries(map)
      .sort((a: any, b: any) => (b[1] as number) - (a[1] as number))
      .slice(0, 8)
      .map((i: any) => ({ name: i[0], value: i[1] }))
    stChart.setOption({ tooltip: { trigger: 'item' }, legend: { bottom: 0 }, series: [{ type: 'pie', radius: '60%', data }] })
  }

  const onFileChange = (e: any) => {
    const files = e?.target?.files
    fileRef.value = files && files[0] ? files[0] : null
  }

  const handleUpload = async () => {
    if (!fileRef.value) return
    const form = new FormData()
    form.append('file', fileRef.value)
    uploading.value = true
    uploadPercent.value = 0
    try {
      await defHttp.post(
        { url: '/api/v1/storage/upload', data: form, headers: { 'Content-Type': 'multipart/form-data' }, onUploadProgress: (e: ProgressEvent) => {
          if (e.lengthComputable) {
            uploadPercent.value = Math.round((e.loaded / e.total) * 100)
          }
        } },
        { isTransformResponse: false },
      )
      createMessage.success('上传成功')
      await fetchFiles()
    } finally {
      setTimeout(() => { uploading.value = false; uploadPercent.value = 0 }, 600)
    }
  }

  const joinPath = (a: string, b: string) => (a.endsWith('/') ? a.slice(0, -1) : a) + '/' + (b.startsWith('/') ? b.slice(1) : b)

  const getDownload = async (record: any) => {
    const file_path = joinPath(currentPath.value, record.name)
    const res = await defHttp.post<any>({ url: '/api/v1/storage/download-link', data: { file_path } })
    const url = res?.data?.download_url || res?.download_url
    if (url) {
      await navigator.clipboard.writeText(url)
      createMessage.success('下载链接已复制')
    }
  }

  const removeFile = async (record: any) => {
    const file_path = joinPath(currentPath.value, record.name)
    await defHttp.post({ url: '/api/v1/storage/delete', data: { file_path } })
    await fetchFiles()
  }

  const handleCleanup = async () => {
    await defHttp.post({ url: '/api/v1/storage/cleanup' })
    createMessage.success('清理任务已执行')
    await fetchFiles()
  }

  const isDir = (r: any) => r?.is_dir === true || r?.is_dir === 1 || r?.type === 'dir'

  const enterDir = async (record: any) => {
    if (!isDir(record)) return
    currentPath.value = joinPath(currentPath.value, record.name)
    await fetchFiles()
    // 强制滚动到顶部以提示已进入
    const wrap = document.querySelector('.ant-table-body') as HTMLElement
    if (wrap) wrap.scrollTop = 0
  }

  const goParent = async () => {
    const parts = currentPath.value.split('/').filter(Boolean)
    if (parts.length <= 2) return // 保持在根 /image/结绳社区
    parts.pop()
    currentPath.value = '/' + parts.join('/')
    await fetchFiles()
  }

  const fetchFiles = async () => {
    try {
      loadingFiles.value = true
      const res = await defHttp.get<any>({ url: '/api/v1/storage/files', params: { path: currentPath.value } })
      const list = res?.data || res || []
      const base = currentPath.value.replace(/\/$/, '')
      files.value = list.map((it: any) => ({ ...it, __k: base + '/' + (it.name || '') }))
      await nextTick()
      renderStChart()
    } finally {
      loadingFiles.value = false
    }
  }

  const fetchStats = async () => {
    try {
      loadingStats.value = true
      const res = await defHttp.get<any>({ url: '/api/v1/storage/stats' })
      stats.value = res?.data || res || null
      const sp = stats.value?.storage_path || stats.value?.data?.storage_path
      if (sp) currentPath.value = sp
    } finally {
      loadingStats.value = false
    }
  }

  onMounted(async () => {
    await Promise.all([fetchStats(), fetchFiles()])
  })
</script> 