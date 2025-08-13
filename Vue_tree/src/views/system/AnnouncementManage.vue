<template>
  <PageWrapper title="公告管理" content="系统公告列表与编辑">
    <Card class="!mb-4">
      <Space>
        <Button type="primary" @click="handleCreate">新建公告</Button>
      </Space>
      <div ref="annChartRef" style="width: 100%; height: 220px" class="mt-2"></div>
    </Card>
    <Card :loading="loading">
      <Table :columns="columns" :data-source="dataSource" :row-selection="rowSelection" row-key="id">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'enabled'">
            <Tag :color="record.enabled ? 'success' : 'default'">{{ record.enabled ? '启用' : '禁用' }}</Tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <Space>
              <Button type="link" @click="handleEdit(record)">编辑</Button>
              <Button type="link" danger @click="() => handleDelete(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
      <Space class="mt-4">
        <Button @click="() => handleBatchStatus(true)">批量启用</Button>
        <Button @click="() => handleBatchStatus(false)">批量禁用</Button>
        <Button danger @click="handleBatchDelete">批量删除</Button>
      </Space>
    </Card>

    <Modal v-model:visible="editOpen" :title="currentId ? '编辑公告' : '新建公告'" :confirm-loading="submitLoading" @ok="handleSubmit" @cancel="() => (editOpen = false)" destroy-on-close>
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 16 }">
        <FormItem label="标题">
          <Input v-model:value="formState.title" placeholder="请输入标题" />
        </FormItem>
        <FormItem label="内容">
          <Input.TextArea v-model:value="formState.content" :rows="5" placeholder="请输入内容" />
        </FormItem>
        <FormItem label="启用">
          <Switch v-model:checked="formState.enabled" />
        </FormItem>
      </Form>
    </Modal>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, computed, nextTick } from 'vue'
  import { Card, Table, Modal, Form, Input, Button, Switch, Tag, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { defHttp } from '/@/utils/http/axios'
  import * as echarts from 'echarts'

  const FormItem = Form.Item

  const loading = ref(true)
  const dataSource = ref<any[]>([])
  const selectedRowKeys = ref<number[]>([])

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '标题', dataIndex: 'title', key: 'title' },
    { title: '启用', dataIndex: 'enabled', key: 'enabled' },
    { title: '操作', key: 'action', width: 160 },
  ]

  const rowSelection = computed(() => ({
    selectedRowKeys: selectedRowKeys.value,
    onChange: (keys: any[]) => (selectedRowKeys.value = keys as number[]),
  }))

  const editOpen = ref(false)
  const submitLoading = ref(false)
  const currentId = ref<number | null>(null)
  const formState = ref<any>({ title: '', content: '', enabled: true })

  const annChartRef = ref<HTMLDivElement | null>(null)
  let annChart: echarts.ECharts | null = null
  const renderAnnChart = () => {
    if (!annChartRef.value) return
    if (!annChart) annChart = echarts.init(annChartRef.value as HTMLDivElement)
    let enabled = 0, disabled = 0
    dataSource.value.forEach((a) => (a.enabled ? enabled++ : disabled++))
    annChart.setOption({
      tooltip: { trigger: 'item' },
      legend: { bottom: 0 },
      series: [{ type: 'pie', radius: '60%', data: [
        { name: '启用', value: enabled },
        { name: '禁用', value: disabled },
      ] }],
    })
  }

  const handleCreate = () => {
    currentId.value = null
    formState.value = { title: '', content: '', enabled: true }
    editOpen.value = true
  }

  const handleEdit = (record: any) => {
    currentId.value = record.id
    formState.value = { title: record.title, content: record.content, enabled: !!record.enabled }
    editOpen.value = true
  }

  const handleDelete = async (record: any) => {
    await defHttp.delete({ url: `/api/v1/admin/announcements/${record.id}` })
    await fetchList()
  }

  const handleBatchDelete = async () => {
    if (!selectedRowKeys.value.length) return
    await defHttp.post({ url: '/api/v1/admin/announcements/batch-delete', data: { ids: selectedRowKeys.value } })
    selectedRowKeys.value = []
    await fetchList()
  }

  const handleBatchStatus = async (enabled: boolean) => {
    if (!selectedRowKeys.value.length) return
    await defHttp.put({ url: '/api/v1/admin/announcements/batch-status', data: { ids: selectedRowKeys.value, enabled } })
    await fetchList()
  }

  const handleSubmit = async () => {
    submitLoading.value = true
    try {
      if (currentId.value) {
        await defHttp.put({ url: `/api/v1/admin/announcements/${currentId.value}`, data: { ...formState.value } })
      } else {
        await defHttp.post({ url: '/api/v1/admin/announcements', data: { ...formState.value } })
      }
      editOpen.value = false
      await fetchList()
    } finally {
      submitLoading.value = false
    }
  }

  const fetchList = async () => {
    try {
      loading.value = true
      const res = await defHttp.get<any>({ url: '/api/v1/admin/announcements' })
      dataSource.value = res?.data?.list || res?.list || []
      await nextTick()
      renderAnnChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 