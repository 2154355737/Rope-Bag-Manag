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
          <span>轮播图设置</span>
        </template>
        <el-form :model="settings" label-width="120px">
          <el-form-item label="轮播图开关">
            <el-switch 
              v-model="carouselSettings.enabled" 
              active-text="启用"
              inactive-text="禁用"
            />
            <div class="form-tip">控制是否在主页显示轮播图</div>
          </el-form-item>
          
          <el-form-item label="轮播图数据" v-if="carouselSettings.enabled">
            <div class="carousel-items-editor">
              <div v-for="(item, index) in carouselSettings.items" :key="index" class="carousel-item-editor">
                <el-card shadow="never" style="margin-bottom: 15px;">
                  <template #header>
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                      <span>轮播图 {{ index + 1 }}</span>
                      <el-button type="danger" size="small" @click="removeCarouselItem(index)">
                        删除
                      </el-button>
                    </div>
                  </template>
                  
                  <el-form-item label="标题">
                    <el-input v-model="item.title" placeholder="请输入轮播图标题" />
                  </el-form-item>
                  
                  <el-form-item label="描述">
                    <el-input 
                      v-model="item.description" 
                      type="textarea" 
                      :rows="2"
                      placeholder="请输入轮播图描述"
                    />
                  </el-form-item>
                  
                  <el-form-item label="图片链接">
                    <el-input v-model="item.image" placeholder="请输入图片链接地址" />
                  </el-form-item>
                  
                  <el-form-item label="跳转链接">
                    <el-input v-model="item.link" placeholder="请输入点击跳转的链接（可选）" />
                  </el-form-item>
                  
                  <el-form-item label="排序">
                    <el-input-number v-model="item.order" :min="1" :max="10" />
                  </el-form-item>
                </el-card>
              </div>
              
              <el-button type="success" @click="addCarouselItem">
                添加轮播图
              </el-button>
            </div>
            <div class="form-tip">配置主页轮播图的内容和链接</div>
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

// 类型定义
interface LinkItem {
  text: string
  url: string
}

interface LinkGroup {
  title: string
  links: LinkItem[]
}

interface FooterLinksData {
  [key: string]: LinkGroup
}

interface Settings {
  hero_title: string
  hero_subtitle: string
  copyright_text: string
  footer_links: string
  seo_keywords: string
  seo_description: string
  seo_author: string
  carousel_settings?: string
}

// 响应式数据
const saving = ref(false)

const settings = reactive<Settings>({
  hero_title: '绳包管理器',
  hero_subtitle: '专业的资源管理与分享平台',
  copyright_text: '© 2024 绳包管理器. All rights reserved.',
  footer_links: '{}',
  seo_keywords: '绳包管理器,资源管理,文件分享,社区',
  seo_description: '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。',
  seo_author: '绳包管理器团队',
  carousel_settings: '{}'
})

// 页脚链接数据
const footerLinksData = reactive<FooterLinksData>({})

// 轮播图设置数据
const carouselSettings = reactive({
  enabled: true,
  items: [
    {
      title: '欢迎使用绳包管理器',
      description: '专业的资源管理与分享平台，为您提供便捷的文件管理和社区交流功能。',
      image: 'https://via.placeholder.com/800x400/667eea/ffffff?text=轮播图1',
      link: '/about',
      order: 1
    },
    {
      title: '丰富的资源库',
      description: '海量优质资源，涵盖各种类型，满足您的不同需求。',
      image: 'https://via.placeholder.com/800x400/764ba2/ffffff?text=轮播图2',
      link: '/resources',
      order: 2
    }
  ]
})

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

// 解析轮播图设置数据
const parseCarouselSettings = () => {
  try {
    const parsed = JSON.parse(settings.carousel_settings || '{}')
    if (parsed.enabled !== undefined) carouselSettings.enabled = parsed.enabled
    if (parsed.items) carouselSettings.items = parsed.items
  } catch {
    // 使用默认值
  }
}

// 序列化页脚链接数据
const serializeFooterLinks = () => {
  settings.footer_links = JSON.stringify(footerLinksData)
}

// 序列化轮播图设置数据
const serializeCarouselSettings = () => {
  settings.carousel_settings = JSON.stringify(carouselSettings)
}

// 添加轮播图项目
const addCarouselItem = () => {
  carouselSettings.items.push({
    title: '新轮播图',
    description: '请输入描述',
    image: 'https://via.placeholder.com/800x400/667eea/ffffff?text=新轮播图',
    link: '',
    order: carouselSettings.items.length + 1
  })
}

// 删除轮播图项目
const removeCarouselItem = (index: number) => {
  carouselSettings.items.splice(index, 1)
  // 重新排序
  carouselSettings.items.forEach((item, idx) => {
    item.order = idx + 1
  })
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
const removeGroup = (groupKey: string | number) => {
  delete footerLinksData[String(groupKey)]
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
    const keys = ['hero_title', 'hero_subtitle', 'copyright_text', 'footer_links', 'seo_keywords', 'seo_description', 'seo_author', 'carousel_settings']
    
    for (const key of keys) {
      try {
        const response = await settingsApi.getSetting(key)
        if (response.code === 0 && response.data) {
          (settings as any)[key] = response.data.value
        }
      } catch (error) {
        console.warn(`加载配置项 ${key} 失败:`, error)
      }
    }
    
    parseFooterLinks()
    parseCarouselSettings()
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
    // 序列化轮播图设置数据
    serializeCarouselSettings()
    
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