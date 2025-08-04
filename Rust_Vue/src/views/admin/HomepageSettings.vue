<template>
  <div class="admin-page homepage-settings">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-content">
        <div class="header-left">
          <div class="header-icon">
            <el-icon :size="32"><House /></el-icon>
          </div>
          <div class="header-info">
            <h1 class="page-title">主页设置</h1>
            <p class="page-subtitle">配置网站主页的显示内容和样式</p>
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
      <el-card>
        <template #header>
          <span>英雄区域设置</span>
        </template>
        <el-form :model="settings" label-width="120px">
          <el-form-item label="主标题">
            <el-input 
              v-model="settings.hero_title" 
              placeholder="请输入主页主标题"
              maxlength="50"
              show-word-limit
            />
            <div class="form-tip">显示在主页顶部的主标题</div>
          </el-form-item>
          
          <el-form-item label="副标题">
            <el-input 
              v-model="settings.hero_subtitle" 
              placeholder="请输入主页副标题"
              maxlength="100"
              show-word-limit
            />
            <div class="form-tip">显示在主标题下方的描述文字</div>
          </el-form-item>
        </el-form>
      </el-card>

      <el-card style="margin-top: 20px;">
        <template #header>
          <span>页脚设置</span>
        </template>
        <el-form :model="settings" label-width="120px">
          <el-form-item label="版权信息">
            <el-input 
              v-model="settings.copyright_text" 
              placeholder="请输入版权信息"
              maxlength="200"
              show-word-limit
            />
            <div class="form-tip">显示在页面底部的版权信息</div>
          </el-form-item>
          
          <el-form-item label="页脚链接">
            <div class="footer-links-editor">
              <div v-for="(group, groupKey) in footerLinksData" :key="groupKey" class="link-group-editor">
                <el-card shadow="never" style="margin-bottom: 15px;">
                  <template #header>
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                      <span>{{ group.title }}</span>
                      <el-button type="danger" size="small" @click="removeGroup(groupKey)">
                        删除分组
                      </el-button>
                    </div>
                  </template>
                  
                  <el-form-item label="分组标题">
                    <el-input v-model="group.title" placeholder="请输入分组标题" />
                  </el-form-item>
                  
                  <div v-for="(link, linkIndex) in group.links" :key="linkIndex" class="link-item">
                    <div style="display: flex; gap: 10px; align-items: center; margin-bottom: 10px;">
                      <el-input v-model="link.text" placeholder="链接文字" style="flex: 1;" />
                      <el-input v-model="link.url" placeholder="链接地址" style="flex: 1;" />
                      <el-button type="danger" size="small" @click="removeLink(group, linkIndex)">
                        删除
                      </el-button>
                    </div>
                  </div>
                  
                  <el-button type="primary" size="small" @click="addLink(group)">
                    添加链接
                  </el-button>
                </el-card>
              </div>
              
              <el-button type="success" @click="addGroup">
                添加新分组
              </el-button>
            </div>
            <div class="form-tip">配置页脚的链接分组和链接项</div>
          </el-form-item>
        </el-form>
      </el-card>

      <el-card style="margin-top: 20px;">
        <template #header>
          <span>SEO设置</span>
        </template>
        <el-form :model="settings" label-width="120px">
          <el-form-item label="网站关键词">
            <el-input 
              v-model="settings.seo_keywords" 
              type="textarea"
              :rows="2"
              placeholder="请输入网站关键词，用逗号分隔"
              maxlength="200"
              show-word-limit
            />
          </el-form-item>
          
          <el-form-item label="网站描述">
            <el-input 
              v-model="settings.seo_description" 
              type="textarea"
              :rows="3"
              placeholder="请输入网站描述"
              maxlength="300"
              show-word-limit
            />
          </el-form-item>
          
          <el-form-item label="作者信息">
            <el-input 
              v-model="settings.seo_author" 
              placeholder="请输入网站作者"
              maxlength="50"
            />
          </el-form-item>
        </el-form>
      </el-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { House, Check, RefreshRight } from '@element-plus/icons-vue'
import { settingsApi } from '../../api/settings'

