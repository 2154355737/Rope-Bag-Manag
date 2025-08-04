import { ref, reactive } from 'vue'
import { settingsApi, type CommunitySettings } from '../api/settings'

// 全局配置状态
const communitySettings = ref<CommunitySettings | null>(null)
const isLoading = ref(false)

// 系统设置状态
const systemSettings = reactive({
  hero_title: '绳包管理器',
  hero_subtitle: '专业的资源管理与分享平台',
  copyright_text: '© 2024 绳包管理器. All rights reserved.',
  footer_links: '{}',
  seo_keywords: '绳包管理器,资源管理,文件分享,社区',
  seo_description: '绳包管理器是一个专业的资源管理与分享平台，提供便捷的文件管理和社区交流功能。',
  seo_author: '绳包管理器团队'
})

export function useSettings() {
  // 加载社区设置
  const loadCommunitySettings = async () => {
    if (communitySettings.value) {
      return communitySettings.value
    }

    isLoading.value = true
    try {
      const response = await settingsApi.getPublicCommunitySettings()
      if (response.code === 0 && response.data) {
        communitySettings.value = response.data
        return response.data
      }
    } catch (error) {
      console.error('加载社区设置失败:', error)
    } finally {
      isLoading.value = false
    }
    
    return null
  }

  // 加载系统设置
  const loadSystemSettings = async () => {
    try {
      const settingKeys = [
        'hero_title',
        'hero_subtitle', 
        'copyright_text',
        'footer_links',
        'seo_keywords',
        'seo_description',
        'seo_author'
      ]

      const promises = settingKeys.map(key => 
        settingsApi.getSetting(key).catch(() => ({ code: -1, data: null }))
      )
      
      const results = await Promise.all(promises)
      
      results.forEach((result, index) => {
        if (result.code === 0 && result.data) {
          const key = settingKeys[index] as keyof typeof systemSettings
          let value = result.data.value
          
          // 所有值都保持为字符串类型
          
          systemSettings[key] = value
        }
      })
    } catch (error) {
      console.error('加载系统设置失败:', error)
    }
  }

  // 获取配置值的便捷方法
  const getSetting = (key: string, defaultValue: any = null) => {
    if (key in systemSettings) {
      return systemSettings[key as keyof typeof systemSettings]
    }
    
    if (communitySettings.value && key in communitySettings.value) {
      return communitySettings.value[key as keyof CommunitySettings]
    }
    
    return defaultValue
  }

  // 获取首页标题
  const getHeroTitle = () => {
    return communitySettings.value?.hero_title || systemSettings.hero_title || '绳包管理器'
  }

  // 获取首页副标题
  const getHeroSubtitle = () => {
    return communitySettings.value?.hero_subtitle || systemSettings.hero_subtitle || '专业的资源管理与分享平台'
  }

  // 获取版权信息
  const getCopyright = () => {
    return systemSettings.copyright_text || '© 2024 绳包管理器. All rights reserved.'
  }

  // 获取SEO信息
  const getSEOInfo = () => {
    return {
      keywords: systemSettings.seo_keywords,
      description: systemSettings.seo_description,
      author: systemSettings.seo_author
    }
  }

  // 检查是否显示某个模块
  const shouldShowSection = (section: string) => {
    const sections = communitySettings.value?.homepage_sections || [
      'hero_section', 'stats_section', 'popular_tags', 
      'recent_resources', 'community_posts', 'announcements'
    ]
    return sections.includes(section)
  }

  // 获取分页配置
  const getPaginationConfig = () => {
    return {
      resourcesPerPage: communitySettings.value?.resources_per_page || 12,
      postsPerPage: communitySettings.value?.posts_per_page || 10
    }
  }

  return {
    // 状态
    communitySettings,
    systemSettings,
    isLoading,
    
    // 方法
    loadCommunitySettings,
    loadSystemSettings,
    getSetting,
    getHeroTitle,
    getHeroSubtitle,
    getCopyright,
    getSEOInfo,
    shouldShowSection,
    getPaginationConfig
  }
} 