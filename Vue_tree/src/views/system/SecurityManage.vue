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
            <FormItem label="同IP下载频率限制">
              <Input v-model:value="dsConfig.max_downloads_per_ip" placeholder="如: 10/分钟" />
            </FormItem>
            <FormItem label="下载窗口(分钟)">
              <Input v-model:value="dsConfig.window_minutes" />
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
          <FormItem label="启用速率限制">
            <Switch v-model:checked="secConfig.enable_rate_limiting" />
          </FormItem>
          <FormItem label="启用IP白名单">
            <Switch v-model:checked="secConfig.enable_ip_whitelist" />
          </FormItem>
          <FormItem label="最大登录失败次数">
            <Input v-model:value="secConfig.max_login_attempts" />
          </FormItem>
          <FormItem label="锁定时长(分钟)">
            <Input v-model:value="secConfig.lockout_duration" />
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
          <Input v-model:value="banForm.ip" placeholder="IP地址" style="width: 200px" />
          <Input v-model:value="banForm.reason" placeholder="原因(可选)" style="width: 220px" />
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
          <Input v-model:value="whiteForm.ip" placeholder="IP地址" style="width: 200px" />
          <Input v-model:value="whiteForm.note" placeholder="备注(可选)" style="width: 220px" />
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

  const banForm = ref<any>({ ip: '', reason: '' })
  const whiteForm = ref<any>({ ip: '', note: '' })

  const statsText = computed(() => (stats.value ? JSON.stringify(stats.value) : '...'))

  const anomalyColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: 'IP', dataIndex: 'ip', key: 'ip' },
    { title: '原因', dataIndex: 'reason', key: 'reason' },
    { title: '时间', dataIndex: 'created_at', key: 'created_at' },
  ]

  const banColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: 'IP', dataIndex: 'ip', key: 'ip' },
    { title: '原因', dataIndex: 'reason', key: 'reason' },
    { title: '操作', key: 'action', width: 100 },
  ]

  const whiteColumns = [
    { title: 'ID', dataIndex: 'id', key: 'id' },
    { title: 'IP', dataIndex: 'ip', key: 'ip' },
    { title: '备注', dataIndex: 'note', key: 'note' },
    { title: '操作', key: 'action', width: 100 },
  ]

  const anChartRef = ref<HTMLDivElement | null>(null)
  let anChart: echarts.ECharts | null = null
  const renderAnChart = () => {
    if (!anChartRef.value) return
    if (!anChart) anChart = echarts.init(anChartRef.value as HTMLDivElement)
    const reasonCount: Record<string, number> = {}
    anomalies.value.forEach((a: any) => {
      const r = a.reason || '其他'
      reasonCount[r] = (reasonCount[r] || 0) + 1
    })
    const top = Object.entries(reasonCount).sort((a: any, b: any) => b[1] - a[1]).slice(0, 6)
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
    anomalies.value = (await getDownloadAnomalies({ page: 1, page_size: 10 }).catch(() => ({ list: [] }))).list || []
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
    bans.value = (await getIpBans({ page: 1, page_size: 10 }).catch(() => ({ list: [] }))).list || []
    whitelist.value = (await getIpWhitelist({ page: 1, page_size: 10 }).catch(() => ({ list: [] }))).list || []
  }

  const addBan = async () => {
    if (!banForm.value.ip) return
    await createIpBan({ ip: banForm.value.ip, reason: banForm.value.reason })
    banForm.value = { ip: '', reason: '' }
    await fetchBanAndWhite()
  }

  const removeBan = async (row: any) => {
    await deleteIpBan(row.id)
    await fetchBanAndWhite()
  }

  const addWhitelist = async () => {
    if (!whiteForm.value.ip) return
    await createIpWhitelist({ ip: whiteForm.value.ip, note: whiteForm.value.note })
    whiteForm.value = { ip: '', note: '' }
    await fetchBanAndWhite()
  }

  const removeWhitelist = async (row: any) => {
    await deleteIpWhitelist(row.id)
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