// 响应式数据
const saving = ref(false)

const settings = reactive({
  hero_title: '绳包管理器',
  hero_subtitle: '专业的资源管理与分享平台',
  copyright_text: '© 2024 绳包管理器. All rights reserved.',
  footer_links: '{}',
  seo_keywords: '绳包管理器,资源管理,文件分享,社区',
  seo_description: '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。',
  seo_author: '绳包管理器团队'
})

// 页脚链接数据
const footerLinksData = reactive({})

// 解析页脚链接数据
const parseFooterLinks = () => {
  try {
    const parsed = JSON.parse(settings.footer_links || '{}')
    Object.assign(footerLinksData, parsed)
  } catch {
    Object.assign(footerLinksData, {
      community: {
        title: "社区",
        links: [{ text: "关于我们", url: "/about" }]
      }
    })
  }
}

// 序列化页脚链接数据
const serializeFooterLinks = () => {
  settings.footer_links = JSON.stringify(footerLinksData)
}

// 添加新的链接分组
const addGroup = () => {
  const groupKey = `group_${Date.now()}`
  footerLinksData[groupKey] = {
    title: '新分组',
    links: [{ text: '新链接', url: '#' }]
  }
}

// 移除链接分组
const removeGroup = (groupKey: string) => {
  delete footerLinksData[groupKey]
}

// 添加新链接
const addLink = (group: any) => {
  group.links.push({ text: '新链接', url: '#' })
}

// 移除链接
const removeLink = (group: any, linkIndex: number) => {
  group.links.splice(linkIndex, 1)
}

// 加载设置
const loadSettings = async () => {
  try {
    const keys = ['hero_title', 'hero_subtitle', 'copyright_text', 'footer_links', 'seo_keywords', 'seo_description', 'seo_author']
    
    for (const key of keys) {
      try {
        const response = await settingsApi.getSetting(key)
        if (response.code === 0 && response.data) {
          settings[key] = response.data.value
        }
      } catch (error) {
        console.warn(`加载配置项 ${key} 失败:`, error)
      }
    }
    
    parseFooterLinks()
  } catch (error) {
    console.error('加载设置失败:', error)
    ElMessage.error('加载设置失败')
  }
}

// 保存设置
const saveSettings = async () => {
  try {
    saving.value = true
    
    // 序列化页脚链接数据
    serializeFooterLinks()
    
    // 保存每个配置项
    const promises = Object.entries(settings).map(([key, value]) =>
      settingsApi.updateSetting(key, value)
    )
    
    await Promise.all(promises)
    
    ElMessage.success('设置保存成功')
  } catch (error) {
    console.error('保存设置失败:', error)
    ElMessage.error('保存设置失败')
  } finally {
    saving.value = false
  }
}

// 重置设置
const resetSettings = async () => {
  try {
    await ElMessageBox.confirm('确定要重置所有主页设置吗？此操作不可撤销。', '确认重置', {
      type: 'warning'
    })
    
    // 重置为默认值
    Object.assign(settings, {
      hero_title: '绳包管理器',
      hero_subtitle: '专业的资源管理与分享平台',
      copyright_text: '© 2024 绳包管理器. All rights reserved.',
      footer_links: '{}',
      seo_keywords: '绳包管理器,资源管理,文件分享,社区',
      seo_description: '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。',
      seo_author: '绳包管理器团队'
    })
    
    parseFooterLinks()
    ElMessage.success('设置已重置')
  } catch {
    // 用户取消
  }
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.admin-page {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
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
  color: var(--el-color-primary);
}

.page-title {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.page-subtitle {
  margin: 4px 0 0 0;
  color: var(--el-text-color-regular);
}

.header-actions {
  display: flex;
  gap: 12px;
}

.settings-content {
  max-width: 800px;
}

.form-tip {
  margin-top: 4px;
  font-size: 12px;
  color: var(--el-text-color-placeholder);
}

.footer-links-editor {
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 15px;
}

.link-group-editor {
  margin-bottom: 15px;
}

.link-item {
  margin-bottom: 10px;
}
</style> 