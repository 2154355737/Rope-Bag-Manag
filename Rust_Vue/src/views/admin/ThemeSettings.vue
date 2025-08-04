<template>
  <div class="system-settings">
    <el-card class="settings-card">
      <template #header>
        <div class="settings-header">
          <h2>系统设置</h2>
          <p>管理系统配置、主题、功能开关等设置</p>
        </div>
      </template>

      <div class="settings-content">
        <!-- 主题设置 -->
        <div class="setting-section">
          <h3>主题设置</h3>
          <el-form :model="settings.theme" label-width="120px">
            <el-form-item label="资源社区主题">
              <el-select v-model="settings.theme.community_theme" placeholder="选择社区主题">
                <el-option label="浅色主题" value="light" />
                <el-option label="深色主题" value="dark" />
                <el-option label="蓝色主题" value="blue" />
                <el-option label="绿色主题" value="green" />
                <el-option label="紫色主题" value="purple" />
                <el-option label="橙色主题" value="orange" />
                <el-option label="红色主题" value="red" />
              </el-select>
            </el-form-item>
            <el-form-item label="后台管理主题">
              <el-select v-model="settings.theme.admin_theme" placeholder="选择管理主题">
                <el-option label="浅色主题" value="light" />
                <el-option label="深色主题" value="dark" />
                <el-option label="蓝色主题" value="blue" />
                <el-option label="绿色主题" value="green" />
                <el-option label="紫色主题" value="purple" />
                <el-option label="橙色主题" value="orange" />
                <el-option label="红色主题" value="red" />
              </el-select>
            </el-form-item>
          </el-form>
        </div>

        <!-- 系统模式 -->
        <div class="setting-section">
          <h3>系统模式</h3>
          <el-form :model="settings" label-width="120px">
            <el-form-item label="系统模式">
              <el-radio-group v-model="settings.system_mode">
                <el-radio label="Normal">正常运行</el-radio>
                <el-radio label="Maintenance">维护中</el-radio>
              </el-radio-group>
              <div class="mode-description">
                <p v-if="settings.system_mode === 'Normal'">
                  <el-icon><InfoFilled /></el-icon>
                  正常运行模式：所有功能正常开放，用户可以正常使用所有功能
                </p>
                <p v-else class="warning-text">
                  <el-icon><WarningFilled /></el-icon>
                  维护模式：仅管理员可登录，用户注册、上传等功能将被禁用
                </p>
              </div>
            </el-form-item>
          </el-form>
        </div>

        <!-- 功能开关 -->
        <div class="setting-section">
          <h3>功能开关</h3>
          <el-form :model="settings.feature_flags" label-width="120px">
            <el-form-item label="用户注册">
              <el-switch v-model="settings.feature_flags.enable_registration" />
              <span class="feature-desc">允许新用户注册账号</span>
            </el-form-item>
            <el-form-item label="资源社区">
              <el-switch v-model="settings.feature_flags.enable_community" />
              <span class="feature-desc">开放资源社区功能</span>
            </el-form-item>
            <el-form-item label="用户上传">
              <el-switch v-model="settings.feature_flags.enable_upload" />
              <span class="feature-desc">允许用户上传资源</span>
            </el-form-item>
            <el-form-item label="用户评论">
              <el-switch v-model="settings.feature_flags.enable_comments" />
              <span class="feature-desc">允许用户发表评论</span>
            </el-form-item>
            <el-form-item label="QQ绑定">
              <el-switch v-model="settings.feature_flags.enable_qq_binding" />
              <span class="feature-desc">允许用户绑定QQ账号</span>
            </el-form-item>
          </el-form>
        </div>

        <!-- 后端配置 -->
        <div class="setting-section">
          <h3>后端配置</h3>
          <el-form :model="settings.backend_config" label-width="120px">
            <el-form-item label="代理地址">
              <el-input v-model="settings.backend_config.proxy_address" placeholder="设置后台代理地址" />
            </el-form-item>
            <el-form-item label="API超时时间">
              <el-input-number 
                v-model="settings.backend_config.api_timeout" 
                :min="1" 
                :max="300" 
                placeholder="秒"
              />
              <span class="config-desc">API请求超时时间（秒）</span>
            </el-form-item>
            <el-form-item label="最大上传大小">
              <el-input-number 
                v-model="settings.backend_config.max_upload_size" 
                :min="1" 
                :max="1000" 
                placeholder="MB"
              />
              <span class="config-desc">最大上传文件大小（MB）</span>
            </el-form-item>
          </el-form>
        </div>

        <!-- 备份设置 -->
        <div class="setting-section">
          <h3>数据库备份</h3>
          <el-form :model="settings.backup_settings" label-width="120px">
            <el-form-item label="自动备份">
              <el-switch v-model="settings.backup_settings.enable_auto_backup" />
              <span class="backup-desc">启用自动备份功能</span>
            </el-form-item>
            <el-form-item label="备份间隔">
              <el-input-number 
                v-model="settings.backup_settings.backup_interval_hours" 
                :min="1" 
                :max="168" 
                placeholder="小时"
                :disabled="!settings.backup_settings.enable_auto_backup"
              />
              <span class="backup-desc">备份间隔时间（小时）</span>
            </el-form-item>
            <el-form-item label="备份位置">
              <el-input 
                v-model="settings.backup_settings.backup_location" 
                placeholder="备份文件存储路径"
                :disabled="!settings.backup_settings.enable_auto_backup"
              />
            </el-form-item>
            <el-form-item label="最大备份文件">
              <el-input-number 
                v-model="settings.backup_settings.max_backup_files" 
                :min="1" 
                :max="100" 
                placeholder="个"
                :disabled="!settings.backup_settings.enable_auto_backup"
              />
              <span class="backup-desc">保留的最大备份文件数量</span>
            </el-form-item>
          </el-form>
        </div>

        <!-- 全局公告 -->
        <div class="setting-section">
          <h3>全局公告</h3>
          <el-form :model="settings.global_announcement" label-width="120px">
            <el-form-item label="启用公告">
              <el-switch v-model="settings.global_announcement.enabled" />
              <span class="announcement-desc">在资源社区显示全局公告</span>
            </el-form-item>
            <el-form-item label="公告标题">
              <el-input 
                v-model="settings.global_announcement.title" 
                placeholder="公告标题"
                :disabled="!settings.global_announcement.enabled"
              />
            </el-form-item>
            <el-form-item label="公告内容">
              <el-input 
                v-model="settings.global_announcement.content" 
                type="textarea" 
                :rows="4"
                placeholder="公告内容"
                :disabled="!settings.global_announcement.enabled"
              />
            </el-form-item>
            <el-form-item label="公告类型">
              <el-select 
                v-model="settings.global_announcement.type_" 
                placeholder="选择公告类型"
                :disabled="!settings.global_announcement.enabled"
              >
                <el-option label="信息" value="Info" />
                <el-option label="警告" value="Warning" />
                <el-option label="错误" value="Error" />
                <el-option label="成功" value="Success" />
              </el-select>
            </el-form-item>
            <el-form-item label="优先级">
              <el-input-number 
                v-model="settings.global_announcement.priority" 
                :min="1" 
                :max="10" 
                placeholder="1-10"
                :disabled="!settings.global_announcement.enabled"
              />
              <span class="announcement-desc">公告显示优先级（1-10）</span>
            </el-form-item>
          </el-form>
        </div>

        <!-- 首页配置 -->
        <div class="setting-section">
          <h3>首页显示配置</h3>
          <el-form :model="settings.homepage_config" label-width="120px">
            <el-form-item label="首页标题">
              <el-input 
                v-model="settings.homepage_config.hero_title" 
                placeholder="请输入首页主标题"
                maxlength="50"
                show-word-limit
              />
            </el-form-item>
            <el-form-item label="首页副标题">
              <el-input 
                v-model="settings.homepage_config.hero_subtitle" 
                type="textarea"
                :rows="2"
                placeholder="请输入首页副标题"
                maxlength="200"
                show-word-limit
              />
            </el-form-item>
            <el-form-item label="显示统计数据">
              <el-switch v-model="settings.homepage_config.show_stats" />
              <span class="config-desc">在首页显示用户、资源等统计数据</span>
            </el-form-item>
            <el-form-item label="显示热门标签">
              <el-switch v-model="settings.homepage_config.show_popular_tags" />
              <span class="config-desc">在首页显示热门标签</span>
            </el-form-item>
            <el-form-item label="显示最新资源">
              <el-switch v-model="settings.homepage_config.show_recent_resources" />
              <span class="config-desc">在首页显示最新上传的资源</span>
            </el-form-item>
            <el-form-item label="显示社区帖子">
              <el-switch v-model="settings.homepage_config.show_community_posts" />
              <span class="config-desc">在首页显示社区讨论帖子</span>
            </el-form-item>
            <el-form-item label="每页显示数量">
              <el-input-number 
                v-model="settings.homepage_config.items_per_page" 
                :min="6" 
                :max="50" 
                placeholder="每页显示的资源数量"
              />
              <span class="config-desc">首页每页显示的资源数量</span>
            </el-form-item>
          </el-form>
        </div>

        <!-- 页脚配置 -->
        <div class="setting-section">
          <h3>页脚配置</h3>
          <el-form :model="settings.footer_config" label-width="120px">
            <el-form-item label="版权信息">
              <el-input 
                v-model="settings.footer_config.copyright_text" 
                placeholder="© 2024 绳包管理器. All rights reserved."
                maxlength="100"
                show-word-limit
              />
            </el-form-item>
            <el-form-item label="备案号">
              <el-input 
                v-model="settings.footer_config.icp_number" 
                placeholder="请输入ICP备案号"
                maxlength="50"
              />
            </el-form-item>
            <el-form-item label="显示友情链接">
              <el-switch v-model="settings.footer_config.show_links" />
              <span class="config-desc">在页脚显示友情链接</span>
            </el-form-item>
            <el-form-item label="显示统计信息">
              <el-switch v-model="settings.footer_config.show_stats" />
              <span class="config-desc">在页脚显示网站统计信息</span>
            </el-form-item>
          </el-form>
        </div>

        <!-- SEO配置 -->
        <div class="setting-section">
          <h3>SEO配置</h3>
          <el-form :model="settings.seo_config" label-width="120px">
            <el-form-item label="网站关键词">
              <el-input 
                v-model="settings.seo_config.keywords" 
                type="textarea"
                :rows="2"
                placeholder="请输入网站关键词，用逗号分隔"
                maxlength="200"
                show-word-limit
              />
            </el-form-item>
            <el-form-item label="网站描述">
              <el-input 
                v-model="settings.seo_config.description" 
                type="textarea"
                :rows="3"
                placeholder="请输入网站描述"
                maxlength="300"
                show-word-limit
              />
            </el-form-item>
            <el-form-item label="作者信息">
              <el-input 
                v-model="settings.seo_config.author" 
                placeholder="请输入网站作者"
                maxlength="50"
              />
            </el-form-item>
          </el-form>
        </div>

        <!-- 操作按钮 -->
        <div class="setting-actions">
          <el-button type="primary" @click="saveSettings" :loading="saving">
            保存设置
          </el-button>
          <el-button @click="resetSettings">重置设置</el-button>
          <el-button @click="testSettings">测试设置</el-button>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { InfoFilled, WarningFilled } from '@element-plus/icons-vue'
