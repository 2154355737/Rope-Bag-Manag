<template>
  <PageWrapper title="用户管理" content="系统用户列表">
    <Card :loading="loading" class="!mb-4">
      <div ref="roleChartRef" style="width: 100%; height: 260px"></div>
    </Card>
    <Card :loading="loading">
      <Table
        :columns="columns"
        :data-source="dataSource"
        :pagination="{ pageSize: 20 }"
        row-key="id"
      >
        <template #bodyCell="{ column, record }">
          <template v-if="column.key === 'action'">
            <Button type="link" @click="handleEdit(record)">编辑</Button>
          </template>
        </template>
      </Table>
    </Card>

    <Modal
      v-model:visible="editOpen"
      title="编辑用户"
      :confirm-loading="submitLoading"
      @ok="handleSubmit"
      @cancel="handleCancel"
      destroy-on-close
    >
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 16 }">
        <FormItem label="用户名">
          <Input v-model:value="formState.username" disabled />
        </FormItem>
        <FormItem label="昵称">
          <Input v-model:value="formState.nickname" placeholder="请输入昵称" />
        </FormItem>
        <FormItem label="邮箱">
          <Input v-model:value="formState.email" placeholder="请输入邮箱" />
        </FormItem>
        <FormItem label="QQ">
          <Input v-model:value="formState.qq_number" placeholder="请输入QQ号" />
        </FormItem>
        <FormItem label="角色">
          <Select v-model:value="formState.role" :options="roleOptions" placeholder="请选择角色" />
        </FormItem>
        <FormItem label="状态">
          <Select v-model:value="formState.ban_status" :options="banStatusOptions" placeholder="请选择状态" />
        </FormItem>
        <FormItem label="封禁原因" v-if="formState.ban_status !== 'normal'">
          <Input v-model:value="formState.ban_reason" placeholder="请输入封禁原因" />
        </FormItem>
        <FormItem label="星级">
          <InputNumber v-model:value="formState.star" :min="0" :max="10" />
        </FormItem>
      </Form>
    </Modal>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, nextTick, watch } from 'vue'
  import { Card, Table, Modal, Form, Input, Select, InputNumber, Button } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { getUsers, updateUser, type UserItem } from '/@/api/users'
  import { useMessage } from '/@/hooks/web/useMessage'
  import * as echarts from 'echarts'

  const { createMessage } = useMessage()

  // 使用 Ant Design Vue 的表单项别名以启用样式
  const FormItem = Form.Item

  const loading = ref(true)
  const dataSource = ref<UserItem[]>([])

  const roleChartRef = ref<HTMLDivElement | null>(null)
  let roleChart: echarts.ECharts | null = null

  const renderRoleChart = () => {
    if (!roleChartRef.value) return
    if (!roleChart) roleChart = echarts.init(roleChartRef.value as HTMLDivElement)
    const roleCount: Record<string, number> = {}
    dataSource.value.forEach((u: any) => {
      const r = u.role || 'user'
      roleCount[r] = (roleCount[r] || 0) + 1
    })
    const data = Object.keys(roleCount).map((k) => ({ name: k, value: roleCount[k] }))
    roleChart.setOption({
      tooltip: { trigger: 'item' },
      legend: { bottom: 0 },
      series: [
        {
          type: 'pie',
          radius: '60%',
          data,
          emphasis: { itemStyle: { shadowBlur: 10, shadowOffsetX: 0, shadowColor: 'rgba(0, 0, 0, 0.5)' } },
        },
      ],
    })
  }

  const columns = [
    { title: 'ID', dataIndex: 'id', key: 'id', width: 80 },
    { title: '用户名', dataIndex: 'username', key: 'username' },
    { title: '昵称', dataIndex: 'nickname', key: 'nickname' },
    { title: '邮箱', dataIndex: 'email', key: 'email' },
    { title: '角色', dataIndex: 'role', key: 'role' },
    { title: '状态', dataIndex: 'ban_status', key: 'ban_status' },
    { title: '创建时间', dataIndex: 'created_at', key: 'created_at' },
    { title: '操作', key: 'action', width: 120 },
  ]

  const editOpen = ref(false)
  const submitLoading = ref(false)
  const currentId = ref<number | null>(null)
  const formState = ref<any>({
    id: 0,
    username: '',
    nickname: '',
    email: '',
    role: '',
    ban_status: 'normal',
    ban_reason: '',
    qq_number: '',
    star: 0,
  })

  const roleOptions = [
    { label: '管理员', value: 'admin' },
    { label: '版主', value: 'moderator' },
    { label: '元老', value: 'elder' },
    { label: '用户', value: 'user' },
  ]

  const banStatusOptions = [
    { label: '正常', value: 'normal' },
    { label: '禁言', value: 'suspended' },
    { label: '封禁', value: 'banned' },
  ]

  const handleEdit = (record: UserItem) => {
    currentId.value = record.id
    formState.value = {
      id: record.id,
      username: record.username,
      nickname: record.nickname || '',
      email: record.email || '',
      role: record.role,
      ban_status: record.ban_status || 'normal',
      ban_reason: record.ban_reason || '',
      qq_number: (record as any).qq_number || '',
      star: record.star || 0,
    }
    editOpen.value = true
  }

  const handleCancel = () => {
    editOpen.value = false
  }

  const handleSubmit = async () => {
    if (!currentId.value) return
    submitLoading.value = true
    try {
      const payload: any = {
        email: formState.value.email,
        nickname: formState.value.nickname,
        role: formState.value.role,
        ban_status: formState.value.ban_status,
        ban_reason: formState.value.ban_reason,
        qq_number: formState.value.qq_number,
        star: formState.value.star,
      }
      await updateUser(currentId.value, payload)
      createMessage.success('保存成功')
      editOpen.value = false
      await fetchList()
    } catch (e) {
      // 已有全局错误提示
    } finally {
      submitLoading.value = false
    }
  }

  const fetchList = async () => {
    try {
      const res = await getUsers({ page: 1, page_size: 20 })
      dataSource.value = res.list || []
      await nextTick()
      renderRoleChart()
    } finally {
      loading.value = false
    }
  }

  watch(dataSource, () => nextTick().then(renderRoleChart))

  onMounted(fetchList)
</script> 