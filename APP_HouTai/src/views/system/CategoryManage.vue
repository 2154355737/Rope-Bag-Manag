<template>
  <PageWrapper title="分类管理" content="系统分类列表">
    <Card :loading="loading" class="!mb-4">
      <Button type="primary" @click="handleCreate">新建分类</Button>
      <div ref="catChartRef" style="width: 100%; height: 220px" class="mt-2"></div>
    </Card>
    <Card :loading="loading">
      <Table
        :columns="columns"
        :data-source="dataSource"
        :pagination="false"
        row-key="id"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'enabled'">
            <Tag :color="record.enabled ? 'success' : 'error'">{{ record.enabled ? '启用' : '禁用' }}</Tag>
          </template>
          <template v-else-if="column.key === 'subscription_locked'">
            <Tag :color="record.subscription_locked ? 'warning' : 'default'">{{ record.subscription_locked ? '锁定' : '否' }}</Tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <Space>
              <Button type="link" @click="handleEdit(record)">编辑</Button>
              <Button type="link" danger @click="() => handleDelete(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>

    <Modal
      v-model:visible="editOpen"
      :title="currentId ? '编辑分类' : '新建分类'"
      :confirm-loading="submitLoading"
      @ok="handleSubmit"
      @cancel="handleCancel"
      destroy-on-close
    >
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 16 }">
        <FormItem label="名称">
          <Input v-model:value="formState.name" placeholder="请输入名称" />
        </FormItem>
        <FormItem label="描述">
          <Input v-model:value="formState.description" placeholder="请输入描述" />
        </FormItem>
        <FormItem label="启用">
          <Switch v-model:checked="formState.enabled" />
        </FormItem>
        <FormItem label="订阅锁定">
          <Switch v-model:checked="formState.subscription_locked" />
        </FormItem>
      </Form>
    </Modal>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, nextTick } from 'vue'
  import { Card, Table, Button, Modal, Form, Input, Switch, Tag, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getCategories, createCategory, updateCategory, deleteCategory, type Category } from '/@/api/dashboard'
  import { useMessage } from '/@/hooks/web/useMessage'
  import * as echarts from 'echarts'

  const { createMessage } = useMessage()

  // FormItem 别名
  const FormItem = Form.Item

  const loading = ref(true)
  const dataSource = ref<Category[]>([])

  const catChartRef = ref<HTMLDivElement | null>(null)
  let catChart: echarts.ECharts | null = null
  const renderCatChart = () => {
    if (!catChartRef.value) return
    if (!catChart) catChart = echarts.init(catChartRef.value as HTMLDivElement)
    let enabled = 0, disabled = 0, locked = 0
    dataSource.value.forEach((c) => {
      c.enabled ? enabled++ : disabled++
      if (c.subscription_locked) locked++
    })
    catChart.setOption({
      tooltip: { trigger: 'item' },
      legend: { bottom: 0 },
      series: [
        { type: 'pie', radius: '60%', data: [
          { name: '启用', value: enabled },
          { name: '禁用', value: disabled },
          { name: '订阅锁定', value: locked },
        ]},
      ],
    })
  }

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '名称', dataIndex: 'name', key: 'name' },
    { title: '描述', dataIndex: 'description', key: 'description' },
    { title: '启用', dataIndex: 'enabled', key: 'enabled' },
    { title: '订阅锁定', dataIndex: 'subscription_locked', key: 'subscription_locked' },
    { title: '操作', key: 'action', width: 160 },
  ]

  const editOpen = ref(false)
  const submitLoading = ref(false)
  const currentId = ref<number | null>(null)
  const formState = ref<any>({ name: '', description: '', enabled: true, subscription_locked: false })

  const handleCreate = () => {
    currentId.value = null
    formState.value = { name: '', description: '', enabled: true, subscription_locked: false }
    editOpen.value = true
  }

  const handleEdit = (record: Category) => {
    currentId.value = record.id
    formState.value = { name: record.name, description: record.description || '', enabled: record.enabled, subscription_locked: record.subscription_locked }
    editOpen.value = true
  }

  const handleDelete = async (record: Category) => {
    await deleteCategory(record.id)
    createMessage.success('删除成功')
    fetchList()
  }

  const handleSubmit = async () => {
    submitLoading.value = true
    try {
      if (currentId.value) {
        await updateCategory(currentId.value, { ...formState.value })
      } else {
        await createCategory({ ...formState.value })
      }
      createMessage.success('保存成功')
      editOpen.value = false
      await fetchList()
    } finally {
      submitLoading.value = false
    }
  }

  const handleCancel = () => (editOpen.value = false)

  const fetchList = async () => {
    try {
      loading.value = true
      const list = await getCategories()
      dataSource.value = list || []
      await nextTick()
      renderCatChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 