import { settingsApi } from '../../api/settings'

// 响应式数据
const saving = ref(false)

const settings = reactive({
  theme: {} as { [key: string]: any },
  system_mode: 'Normal',
  feature_flags: {} as { [key: string]: any },
  backend_config: {
    proxy_address: '',
    api_timeout: 30,
    max_upload_size: 100
  },
  backup_settings: {
    enable_auto_backup: false,
    backup_interval_hours: 24,
    backup_location: './backups',
    max_backup_files: 10
  },
  global_announcement: {
    enabled: false,
    title: '',
    content: '',
    type_: 'Info',
    start_time: '',
    end_time: '',
    priority: 5
  },
  homepage_config: {
    hero_title: '绳包管理器',
    hero_subtitle: '专业的资源管理与分享平台',
    show_stats: true,
    show_popular_tags: true,
    show_recent_resources: true,
    show_community_posts: true,
    items_per_page: 12
  },
  footer_config: {
    copyright_text: '© 2024 绳包管理器. All rights reserved.',
    icp_number: '',
    show_links: true,
    show_stats: true
  },
  seo_config: {
    keywords: '绳包管理器,资源管理,文件分享,社区',
    description: '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。',
    author: '绳包管理器团队'
  }
})

// 方法
async function loadSettings() {
  try {
    // 加载主题设置
    const themeResponse = await settingsApi.getThemeSettings()
    if (themeResponse.code === 0 && themeResponse.data) {
      const themeData = reactive<{ [key: string]: any }>({})
      themeData.primary_color = themeResponse.data.primary_color
      themeData.secondary_color = themeResponse.data.secondary_color
      themeData.dark_mode = themeResponse.data.dark_mode
      themeData.font_size = themeResponse.data.font_size
      themeData.language = themeResponse.data.language
      settings.theme = themeData
    }
    
    // 加载系统模式
    const modeResponse = await settingsApi.getSetting('system_mode')
    if (modeResponse.code === 0 && modeResponse.data) {
      settings.system_mode = modeResponse.data.value
    }
    
    // 加载功能开关
    const featurePromises = [
      settingsApi.getSetting('enable_registration'),
      settingsApi.getSetting('enable_community'),
      settingsApi.getSetting('enable_upload'),
      settingsApi.getSetting('enable_comments')
    ]
    
    const featureResults = await Promise.all(featurePromises)
    
    featureResults.forEach(result => {
      if (result.code === 0 && result.data) {
        const key = result.data.key
        const value = result.data.value === 'true'
        if (key in settings.feature_flags) {
          settings.feature_flags[key] = value
        }
      }
    })

    // 加载首页配置
    const homepagePromises = [
      settingsApi.getSetting('hero_title'),
      settingsApi.getSetting('hero_subtitle'),
      settingsApi.getSetting('show_stats'),
      settingsApi.getSetting('show_popular_tags'),
      settingsApi.getSetting('show_recent_resources'),
      settingsApi.getSetting('show_community_posts'),
      settingsApi.getSetting('items_per_page')
    ]
    
    const homepageResults = await Promise.all(homepagePromises)
    
    homepageResults.forEach(result => {
      if (result.code === 0 && result.data) {
        const key = result.data.key
        const value = result.data.value
        
        // 处理不同类型的值
        if (key === 'hero_title') {
          settings.homepage_config.hero_title = value
        } else if (key === 'hero_subtitle') {
          settings.homepage_config.hero_subtitle = value
        } else if (key === 'show_stats') {
          settings.homepage_config.show_stats = value === 'true'
        } else if (key === 'show_popular_tags') {
          settings.homepage_config.show_popular_tags = value === 'true'
        } else if (key === 'show_recent_resources') {
          settings.homepage_config.show_recent_resources = value === 'true'
        } else if (key === 'show_community_posts') {
          settings.homepage_config.show_community_posts = value === 'true'
        } else if (key === 'items_per_page') {
          settings.homepage_config.items_per_page = parseInt(value) || 12
        }
      }
    })

    // 加载页脚配置
    const footerPromises = [
      settingsApi.getSetting('copyright_text'),
      settingsApi.getSetting('icp_number'),
      settingsApi.getSetting('footer_show_links'),
      settingsApi.getSetting('footer_show_stats')
    ]
    
    const footerResults = await Promise.all(footerPromises)
    
    footerResults.forEach(result => {
      if (result.code === 0 && result.data) {
        const key = result.data.key
        const value = result.data.value
        
        if (key === 'copyright_text') {
          settings.footer_config.copyright_text = value
        } else if (key === 'icp_number') {
          settings.footer_config.icp_number = value
        } else if (key === 'footer_show_links') {
          settings.footer_config.show_links = value === 'true'
        } else if (key === 'footer_show_stats') {
          settings.footer_config.show_stats = value === 'true'
        }
      }
    })

    // 加载SEO配置
    const seoPromises = [
      settingsApi.getSetting('seo_keywords'),
      settingsApi.getSetting('seo_description'),
      settingsApi.getSetting('seo_author')
    ]
    
    const seoResults = await Promise.all(seoPromises)
    
    seoResults.forEach(result => {
      if (result.code === 0 && result.data) {
        const key = result.data.key
        const value = result.data.value
        
        if (key === 'seo_keywords') {
          settings.seo_config.keywords = value
        } else if (key === 'seo_description') {
          settings.seo_config.description = value
        } else if (key === 'seo_author') {
          settings.seo_config.author = value
        }
      }
    })
  } catch (error) {
    console.error('加载设置失败:', error)
    ElMessage.error('加载设置失败')
  }
}

