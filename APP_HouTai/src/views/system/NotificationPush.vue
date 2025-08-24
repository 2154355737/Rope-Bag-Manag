<template>
  <PageWrapper title="推送通知" content="向用户群体推送通知">
    <Card :loading="loading" class="!mb-4">
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 14 }">
        <FormItem label="目标">
          <Select v-model:value="formState.target" :options="targetOptions" />
        </FormItem>
        <FormItem label="分类(可选)" v-if="formState.target === 'subscribers' || formState.target === 'app_subscribers'">
          <Select v-model:value="formState.category_id" :options="categoryOptions" placeholder="选择分类以通知订阅者" />
        </FormItem>
        <FormItem label="用户邮箱" v-if="formState.target === 'single' || formState.target === 'app_single'">
          <Input v-model:value="formState.email" placeholder="user@example.com" />
        </FormItem>
        <FormItem label="标题">
          <Input v-model:value="formState.title" />
        </FormItem>
        <FormItem label="内容">
          <Input.TextArea v-model:value="formState.content" :rows="5" />
        </FormItem>
        <FormItem>
          <Space>
            <Button type="primary" @click="handleSend">发送</Button>
          </Space>
        </FormItem>
      </Form>
    </Card>

    <Card :loading="siteLoading">
      <template #title>历史通知（全站）</template>
      <Table :columns="siteColumns" :data-source="siteHistory" row-key="id" :pagination="sitePagination" @change="onTableChange">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'is_read'">
            <span>{{ record.is_read ? '已读' : '未读' }}</span>
          </template>
        </template>
      </Table>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue'
  import { Card, Form, Input, Button, Space, Select, Table } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { defHttp } from '/@/utils/http/axios'
  import { getCategories, broadcastNotifications } from '/@/api/dashboard'
  import { useMessage } from '/@/hooks/web/useMessage'

  const FormItem = Form.Item
  const { createMessage } = useMessage()

  const loading = ref(false)
  const categoryOptions = ref<any[]>([])
  const targetOptions = [
    { label: '订阅某分类的用户(邮件)', value: 'subscribers' },
    { label: '单个用户(邮件)', value: 'single' },
    { label: '发布公告(全体可见)', value: 'announcement' },
    { label: 'App站内通知-全体', value: 'app_all' },
    { label: 'App站内通知-分类订阅者', value: 'app_subscribers' },
    { label: 'App站内通知-单人(邮箱)', value: 'app_single' },
  ]

  const formState = ref<any>({ target: 'announcement', category_id: undefined, email: '', title: '', content: '' })

  // 全站通知
  const siteLoading = ref(false)
  const siteHistory = ref<any[]>([])
  const sitePage = ref<number>(1)
  const sitePageSize = ref<number>(10)
  const siteTotal = ref<number>(0)
  const siteColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '用户ID', dataIndex: 'user_id', key: 'user_id', width: 100 },
    { title: '标题', dataIndex: 'title', key: 'title', width: 220 },
    { title: '内容', dataIndex: 'content', key: 'content' },
    { title: '类型', dataIndex: 'notif_type', key: 'notif_type', width: 120 },
    { title: '是否已读', dataIndex: 'is_read', key: 'is_read', width: 100 },
    { title: '时间', dataIndex: 'created_at', key: 'created_at', width: 200 },
  ]
  const sitePagination = ref<any>({ current: sitePage.value, pageSize: sitePageSize.value, total: siteTotal.value, showSizeChanger: false })

  const fetchSiteHistory = async () => {
    siteLoading.value = true
    const res = await defHttp.get<any>({ url: '/api/v1/admin/notifications', params: { page: sitePage.value, page_size: sitePageSize.value } }, { errorMessageMode: 'none' }).catch(() => null)
    const list = res?.list || []
    const total = res?.total || 0
    siteHistory.value = list
    siteTotal.value = total
    sitePagination.value = { current: sitePage.value, pageSize: sitePageSize.value, total: siteTotal.value, showSizeChanger: false }
    siteLoading.value = false
  }

  const onTableChange = (pagination: any) => {
    sitePage.value = pagination.current || 1
    sitePageSize.value = pagination.pageSize || 10
    fetchSiteHistory()
  }

  const handleSend = async () => {
    if (!formState.value.title || !formState.value.content) {
      createMessage.warning('请填写标题与内容')
      return
    }
    const record = { target: formState.value.target, category_id: formState.value.category_id, email: formState.value.email, title: formState.value.title, content: formState.value.content }

    if (formState.value.target === 'announcement') {
      await defHttp.post({ url: '/api/v1/admin/announcements', data: { title: formState.value.title, content: formState.value.content, enabled: true } })
      createMessage.success('公告已发布')
      return
    }
    if (formState.value.target === 'subscribers') {
      if (!formState.value.category_id) {
        createMessage.warning('请选择分类')
        return
      }
      await defHttp.post({ url: '/api/v1/admin/subscriptions/notify', data: { category_id: formState.value.category_id, title: formState.value.title, content: formState.value.content } })
      createMessage.success('已向订阅用户发送(邮件)')
      return
    }
    if (formState.value.target === 'single') {
      if (!formState.value.email) {
        createMessage.warning('请输入用户邮箱')
        return
      }
      await defHttp.post({ url: '/api/v1/admin/test-email', data: { email: formState.value.email, title: formState.value.title, content: formState.value.content } as any })
      createMessage.success('已发送邮件')
      return
    }
    if (formState.value.target === 'app_all') {
      await broadcastNotifications({ target: 'all', title: formState.value.title, content: formState.value.content })
      createMessage.success('已推送App站内通知-全体')
      return
    }
    if (formState.value.target === 'app_subscribers') {
      if (!formState.value.category_id) {
        createMessage.warning('请选择分类')
        return
      }
      await broadcastNotifications({ target: 'subscribers', category_id: formState.value.category_id, title: formState.value.title, content: formState.value.content })
      createMessage.success('已推送App站内通知-订阅者')
      return
    }
    if (formState.value.target === 'app_single') {
      if (!formState.value.email) {
        createMessage.warning('请输入用户邮箱')
        return
      }
      await broadcastNotifications({ target: 'single', email: formState.value.email, title: formState.value.title, content: formState.value.content })
      createMessage.success('已推送App站内通知-单人')
      return
    }
  }

  const fetchCats = async () => {
    const list = await getCategories().catch(() => [])
    categoryOptions.value = (list || []).map((c: any) => ({ label: c.name, value: c.id }))
  }

  onMounted(async () => {
    await Promise.all([fetchCats(), fetchSiteHistory()])
  })
</script> 