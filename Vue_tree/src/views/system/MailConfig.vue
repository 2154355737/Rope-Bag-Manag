<template>
  <PageWrapper title="邮箱配置" content="SMTP 邮件服务配置与测试">
    <Card :loading="loading">
      <Form :model="formState" :label-col="{ span: 6 }" :wrapper-col="{ span: 14 }">
        <FormItem label="SMTP 主机">
          <Input v-model:value="formState.smtp_host" />
        </FormItem>
        <FormItem label="SMTP 端口">
          <Input v-model:value="formState.smtp_port" />
        </FormItem>
        <FormItem label="账号">
          <Input v-model:value="formState.username" />
        </FormItem>
        <FormItem label="密码">
          <Input v-model:value="formState.password" type="password" />
        </FormItem>
        <FormItem label="发件人">
          <Input v-model:value="formState.from" />
        </FormItem>
        <FormItem label="SSL/TLS">
          <Switch v-model:checked="formState.enable_ssl" />
        </FormItem>
        <FormItem label="测试邮箱">
          <Input v-model:value="testEmail" placeholder="you@example.com" />
        </FormItem>
        <FormItem>
          <Space>
            <Button type="primary" @click="handleSave">保存</Button>
            <Button @click="handleTest">发送测试邮件</Button>
          </Space>
        </FormItem>
      </Form>
    </Card>
  </PageWrapper>
</template>

<script lang="ts" setup>
  import { ref, onMounted } from 'vue'
  import { Card, Form, Input, Switch, Button, Space } from 'ant-design-vue'
  import { PageWrapper } from '/@/components/Page'
  import { defHttp } from '/@/utils/http/axios'
  import { useMessage } from '/@/hooks/web/useMessage'

  const { createMessage } = useMessage()
  const FormItem = Form.Item

  const loading = ref(true)
  const formState = ref<any>({ smtp_host: '', smtp_port: 465, username: '', password: '', from: '', enable_ssl: true })
  const testEmail = ref('')

  const handleSave = async () => {
    await defHttp.post({ url: '/api/v1/admin/mail-settings', data: formState.value })
    createMessage.success('保存成功，已热更新邮件服务')
  }

  const handleTest = async () => {
    if (!testEmail.value) return
    await defHttp.post({ url: '/api/v1/admin/test-email', data: { email: testEmail.value } })
    createMessage.success('测试邮件已发送')
  }

  const fetchConfig = async () => {
    try {
      loading.value = true
      const res = await defHttp.get<any>({ url: '/api/v1/admin/mail-settings' })
      formState.value = res?.data || res || formState.value
    } finally {
      loading.value = false
    }
  }

  onMounted(fetchConfig)
</script> 