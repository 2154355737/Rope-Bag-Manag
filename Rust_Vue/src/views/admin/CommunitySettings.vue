<template>
  <div class="admin-page community-settings">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><Setting /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">社区设置</h1>
            <p class="page-subtitle">管理社区主页的显示内容和基本信息</p>
          </div>
        </div>
        <div class="header-actions">
          <el-button type="primary" @click="saveSettings" :loading="saving">
            <el-icon><Check /></el-icon>
            保存设置
          </el-button>
          <el-button @click="resetSettings" :disabled="saving">
            <el-icon><RefreshRight /></el-icon>
            重置
          </el-button>
        </div>
      </div>
    </div>

    <!-- 设置表单 -->
    <div class="settings-content">
      <el-card shadow="never">
        <template #header>
          <div class="card-header">
            <el-icon><House /></el-icon>
            <span>基本信息</span>
          </div>
        </template>
        
        <el-form
          ref="formRef"
          :model="settingsForm"
          :rules="formRules"
          label-width="120px"
          label-position="left"
        >
          <el-row :gutter="24">
            <el-col :span="12">
              <el-form-item label="网站标题" prop="site_title">
                <el-input
                  v-model="settingsForm.site_title"
                  placeholder="请输入网站标题"
                  clearable
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="网站副标题" prop="site_subtitle">
                <el-input
                  v-model="settingsForm.site_subtitle"
                  placeholder="请输入网站副标题"
                  clearable
                />
              </el-form-item>
            </el-col>
          </el-row>

          <el-form-item label="网站描述" prop="site_description">
            <el-input
              v-model="settingsForm.site_description"
              type="textarea"
              :rows="3"
              placeholder="请输入网站描述"
              show-word-limit
              maxlength="200"
            />
          </el-form-item>

          <el-form-item label="欢迎消息" prop="welcome_message">
            <el-input
              v-model="settingsForm.welcome_message"
              type="textarea"
              :rows="4"
              placeholder="请输入欢迎消息"
              show-word-limit
              maxlength="500"
            />
          </el-form-item>

          <el-form-item label="网站公告">
            <el-input
              v-model="settingsForm.announcement"
              type="textarea"
              :rows="3"
              placeholder="请输入网站公告（留空则不显示公告）"
              show-word-limit
              maxlength="300"
            />
            <div class="form-tip">
              公告将在社区主页顶部显示，留空则隐藏公告栏
            </div>
          </el-form-item>
        </el-form>
      </el-card>

      <el-card shadow="never" style="margin-top: 20px;">
        <template #header>
          <div class="card-header">
            <el-icon><Link /></el-icon>
            <span>联系方式</span>
          </div>
        </template>
        
        <el-form
          :model="settingsForm"
          :rules="formRules"
          label-width="120px"
          label-position="left"
        >
          <el-row :gutter="24">
            <el-col :span="12">
              <el-form-item label="联系邮箱" prop="contact_email">
                <el-input
                  v-model="settingsForm.contact_email"
                  placeholder="请输入联系邮箱"
                  clearable
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="GitHub 链接">
                <el-input
                  v-model="settingsForm.github_link"
                  placeholder="请输入GitHub项目链接（可选）"
                  clearable
                />
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="24">
            <el-col :span="12">
              <el-form-item label="QQ 群">
                <el-input
                  v-model="settingsForm.qq_group"
                  placeholder="请输入QQ群号（可选）"
                  clearable
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="微信群">
                <el-input
                  v-model="settingsForm.wechat_group"
                  placeholder="请输入微信群信息（可选）"
                  clearable
                />
              </el-form-item>
            </el-col>
          </el-row>
        </el-form>
      </el-card>

      <el-card shadow="never" style="margin-top: 20px;">
        <template #header>
          <div class="card-header">
            <el-icon><DocumentCopy /></el-icon>
            <span>页脚设置</span>
          </div>
        </template>
        
        <el-form
          :model="settingsForm"
          :rules="formRules"
          label-width="120px"
          label-position="left"
        >
          <el-form-item label="页脚文本" prop="footer_text">
            <el-input
              v-model="settingsForm.footer_text"
              type="textarea"
              :rows="2"
              placeholder="请输入页脚文本"
              show-word-limit
              maxlength="100"
            />
          </el-form-item>
        </el-form>
      </el-card>

      <!-- 预览区域 -->
      <el-card shadow="never" style="margin-top: 20px;">
        <template #header>
          <div class="card-header">
            <el-icon><View /></el-icon>
            <span>设置预览</span>
          </div>
        </template>
        
        <div class="preview-section">
          <div class="preview-item">
            <label>网站标题：</label>
            <span>{{ settingsForm.site_title || '未设置' }}</span>
          </div>
          <div class="preview-item">
            <label>网站副标题：</label>
            <span>{{ settingsForm.site_subtitle || '未设置' }}</span>
          </div>
          <div class="preview-item">
            <label>网站描述：</label>
            <span>{{ settingsForm.site_description || '未设置' }}</span>
          </div>
          <div class="preview-item" v-if="settingsForm.announcement">
            <label>网站公告：</label>
            <span class="announcement-preview">{{ settingsForm.announcement }}</span>
          </div>
          <div class="preview-item">
            <label>联系邮箱：</label>
            <span>{{ settingsForm.contact_email || '未设置' }}</span>
          </div>
        </div>
      </el-card>

      <!-- 主页显示配置 -->
      <el-card shadow="never" style="margin-top: 20px;">
        <template #header>
          <div class="card-header">
            <el-icon><Monitor /></el-icon>
            <span>主页显示配置</span>
          </div>
        </template>
        
        <el-form
          :model="settingsForm"
          label-width="120px"
          label-position="left"
        >
          <el-row :gutter="24">
            <el-col :span="12">
              <el-form-item label="首页主标题">
                <el-input
                  v-model="settingsForm.hero_title"
                  placeholder="请输入首页主标题"
                  clearable
                  maxlength="50"
                  show-word-limit
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="首页副标题">
                <el-input
                  v-model="settingsForm.hero_subtitle"
                  placeholder="请输入首页副标题"
                  clearable
                  maxlength="100"
                  show-word-limit
                />
              </el-form-item>
            </el-col>
          </el-row>

          <el-form-item label="主页布局选项">
            <el-checkbox-group v-model="settingsForm.homepage_sections">
              <el-checkbox label="hero_section">显示轮播横幅</el-checkbox>
              <el-checkbox label="stats_section">显示统计数据</el-checkbox>
              <el-checkbox label="popular_tags">显示热门标签</el-checkbox>
              <el-checkbox label="recent_resources">显示最新资源</el-checkbox>
              <el-checkbox label="community_posts">显示社区帖子</el-checkbox>
              <el-checkbox label="announcements">显示公告栏</el-checkbox>
            </el-checkbox-group>
            <div class="form-tip">
              选择在首页显示的内容模块
            </div>
          </el-form-item>

          <el-row :gutter="24">
            <el-col :span="12">
              <el-form-item label="每页资源数量">
                <el-input-number
                  v-model="settingsForm.resources_per_page"
                  :min="6"
                  :max="50"
                  :step="6"
                  placeholder="每页显示的资源数量"
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="每页帖子数量">
                <el-input-number
                  v-model="settingsForm.posts_per_page"
                  :min="5"
                  :max="30"
                  :step="5"
                  placeholder="每页显示的帖子数量"
                />
              </el-form-item>
            </el-col>
          </el-row>

          <el-form-item label="默认排序方式">
            <el-select v-model="settingsForm.default_sort" placeholder="选择默认排序方式">
              <el-option label="最新上传" value="latest" />
              <el-option label="最多下载" value="downloads" />
              <el-option label="最多评论" value="comments" />
              <el-option label="评分最高" value="rating" />
            </el-select>
          </el-form-item>
        </el-form>
      </el-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { 
  Setting, 
  Check, 
  RefreshRight, 
  House, 
  Link, 
  DocumentCopy, 
  View,
  Monitor
} from '@element-plus/icons-vue'
import { settingsApi, type CommunitySettings, type UpdateCommunitySettingsRequest } from '../../api/settings'

