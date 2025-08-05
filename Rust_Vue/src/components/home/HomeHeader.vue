<template>
  <header class="home-header">
    <div class="header-container">
      <!-- 品牌区域 -->
      <div class="brand-section">
        <div class="brand-icon">🏘️</div>
        <div class="brand-text">
          <h1>{{ systemSettings?.hero_title || '社区' }}</h1>
          <span>{{ systemSettings?.hero_subtitle || '资源分享平台' }}</span>
        </div>
      </div>
      
      <!-- 搜索区域 -->
      <div class="search-section">
        <div class="search-container">
          <el-input
            v-model="searchQuery"
            placeholder="搜索资源、帖子..."
            size="large"
            clearable
            @keyup.enter="handleSearch"
            @input="debouncedSearch"
            @focus="showSuggestions = true"
            @blur="handleSearchBlur"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
          
          <!-- 搜索建议 -->
          <div 
            v-if="showSuggestions && suggestions.length > 0" 
            class="search-suggestions"
          >
            <div 
              v-for="suggestion in suggestions" 
              :key="suggestion.text"
              class="suggestion-item"
              @click="selectSuggestion(suggestion.text)"
            >
              <span class="suggestion-text">{{ suggestion.text }}</span>
              <span class="suggestion-type">{{ suggestion.type }}</span>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 用户区域 -->
      <div class="user-section">
        <ThemeSwitcher />
        <template v-if="!isLoggedIn">
          <el-button type="primary" @click="$router.push('/login')">
            <el-icon><User /></el-icon>
            登录
          </el-button>
        </template>
        <template v-else>
          <UserMenu :user-info="userInfo" />
        </template>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, inject } from 'vue'
import { useRouter } from 'vue-router'
import { Search, User } from '@element-plus/icons-vue'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import UserMenu from './UserMenu.vue'
import { getUserInfo } from '@/utils/auth'
import { debounce } from '@/utils/debounce'

// Props
interface Props {
  systemSettings?: any
  suggestions?: Array<{ text: string; type: string }>
}

const props = withDefaults(defineProps<Props>(), {
  suggestions: () => []
})

// Emits
const emit = defineEmits<{
  search: [query: string]
}>()

// Router
const router = useRouter()

// 响应式数据
const searchQuery = ref('')
const showSuggestions = ref(false)

// 计算属性
const userInfo = computed(() => getUserInfo())
const isLoggedIn = computed(() => !!userInfo.value)

// 防抖搜索
const debouncedSearch = debounce(() => {
  if (searchQuery.value.trim()) {
    emit('search', searchQuery.value.trim())
  }
}, 300)

// 方法
const handleSearch = () => {
  if (searchQuery.value.trim()) {
    emit('search', searchQuery.value.trim())
    showSuggestions.value = false
  }
}

const selectSuggestion = (text: string) => {
  searchQuery.value = text
  showSuggestions.value = false
  emit('search', text)
}

const handleSearchBlur = () => {
  setTimeout(() => {
    showSuggestions.value = false
  }, 200)
}
</script>

<style scoped>
.home-header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--border-color-light);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
}

html.dark .home-header {
  background: rgba(17, 24, 39, 0.95);
}

.header-container {
  max-width: 1440px;
  margin: 0 auto;
  padding: 0 24px;
  height: 72px;
  display: flex;
  align-items: center;
  gap: 32px;
}

.brand-section {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.brand-icon {
  font-size: 32px;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light));
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.brand-text h1 {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.2;
}

.brand-text span {
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.2;
}

.search-section {
  flex: 1;
  max-width: 600px;
  position: relative;
}

.search-container {
  position: relative;
}

.search-suggestions {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--bg-elevated);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  z-index: 1001;
  max-height: 200px;
  overflow-y: auto;
}

.suggestion-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.suggestion-item:hover {
  background: var(--bg-hover);
}

.suggestion-text {
  font-size: 14px;
  color: var(--text-primary);
}

.suggestion-type {
  font-size: 12px;
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: 2px 8px;
  border-radius: 4px;
}

.user-section {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-shrink: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .header-container {
    padding: 0 16px;
    gap: 16px;
  }
  
  .brand-text h1 {
    font-size: 16px;
  }
  
  .brand-text span {
    display: none;
  }
  
  .search-section {
    max-width: none;
  }
}

@media (max-width: 480px) {
  .header-container {
    gap: 8px;
  }
  
  .brand-icon {
    width: 40px;
    height: 40px;
    font-size: 24px;
  }
  
  .brand-text h1 {
    font-size: 14px;
  }
}
</style> 