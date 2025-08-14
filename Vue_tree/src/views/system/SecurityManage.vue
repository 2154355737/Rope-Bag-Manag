<template>
  <PageWrapper title="安全管理" content="下载防刷、IP封禁、白名单与安全配置">
    <div class="md:flex md:space-x-4">
      <Card class="md:w-1/2 w-full !mb-4" :loading="loading">
        <template #title>下载防刷统计</template>
        <div class="text-secondary">{{ statsText }}</div>
        <div class="mt-2">
          <Button size="small" @click="fetchDownloadStats">刷新统计</Button>
        </div>
        <div class="mt-4">
          <Form :model="dsConfig" :label-col="{ span: 10 }" :wrapper-col="{ span: 12 }">
            <FormItem label="同IP下载频率限制(每小时)">
              <Input v-model:value="dsConfig.max_downloads_per_ip_per_hour" placeholder="如: 20" />
            </FormItem>
            <FormItem label="同用户下载频率(每小时)">
              <Input v-model:value="dsConfig.max_downloads_per_user_per_hour" />
            </FormItem>
            <FormItem label="资源日下载上限">
              <Input v-model:value="dsConfig.max_downloads_per_resource_per_day" />
            </FormItem>
            <FormItem>
              <Button type="primary" size="small" @click="saveDownloadConfig">保存下载防刷配置</Button>
            </FormItem>
          </Form>
        </div>
      </Card>

      <Card class="md:w-1/2 w-full !mb-4" :loading="loading">
        <template #title>安全配置</template>
        <Form :model="secConfig" :label-col="{ span: 8 }" :wrapper-col="{ span: 14 }">
          <FormItem label="启用自动封禁">
            <Switch v-model:checked="secConfig.enable_auto_ban" />
          </FormItem>
          <FormItem label="封禁阈值(异常次数)">
            <Input v-model:value="secConfig.auto_ban_threshold" />
          </FormItem>
          <FormItem label="默认封禁时长(小时)">
            <Input v-model:value="secConfig.ban_duration_hours" />
          </FormItem>
          <FormItem label="启用IP白名单">
            <Switch v-model:checked="secConfig.enable_ip_whitelist" />
          </FormItem>
          <FormItem>
            <Button type="primary" size="small" @click="saveSecurityConfig">保存安全配置</Button>
          </FormItem>
        </Form>
      </Card>
    </div>

    <Card class="!mb-2" :loading="loading">
      <template #title>异常原因分布</template>
      <div ref="anChartRef" style="width: 100%; height: 220px"></div>
    </Card>

    <Card class="!mb-4" :loading="loading">
      <template #title>异常下载记录</template>
      <Table :columns="anomalyColumns" :data-source="anomalies" row-key="id" :pagination="{ pageSize: 10 }" />
    </Card>

    <div class="md:flex md:space-x-4">
      <Card class="md:w-1/2 w-full !mb-4" :loading="loading">
        <template #title>IP 封禁</template>
        <Space class="!mb-2">
          <Input v-model:value="banForm.ip_address" placeholder="IP地址" style="width: 200px" />
          <Input v-model:value="banForm.reason" placeholder="原因(可选)" style="width: 220px" />
          <Input v-model:value="banForm.duration_hours" placeholder="时长(小时 可选)" style="width: 140px" />
          <Button type="primary" size="small" @click="addBan">封禁</Button>
        </Space>
        <Table :columns="banColumns" :data-source="bans" row-key="id" :pagination="{ pageSize: 10 }">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'action'">
              <Button type="link" danger @click="() => removeBan(record)">解除</Button>
            </template>
          </template>
        </Table>
      </Card>

      <Card class="md:w-1/2 w-full !mb-4" :loading="loading">
        <template #title>IP 白名单</template>
        <Space class="!mb-2">
          <Input v-model:value="whiteForm.ip_address" placeholder="IP地址" style="width: 200px" />
          <Input v-model:value="whiteForm.description" placeholder="备注(可选)" style="width: 220px" />
          <Button type="primary" size="small" @click="addWhitelist">添加</Button>
        </Space>
        <Table :columns="whiteColumns" :data-source="whitelist" row-key="id" :pagination="{ pageSize: 10 }">
          <template #bodyCell="{ column, record }">
            <template v-if="column.key === 'action'">
              <Button type="link" danger @click="() => removeWhitelist(record)">删除</Button>
            </template>
          </template>
        </Table>
      </Card>
    </div>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted, computed, nextTick } from 'vue'
  import { Card, Table, Button, Space, Form, Input, Switch } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import {
    getDownloadSecurityStats, getDownloadSecurityConfig, updateDownloadSecurityConfig, getDownloadAnomalies,
    getIpBans, createIpBan, deleteIpBan, getIpWhitelist, createIpWhitelist, deleteIpWhitelist,
    getSecurityConfig, updateSecurityConfig
  } from '/@/api/security'
  import * as echarts from 'echarts'

  const FormItem = Form.Item

  const loading = ref(true)
  const stats = ref<any>(null)
  const dsConfig = ref<any>({})
  const secConfig = ref<any>({})
  const anomalies = ref<any[]>([])
  const bans = ref<any[]>([])
  const whitelist = ref<any[]>([])
  const anomalyStats = ref<any>(null)

  const banForm = ref<any>({ ip_address: '', reason: '', duration_hours: '' })
  const whiteForm = ref<any>({ ip_address: '', description: '' })

  const statsText = computed(() => (stats.value ? JSON.stringify(stats.value) : '...'))

  const anomalyColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: 'IP', dataIndex: 'ip_address', key: 'ip_address' },
    { title: '原因', dataIndex: 'anomaly_type', key: 'anomaly_type' },
    { title: '时间', dataIndex: 'created_at', key: 'created_at' },
  ]

  const banColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: 'IP', dataIndex: 'ip_address', key: 'ip_address' },
    { title: '原因', dataIndex: 'reason', key: 'reason' },
    { title: '操作', key: 'action', width: 100 },
  ]

  const whiteColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: 'IP', dataIndex: 'ip_address', key: 'ip_address' },
    { title: '备注', dataIndex: 'description', key: 'description' },
    { title: '操作', key: 'action', width: 100 },
  ]

  const anChartRef = ref<HTMLDivElement | null>(null)
  let anChart: echarts.ECharts | null = null
  const renderAnChart = () => {
    if (!anChartRef.value) return
    if (!anChart) anChart = echarts.init(anChartRef.value as HTMLDivElement)
    // 优先使用统计接口 by_type 数据
    const byType = (anomalyStats.value && anomalyStats.value.by_type) || {}
    const entries = Object.entries(byType as Record<string, number>)
    const top = entries.sort((a: any, b: any) => b[1] - a[1]).slice(0, 6)
    anChart.setOption({
      tooltip: { trigger: 'axis' },
      grid: { left: 40, right: 10, top: 10, bottom: 30 },
      xAxis: { type: 'category', data: top.map((i) => i[0]) },
      yAxis: { type: 'value' },
      series: [{ type: 'bar', data: top.map((i) => i[1]) }],
    })
  }

  const fetchDownloadStats = async () => {
    stats.value = await getDownloadSecurityStats().catch(() => null)
    dsConfig.value = await getDownloadSecurityConfig().catch(() => ({}))
    const an = await getDownloadAnomalies({ page: 1, page_size: 10 }).catch(() => ({} as any))
    anomalyStats.value = an?.data || an || null
  }

  const saveDownloadConfig = async () => {
    await updateDownloadSecurityConfig(dsConfig.value)
  }

  const fetchSecurityConfig = async () => {
    secConfig.value = await getSecurityConfig().catch(() => ({}))
  }

  const saveSecurityConfig = async () => {
    await updateSecurityConfig(secConfig.value)
  }

  const fetchBanAndWhite = async () => {
    const bansRes = await getIpBans({ page: 1, page_size: 10 }).catch(() => ({ data: { bans: [] } }))
    bans.value = bansRes?.bans || bansRes?.data?.bans || []
    const whiteRes = await getIpWhitelist({ page: 1, page_size: 10 }).catch(() => ({ data: { whitelist: [] } }))
    whitelist.value = whiteRes?.whitelist || whiteRes?.data?.whitelist || []
  }

  const addBan = async () => {
    if (!banForm.value.ip_address) return
    await createIpBan({ ip_address: banForm.value.ip_address, reason: banForm.value.reason, duration_hours: Number(banForm.value.duration_hours) || undefined })
    banForm.value = { ip_address: '', reason: '', duration_hours: '' }
    await fetchBanAndWhite()
  }

  const removeBan = async (row: any) => {
    await deleteIpBan(row.ip_address)
    await fetchBanAndWhite()
  }

  const addWhitelist = async () => {
    if (!whiteForm.value.ip_address) return
    await createIpWhitelist({ ip_address: whiteForm.value.ip_address, description: whiteForm.value.description })
    whiteForm.value = { ip_address: '', description: '' }
    await fetchBanAndWhite()
  }

  const removeWhitelist = async (row: any) => {
    await deleteIpWhitelist(row.ip_address)
    await fetchBanAndWhite()
  }

  const init = async () => {
    try {
      loading.value = true
      await Promise.all([fetchDownloadStats(), fetchSecurityConfig(), fetchBanAndWhite()])
      await nextTick()
      renderAnChart()
    } finally {
      loading.value = false
    }
  }

  onMounted(init)
</script> 