async function saveSettings() {
  saving.value = true
  try {
    // 先保存主题设置
    const themeResponse = await settingsApi.updateThemeSettings({
      primary_color: settings.theme.primary_color || '#409EFF',
      secondary_color: settings.theme.secondary_color || '#67C23A',
      dark_mode: settings.theme.dark_mode || false,
      font_size: settings.theme.font_size || '14px',
      language: settings.theme.language || 'zh-CN'
    })
    
    if (themeResponse.code === 0) {
      // 逐个保存其他设置
      const promises = [
        settingsApi.updateSetting('system_mode', settings.system_mode),
        settingsApi.updateSetting('enable_registration', settings.feature_flags.enable_registration),
        settingsApi.updateSetting('enable_community', settings.feature_flags.enable_community),
        settingsApi.updateSetting('enable_upload', settings.feature_flags.enable_upload),
        settingsApi.updateSetting('enable_comments', settings.feature_flags.enable_comments),
        
        // 首页配置
        settingsApi.updateSetting('hero_title', settings.homepage_config.hero_title),
        settingsApi.updateSetting('hero_subtitle', settings.homepage_config.hero_subtitle),
        settingsApi.updateSetting('show_stats', settings.homepage_config.show_stats),
        settingsApi.updateSetting('show_popular_tags', settings.homepage_config.show_popular_tags),
        settingsApi.updateSetting('show_recent_resources', settings.homepage_config.show_recent_resources),
        settingsApi.updateSetting('show_community_posts', settings.homepage_config.show_community_posts),
        settingsApi.updateSetting('items_per_page', settings.homepage_config.items_per_page),
        
        // 页脚配置
        settingsApi.updateSetting('copyright_text', settings.footer_config.copyright_text),
        settingsApi.updateSetting('icp_number', settings.footer_config.icp_number),
        settingsApi.updateSetting('footer_show_links', settings.footer_config.show_links),
        settingsApi.updateSetting('footer_show_stats', settings.footer_config.show_stats),
        
        // SEO配置
        settingsApi.updateSetting('seo_keywords', settings.seo_config.keywords),
        settingsApi.updateSetting('seo_description', settings.seo_config.description),
        settingsApi.updateSetting('seo_author', settings.seo_config.author)
      ]
      
      await Promise.all(promises)
      ElMessage.success('设置保存成功')
    } else {
              ElMessage.error(themeResponse.msg || themeResponse.message || '保存失败')
    }
  } catch (error) {
    console.error('保存设置失败:', error)
    ElMessage.error('保存设置失败')
  } finally {
    saving.value = false
  }
}

