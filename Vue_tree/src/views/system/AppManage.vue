<template>
  <PageWrapper title="App管理" content="版本与发布管理">
    <Card class="!mb-4" :loading="loading">
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 14 }">
        <FormItem label="当前版本">
          <Input v-model:value="formState.current_version" placeholder="如 1.0.0" />
        </FormItem>
        <FormItem label="最新版本">
          <Input v-model:value="formState.latest_version" placeholder="如 1.1.0" />
        </FormItem>
        <FormItem label="下载链接">
          <Input v-model:value="formState.latest_download_url" placeholder="https://..." />
        </FormItem>
        <FormItem label="更新内容">
          <Input.TextArea v-model:value="formState.update_notes" :rows="6" placeholder="本次更新说明" />
        </FormItem>
        <FormItem label="上传安装包">
          <Space>
            <input type="file" @change="onFileChange" />
            <Button type="primary" :disabled="!fileRef" @click="handleUpload">上传并生成下载链接</Button>
          </Space>
        </FormItem>
        <FormItem>
          <Space>
            <Button type="primary" @click="handleSave">保存设置</Button>
            <Button @click="addToHistory">加入历史版本</Button>
            <Button type="dashed" @click="pushNewVersion">发布公告</Button>
            <Button type="dashed" @click="checkUpdate">一键检查更新</Button>
          </Space>
        </FormItem>
        <FormItem label="站内通知">
          <Space>
            <Button type="dashed" @click="pushAppAll">推送App站内通知(全体)</Button>
            <Input v-model:value="categoryForPush" placeholder="分类ID" style="width: 120px" />
            <Button type="dashed" @click="pushAppSubscribers">推送App站内通知(订阅者)</Button>
          </Space>
        </FormItem>
      </Form>
    </Card>

    <Card :loading="loading">
      <template #title>历史版本</template>
      <Table :columns="columns" :data-source="history" row-key="version" :pagination="false">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'action'">
            <Space>
              <Button type="link" @click="() => copyLink(record.download_url)">复制链接</Button>
              <Button type="link" danger @click="() => removeHistory(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue'
  import { Card, Form, Input, Button, Space, Table } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { defHttp } from '/@/utils/http/axios'
  import { useMessage } from '/@/hooks/web/useMessage'
  import { broadcastNotifications } from '/@/api/dashboard'

  const FormItem = Form.Item
  const { createMessage } = useMessage()

  const loading = ref(true)
  const fileRef = ref<File | null>(null)
  const formState = ref<any>({ current_version: '', latest_version: '', latest_download_url: '', update_notes: '' })
  const history = ref<any[]>([])
  const categoryForPush = ref<string>('')

  const columns = [
    { title: '版本', dataIndex: 'version', key: 'version' },
    { title: '下载链接', dataIndex: 'download_url', key: 'download_url' },
    { title: '备注', dataIndex: 'notes', key: 'notes' },
    { title: '时间', dataIndex: 'created_at', key: 'created_at' },
    { title: '操作', key: 'action', width: 160 },
  ]

  const onFileChange = (e: any) => {
    const files = e?.target?.files
    fileRef.value = files && files[0] ? files[0] : null
  }

  const handleUpload = async () => {
    if (!fileRef.value) return
    const form = new FormData()
    form.append('file', fileRef.value)
    await defHttp.post({ url: '/api/v1/storage/upload', data: form }, { isTransformResponse: false })
    const res = await defHttp.post<any>({ url: '/api/v1/storage/download-link', data: { file_path: fileRef.value.name } })
    const url = res?.data?.download_url || res?.download_url
    if (url) {
      formState.value.latest_download_url = url
      createMessage.success('上传成功，已生成下载链接')
    }
  }

  const copyLink = async (link: string) => {
    if (!link) return
    await navigator.clipboard.writeText(link)
    createMessage.success('已复制到剪贴板')
  }

  const removeHistory = (row: any) => {
    history.value = history.value.filter((h) => h.version !== row.version)
    saveSetting('app_version_history', JSON.stringify(history.value))
  }

  const addToHistory = () => {
    if (!formState.value.latest_version || !formState.value.latest_download_url) {
      createMessage.warning('请先填写最新版本与下载链接')
      return
    }
    const item = {
      version: formState.value.latest_version,
      download_url: formState.value.latest_download_url,
      notes: formState.value.update_notes || '',
      created_at: new Date().toISOString(),
    }
    const exists = history.value.some((h) => h.version === item.version)
    if (!exists) history.value.unshift(item)
    saveSetting('app_version_history', JSON.stringify(history.value))
  }

  const pushNewVersion = async () => {
    if (!formState.value.latest_version || !formState.value.latest_download_url) {
      createMessage.warning('请先填写最新版本与下载链接')
      return
    }
    const title = `新版本 v${formState.value.latest_version} 发布`
    const content = `下载链接：${formState.value.latest_download_url}\n更新内容：\n${formState.value.update_notes || ''}`
    await defHttp.post({ url: '/api/v1/admin/announcements', data: { title, content, enabled: true } })
    createMessage.success('已发布公告通知')
  }

  const getSetting = async (key: string) => {
    const res = await defHttp
      .get<any>({ url: `/api/v1/admin/settings/${key}` }, { errorMessageMode: 'none', retryRequest: { isOpenRetry: false, count: 0, waitTime: 0 } })
      .catch(() => null)
    return res?.data?.value ?? res?.value ?? null
  }

  const getAllSettings = async () => {
    const res = await defHttp
      .get<Record<string, string>>({ url: `/api/v1/admin/settings` }, { errorMessageMode: 'none', retryRequest: { isOpenRetry: false, count: 0, waitTime: 0 } })
      .catch(() => ({} as Record<string, string>))
    return (res as any) || {}
  }

  const saveSetting = async (key: string, value: string) => {
    await defHttp.post({ url: `/api/v1/admin/settings/${key}`, data: { value } })
  }

  const fetchAll = async () => {
    loading.value = true
    // 优先一次性获取所有设置，避免多次请求与404噪音
    const settings = await getAllSettings()
    // 兜底：若全量接口失败则并行单个拉取
    if (!settings || Object.keys(settings).length === 0) {
      const [cur, latest, link, hist, notes] = await Promise.all([
        getSetting('app_current_version'),
        getSetting('app_latest_version'),
        getSetting('app_latest_download_url'),
        getSetting('app_version_history'),
        getSetting('app_update_notes'),
      ])
      formState.value.current_version = cur || ''
      formState.value.latest_version = latest || ''
      formState.value.latest_download_url = link || ''
      formState.value.update_notes = notes || ''
      history.value = hist ? JSON.parse(hist) : []
      loading.value = false
      return
    }

    formState.value.current_version = settings['app_current_version'] || ''
    formState.value.latest_version = settings['app_latest_version'] || ''
    formState.value.latest_download_url = settings['app_latest_download_url'] || ''
    formState.value.update_notes = settings['app_update_notes'] || ''
    history.value = settings['app_version_history'] ? JSON.parse(settings['app_version_history']) : []
    loading.value = false
  }

  const handleSave = async () => {
    await Promise.all([
      saveSetting('app_current_version', formState.value.current_version || ''),
      saveSetting('app_latest_version', formState.value.latest_version || ''),
      saveSetting('app_latest_download_url', formState.value.latest_download_url || ''),
      saveSetting('app_update_notes', formState.value.update_notes || ''),
    ])
    createMessage.success('已保存')
  }

  const pushAppAll = async () => {
    if (!formState.value.latest_version) {
      createMessage.warning('请先填写最新版本')
      return
    }
    const title = `新版本 v${formState.value.latest_version} 发布`
    const content = `下载链接：${formState.value.latest_download_url}\n更新内容：\n${formState.value.update_notes || ''}`
    await broadcastNotifications({ target: 'all', title, content })
    createMessage.success('已推送App站内通知(全体)')
  }

  const pushAppSubscribers = async () => {
    const cid = Number(categoryForPush.value)
    if (!cid) {
      createMessage.warning('请输入分类ID')
      return
    }
    const title = `新版本 v${formState.value.latest_version} 发布`
    const content = `下载链接：${formState.value.latest_download_url}\n更新内容：\n${formState.value.update_notes || ''}`
    await broadcastNotifications({ target: 'subscribers', category_id: cid, title, content })
    createMessage.success('已推送App站内通知(订阅者)')
  }

  const checkUpdate = () => {
    if (!formState.value.current_version || !formState.value.latest_version) {
      createMessage.info('请先填写当前版本与最新版本')
      return
    }
    if (formState.value.current_version === formState.value.latest_version) {
      createMessage.success('当前已是最新版本')
    } else {
      createMessage.warning(`检测到新版本 v${formState.value.latest_version}`)
    }
  }

  onMounted(fetchAll)
</script> 