// 响应式数据
const loading = ref(false)
const saving = ref(false)
const formRef = ref()

// 表单数据
const settingsForm = reactive<CommunitySettings>({
  site_title: '',
  site_subtitle: '',
  site_description: '',
  welcome_message: '',
  announcement: '',
  footer_text: '',
  contact_email: '',
  github_link: '',
  qq_group: '',
  wechat_group: '',
  // 新增主页配置字段
  hero_title: '绳包管理器',
  hero_subtitle: '专业的资源管理与分享平台',
  homepage_sections: ['hero_section', 'stats_section', 'popular_tags', 'recent_resources', 'community_posts', 'announcements'],
  resources_per_page: 12,
  posts_per_page: 10,
  default_sort: 'latest'
})

// 原始数据（用于重置）
const originalSettings = reactive<CommunitySettings>({
  site_title: '',
  site_subtitle: '',
  site_description: '',
  welcome_message: '',
  announcement: '',
  footer_text: '',
  contact_email: '',
  github_link: '',
  qq_group: '',
  wechat_group: '',
  // 新增主页配置字段
  hero_title: '',
  hero_subtitle: '',
  homepage_sections: [],
  resources_per_page: 12,
  posts_per_page: 10,
  default_sort: 'latest'
})

// 表单验证规则
const formRules = {
  site_title: [
    { required: true, message: '请输入网站标题', trigger: 'blur' },
    { min: 1, max: 50, message: '标题长度应在1-50个字符之间', trigger: 'blur' }
  ],
  site_subtitle: [
    { required: true, message: '请输入网站副标题', trigger: 'blur' },
    { min: 1, max: 100, message: '副标题长度应在1-100个字符之间', trigger: 'blur' }
  ],
  site_description: [
    { required: true, message: '请输入网站描述', trigger: 'blur' },
    { min: 1, max: 200, message: '描述长度应在1-200个字符之间', trigger: 'blur' }
  ],
  welcome_message: [
    { required: true, message: '请输入欢迎消息', trigger: 'blur' },
    { min: 1, max: 500, message: '欢迎消息长度应在1-500个字符之间', trigger: 'blur' }
  ],
  contact_email: [
    { required: true, message: '请输入联系邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  footer_text: [
    { required: true, message: '请输入页脚文本', trigger: 'blur' },
    { min: 1, max: 100, message: '页脚文本长度应在1-100个字符之间', trigger: 'blur' }
  ]
}

// 方法
const loadSettings = async () => {
  loading.value = true
  try {
    const res = await settingsApi.getCommunitySettings()
    if (res.code === 0 && res.data) {
      Object.assign(settingsForm, res.data)
      Object.assign(originalSettings, res.data)
    } else {
      ElMessage.error(res.message || '加载设置失败')
    }
  } catch (error) {
    console.error('加载社区设置出错:', error)
    ElMessage.error('加载设置时发生错误')
  } finally {
    loading.value = false
  }
}

const saveSettings = async () => {
  if (!formRef.value) return
  
  try {
    await formRef.value.validate()
  } catch (error) {
    ElMessage.warning('请检查表单输入')
    return
  }
  
  saving.value = true
  try {
    // 构建更新请求，只发送有变化的字段
    const updateData: UpdateCommunitySettingsRequest = {}
    
    if (settingsForm.site_title !== originalSettings.site_title) {
      updateData.site_title = settingsForm.site_title
    }
    if (settingsForm.site_subtitle !== originalSettings.site_subtitle) {
      updateData.site_subtitle = settingsForm.site_subtitle
    }
    if (settingsForm.site_description !== originalSettings.site_description) {
      updateData.site_description = settingsForm.site_description
    }
    if (settingsForm.welcome_message !== originalSettings.welcome_message) {
      updateData.welcome_message = settingsForm.welcome_message
    }
    if (settingsForm.announcement !== originalSettings.announcement) {
      updateData.announcement = settingsForm.announcement || ''
    }
    if (settingsForm.footer_text !== originalSettings.footer_text) {
      updateData.footer_text = settingsForm.footer_text
    }
    if (settingsForm.contact_email !== originalSettings.contact_email) {
      updateData.contact_email = settingsForm.contact_email
    }
    if (settingsForm.github_link !== originalSettings.github_link) {
      updateData.github_link = settingsForm.github_link || ''
    }
    if (settingsForm.qq_group !== originalSettings.qq_group) {
      updateData.qq_group = settingsForm.qq_group || ''
    }
    if (settingsForm.wechat_group !== originalSettings.wechat_group) {
      updateData.wechat_group = settingsForm.wechat_group || ''
    }
    if (settingsForm.hero_title !== originalSettings.hero_title) {
      updateData.hero_title = settingsForm.hero_title
    }
    if (settingsForm.hero_subtitle !== originalSettings.hero_subtitle) {
      updateData.hero_subtitle = settingsForm.hero_subtitle
    }
    if (JSON.stringify(settingsForm.homepage_sections) !== JSON.stringify(originalSettings.homepage_sections)) {
      updateData.homepage_sections = settingsForm.homepage_sections
    }
    if (settingsForm.resources_per_page !== originalSettings.resources_per_page) {
      updateData.resources_per_page = settingsForm.resources_per_page
    }
    if (settingsForm.posts_per_page !== originalSettings.posts_per_page) {
      updateData.posts_per_page = settingsForm.posts_per_page
    }
    if (settingsForm.default_sort !== originalSettings.default_sort) {
      updateData.default_sort = settingsForm.default_sort
    }
    
    if (Object.keys(updateData).length === 0) {
      ElMessage.info('没有需要更新的设置')
      return
    }
    
    const res = await settingsApi.updateCommunitySettings(updateData)
    
    if (res.code === 0) {
      ElMessage.success('社区设置保存成功')
      Object.assign(originalSettings, settingsForm)
    } else {
      ElMessage.error(res.message || '保存设置失败')
    }
  } catch (error) {
    console.error('保存社区设置出错:', error)
    ElMessage.error('保存设置时发生错误')
  } finally {
    saving.value = false
  }
}

const resetSettings = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要重置所有设置到原始状态吗？',
      '重置确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    Object.assign(settingsForm, originalSettings)
    ElMessage.success('设置已重置')
  } catch (error) {
    // 用户取消操作
  }
}

