<template>
  <PageWrapper title="资源包管理" content="系统资源包列表">
    <Card :loading="loading" class="!mb-4">
      <Button type="primary" @click="handleCreate">新建资源</Button>
    </Card>
    <Card :loading="loading">
      <Table
        :columns="columns"
        :data-source="dataSource"
        :pagination="{ pageSize: 20 }"
        row-key="id"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'status'">
            <Tag :color="statusColor(record.status)">{{ record.status }}</Tag>
          </template>
          <template v-else-if="column.key === 'action'">
            <Space>
              <Button type="link" @click="handleEdit(record)">编辑</Button>
              <Button type="link" @click="() => handleReview(record, 'Active')">通过</Button>
              <Button type="link" danger @click="() => handleReview(record, 'Rejected')">驳回</Button>
              <Button type="link" danger @click="() => handleDelete(record)">删除</Button>
            </Space>
          </template>
        </template>
      </Table>
    </Card>

    <Modal
      v-model:visible="editOpen"
      :title="currentId ? '编辑资源' : '新建资源'"
      :confirm-loading="submitLoading"
      @ok="handleSubmit"
      @cancel="handleCancel"
      destroy-on-close
      width="720px"
    >
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 16 }">
        <FormItem label="标题">
          <Input v-model:value="formState.name" placeholder="请输入标题" />
        </FormItem>
        <FormItem label="版本">
          <Input v-model:value="formState.version" placeholder="请输入版本" />
        </FormItem>
        <FormItem label="内容">
          <Input.TextArea v-model:value="formState.description" :rows="4" placeholder="请输入内容" />
        </FormItem>
        <FormItem label="分类">
          <Select v-model:value="formState.category_id" :options="categoryOptions" placeholder="请选择分类" />
        </FormItem>
        <FormItem label="标签">
          <Select v-model:value="formState.tags" mode="tags" placeholder="输入后回车添加标签" />
        </FormItem>
        <FormItem label="置顶">
          <Switch v-model:checked="formState.is_pinned" />
        </FormItem>
        <FormItem label="精选">
          <Switch v-model:checked="formState.is_featured" />
        </FormItem>
        <FormItem label="审核状态" v-if="currentId">
          <Select v-model:value="formState.status" :options="statusOptions" />
        </FormItem>
        <FormItem label="审核说明" v-if="currentId">
          <Input v-model:value="formState.review_comment" placeholder="审核备注" />
        </FormItem>
        <FormItem label="上传文件" v-if="currentId">
          <input type="file" @change="onFileChange" />
        </FormItem>
      </Form>
    </Modal>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue'
  import { Card, Table, Modal, Form, Input, Select, Switch, Button, Tag, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getPackages, createPackage, updatePackage, deletePackage, uploadPackageFile, reviewPackage, type PackageItem } from '/@/api/packages'
  import { getCategories } from '/@/api/dashboard'
  import { useMessage } from '/@/hooks/web/useMessage'

  const { createMessage } = useMessage()

  // FormItem 别名，确保样式
  const FormItem = Form.Item

  const loading = ref(true)
  const dataSource = ref<PackageItem[]>([])
  const categoryOptions = ref<any[]>([])

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '标题', dataIndex: 'name', key: 'name' },
    { title: '作者', dataIndex: 'author', key: 'author' },
    { title: '分类', dataIndex: 'category_id', key: 'category_id' },
    { title: '状态', dataIndex: 'status', key: 'status' },
    { title: '下载', dataIndex: 'download_count', key: 'download_count' },
    { title: '点赞', dataIndex: 'like_count', key: 'like_count' },
    { title: '操作', key: 'action', width: 220 },
  ]

  const statusColor = (s: string) => ({ Active: 'success', Pending: 'warning', Rejected: 'error', Inactive: 'default', Deleted: 'default' }[s] || 'default')

  const editOpen = ref(false)
  const submitLoading = ref(false)
  const currentId = ref<number | null>(null)
  const fileRef = ref<File | null>(null)

  const formState = ref<any>({
    name: '',
    version: '',
    description: '',
    category_id: undefined,
    tags: [] as string[],
    is_pinned: false,
    is_featured: false,
    status: 'Pending',
    review_comment: '',
  })

  const statusOptions = [
    { label: '待审核', value: 'Pending' },
    { label: '已发布', value: 'Active' },
    { label: '已驳回', value: 'Rejected' },
    { label: '下线', value: 'Inactive' },
  ]

  const onFileChange = (e: any) => {
    const files = e?.target?.files
    fileRef.value = files && files[0] ? files[0] : null
  }

  const handleCreate = () => {
    currentId.value = null
    formState.value = { name: '', version: '', description: '', category_id: undefined, tags: [], is_pinned: false, is_featured: false, status: 'Pending', review_comment: '' }
    editOpen.value = true
  }

  const handleEdit = (record: PackageItem) => {
    currentId.value = record.id
    formState.value = {
      name: record.name,
      version: record.version,
      description: record.description,
      category_id: record.category_id,
      tags: record.tags || [],
      is_pinned: record.is_pinned,
      is_featured: record.is_featured,
      status: record.status,
      review_comment: record.review_comment || '',
    }
    editOpen.value = true
  }

  const handleDelete = async (record: PackageItem) => {
    await deletePackage(record.id)
    createMessage.success('删除成功')
    fetchList()
  }

  const handleReview = async (record: PackageItem, status: 'Active' | 'Rejected') => {
    await reviewPackage(record.id, { status, review_comment: formState.value.review_comment || '' })
    createMessage.success('审核已提交')
    fetchList()
  }

  const handleSubmit = async () => {
    submitLoading.value = true
    try {
      if (currentId.value) {
        await updatePackage(currentId.value, { ...formState.value })
        if (fileRef.value) {
          await uploadPackageFile(currentId.value, fileRef.value)
        }
      } else {
        await createPackage({ ...formState.value, author: 'admin' })
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
      const [res, cats] = await Promise.all([
        getPackages({ page: 1, page_size: 20 }),
        getCategories(),
      ])
      dataSource.value = res.list || []
      categoryOptions.value = (cats || []).map((c: any) => ({ label: c.name, value: c.id }))
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchList)
</script> 