function resetSettings() {
  ElMessageBox.confirm(
    '确定要重置所有设置吗？这将恢复默认配置。',
    '确认重置',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    }
  ).then(async () => {
    try {
      const response = await settingsApi.resetSettings()
      if (response.code === 0) {
        ElMessage.success('设置已重置')
        // 重新加载设置
        loadSettings()
      } else {
        ElMessage.error(response.msg || response.message || '重置失败')
      }
    } catch (error) {
      console.error('重置设置失败:', error)
      ElMessage.error('重置设置失败')
    }
  }).catch(() => {
    // 用户取消操作
  })
}

function testSettings() {
  ElMessage.info('测试功能开发中...')
}

onMounted(() => {
  loadSettings()
})
</script>

<style scoped>
.system-settings {
  padding: 20px;
}

.settings-card {
  max-width: 1200px;
  margin: 0 auto;
}

.settings-header {
  text-align: center;
}

.settings-header h2 {
  margin: 0 0 10px 0;
  color: var(--el-text-color-primary);
}

.settings-header p {
  margin: 0;
  color: var(--el-text-color-secondary);
}

.settings-content {
  padding: 20px 0;
}

.setting-section {
  margin-bottom: 40px;
  padding: 20px;
  border: 1px solid var(--el-border-color-light);
  border-radius: 8px;
  background: var(--el-bg-color);
}

.setting-section h3 {
  margin: 0 0 20px 0;
  color: var(--el-text-color-primary);
  font-size: 18px;
  font-weight: 600;
}

.mode-description {
  margin-top: 10px;
  padding: 10px;
  border-radius: 4px;
  background: var(--el-bg-color-page);
}

.mode-description p {
  margin: 0;
  display: flex;
  align-items: center;
  gap: 8px;
}

.warning-text {
  color: var(--el-color-warning);
}

.feature-desc,
.config-desc,
.backup-desc,
.announcement-desc {
  margin-left: 10px;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.setting-actions {
  text-align: center;
  margin-top: 40px;
  padding-top: 20px;
  border-top: 1px solid var(--el-border-color-light);
}

.setting-actions .el-button {
  margin: 0 10px;
}
</style> 