// 初始化
onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.admin-page {
  padding: 24px;
  background: var(--bg-primary);
  min-height: 100vh;
}

/* 页面头部 */
.page-header {
  background: var(--bg-card);
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-light);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon {
  width: 64px;
  height: 64px;
  background: linear-gradient(135deg, #409eff, #66b1ff);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.page-title {
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
}

.page-subtitle {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.header-actions {
  display: flex;
  gap: 12px;
}

/* 设置内容 */
.settings-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
  color: var(--text-primary);
}

.form-tip {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}

/* 预览区域 */
.preview-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.preview-item {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  padding: 8px;
  background: var(--bg-light);
  border-radius: 6px;
}

.preview-item label {
  font-weight: 600;
  color: var(--text-primary);
  min-width: 80px;
  flex-shrink: 0;
}

.preview-item span {
  color: var(--text-secondary);
  word-break: break-all;
}

.announcement-preview {
  background: linear-gradient(135deg, #f0f9ff, #e0f2fe);
  padding: 8px 12px;
  border-radius: 6px;
  border-left: 4px solid #0ea5e9;
}

/* 深色模式适配 */
.dark .settings-content .el-card {
  background: #1f2937;
}

.dark .preview-item {
  background: #374151;
}

.dark .announcement-preview {
  background: linear-gradient(135deg, #1e3a8a, #1e40af);
  border-left-color: #3b82f6;
}
</style> 