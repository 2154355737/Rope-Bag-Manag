<template>
  <PageWrapper title="标签管理" content="标签列表与编辑">
    <Card class="!mb-4">
      <Space>
        <Input v-model:value="search" placeholder="搜索标签" style="width: 240px" />
        <Button type="primary" @click="fetchList">搜索</Button>
        <Button @click="handleCreate">新建标签</Button>
      </Space>
    </Card>
    <Card :loading="loading">
      <Table :columns="columns" :data-source="dataSource" :pagination="{ pageSize: 20 }" row-key="id">
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'action'">
            <Space>
              <Button type="link" @click="handleEdit(record)">编辑</Button>
              <Button type="link" danger @click="() => handleDelete(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>

    <Modal v-model:visible="editOpen" :title="currentId ? '编辑标签' : '新建标签'" :confirm-loading="submitLoading" @ok="handleSubmit" @cancel="() => (editOpen = false)" destroy-on-close>
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 16 }">
        <FormItem label="名称">
          <Input v-model:value="formState.name" placeholder="请输入名称" />
        </FormItem>
        <FormItem label="描述">
          <Input v-model:value="formState.description" placeholder="请输入描述" />
        </FormItem>
      </Form>
    </Modal>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue'
  import { Card, Table, Modal, Form, Input, Button, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getTags, createTag, updateTag, deleteTag, type TagItem } from '/@/api/tags'

  const FormItem = Form.Item

  const loading = ref(true)
  const dataSource = ref<TagItem[]>([])
  const search = ref('')

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '名称', dataIndex: 'name', key: 'name' },
    { title: '使用次数', dataIndex: 'use_count', key: 'use_count' },
    { title: '操作', key: 'action', width: 160 },
  ]

  const editOpen = ref(false)
  const submitLoading = ref(false)
  const currentId = ref<number | null>(null)
  const formState = ref<any>({ name: '', description: '' })

  const handleCreate = () => {
    currentId.value = null
    formState.value = { name: '', description: '' }
    editOpen.value = true
  }

  const handleEdit = (record: TagItem) => {
    currentId.value = record.id
    formState.value = { name: record.name, description: record.description || '' }
    editOpen.value = true
  }

  const handleDelete = async (record: TagItem) => {
    await deleteTag(record.id)
    await fetchList()
  }

  const handleSubmit = async () => {
    submitLoading.value = true
    try {
      if (currentId.value) {
        await updateTag(currentId.value, { ...formState.value })
      } else {
        await createTag({ ...formState.value })
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
      const list = await getTags({ page: 1, page_size: 20, search: search.value || undefined })
      dataSource.value = list.list || []
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 