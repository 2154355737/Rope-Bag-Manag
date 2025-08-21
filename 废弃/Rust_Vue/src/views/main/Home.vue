<template>
  <div class="home-container">
    <!-- 动态背景层 -->
    <div class="background-layer">
      <div class="gradient-bg"></div>
      <div class="pattern-overlay"></div>
    </div>

    <!-- 顶部导航栏 - 固定定位 -->
    <header class="top-header">
      <div class="header-wrapper">
        <div class="header-left">
          <div class="brand">
            <div class="brand-icon">🏘️</div>
            <div class="brand-text">
              <h1>{{ systemSettings.hero_title }}</h1>
              <span>{{ systemSettings.hero_subtitle }}</span>
            </div>
          </div>
        </div>
        
        <div class="header-center">
          <div class="search-container">
            <el-input
              v-model="searchQuery"
              placeholder="搜索资源、帖子、用户..."
              size="large"
              clearable
              @keyup.enter="handleSearch"
              @input="handleSearchInput"
              @clear="handleSearch"
              @focus="showSuggestions = true"
              @blur="handleSearchBlur"
              class="main-search"
            >
              <template #prefix>
                <el-icon><Search /></el-icon>
              </template>
              <template #suffix>
                <el-button 
                  type="primary" 
                  text 
                  @click="handleSearch"
                  :loading="loading"
                  :disabled="!searchQuery.trim()"
                >
                  <el-icon v-if="!loading"><Search /></el-icon>
                  <span v-if="!loading">搜索</span>
                </el-button>
              </template>
            </el-input>
            
            <!-- 搜索建议下拉框 -->
            <div 
              v-if="showSuggestions && searchSuggestions.length > 0" 
              class="search-suggestions"
            >
              <div 
                v-for="suggestion in searchSuggestions" 
                :key="suggestion.text"
                class="suggestion-item"
                @click="selectSuggestion(suggestion.text)"
              >
                <span class="suggestion-icon">{{ suggestion.icon }}</span>
                <span class="suggestion-text">{{ suggestion.text }}</span>
                <span class="suggestion-type">{{ suggestion.type === 'resource' ? '资源' : '分类' }}</span>
              </div>
            </div>
          </div>
        </div>
        
        <div class="header-right">
          <div class="header-actions">
            <ThemeSwitcher />
            <el-button 
              v-if="!isLoggedIn" 
              type="primary" 
              size="default"
              @click="goToLogin"
            >
              <el-icon><User /></el-icon>
              登录
            </el-button>
            <div v-if="isLoggedIn" class="user-menu">
              <el-dropdown trigger="click" placement="bottom-end">
                <div class="user-trigger">
                  <el-avatar :size="36" :src="userInfo.avatar_url">
                    <el-icon><User /></el-icon>
                  </el-avatar>
                  <span class="user-name">{{ userInfo.nickname || userInfo.username }}</span>
                  <el-icon class="dropdown-arrow"><ArrowDown /></el-icon>
                </div>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item @click="goToUserProfile">
                      <el-icon><User /></el-icon>
                      个人中心
                    </el-dropdown-item>
                    <el-dropdown-item @click="goToAdmin">
                      <el-icon><Setting /></el-icon>
                      管理后台
                    </el-dropdown-item>
                    <el-dropdown-item divided @click="handleLogout">
                      <el-icon><Switch /></el-icon>
                      退出登录
                    </el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- 主内容区域 -->
    <main class="main-content">
      <div class="content-wrapper">
        
        <!-- 轮播图区域 -->
        <section class="carousel-section" v-if="carouselSettings.enabled && carouselSettings.items.length > 0">
          <div class="carousel-container">
            <el-carousel 
              :interval="5000" 
              indicator-position="none"
              arrow="hover"
              height="380px"
              class="hero-carousel"
              :autoplay="true"
              :loop="true"
            >
              <el-carousel-item 
                v-for="(item, index) in carouselSettings.items" 
                :key="index"
                class="carousel-item"
              >
                <div class="carousel-slide" @click="handleCarouselClick(item)">
                  <div class="slide-background">
                    <img 
                      :src="item.image || '/api/placeholder/1200/380'" 
                      :alt="item.title || '轮播图片'" 
                      class="background-image"
                    />
                    <div class="background-overlay"></div>
                  </div>
                  
                  <div class="slide-content">
                    <!-- 移除了左侧图标、标题文字和描述文字 -->
                    <div class="content-right" v-if="item.preview">
                      <div class="preview-container">
                        <img :src="item.preview" :alt="item.title + ' 预览'" class="preview-image" />
                      </div>
                    </div>
                  </div>
                </div>
              </el-carousel-item>
            </el-carousel>
          </div>
        </section>

        <!-- 原始三栏内容布局 -->
        <section class="three-column-section">
          <div class="column-wrapper">
            
            <!-- 左侧栏：快速导航 + 公告 -->
            <aside class="left-column">
              <!-- 内容分类 -->
              <div class="nav-panel">
                <div class="panel-header" @click="toggleCategoryPanel">
                  <h3>
                    <el-icon><Menu /></el-icon>
                    内容分类
                  </h3>
                  <el-icon class="collapse-icon" :class="{ collapsed: !categoryPanelExpanded }">
                    <ArrowDown />
                  </el-icon>
                </div>
                <el-collapse-transition>
                  <div v-show="categoryPanelExpanded" class="panel-content">
                    <div class="quick-nav">
                      <div 
                        class="nav-item"
                        :class="{ active: activeCategory === 'all' }"
                        @click="handleCategoryChange({ props: { name: 'all' } })"
                      >
                        <el-icon><Grid /></el-icon>
                        <span>全部资源</span>
                        <div class="nav-count">{{ totalResources }}</div>
                      </div>
                      <div 
                        v-for="category in categories" 
                        :key="category.id"
                        class="nav-item"
                        :class="{ active: activeCategory === category.id.toString() }"
                        @click="handleCategoryChange({ props: { name: category.id.toString() } })"
                      >
                        <el-icon><Document /></el-icon>
                        <span>{{ category.name }}</span>
                        <div class="nav-count">{{ getCategoryCount(category.id) }}</div>
                      </div>
                    </div>
                  </div>
                </el-collapse-transition>
              </div>
              
              <!-- 社区公告 -->
              <div class="announcement-panel">
                <div class="panel-header" @click="toggleAnnouncementPanel">
                  <h3>
                    <el-icon><Bell /></el-icon>
                    社区公告
                  </h3>
                  <el-icon class="collapse-icon" :class="{ collapsed: !announcementPanelExpanded }">
                    <ArrowDown />
                  </el-icon>
                </div>
                <el-collapse-transition>
                  <div v-show="announcementPanelExpanded" class="panel-content">
                  <div v-if="notices.length === 0" class="empty-state">
                    <el-empty description="暂无公告" :image-size="80" />
                  </div>
                  <div v-else class="announcement-list">
                    <div 
                      v-for="notice in notices.slice(0, 5)" 
                      :key="notice.id" 
                      class="announcement-item"
                    >
                      <div class="announcement-dot"></div>
                      <div class="announcement-content">
                        <div class="announcement-text">{{ notice.text }}</div>
                        <div class="announcement-time">{{ formatTime(notice.created_at) }}</div>
                      </div>
                    </div>
                  </div>
                </div>
                </el-collapse-transition>
              </div>
            </aside>

            <!-- 中间主内容：资源展示 -->
            <main class="center-column">
              <div class="content-panel">
                <!-- 内容头部 -->
                <div class="content-header">
                  <div class="header-left">
                    <h2>社区内容</h2>
                    <span class="content-count">
                      <span v-if="contentType === 'resources'">共 {{ filteredResources.length }} 个资源</span>
                      <span v-else>共 {{ posts.length }} 个帖子</span>
                    </span>
                  </div>
                  <div class="header-right">
                    <div class="content-controls">
                      <!-- 内容类型切换 -->
                      <el-select 
                        v-model="contentType" 
                        placeholder="内容类型"
                      >
                        <el-option label="资源" value="resources" />
                        <el-option label="帖子" value="posts" />
                      </el-select>
                      
                      <!-- 筛选器 -->
                      <el-select 
                        v-model="sortBy" 
                        placeholder="排序方式"
                        @change="applySorting"
                      >
                        <el-option label="最新发布" value="created_at" />
                        <el-option label="下载最多" value="download_count" v-if="contentType === 'resources'" />
                        <el-option label="最多查看" value="view_count" v-if="contentType === 'posts'" />
                        <el-option label="最多点赞" value="like_count" v-if="contentType === 'posts'" />
                        <el-option label="评分最高" value="rating" v-if="contentType === 'resources'" />
                        <el-option label="名称排序" value="name" v-if="contentType === 'resources'" />
                        <el-option label="标题排序" value="title" v-if="contentType === 'posts'" />
                      </el-select>
                      


                      <!-- 发布按钮 -->
                      <el-button 
                        v-if="contentType === 'posts'" 
                        type="primary" 
                        @click="goToCreatePost"
                      >
                        <el-icon><Plus /></el-icon>
                        发布帖子
                      </el-button>
                    </div>
                  </div>
                </div>

                <!-- 内容列表 -->
                <div class="content-display">
                  <div v-if="loading" class="loading-state">
                    <el-skeleton :rows="6" animated />
                  </div>
                  
                  <!-- 资源列表 -->
                  <div v-else-if="contentType === 'resources'">
                    <div v-if="filteredResources.length === 0" class="empty-state">
                    <el-empty description="暂无资源" :image-size="120">
                      <el-button type="primary" @click="resetFilters">
                        重置筛选
                      </el-button>
                    </el-empty>
                  </div>
                  
                  <div v-else class="resource-grid" :class="[viewMode]">
                    <div 
                      v-for="resource in paginatedResources" 
                      :key="resource.id"
                      class="resource-card"
                      @click="viewResource(resource.id)"
                    >
                      <div class="card-header">
                        <div class="card-icon">
                          <el-icon size="20" :color="getCategoryColor(resource.category_id)">
                            <Document />
                          </el-icon>
                        </div>
                        <div class="card-title">
                          <div class="title-row">
                          <h4>{{ resource.name }}</h4>
                            <div class="status-badges" v-if="resource.is_featured || resource.is_pinned">
                              <el-tag v-if="resource.is_pinned" type="danger" size="small" effect="dark">
                                <el-icon><Star /></el-icon>
                                置顶
                              </el-tag>
                              <el-tag v-if="resource.is_featured" type="warning" size="small" effect="dark">
                                <el-icon><Trophy /></el-icon>
                                精华
                              </el-tag>
                            </div>
                          </div>
                          <span class="card-category">{{ getCategoryLabel(resource.category_id) }}</span>
                        </div>
                        <div class="card-actions">
                          <el-dropdown trigger="click">
                            <el-button link size="small">
                              <el-icon><MoreFilled /></el-icon>
                            </el-button>
                            <template #dropdown>
                              <el-dropdown-menu>
                                <el-dropdown-item @click.stop="viewResource(resource.id)">
                                  <el-icon><View /></el-icon>
                                  查看详情
                                </el-dropdown-item>
                                <el-dropdown-item @click.stop="downloadResource(resource.id)">
                                  <el-icon><Download /></el-icon>
                                  立即下载
                                </el-dropdown-item>
                              </el-dropdown-menu>
                            </template>
                          </el-dropdown>
                        </div>
                      </div>
                      
                      <div class="card-body">
                        <p class="card-description">{{ resource.description }}</p>
                        <div class="card-tags" v-if="resource.tags && resource.tags.length > 0">
                          <el-tag 
                            v-for="tag in resource.tags.slice(0, 3)" 
                            :key="tag"
                            size="small"
                            effect="plain"
                          >
                            {{ tag }}
                          </el-tag>
                        </div>
                      </div>
                      
                      <div class="card-footer">
                        <div class="footer-meta">
                          <span class="meta-author">
                            <el-icon><User /></el-icon>
                            {{ resource.author }}
                          </span>
                          <span class="meta-downloads">
                            <el-icon><Download /></el-icon>
                            {{ resource.download_count }}
                          </span>
                          <span class="meta-time">
                            <el-icon><Clock /></el-icon>
                            {{ formatTime(resource.created_at) }}
                          </span>
                        </div>
                        <div class="footer-actions">
                          <el-button 
                            type="primary" 
                            size="small"
                            @click.stop="downloadResource(resource.id)"
                          >
                            下载
                          </el-button>
                        </div>
                      </div>
                    </div>
                  </div>

                    <!-- 分页器 - 资源 -->
                  <div class="pagination-wrapper" v-if="filteredResources.length > 0">
                    <el-pagination
                      v-model:current-page="currentPage"
                      v-model:page-size="pageSize"
                      :page-sizes="[12, 24, 48, 96]"
                      :total="filteredResources.length"
                      layout="total, sizes, prev, pager, next, jumper"
                      background
                      @size-change="handleSizeChange"
                      @current-change="handleCurrentChange"
                    />
                    </div>
                  </div>

                  <!-- 帖子列表 -->
                  <div v-else-if="contentType === 'posts'">
                    <div v-if="posts.length === 0" class="empty-state">
                      <el-empty description="暂无帖子" :image-size="120">
                        <el-button type="primary" @click="goToCreatePost">
                          发布帖子
                        </el-button>
                      </el-empty>
                    </div>
                    
                    <div v-else class="posts-grid" :class="[viewMode]">
                      <div 
                        v-for="post in posts" 
                        :key="post.id"
                        class="post-card"
                        @click="goToPostDetail(post.id)"
                      >
                        <div class="post-header">
                          <div class="post-avatar">
                            <el-avatar :size="36" :src="post.author_avatar">
                              <el-icon><User /></el-icon>
                            </el-avatar>
                          </div>
                          <div class="post-author">
                            <div class="author-name">{{ post.author_name }}</div>
                            <div class="post-time">{{ formatTime(post.created_at) }}</div>
                          </div>
                          <div class="post-category">
                            <el-tag size="small">{{ post.category }}</el-tag>
                          </div>
                        </div>
                        
                        <div class="post-content">
                          <div class="post-title-row">
                            <h3 class="post-title">{{ post.title }}</h3>
                            <div class="status-badges" v-if="post.is_pinned || post.is_featured">
                              <el-tag v-if="post.is_pinned" type="danger" size="small" effect="dark">
                                <el-icon><Star /></el-icon>
                                置顶
                              </el-tag>
                              <el-tag v-if="post.is_featured" type="warning" size="small" effect="dark">
                                <el-icon><Trophy /></el-icon>
                                精华
                              </el-tag>
                            </div>
                          </div>
                          <p class="post-excerpt">{{ post.content.substring(0, 120) }}...</p>
                        </div>
                        
                        <div class="post-footer">
                          <div class="post-stats">
                            <span class="stat-item">
                              <el-icon><View /></el-icon>
                              {{ post.view_count }}
                            </span>
                            <span class="stat-item">
                              <el-icon><ChatLineRound /></el-icon>
                              {{ post.comment_count }}
                            </span>
                            <span class="stat-item">
                              <el-icon><Star /></el-icon>
                              {{ post.like_count }}
                            </span>
                          </div>
                          <div class="post-actions">
                            <el-button link size="small">
                              阅读更多
                            </el-button>
                          </div>
                        </div>
                      </div>
                    </div>

                    <!-- 分页器 - 帖子 -->
                    <div class="pagination-wrapper" v-if="posts.length > 0">
                      <el-pagination
                        v-model:current-page="postsCurrentPage"
                        v-model:page-size="postsPageSize"
                        :page-sizes="[12, 24, 48, 96]"
                        :total="postsTotal"
                        layout="total, sizes, prev, pager, next, jumper"
                        background
                        @size-change="handlePostsSizeChange"
                        @current-change="handlePostsCurrentChange"
                      />
                    </div>
                  </div>
                </div>
              </div>
            </main>

            <!-- 右侧栏：活动面板 + 统计信息 -->
            <aside class="right-column">
              <!-- 热门内容 -->
              <div class="stats-panel">
                <div class="panel-header">
                  <h3>
                    <el-icon><Star /></el-icon>
                    热门内容
                  </h3>
                  <el-button 
                    link 
                    size="small" 
                    :loading="popularContentLoading"
                    @click="fetchPopularContent"
                    class="refresh-btn"
                  >
                    <el-icon><Refresh /></el-icon>
                  </el-button>
                </div>
                <div class="panel-content">
                  <div v-if="popularContentLoading" class="loading-state">
                    <el-skeleton :rows="4" animated />
                  </div>
                  <div v-else-if="popularContent.length === 0" class="empty-state">
                    <p>暂无热门内容</p>
                  </div>
                  <div v-else class="popular-content-list">
                    <div
                      v-for="(item, index) in popularContent"
                      :key="`${item.type}-${item.id}`"
                      class="popular-item-new"
                      @click="goToContent(item)"
                    >
                      <div class="item-rank">
                        <span class="rank-number" :class="getRankClass(index)">{{ index + 1 }}</span>
                      </div>
                      <div class="item-content">
                        <div class="item-title">{{ item.title }}</div>
                        <div class="item-stats">
                          <span class="stat-item">
                            <el-icon><View /></el-icon>
                            {{ formatNumber(item.views) }}
                          </span>
                          <span v-if="item.type === 'resource'" class="stat-item">
                            <el-icon><Download /></el-icon>
                            {{ formatNumber(item.downloads ?? 0) }}
                          </span>
                          <span v-else class="stat-item">
                            <el-icon><Star /></el-icon>
                            {{ formatNumber(item.likes ?? 0) }}
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 社区动态 -->
              <div class="activity-panel">
                <div class="panel-header">
                  <h3>
                    <el-icon><ChatDotRound /></el-icon>
                    社区动态
                  </h3>
                  <div class="panel-actions">
                    <el-button 
                      link 
                      size="small" 
                      :loading="activitiesLoading"
                      @click="fetchRecentActivities"
                      class="refresh-btn"
                    >
                      <el-icon><Refresh /></el-icon>
                    </el-button>
                    <el-button link size="small" @click="scrollToPosts">
                      查看更多
                    </el-button>
                  </div>
                </div>
                <div class="panel-content">
                  <div v-if="activitiesLoading" class="loading-state">
                    <el-skeleton :rows="3" animated />
                  </div>
                  <div v-else-if="recentActivities.length === 0" class="empty-state">
                    <el-empty description="暂无动态" :image-size="80" />
                  </div>
                  <div v-else class="activity-list">
                    <div 
                      v-for="activity in recentActivities.slice(0, 8)" 
                      :key="activity.id"
                      class="activity-item"
                      @click="handleActivityClick(activity)"
                    >
                      <div class="activity-avatar" v-if="activity.username && activity.username !== 'null'">
                        <el-avatar :size="32" :src="activity.user_avatar">
                          <el-icon><User /></el-icon>
                        </el-avatar>
                      </div>
                      <div class="activity-content" :class="{ 'no-avatar': !activity.username || activity.username === 'null' }">
                        <div class="activity-text">
                          <template v-if="activity.username && activity.username !== 'null'">
                            <strong>{{ activity.username }}</strong>
                            {{ activity.action }}
                            <span class="activity-target">{{ activity.target }}</span>
                          </template>
                          <template v-else>
                            <span class="activity-target">{{ activity.target }}</span>
                            {{ activity.action }}
                          </template>
                        </div>
                        <div class="activity-time">{{ formatTime(activity.created_at) }}</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- 热门标签 -->
              <div class="tags-panel">
                <div class="panel-header">
                  <h3>
                    <el-icon><PriceTag /></el-icon>
                    热门标签
                  </h3>
                  <el-button 
                    link 
                    size="small" 
                    :loading="tagsLoading"
                    @click="fetchPopularTags"
                    class="refresh-btn"
                  >
                    <el-icon><Refresh /></el-icon>
                  </el-button>
                </div>
                <div class="panel-content">
                  <div v-if="tagsLoading" class="loading-state">
                    <el-skeleton :rows="3" animated />
                  </div>
                  <div v-else-if="hotTags.length === 0" class="empty-state">
                    <p>暂无热门标签</p>
                  </div>
                  <div v-else class="hot-tags">
                    <el-tag 
                      v-for="tag in hotTags.slice(0, 15)"
                      :key="tag.id || tag.name"
                      size="small"
                      class="tag-item"
                      :style="{ 
                        backgroundColor: tag.color, 
                        borderColor: 'transparent',
                        color: '#FFFFFF',
                        '--el-tag-bg-color': tag.color,
                        '--el-tag-border-color': 'transparent',
                        '--el-tag-text-color': '#FFFFFF',
                        boxShadow: '0 2px 4px rgba(0, 0, 0, 0.1)'
                      }"
                      @click="searchByTag(tag.name)"
                    >
                      {{ tag.name }}
                      <span class="tag-count">({{ tag.count }})</span>
                    </el-tag>
                  </div>
                </div>
              </div>
            </aside>
            
          </div>
        </section>



      </div>
    </main>

    <!-- 页脚 -->
    <footer class="site-footer">
      <div class="footer-content">
        <div class="footer-info">
          <div class="footer-brand">
            <div class="brand-icon">🏘️</div>
            <div>
              <h3>{{ systemSettings.hero_title }}</h3>
              <p>{{ systemSettings.hero_subtitle }}</p>
            </div>
          </div>
          <div class="footer-links">
            <div 
              v-for="(group, key) in footerLinks" 
              :key="key" 
              class="link-group"
            >
              <h4>{{ group.title }}</h4>
              <a 
                v-for="link in group.links" 
                :key="link.text"
                :href="link.url"
              >
                {{ link.text }}
              </a>
            </div>
          </div>
        </div>
        <div class="footer-bottom">
          <p>{{ systemSettings.copyright_text }}</p>
          <div class="footer-social">
            <el-button link size="small">
              <el-icon><Message /></el-icon>
            </el-button>
            <el-button link size="small">
              <el-icon><Share /></el-icon>
            </el-button>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { getPopularTags } from '@/api/tags'
import { userActionApi, type UserAction } from '@/api/userActions'
import { packageApi } from '@/api/packages'
import { useSettings } from '@/composables/useSettings'
import {
  Search,
  User,
  Setting,
  Download,
  Star,
  Calendar,
  Document,
  Collection,
  Bottom,
  Switch,
  Message,
  Phone,
  Location,
  Grid,
  RefreshLeft,
  Filter,
  PriceTag,
  ChatDotRound,
  ArrowRight,
  View,
  BellFilled,
  InfoFilled,
  TrendCharts,
  Plus,
  ChatLineRound,
  Clock,
  Menu,
  Bell,
  MoreFilled,
  ArrowDown,
  Share,
  Refresh,
  Trophy
} from '@element-plus/icons-vue'
import type { FormInstance } from 'element-plus'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import { communityApi } from '@/api/community'
import type { Resource } from '@/types'
import { getUserInfo } from '@/utils/auth'
import { formatDate as formatDateUtil, formatFileSize } from '@/utils/format'
import { categoryApi, type Category } from '@/api/categories'
import { getActiveAnnouncements, type Announcement } from '@/api/announcements'
import type { Package } from '@/api/packages'
import { getPosts, type Post, type PostListResponse } from '@/api/posts'
import { settingsApi } from '@/api/settings'
import userActionService from '@/utils/userActionService'
import { handleDownloadError } from '@/utils/downloadErrorHandler'


const router = useRouter()

// 系统设置
const { systemSettings, loadSystemSettings, loadCommunitySettings } = useSettings()

// 解析页脚链接配置
const footerLinks = computed(() => {
  try {
    return JSON.parse(systemSettings.footer_links || '{}')
  } catch {
    return {
      community: { title: "社区", links: [{ text: "关于我们", url: "/about" }] },
      developer: { title: "开发者", links: [{ text: "API 文档", url: "/api-docs" }] },
      support: { title: "支持", links: [{ text: "联系我们", url: "/contact" }] }
    }
  }
})

// 响应式数据
const searchQuery = ref('')
const activeCategory = ref('all')
const sortBy = ref('latest')
const filterType = ref('all')
const contentType = ref('resources') // 内容类型：'resources' 或 'posts'

// 面板折叠状态
const categoryPanelExpanded = ref(true)
const announcementPanelExpanded = ref(true)

const totalResources = ref(0)

const loading = ref(false)

// 统计数据
const totalDownloads = ref(0)
const totalUsers = ref(0)
const totalLikes = ref(0)

// 分页数据
const currentPage = ref(1)
const pageSize = ref(12)

// 公告数据
const notices = ref<{ id: number; text: string; created_at: string }[]>([])

// 用户信息
const userInfo = computed(() => {
  return getUserInfo() || { username: '用户' }
})

// 登录状态
const isLoggedIn = computed(() => {
  return !!getUserInfo()
})

// 获取公告数据
const fetchAnnouncements = async () => {
  try {
    console.log('🔔 开始获取公告数据...')
    const res = await getActiveAnnouncements()
    console.log('🔔 公告API响应:', res)
    
    if (res.code === 0 && res.data) {
      // 处理不同的数据结构可能性
      if (Array.isArray(res.data)) {
        // 如果是数组，直接使用
        notices.value = res.data.map(announcement => ({
          id: announcement.id,
          text: announcement.title + ': ' + announcement.content,
          created_at: announcement.created_at || new Date().toISOString()
        }))
      } else if (res.data.list && Array.isArray(res.data.list)) {
        // 如果是 {list: []} 格式
        notices.value = res.data.list.map((announcement: any) => ({
          id: announcement.id,
          text: announcement.title + ': ' + announcement.content,
          created_at: announcement.created_at || new Date().toISOString()
        }))
      } else {
        // 如果是单个对象
        console.warn('公告数据不是预期的数组格式:', res.data)
      }
    } else {
      console.error('获取公告失败:', res.message)
    }
  } catch (error: any) {
    console.error('❌ 加载公告出错:', error)
    if (error.response) {
      console.error('API错误响应:', error.response.data)
      ElMessage.warning(`获取公告失败: ${error.response.data?.message || error.message}`)
    } else {
      ElMessage.warning('获取公告失败：网络连接错误')
    }
  }
}

// 分类数据
const categories = ref<Category[]>([])

// 分类标签映射
const categoryLabels = {
  tutorial: '教程',
  tool: '工具',
  template: '模板',
  other: '其他'
}

// 资源数据
const resources = ref<Package[]>([])

// 帖子数据
const posts = ref<Post[]>([])
const postsTotal = ref(0)
const postsCurrentPage = ref(1)
const postsPageSize = ref(5)

// 精选资源数据
const featuredResources = computed(() => {
  // 选择下载量最高的资源作为精选
  return resources.value
    .slice()
    .sort((a, b) => b.download_count - a.download_count)
    .slice(0, 6) // 显示前6个精选资源
})

// 精选资源分组（每组3个，用于轮播）

// 轮播图项目类型定义
interface CarouselItem {
  title: string
  description?: string
  image?: string
  icon?: string
  preview?: string
  link?: string
  order?: number
  tags?: string[]
}

// 轮播图设置类型定义
interface CarouselSettings {
  enabled: boolean
  items: CarouselItem[]
}

// 轮播图设置
const carouselSettings = ref<CarouselSettings>({
  enabled: true,
  items: []
})

// 加载轮播图设置
const loadCarouselSettings = async () => {
  try {
    const response = await settingsApi.getSetting('carousel_settings')
    if (response.code === 0 && response.data) {
      const settings = JSON.parse(response.data.value || '{}')
      carouselSettings.value = {
        enabled: settings.enabled !== false,
        items: (settings.items || []) as CarouselItem[]
      }
    }
  } catch (error) {
    console.warn('加载轮播图设置失败:', error)
    // 提供默认示例数据
    carouselSettings.value = {
      enabled: true,
      items: [
        {
          title: '绳包管理系统',
          description: '专业的绳包资源管理平台',
          image: 'https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=1200&h=380&fit=crop',
          tags: ['完全免费', '最新技术', '丰富功能'],
          link: '/packages'
        }
      ]
    }
  }
}

// 处理轮播图点击
const handleCarouselClick = (item: CarouselItem) => {
  if (item.link) {
    if (item.link.startsWith('http')) {
      window.open(item.link, '_blank')
    } else {
      router.push(item.link)
    }
  } else {
    ElMessage.info(`了解更多关于：${item.title}`)
  }
}


const featuredResourceGroups = computed(() => {
  const groups = []
  for (let i = 0; i < featuredResources.value.length; i += 3) {
    groups.push(featuredResources.value.slice(i, i + 3))
  }
  return groups
})



// 计算属性
const filteredResources = computed(() => {
  // 先复制一份数据，避免直接修改响应式源数组导致递归更新
  let filtered = [...resources.value]

  // 分类筛选
  if (activeCategory.value !== 'all') {
    filtered = filtered.filter(resource => resource.category_id === parseInt(activeCategory.value))
  }

  // 搜索筛选
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(resource =>
      resource.name.toLowerCase().includes(query) ||
      resource.description?.toLowerCase().includes(query) ||
      resource.author.toLowerCase().includes(query)
    )
  }

  // 排序（置顶和精华内容优先）
  filtered.sort((a, b) => {
    // 置顶优先级最高
    if (a.is_pinned && !b.is_pinned) return -1
    if (!a.is_pinned && b.is_pinned) return 1
    
    // 在相同置顶状态下，精华内容优先
    if (a.is_featured && !b.is_featured) return -1
    if (!a.is_featured && b.is_featured) return 1
    
    // 在相同置顶和精华状态下，按指定排序方式排序
  switch (sortBy.value) {
    case 'latest':
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    case 'downloads':
        return b.download_count - a.download_count
    case 'likes':
        return (b.like_count || 0) - (a.like_count || 0)
    case 'favorites':
        return (b.favorite_count || 0) - (a.favorite_count || 0)
      default:
        return 0
  }
  })

  return filtered
})

// 加载帖子数据
const loadPosts = async () => {
  try {
    const params = {
      page: postsCurrentPage.value,
      page_size: postsPageSize.value,
      status: 'Published'
    }
    
    const res = await getPosts(params)
    
    if (res.code === 0 && res.data) {
      // 批量更新帖子数据
      await nextTick()
      posts.value = res.data.list
      postsTotal.value = res.data.total
    } else {
      console.error('获取帖子失败:', res.message)
    }
  } catch (error) {
    console.error('加载帖子出错:', error)
  }
}

// 方法
const loadResources = async () => {
    loading.value = true
  try {
    const params: any = {}
    
    // 添加分类过滤
    if (activeCategory.value !== 'all') {
      params.category_id = parseInt(activeCategory.value)
    }
    
    // 添加搜索过滤
    if (searchQuery.value) {
      params.search = searchQuery.value
    }
    
    // 添加排序和类型过滤
    params.sort = sortBy.value
    if (filterType.value !== 'all') {
      params.type = filterType.value
    }
    
    // 转换分页参数以符合后端 packages 接口
    params.page = currentPage.value
    params.pageSize = pageSize.value
    
    console.log("请求资源参数:", params)
    const res = await packageApi.getPackages(params)
    
    if (res.code === 0 && res.data) {
      // 使用nextTick批量更新，避免同时触发多个响应式更新
      await nextTick()
      
      // 首先设置主要数据
      resources.value = res.data.list
      totalResources.value = res.data.total
      
      // 延迟计算统计数据，避免在同一渲染周期内多次更新
      nextTick(() => {
        totalDownloads.value = resources.value.reduce((sum, resource) => sum + (resource.download_count || 0), 0)
        totalLikes.value = resources.value.reduce((sum, resource) => sum + (resource.like_count || 0), 0)
        totalUsers.value = Math.floor(Math.random() * 1000) + 500
      })
    } else {
      ElMessage.error(res.message || '加载资源失败')
    }
  } catch (error) {
    console.error('加载资源出错:', error)
    ElMessage.error('加载资源时发生错误')
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  // 重置到第一页
  currentPage.value = 1
  // 重新加载资源
  loadResources()
}

// 防抖函数
const debounce = (func: Function, delay: number) => {
  let timeoutId: number
  return (...args: any[]) => {
    clearTimeout(timeoutId)
    timeoutId = setTimeout(() => func.apply(null, args), delay)
  }
}

// 防抖搜索函数
const debouncedSearch = debounce(() => {
  handleSearch()
}, 500)

// 处理搜索输入变化
const handleSearchInput = () => {
  if (searchQuery.value.trim()) {
    debouncedSearch()
  } else {
    // 如果搜索框为空，立即重置
    handleSearch()
  }
}

// 搜索建议相关状态
const showSuggestions = ref(false)

// 搜索建议
const searchSuggestions = computed(() => {
  if (!searchQuery.value.trim()) return []
  
  const query = searchQuery.value.toLowerCase()
  const suggestions: Array<{
    type: 'resource' | 'category'
    text: string
    icon: string
  }> = []
  
  // 从资源名称中提取建议
  resources.value.forEach(resource => {
    if (resource.name.toLowerCase().includes(query)) {
      suggestions.push({
        type: 'resource',
        text: resource.name,
        icon: '📦'
      })
    }
  })
  
  // 从分类中提取建议
  categories.value.forEach(category => {
    if (category.name.toLowerCase().includes(query)) {
      suggestions.push({
        type: 'category',
        text: category.name,
        icon: '🏷️'
      })
    }
  })
  
  // 去重并限制数量
  return suggestions
    .filter((item, index, self) => 
      index === self.findIndex(t => t.text === item.text)
    )
    .slice(0, 5)
})

// 选择搜索建议
const selectSuggestion = (text: string) => {
  searchQuery.value = text
  showSuggestions.value = false
  handleSearch()
}

// 处理搜索框失焦
const handleSearchBlur = () => {
  setTimeout(() => {
    showSuggestions.value = false
  }, 200)
}

// 处理分类切换
const handleCategoryChange = (tab: any) => {
  console.log("分类切换到:", tab.props.name)
  activeCategory.value = tab.props.name
  loadResources() // 重新加载资源
}

const handleSortChange = () => {
  // 客户端排序，无需重新请求
  // 如果后端支持排序，可以在这里重新请求数据
}

const handleFilterChange = () => {
  // 如果后端支持按资源类型筛选，可以在这里重新请求数据
}



const viewResource = (id: number) => {
  router.push(`/resource/${id}`)
}

const downloadResource = async (id: number) => {
  try {
    const res = await communityApi.downloadResource(id)
    if (res.code === 0) {
      ElMessage.success('下载统计成功')
      
      // 记录下载行为
      const resource = resources.value.find((r: any) => r.id === id)
      userActionService.logDownload('Package', id, `下载了资源: ${resource?.name || '未知资源'}`)
        .catch((err: any) => console.error('记录下载行为失败:', err))
      
      loadResources()
    } else {
      ElMessage.error(res.msg || '下载失败')
    }
  } catch (error: any) {
    handleDownloadError(error, '下载失败')
  }
}

const goToLogin = () => {
  router.push('/login')
}

// 切换分类面板展开/折叠状态
const toggleCategoryPanel = () => {
  categoryPanelExpanded.value = !categoryPanelExpanded.value
}

// 切换公告面板展开/折叠状态
const toggleAnnouncementPanel = () => {
  announcementPanelExpanded.value = !announcementPanelExpanded.value
}

const goToAdmin = () => {
  console.log('isLoggedIn:', localStorage.getItem('isLoggedIn'))
  console.log('userInfo:', localStorage.getItem('userInfo'))
  const user = getUserInfo && getUserInfo()
  if (!user) {
    router.push('/login')
    return
  }
  if (user.role === 'admin' || user.role === 'moderator') {
    router.push('/admin')
  } else if (user.role === 'elder') {
    router.push('/elder')
  } else if (user.role === 'user') {
    router.push('/user')
  } else {
    router.push('/403')
  }
}

const goToUserProfile = () => {
  const user = getUserInfo()
  if (user?.role === 'elder') {
    router.push('/elder/profile')
  } else if (user?.role === 'user') {
    router.push('/user/profile')
  } else if (user?.role === 'admin') {
    router.push('/admin/users')
  } else {
    router.push('/login')
  }
}

const handleLogout = async () => {
  try {
    await ElMessageBox.confirm(
      '确定要退出登录吗？',
      '确认退出',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )
    
    // 清除用户信息
    localStorage.removeItem('token')
    localStorage.removeItem('userInfo')
    localStorage.removeItem('isLoggedIn')
    localStorage.removeItem('loginTime')
    localStorage.removeItem('remember_me')
    localStorage.removeItem('username')
    
    // 清除Cookie
    document.cookie = 'auth_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
    document.cookie = 'user_info=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
    document.cookie = 'remember_token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;'
    
    ElMessage.success('已退出登录')
    router.push('/login')
  } catch (error) {
    // 用户取消
  }
}

const getCategoryLabel = (categoryId: number | null) => {
  if (!categoryId) return '未分类'
  const category = categories.value.find(c => c.id === categoryId)
  return category ? category.name : '未分类'
}

const getCategoryColor = (categoryId: any) => {
  const colorMap: { [key: string]: string } = {
    1: '#409EFF', // 蓝色
    2: '#67C23A', // 绿色
    3: '#E6A23C', // 黄色
    4: '#F56C6C', // 红色
    5: '#909399'  // 灰色
  }
  return colorMap[categoryId] || '#409EFF'
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('zh-CN')
}

const formatNumber = (num: number) => {
  if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toLocaleString()
}



// 加载分类数据
const loadCategories = async () => {
  try {
    const res = await categoryApi.getCategories()
    if (res.code === 0 && res.data) {
      categories.value = res.data.list || []
      console.log("获取到的分类:", categories.value)
    } else {
      console.error('获取分类失败:', res.message)
    }
  } catch (error) {
    console.error('加载分类出错:', error)
  }
}

// 移除重复的初始化 - 使用下面的统一初始化

const scrollToTop = () => {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

const scrollToPosts = () => {
  const postsSection = document.getElementById('posts-section')
  if (postsSection) {
    postsSection.scrollIntoView({ behavior: 'smooth' })
  }
}

// 生成粒子样式
const getParticleStyle = (index: number) => {
  const size = Math.random() * 8 + 4 // 4-12px
  const left = Math.random() * 100 // 0-100%
  const top = Math.random() * 100 // 0-100%
  const animationDelay = Math.random() * 15 // 0-15s
  const animationDuration = Math.random() * 10 + 10 // 10-20s
  
  return {
    width: `${size}px`,
    height: `${size}px`,
    left: `${left}%`,
    top: `${top}%`,
    animationDelay: `${animationDelay}s`,
    animationDuration: `${animationDuration}s`
  }
}

const getCategoryCount = (categoryId: number) => {
  return resources.value.filter(resource => resource.category_id === categoryId).length
}

const viewMode = ref('grid')

const authorFilter = ref('')
const downloadFilter = ref('')
const dateFilter = ref('')
const keywordFilter = ref('')
const sizeFilter = ref('')
const tagFilter = ref('')
const advancedSortBy = ref('newest')
const showAdvancedFilters = ref(false)

const hasActiveFilters = computed(() => {
  return authorFilter.value || downloadFilter.value || dateFilter.value || keywordFilter.value || sizeFilter.value || tagFilter.value
})

const uniqueAuthors = computed(() => {
  return Array.from(new Set(resources.value.map(resource => resource.author)))
})

// 添加计算缓存以优化性能
const paginatedResourcesCache = ref(null)
let paginatedResourcesTimer: number | null = null

const paginatedResources = computed(() => {
  // 简单的防抖机制，避免频繁重新计算
  if (paginatedResourcesTimer) {
    clearTimeout(paginatedResourcesTimer)
  }
  
  let filtered = filteredResources.value
  // 注意：filteredResources已经包含了置顶和精华的排序逻辑，确保在这里不会被覆盖

  // 作者筛选 - 优化字符串比较
  if (authorFilter.value) {
    const authorQuery = authorFilter.value.toLowerCase()
    filtered = filtered.filter(resource => resource.author.toLowerCase().includes(authorQuery))
  }

  // 下载量筛选
  if (downloadFilter.value) {
    filtered = filtered.filter(resource => {
      const downloadCount = resource.download_count || 0
      switch (downloadFilter.value) {
        case '0-10':
          return downloadCount >= 0 && downloadCount <= 10
        case '11-50':
          return downloadCount >= 11 && downloadCount <= 50
        case '51-100':
          return downloadCount >= 51 && downloadCount <= 100
        case '100+':
          return downloadCount > 100
        default:
          return true
      }
    })
  }

  // 日期筛选 - 优化：缓存日期计算结果
  if (dateFilter.value) {
    // 简化日期筛选逻辑，避免复杂的日期计算
    const now = Date.now()
    const filterDate = dateFilter.value
    
    filtered = filtered.filter(resource => {
      const createdAt = new Date(resource.created_at).getTime()
      const dayMs = 24 * 60 * 60 * 1000
      
      switch (filterDate) {
        case 'today':
          return now - createdAt <= dayMs
        case 'week':
          return now - createdAt <= 7 * dayMs
        case 'month':
          return now - createdAt <= 30 * dayMs
        case 'quarter':
          return now - createdAt <= 90 * dayMs
        default:
          return true
      }
    })
  }

  // 关键词筛选
  if (keywordFilter.value) {
    const query = keywordFilter.value.toLowerCase()
    filtered = filtered.filter(resource =>
      resource.name.toLowerCase().includes(query) ||
      resource.description?.toLowerCase().includes(query) ||
      resource.author.toLowerCase().includes(query)
    )
  }

  // 文件大小筛选
  if (sizeFilter.value) {
    filtered = filtered.filter(resource => {
      const fileSize = resource.file_size || 0
      switch (sizeFilter.value) {
        case 'small':
          return fileSize < 1024 * 1024
        case 'medium':
          return fileSize >= 1024 * 1024 && fileSize < 10 * 1024 * 1024
        case 'large':
          return fileSize >= 10 * 1024 * 1024
        default:
          return true
      }
    })
  }

  // 标签筛选
  if (tagFilter.value) {
    const tags = tagFilter.value.split(',').map(tag => tag.trim())
    filtered = filtered.filter(resource => {
      // 简化标签筛选：在资源名称和描述中搜索标签关键词
      const query = tagFilter.value.toLowerCase()
      return resource.name.toLowerCase().includes(query) || 
             resource.description?.toLowerCase().includes(query)
    })
  }

  // 排序 - 修改为保持filteredResources中的置顶-精华-普通排序逻辑
  switch (advancedSortBy.value) {
    case 'newest':
      // 在保持置顶和精华优先级的前提下，按时间排序
      filtered = [...filtered].sort((a, b) => {
        // 如果两个资源的置顶状态不同，保持原有排序
        if ((a.is_pinned && !b.is_pinned) || (!a.is_pinned && b.is_pinned)) return a.is_pinned ? -1 : 1;
        
        // 如果两个资源的精华状态不同，保持原有排序
        if ((a.is_featured && !b.is_featured) || (!a.is_featured && b.is_featured)) return a.is_featured ? -1 : 1;
        
        // 相同优先级下，按时间排序
        return new Date(b.created_at).getTime() - new Date(a.created_at).getTime();
      });
      break;
    case 'oldest':
      filtered.sort((a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime())
      break
    case 'most_downloaded':
      filtered.sort((a, b) => b.download_count - a.download_count)
      break
    case 'least_downloaded':
      filtered.sort((a, b) => a.download_count - b.download_count)
      break
    case 'name_asc':
      filtered.sort((a, b) => a.name.localeCompare(b.name))
      break
    case 'name_desc':
      filtered.sort((a, b) => b.name.localeCompare(a.name))
      break
    default:
      // 确保置顶和精华的优先级始终得到保持
      break;
  }

  // 分页逻辑
  const start = (currentPage.value - 1) * pageSize.value
  const end = start + pageSize.value
  return filtered.slice(start, end)
})

// 已删除重复的filteredResourcesForPagination computed属性
// 统一使用paginatedResources来避免重复计算

const clearFilters = () => {
  authorFilter.value = ''
  downloadFilter.value = ''
  dateFilter.value = ''
  keywordFilter.value = ''
  sizeFilter.value = ''
  tagFilter.value = ''
  advancedSortBy.value = 'newest'
  showAdvancedFilters.value = false
  currentPage.value = 1
}

// 监听筛选条件变化，重置页码 - 添加防抖
let filterWatchTimer: number | null = null
watch([authorFilter, downloadFilter, dateFilter, keywordFilter, sizeFilter, tagFilter, advancedSortBy], () => {
  if (filterWatchTimer) {
    clearTimeout(filterWatchTimer)
  }
  filterWatchTimer = window.setTimeout(() => {
    currentPage.value = 1
  }, 100)
})

const toggleAdvancedFilters = () => {
  showAdvancedFilters.value = !showAdvancedFilters.value
}

// 防抖计时器
let backgroundUpdateTimer: number | null = null

// 动态设置背景高度以覆盖整个页面（防抖版本）
const updateBackgroundHeight = () => {
  if (backgroundUpdateTimer) {
    clearTimeout(backgroundUpdateTimer)
  }
  
  backgroundUpdateTimer = window.setTimeout(() => {
    nextTick(() => {
      const homeContainer = document.querySelector('.home-container')
      const dynamicBackground = document.querySelector('.dynamic-background')
      const gradientLayer = document.querySelector('.gradient-layer')
      
      if (homeContainer && dynamicBackground && gradientLayer) {
        const containerHeight = homeContainer.scrollHeight
        const minHeight = Math.max(containerHeight, window.innerHeight)
        
        ;(dynamicBackground as HTMLElement).style.height = `${minHeight}px`
        ;(gradientLayer as HTMLElement).style.height = `${minHeight}px`
      }
      backgroundUpdateTimer = null
    })
  }, 100) // 100ms防抖延迟
}

// 存储observer引用以便清理
let mutationObserver: MutationObserver | null = null

// 背景高度相关逻辑已合并到主onMounted钩子中

// 清理资源
onUnmounted(() => {
  // 清理事件监听器
  window.removeEventListener('resize', updateBackgroundHeight)
  
  // 清理MutationObserver
  if (mutationObserver) {
    mutationObserver.disconnect()
    mutationObserver = null
  }
  
  // 清理所有计时器
  if (backgroundUpdateTimer) {
    clearTimeout(backgroundUpdateTimer)
    backgroundUpdateTimer = null
  }
  
  if (filterWatchTimer) {
    clearTimeout(filterWatchTimer)
    filterWatchTimer = null
  }
  
  if (paginatedResourcesTimer) {
    clearTimeout(paginatedResourcesTimer)
    paginatedResourcesTimer = null
  }
})

// onUpdated钩子已移除，因为MutationObserver已经监听所有DOM变化
// 避免与MutationObserver形成无限递归循环

// 社区动态数据
interface ActivityItem {
  id: number
  username: string
  action: string
  target: string
  user_avatar?: string
  created_at: string
}

const recentActivities = ref<ActivityItem[]>([])
const activitiesLoading = ref(false)

// 获取社区动态
const fetchRecentActivities = async () => {
  activitiesLoading.value = true
  try {
    console.log('🎭 开始获取社区动态...')
    // 调用公开的社区动态API，无需认证
    const response = await fetch('http://localhost:15201/api/v1/community/activities?page=1&page_size=20', {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      }
    }).then(res => res.json())
    console.log('🎭 社区动态API响应:', response)
    console.log('🎭 响应数据结构:', JSON.stringify(response, null, 2))

    if (response.code === 0) {
      // 检查多种可能的数据结构
      let actionsData = response.data?.actions || response.data || []
      console.log('🎭 提取的actions数据:', actionsData)
      
      if (Array.isArray(actionsData) && actionsData.length > 0) {
        console.log('🎭 原始actions数据:', actionsData)
        
        const filteredActivities = actionsData
        .filter((action: UserAction) => {
            // 只保留真正有意义的用户操作
            const meaningfulActions = ['Upload', 'Create', 'Comment', 'Download', 'Like', 'Share', 'Edit', 'Register']
            return meaningfulActions.includes(action.action_type)
        })
          .map((action: any) => {
            const { username, actionText, target, avatar } = parseActivityInfo(action)
          
          return {
            id: action.id,
            username,
            action: actionText,
            target,
              user_avatar: avatar,
            created_at: action.created_at
          }
        })
        .slice(0, 10) // 取前10条有效记录

      recentActivities.value = filteredActivities
        console.log('🎭 设置的社区动态:', filteredActivities)
      } else {
        console.log('🎭 没有获取到有效的actions数据')
        recentActivities.value = []
      }
    } else {
      console.log('🎭 API响应code不为0:', response.code, response.message)
      recentActivities.value = []
    }
  } catch (error: any) {
    console.error('❌ 获取社区动态失败:', error)
    recentActivities.value = []
    
    if (error.response) {
      console.error('API错误响应:', error.response.data)
      ElMessage.warning(`获取社区动态失败: ${error.response.data?.message || error.message}`)
    } else {
      ElMessage.warning('获取社区动态失败：网络连接错误')
    }
  } finally {
    activitiesLoading.value = false
  }
}

// 判断是否需要过滤的操作
const isFilteredAction = (actionType: string, details?: string): boolean => {
  // 只过滤掉真正不需要显示的操作类型
  const filteredActions = [
    'Login',       // 登录
    'Logout',      // 登出
    'Search',      // 搜索操作
    'Click'        // 点击操作
  ]
  
  // 过滤敏感信息关键词
  const sensitiveKeywords = [
    'ip_address',
    'user_agent',
    'session',
    'token',
    'password',
    '访问页面',
    '点击了',
    '搜索了'
  ]
  
  if (filteredActions.includes(actionType)) {
    return true
  }
  
  if (details) {
    return sensitiveKeywords.some(keyword => 
      details.toLowerCase().includes(keyword.toLowerCase())
    )
  }
  
  return false
}

// 解析活动信息
const parseActivityInfo = (action: any) => {
  // 使用返回的用户信息
  let username = null
  let avatar = null
  
  // 检查是否有有效的用户信息
  if (action.user_id === null || action.user_id === -1 || !action.username || action.username === 'null') {
    username = null // 未登录用户不显示用户名
  } else if (action.nickname) {
    username = action.nickname
  } else if (action.username) {
    username = action.username
  } else {
    username = `用户${action.user_id}`
  }
  
  // 获取用户头像
  if (action.avatar) {
    avatar = action.avatar
  }
  
  // 根据是否有用户信息决定动作文本格式
  let actionText = username ? formatActionType(action.action_type) : formatActionTypePassive(action.action_type)
  let target = '某个内容'
  
  // 解析details获取更具体的信息
  if (action.details) {
    // 页面访问特殊处理
    if (action.action_type === 'PageView') {
      // 解析页面路径
      const pageMatch = action.details.match(/访问页面[:：]\s*(.+)/)
      if (pageMatch) {
        const pagePath = pageMatch[1]
        // 将路径转换为友好的页面名称
        const pageNames: Record<string, string> = {
          '/admin': '管理后台',
          '/admin/posts': '帖子管理',
          '/admin/packages': '资源管理',
          '/admin/categories': '分类管理',
          '/admin/users': '用户管理',
          '/posts': '帖子列表',
          '/packages': '资源列表',
          '/categories': '分类页面',
          '/home': '首页'
        }
        target = pageNames[pagePath] || pagePath.replace('/', '')
      } else {
        target = '某个页面'
      }
    }
    // 其他类型的详情解析
    else if (!isFilteredAction(action.action_type, action.details)) {
    // 尝试提取内容标题或名称
    if (action.details.includes('帖子:') || action.details.includes('文章:')) {
      const titleMatch = action.details.match(/(?:帖子|文章)[:：]\s*(.+)/)
      if (titleMatch) {
        target = titleMatch[1].slice(0, 20) + (titleMatch[1].length > 20 ? '...' : '')
      }
    } else if (action.details.includes('资源:') || action.details.includes('包:')) {
      const titleMatch = action.details.match(/(?:资源|包)[:：]\s*(.+)/)
      if (titleMatch) {
        target = titleMatch[1].slice(0, 20) + (titleMatch[1].length > 20 ? '...' : '')
      }
    } else if (action.details.includes('评论:')) {
      const commentMatch = action.details.match(/评论[:：]\s*(.+)/)
      if (commentMatch) {
        target = `"${commentMatch[1].slice(0, 30)}${commentMatch[1].length > 30 ? '...' : ''}"`
      }
    } else {
      // 通用解析：取第一个有意义的词作为目标
        const words = action.details.split(/\s+/).filter((word: string) => 
        word.length > 1 && !word.includes('用户') && !word.includes('操作')
      )
      if (words.length > 0) {
        target = words[0].slice(0, 15) + (words[0].length > 15 ? '...' : '')
        }
      }
    }
  }
  
  return { username, actionText, target, avatar }
}

// 格式化操作类型为友好显示
const formatActionType = (actionType: string): string => {
  const actionMap: Record<string, string> = {
    'Register': '注册了账号',
    'Upload': '上传了资源',
    'Download': '下载了',
    'Comment': '评论了',
    'Like': '点赞了',
    'Share': '分享了',
    'Edit': '编辑了',
    'Delete': '删除了',
    'Create': '创建了',
    'Update': '更新了',
    'Post': '发布了帖子',
    'Reply': '回复了',
    'PageView': '访问了',
    'Login': '登录了系统',
    'Logout': '退出了系统'
  }
  return actionMap[actionType] || `${actionType.toLowerCase()}`
}

// 被动语态格式（用于未登录用户）
const formatActionTypePassive = (actionType: string): string => {
  const actionMap: Record<string, string> = {
    'Register': '被注册',
    'Upload': '被上传',
    'Download': '被下载',
    'Comment': '被评论',
    'Like': '被点赞',
    'Share': '被分享',
    'Edit': '被编辑',
    'Delete': '被删除',
    'Create': '被创建',
    'Update': '被更新',
    'Post': '被发布',
    'Reply': '被回复',
    'PageView': '被访问',
    'Login': '被登录',
    'Logout': '被退出'
  }
  return actionMap[actionType] || `被${actionType.toLowerCase()}`
}

// 格式化时间方法
const formatTime = (dateString: string) => {
  if (!dateString) return '刚刚'
  
  const date = new Date(dateString)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  const minutes = Math.floor(diff / (1000 * 60))
  const hours = Math.floor(diff / (1000 * 60 * 60))
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 30) return `${days}天前`
  
  return date.toLocaleDateString()
}

// 应用排序方法
const applySorting = () => {
  // 排序逻辑已在computed中实现
  currentPage.value = 1
}

// 重置筛选方法
const resetFilters = () => {
  searchQuery.value = ''
  activeCategory.value = 'all'
  sortBy.value = 'created_at'
  currentPage.value = 1
  loadResources()
}

// 分页处理方法
const handleSizeChange = (newSize: number) => {
  pageSize.value = newSize
  currentPage.value = 1
}

const handleCurrentChange = (newPage: number) => {
  currentPage.value = newPage
}

// 帖子分页处理方法
const handlePostsSizeChange = (newSize: number) => {
  postsPageSize.value = newSize
  postsCurrentPage.value = 1
  loadPosts()
}

const handlePostsCurrentChange = (newPage: number) => {
  postsCurrentPage.value = newPage
  loadPosts()
}

// 标签搜索方法
const searchByTag = (tagName: string) => {
  searchQuery.value = tagName
  handleSearch()
}

// 帖子相关方法
const goToCreatePost = () => {
  if (!isLoggedIn.value) {
    ElMessage.warning('请先登录')
    router.push('/login')
    return
  }
  router.push('/posts/create')
}

const goToPost = (postId: number) => {
  router.push(`/posts/${postId}`)
}

const goToPostDetail = (postId: number) => {
  router.push(`/posts/${postId}`)
}

const goToPosts = () => {
  router.push('/posts')
}

// 统计数据
const totalPosts = computed(() => posts.value.length)

// 为资源对象添加tags支持
const getResourceWithTags = (resource: any) => {
  return {
    ...resource,
    tags: resource.tags || []
  }
}

// 更新公告数据结构
const updateNotices = () => {
  notices.value = notices.value.map(notice => ({
    ...notice,
    created_at: notice.created_at || new Date().toISOString()
  }))
}

// 热门标签数据
interface HotTag {
  id?: number
  name: string
  count: number
  color: string
  description?: string
}

const hotTags = ref<HotTag[]>([])
const tagsLoading = ref(false)

// 热门内容数据
interface PopularContent {
  id: number
  type: 'resource' | 'post'
  title: string
  views: number
  downloads?: number
  likes?: number
}

const popularContent = ref<PopularContent[]>([])
const popularContentLoading = ref(false)

// 获取热门标签
const fetchPopularTags = async () => {
  tagsLoading.value = true
  try {
    console.log('🏷️ 开始获取热门标签...')
    const response = await getPopularTags()
    console.log('🏷️ 标签API响应:', response)
    if (response.code === 0 && response.data) {
      // 预定义的丰富多彩调色板
      const colorPalette = [
        '#4096ff', '#9254de', '#f759ab', '#36cfc9', '#52c41a', 
        '#faad14', '#ff4d4f', '#7356f1', '#13c2c2', '#fa541c',
        '#eb2f96', '#722ed1', '#1677ff', '#fadb14', '#a0d911',
        '#f5222d', '#fa8c16', '#a0d911', '#52c41a', '#13c2c2'
      ]
      
      hotTags.value = response.data.map((tag, index) => ({
        id: tag.id,
        name: tag.name,
        count: tag.use_count || 0,
        color: colorPalette[index % colorPalette.length],
        description: tag.description
      }))
    }
  } catch (error: any) {
    console.error('❌ 获取热门标签失败:', error)
    if (error.response) {
      console.error('API错误响应:', error.response.data)
      ElMessage.warning(`获取热门标签失败: ${error.response.data?.message || error.message}`)
    } else {
      ElMessage.warning('获取热门标签失败：网络连接错误')
    }
  } finally {
    tagsLoading.value = false
  }
}

// 获取热门内容
const fetchPopularContent = async () => {
  popularContentLoading.value = true
  try {
    // 获取热门资源和帖子，合并并排序
    const [resourcesRes, postsRes] = await Promise.all([
      packageApi.getPackages({ pageSize: 10, status: 'Active' }),
      getPosts({ page_size: 10 })
    ])

    const popularItems: PopularContent[] = []

    // 处理资源数据
    if (resourcesRes.code === 0 && resourcesRes.data?.list) {
      resourcesRes.data.list.forEach((resource: any) => {
        popularItems.push({
          id: resource.id,
          type: 'resource',
          title: resource.name,
          views: resource.download_count || 0, // 资源用下载量作为热度指标
          downloads: resource.download_count || 0,
          likes: resource.like_count || 0
        })
      })
    }

    // 处理帖子数据
    if (postsRes.code === 0 && postsRes.data?.list) {
      postsRes.data.list.forEach((post: any) => {
        popularItems.push({
          id: post.id,
          type: 'post',
          title: post.title,
          views: post.view_count || 0,
          likes: post.like_count || 0
        })
      })
    }

    // 按热度排序并取前8个
    popularContent.value = popularItems
      .sort((a, b) => b.views - a.views)
      .slice(0, 8)

  } catch (error) {
    console.error('获取热门内容失败:', error)
    ElMessage.warning('获取热门内容失败')
  } finally {
    popularContentLoading.value = false
  }
}

// 获取排名样式类
const getRankClass = (index: number) => {
  if (index === 0) return 'rank-first'
  if (index === 1) return 'rank-second'  
  if (index === 2) return 'rank-third'
  return 'rank-normal'
}

// 轮播图相关
const currentSlide = ref(0)

// 格式化下载数量
const formatDownloadCount = (count: number) => {
  if (count >= 10000) {
    return (count / 10000).toFixed(1) + 'w'
  } else if (count >= 1000) {
    return (count / 1000).toFixed(1) + 'k'
  }
  return count.toString()
}



// 跳转到指定轮播项
const goToSlide = (index: number) => {
  currentSlide.value = index
  // 这里可以添加控制轮播图的逻辑
}

// 跳转到内容详情
const goToContent = (item: PopularContent) => {
  if (item.type === 'resource') {
    router.push(`/resource/${item.id}`)
  } else {
    router.push(`/posts/${item.id}`)
  }
}

// 处理社区动态点击事件
const handleActivityClick = (activity: ActivityItem) => {
  const action = activity.action.toLowerCase()
  
  if (action.includes('上传') || action.includes('资源')) {
    // 资源相关操作，跳转到资源分类页面
    router.push('/category')
    ElMessage.success('查看最新资源')
  } else if (action.includes('帖子') || action.includes('发布了帖子')) {
    // 帖子相关操作，跳转到帖子列表
    router.push('/posts')
    ElMessage.success('查看社区帖子')
  } else if (action.includes('评论') || action.includes('点赞')) {
    // 评论和点赞操作，跳转到帖子列表查看讨论
    router.push('/posts')
    ElMessage.success('查看社区讨论')
  } else if (action.includes('下载')) {
    // 下载操作，跳转到资源页面
    router.push('/category')
    ElMessage.success('查看热门资源')
  } else {
    // 其他操作，跳转到对应页面
    router.push('/posts')
    ElMessage.info('查看社区动态')
  }
}

// 初始化时更新数据结构
onMounted(async () => {
  console.log('🚀 开始初始化Home页面...')
  
  // 初始化背景高度
  updateBackgroundHeight()
  window.addEventListener('resize', updateBackgroundHeight)
  
  updateNotices()
  
  try {
    // 加载系统设置和社区设置
    console.log('⚙️ 开始加载系统配置...')
    await loadSystemSettings().catch(err => console.error('系统设置加载失败:', err))
    await loadCommunitySettings().catch(err => console.error('社区设置加载失败:', err))
    await loadCarouselSettings().catch((err: any) => console.error('轮播图设置加载失败:', err))
    
    console.log('✅ Home页面初始化完成!')
    
    // 延迟加载其他数据以避免递归更新
    setTimeout(async () => {
      console.log('📦 开始加载基础数据...')
      await loadCategories().catch(err => console.error('分类加载失败:', err))
      await loadResources().catch(err => console.error('资源加载失败:', err))
      await loadPosts().catch(err => console.error('帖子加载失败:', err))
      
      console.log('🔥 开始加载热门数据...')
      await fetchPopularTags().catch(err => console.error('热门标签加载失败:', err))
      await fetchPopularContent().catch(err => console.error('热门内容加载失败:', err))
      await fetchRecentActivities().catch(err => console.error('社区动态加载失败:', err))
      await fetchAnnouncements().catch(err => console.error('公告加载失败:', err))
      
      console.log('🎉 所有数据加载完成!')
    }, 100)
  } catch (error) {
    console.error('❌ 初始化过程中出错:', error)
  }
})
</script>

<style scoped>
/* 引入外部样式文件 */
@import '@/assets/home-styles.css';

/* ===== 三栏布局样式优化 ===== */
.three-column-section {
  margin-bottom: var(--space-4xl);
  position: relative;
}

.column-wrapper {
  display: grid;
  grid-template-columns: 300px 1fr 320px;
  gap: var(--space-2xl);
  align-items: start;
  max-width: 1440px;
  margin: 0 auto;
  padding: 0 var(--space-2xl);
  box-sizing: border-box;
}

/* 三栏布局响应式优化 */
@media (min-width: 1600px) {
  .column-wrapper {
    grid-template-columns: 320px 1fr 360px;
    gap: var(--space-3xl);
    max-width: 1600px;
  }
}

@media (max-width: 1400px) {
  .column-wrapper {
    grid-template-columns: 280px 1fr 300px;
    gap: var(--space-2xl);
  }
}

@media (max-width: 1200px) {
  .column-wrapper {
    grid-template-columns: 260px 1fr 280px;
    gap: var(--space-xl);
    padding: 0 var(--space-xl);
  }
}

@media (max-width: 1024px) {
  .column-wrapper {
    grid-template-columns: 1fr;
    gap: var(--space-xl);
    padding: 0 var(--space-lg);
  }
  
  .left-column,
  .right-column {
    position: static;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: var(--space-lg);
  }
}

@media (max-width: 768px) {
  .column-wrapper {
    padding: 0 var(--space-lg);
    gap: var(--space-lg);
  }
  
  .left-column,
  .right-column {
    grid-template-columns: 1fr;
    gap: var(--space-md);
  }
}

@media (max-width: 480px) {
  .column-wrapper {
    padding: 0 var(--space-md);
    gap: var(--space-md);
  }
}

/* 左侧栏样式优化 */
.left-column {
  display: flex;
  flex-direction: column;
  gap: var(--space-xl);
  position: sticky;
  top: calc(72px + var(--space-2xl));
  max-height: calc(100vh - 72px - var(--space-4xl));
  overflow-y: auto;
  scrollbar-width: thin;
}

/* 中间主内容样式优化 */
.center-column {
  min-height: 600px;
  display: flex;
  flex-direction: column;
  gap: var(--space-xl);
}

/* 右侧栏样式优化 */
.right-column {
  display: flex;
  flex-direction: column;
  gap: var(--space-xl);
  position: sticky;
  top: calc(72px + var(--space-2xl));
  max-height: calc(100vh - 72px - var(--space-4xl));
  overflow-y: auto;
  scrollbar-width: thin;
}

/* ===== 面板通用样式优化 ===== */
.nav-panel,
.announcement-panel,
.stats-panel,
.activity-panel,
.tags-panel,
.content-panel {
  background: linear-gradient(135deg, 
    var(--bg-glass-strong) 0%, 
    var(--bg-elevated) 100%);
  backdrop-filter: blur(25px);
  border: 1px solid var(--border-color-light);
  border-radius: var(--radius-2xl);
  box-shadow: var(--shadow-xl);
  overflow: hidden;
  transition: var(--transition-normal);
  position: relative;
}

.nav-panel::before,
.announcement-panel::before,
.stats-panel::before,
.activity-panel::before,
.tags-panel::before,
.content-panel::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 50%,
    rgba(255, 255, 255, 0.02) 100%);
  z-index: 1;
  pointer-events: none;
  border-radius: inherit;
}

.nav-panel:hover,
.announcement-panel:hover,
.stats-panel:hover,
.activity-panel:hover,
.tags-panel:hover {
  box-shadow: var(--shadow-2xl);
  transform: translateY(-6px);
  border-color: var(--border-color-hover);
}

.content-panel:hover {
  box-shadow: var(--shadow-2xl);
  transform: translateY(-2px);
  border-color: var(--border-color-hover);
}

html.dark .nav-panel,
html.dark .announcement-panel,
html.dark .stats-panel,
html.dark .activity-panel,
html.dark .tags-panel,
html.dark .content-panel {
  background: var(--bg-glass-strong);
  border-color: var(--border-color);
}

.panel-header {
  padding: var(--space-xl) var(--space-2xl);
  border-bottom: 1px solid var(--border-color-light);
  background: linear-gradient(135deg, 
    var(--bg-secondary) 0%, 
    var(--bg-tertiary) 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  transition: var(--transition-fast);
  position: relative;
  z-index: 2;
}

.panel-header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.15) 0%, 
    rgba(255, 255, 255, 0.08) 50%,
    rgba(255, 255, 255, 0.05) 100%);
  opacity: 0;
  transition: var(--transition-fast);
  z-index: -1;
  border-radius: var(--radius-lg) var(--radius-lg) 0 0;
}

.panel-header:hover {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%);
  border-bottom-color: var(--border-color-hover);
}

.panel-header:hover::before {
  opacity: 1;
}

.panel-header h3 {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0;
  line-height: var(--line-height-tight);
}

.panel-header h3 .el-icon {
  color: var(--brand-color);
  font-size: var(--font-size-xl);
}

html.dark .panel-header {
  background: var(--bg-secondary);
  border-bottom-color: var(--border-color);
}

html.dark .panel-header:hover {
  background: var(--bg-hover);
}

.panel-actions {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

/* 折叠图标样式优化 */
.collapse-icon {
  color: var(--text-tertiary);
  transition: var(--transition-fast);
  font-size: var(--font-size-lg);
}

.collapse-icon.collapsed {
  transform: rotate(-90deg);
}

.panel-header:hover .collapse-icon {
  color: var(--text-secondary);
}

/* 面板内容样式优化 */
.panel-content {
  padding: var(--space-xl);
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
  min-height: 200px;
}

/* 快速导航样式优化 */
.quick-nav {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  padding: var(--space-lg) var(--space-xl);
  border-radius: var(--radius-xl);
  color: var(--text-secondary);
  cursor: pointer;
  transition: var(--transition-fast);
  position: relative;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
  min-height: 48px;
  box-sizing: border-box;
  background: linear-gradient(135deg, 
    transparent 0%, 
    rgba(255, 255, 255, 0.03) 100%);
  border: 1px solid transparent;
  z-index: 2;
}

.nav-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%);
  border-radius: inherit;
  opacity: 0;
  transition: var(--transition-fast);
  z-index: -1;
}

.nav-item:hover {
  color: var(--text-primary);
  transform: translateX(6px) scale(1.02);
  border-color: var(--border-color-hover);
}

.nav-item:hover::before {
  opacity: 1;
}

.nav-item.active {
  color: var(--text-brand);
  font-weight: var(--font-weight-semibold);
  background: linear-gradient(135deg, 
    var(--bg-selected) 0%, 
    var(--bg-brand-subtle) 100%);
  border: 1px solid var(--border-color-brand);
  transform: translateX(8px) scale(1.03);
  box-shadow: var(--shadow-md);
}

.nav-item.active::before {
  background: linear-gradient(135deg, 
    var(--bg-brand-subtle) 0%, 
    var(--bg-selected) 100%);
  opacity: 0.7;
}

.nav-item.active::after {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 4px;
  height: 24px;
  background: var(--brand-color);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  z-index: 3;
}

.nav-item .el-icon {
  flex-shrink: 0;
  font-size: var(--font-size-lg);
  color: inherit;
}

.nav-item span {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-count {
  flex-shrink: 0;
  background: var(--bg-tertiary);
  color: var(--text-tertiary);
  font-size: var(--font-size-xs);
  font-weight: var(--font-weight-semibold);
  padding: var(--space-xs) var(--space-sm);
  border-radius: var(--radius-full);
  min-width: 24px;
  text-align: center;
  line-height: 1;
  border: 1px solid var(--border-color-light);
}

.nav-item.active .nav-count {
  background: var(--brand-color);
  color: var(--text-inverse);
  border-color: var(--brand-color);
}

/* 筛选工具栏样式优化 */
.filter-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-2xl) 0;
  gap: var(--space-xl);
  border-bottom: 1px solid var(--border-color-light);
  margin-bottom: var(--space-2xl);
}

.filter-left {
  flex: 1;
  display: flex;
  align-items: center;
  gap: var(--space-lg);
}

.filter-right {
  display: flex;
  gap: var(--space-lg);
  align-items: center;
  flex-shrink: 0;
}

.filter-search {
  min-width: 300px;
  max-width: 500px;
}

.filter-select {
  min-width: 140px;
}

.filter-actions {
  display: flex;
  gap: var(--space-sm);
}

/* 内容网格布局优化 */
.content-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: var(--space-2xl);
  padding: var(--space-xl) 0;
}

@media (max-width: 1200px) {
  .content-grid {
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: var(--space-xl);
  }
}

@media (max-width: 768px) {
  .content-grid {
    grid-template-columns: 1fr;
    gap: var(--space-lg);
  }
  
  .filter-toolbar {
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-lg);
  }
  
  .filter-left,
  .filter-right {
    justify-content: stretch;
  }
  
  .filter-search {
    min-width: 100%;
  }
}

/* 资源卡片样式优化 */
.resource-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color-light);
  border-radius: var(--radius-xl);
  overflow: hidden;
  transition: var(--transition-normal);
  box-shadow: var(--shadow-sm);
  display: flex;
  flex-direction: column;
  height: 100%;
}

.resource-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
  border-color: var(--border-hover);
}

/* 侧边栏滚动条优化 */
.left-column::-webkit-scrollbar,
.right-column::-webkit-scrollbar {
  width: 6px;
}

.left-column::-webkit-scrollbar-track,
.right-column::-webkit-scrollbar-track {
  background: var(--bg-secondary);
  border-radius: var(--radius-full);
}

.left-column::-webkit-scrollbar-thumb,
.right-column::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: var(--radius-full);
  transition: var(--transition-fast);
}

.left-column::-webkit-scrollbar-thumb:hover,
.right-column::-webkit-scrollbar-thumb:hover {
  background: var(--brand-color);
}

/* 减少动画模式支持 */
@media (prefers-reduced-motion: reduce) {
  .nav-panel,
  .announcement-panel,
  .stats-panel,
  .activity-panel,
  .tags-panel,
  .content-panel,
  .nav-item,
  .resource-card {
    transition: none !important;
  }
  
  .nav-panel:hover,
  .announcement-panel:hover,
  .stats-panel:hover,
  .activity-panel:hover,
  .tags-panel:hover,
  .content-panel:hover,
  .nav-item:hover,
  .resource-card:hover {
    transform: none !important;
  }
}

/* 高对比度模式支持 */
@media (prefers-contrast: high) {
  .nav-panel,
  .announcement-panel,
  .stats-panel,
  .activity-panel,
  .tags-panel,
  .content-panel {
    border-width: 2px;
    border-color: var(--brand-color);
  }
  
  .nav-item,
  .resource-card {
    border: 1px solid var(--border-color-dark);
  }
  
  .nav-item.active {
    border-color: var(--brand-color);
    border-width: 2px;
  }
}

/* ===== 内容面板样式重新设计 ===== */
.content-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-lg) var(--space-xl);
  background: var(--bg-elevated);
  border-radius: var(--radius-lg) var(--radius-lg) 0 0;
  border: 1px solid var(--border-color-light);
  border-bottom: none;
  position: relative;
  min-height: 56px;
  box-sizing: border-box;
  box-shadow: var(--shadow-sm);
}



.header-left {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.header-left h2 {
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0;
  line-height: var(--line-height-tight);
}

.content-count {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  position: relative;
  z-index: 1;
}

.content-controls {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  background: var(--bg-secondary);
  padding: var(--space-sm) var(--space-md);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color-light);
}

.content-controls :deep(.el-select) {
  min-width: 110px;
}

.content-controls :deep(.el-select .el-input) {
  border: none !important;
  background: transparent !important;
  box-shadow: none !important;
}

.content-controls :deep(.el-select .el-input__wrapper) {
  background: transparent !important;
  border: none !important;
  box-shadow: none !important;
  padding: var(--space-xs) var(--space-sm) !important;
  border-radius: var(--radius-sm) !important;
  min-height: 32px !important;
}

.content-controls :deep(.el-select .el-input__wrapper:hover) {
  background: var(--bg-hover) !important;
}

.content-controls :deep(.el-select .el-input__wrapper.is-focus) {
  background: var(--bg-hover) !important;
  border-color: var(--brand-color) !important;
}

.content-controls :deep(.el-select .el-input__inner) {
  color: var(--text-primary) !important;
  font-weight: var(--font-weight-medium) !important;
  font-size: var(--font-size-sm) !important;
}

.content-controls :deep(.el-select:focus-within) {
  outline: none !important;
}

.content-controls :deep(.el-select-dropdown__item:hover),
.content-controls :deep(.el-select-dropdown__item:focus) {
  outline: none !important;
  background: var(--bg-hover) !important;
  color: var(--text-primary) !important;
}



/* 发布按钮样式 */
.content-controls :deep(.el-button.el-button--primary) {
  background: var(--brand-color) !important;
  border: none !important;
  color: white !important;
  padding: var(--space-xs) var(--space-md) !important;
  border-radius: var(--radius-sm) !important;
  font-weight: var(--font-weight-medium) !important;
  font-size: var(--font-size-sm) !important;
  min-height: 32px !important;
}

.content-controls :deep(.el-button.el-button--primary:hover) {
  background: var(--brand-color-dark) !important;
}

.content-controls :deep(.el-button.el-button--primary .el-icon) {
  margin-right: var(--space-xs) !important;
  font-size: var(--font-size-sm) !important;
}

/* 响应式优化 */
@media (max-width: 1024px) {
  .content-header {
    padding: var(--space-md) var(--space-lg);
    min-height: 48px;
  }
  
  .content-controls {
    gap: var(--space-sm);
    padding: var(--space-xs) var(--space-sm);
  }
  
  .content-controls :deep(.el-select) {
    min-width: 100px;
  }
}

@media (max-width: 768px) {
  .content-header {
    flex-direction: column;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    min-height: auto;
    border-radius: var(--radius-md) var(--radius-md) 0 0;
  }
  
  .header-left {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-sm);
    width: 100%;
  }
  
  .header-left h2 {
    font-size: var(--font-size-lg);
  }
  
  .header-right {
    width: 100%;
    justify-content: center;
  }
  
  .content-controls {
    flex-wrap: wrap;
    justify-content: center;
    gap: var(--space-xs);
    padding: var(--space-xs);
  }
  
  .content-controls :deep(.el-select) {
    min-width: 90px;
  }
}

@media (max-width: 480px) {
  .content-header {
    padding: var(--space-sm) var(--space-md);
  }
  
  .content-controls {
    flex-direction: column;
    gap: var(--space-xs);
    align-items: stretch;
  }
  
  .content-controls :deep(.el-select) {
    min-width: auto;
    width: 100%;
  }
  

}

/* 移除所有黑色边框和outline */
* {
  outline: none !important;
}

button,
.el-button,
input,
.el-input,
select,
.el-select,
textarea,
.el-textarea,
a,
[tabindex] {
  outline: none !important;
  border-color: transparent !important;
}

button:focus,
.el-button:focus,
.el-button:active,
input:focus,
.el-input:focus,
.el-input__wrapper:focus,
select:focus,
.el-select:focus,
textarea:focus,
.el-textarea:focus,
a:focus,
[tabindex]:focus {
  outline: none !important;
  border-color: transparent !important;
  box-shadow: none !important;
}

/* Element Plus 组件黑边框移除 */
:deep(.el-button) {
  outline: none !important;
  border: 1px solid transparent !important;
}

:deep(.el-button:focus),
:deep(.el-button:active) {
  outline: none !important;
  border-color: transparent !important;
  box-shadow: none !important;
}

:deep(.el-input__wrapper) {
  outline: none !important;
  border: 1px solid var(--border-color-light) !important;
}

:deep(.el-input__wrapper:focus),
:deep(.el-input__wrapper.is-focus) {
  outline: none !important;
  border-color: var(--brand-color) !important;
  box-shadow: 0 0 0 2px rgba(var(--brand-color-rgb), 0.2) !important;
}

:deep(.el-select .el-input__wrapper:focus),
:deep(.el-select .el-input__wrapper.is-focus) {
  outline: none !important;
  border-color: transparent !important;
  box-shadow: none !important;
}

:deep(.el-dropdown-menu__item:focus) {
  outline: none !important;
  background: var(--bg-hover) !important;
}

:deep(.el-carousel__arrow:focus) {
  outline: none !important;
}

:deep(.el-pagination button:focus) {
  outline: none !important;
}

:deep(.el-avatar:focus) {
  outline: none !important;
}

/* 导航项和资源卡片focus状态优化 */
.nav-item:focus {
  outline: none !important;
  background: var(--bg-hover) !important;
  transform: translateX(6px) scale(1.02) !important;
  border-color: var(--brand-color) !important;
}

.resource-card:focus {
  outline: none !important;
  transform: translateY(-6px) !important;
  box-shadow: var(--shadow-2xl) !important;
  border-color: var(--brand-color) !important;
}

.post-card:focus {
  outline: none !important;
  transform: translateY(-4px) !important;
  box-shadow: var(--shadow-xl) !important;
  border-color: var(--brand-color) !important;
}

.popular-item:focus,
.activity-item:focus,
.announcement-item:focus {
  outline: none !important;
  background: var(--bg-hover) !important;
  transform: translateY(-2px) !important;
}

/* 轮播图focus状态 */
:deep(.el-carousel__item:focus) {
  outline: none !important;
}

.carousel-slide:focus {
  outline: none !important;
}

/* 分页器focus状态 */
:deep(.el-pagination .el-pager li:focus) {
  outline: none !important;
  background: var(--bg-hover) !important;
  color: var(--brand-color) !important;
}

/* 标签和其他元素focus状态 */
:deep(.el-tag:focus) {
  outline: none !important;
  transform: scale(1.05) !important;
  box-shadow: var(--shadow-sm) !important;
}

.panel-header:focus {
  outline: none !important;
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%) !important;
}

.panel-header:focus::before {
  opacity: 1 !important;
}

/* 搜索相关focus状态 */
:deep(.el-input:focus) {
  outline: none !important;
}

:deep(.el-input:focus .el-input__wrapper) {
  border-color: var(--brand-color) !important;
  box-shadow: 0 0 0 2px rgba(var(--brand-color-rgb), 0.2) !important;
}

.search-suggestions .suggestion-item:focus {
  outline: none !important;
  background: var(--bg-hover) !important;
  transform: translateX(4px) !important;
}

/* 键盘导航可访问性 */
*:focus-visible {
  outline: 2px solid var(--brand-color) !important;
  outline-offset: 2px !important;
  border-radius: var(--radius-sm) !important;
}

/* 但对于我们自定义的交互元素，移除这个outline，使用自定义样式 */
.nav-item:focus-visible,
.resource-card:focus-visible,
.post-card:focus-visible,
.popular-item:focus-visible,
.activity-item:focus-visible,
.announcement-item:focus-visible,
.panel-header:focus-visible,
.carousel-slide:focus-visible,
:deep(.el-button:focus-visible),
:deep(.el-input:focus-visible),
:deep(.el-select:focus-visible),
:deep(.el-dropdown-menu__item:focus-visible),
:deep(.el-pagination button:focus-visible),
:deep(.el-tag:focus-visible),
:deep(.el-avatar:focus-visible),
:deep(.el-carousel__arrow:focus-visible) {
  outline: none !important;
}

/* 深色主题适配 */
:global(:root.dark) .content-header {
  background: linear-gradient(135deg, 
    var(--bg-secondary) 0%, 
    var(--bg-tertiary) 100%);
}

:global(:root.dark) .content-header::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%);
}

/* ===== 资源网格样式 ===== */
.resource-content {
  padding: var(--space-xl);
}

.resource-grid {
  display: grid;
  gap: var(--space-xl);
  padding: var(--space-xl);
  min-height: 400px;
}

.resource-grid.grid {
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: var(--space-xl);
  align-items: start;
}

.resource-grid.list {
  grid-template-columns: 1fr;
  gap: var(--space-xl);
}

.footer-actions .el-button--primary {
  background: var(--bg-elevated);
  border-color: var(--border-color);
  color: var(--text-muted);
  transition: var(--transition-normal);
}

.footer-actions .el-button--primary:hover {
  background: var(--bg-hover);
  border-color: var(--border-color);
  color: var(--text-primary);
}

.resource-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color-light);
  border-radius: var(--radius-xl);
  overflow: hidden;
  transition: var(--transition-normal);
  cursor: pointer;
  box-shadow: var(--shadow-md);
  height: fit-content;
  display: flex;
  flex-direction: column;
  position: relative;
}

.resource-card:hover {
  border-color: var(--brand-color);
  box-shadow: var(--shadow-xl);
  transform: translateY(-6px);
}

.card-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  padding: var(--space-lg) var(--space-lg) var(--space-lg) 24px;
  border-bottom: 1px solid var(--border-color-light);
  background: var(--bg-secondary);
  min-height: 80px;
  max-height: 80px;
}

.card-icon {
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color-light);
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--brand-color);
  transition: var(--transition-normal);
  font-size: var(--font-size-lg);
}

.card-title {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-1);
}

.status-badges {
  display: flex;
  gap: var(--space-2);
  flex-shrink: 0;
}

.status-badges .el-tag {
  font-size: var(--font-size-xs);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

.status-badges .el-icon {
  font-size: 12px;
  margin-right: 2px;
}

.card-title h4 {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--space-xs) 0;
  line-height: var(--line-height-tight);
  display: -webkit-box;
  -webkit-line-clamp: 1;
  line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.card-category {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  background: var(--bg-tertiary);
  padding: var(--space-xs) var(--space-sm);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color-light);
  font-weight: var(--font-weight-medium);
  white-space: nowrap;
  width: fit-content;
  max-width: 120px;
  flex-shrink: 0;
}

.card-actions {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-body {
  padding: var(--space-lg);
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.card-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: var(--line-height-relaxed);
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  flex: 1;
}

.card-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-sm);
  margin-top: auto;
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-lg) var(--space-xl);
  background: var(--bg-tertiary);
  border-top: 1px solid var(--border-color-light);
  margin-top: auto;
  position: relative;
}

.footer-meta {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
  font-weight: var(--font-weight-medium);
}

.footer-meta span {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-xs) var(--space-sm);
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color-light);
}

/* 热门内容项样式重新设计 */
.popular-item {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  padding: var(--space-lg) var(--space-xl);
  border-radius: var(--radius-xl);
  transition: var(--transition-fast);
  cursor: pointer;
  background: linear-gradient(135deg, 
    var(--bg-elevated) 0%, 
    var(--bg-secondary) 100%);
  border: 1px solid var(--border-color-light);
  position: relative;
  overflow: hidden;
  margin-bottom: var(--space-md);
  box-shadow: var(--shadow-sm);
}

.popular-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 100%);
  opacity: 0;
  transition: var(--transition-fast);
  pointer-events: none;
}

.popular-item:hover {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%);
  border-color: var(--brand-color);
  transform: translateX(6px) translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.popular-item:hover::before {
  opacity: 1;
}

.popular-item .item-icon {
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, 
    var(--brand-color) 0%, 
    var(--brand-color-light) 100%);
  border-radius: var(--radius-xl);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: var(--font-size-lg);
  box-shadow: var(--shadow-md);
  position: relative;
  z-index: 1;
}

.popular-item .item-content {
  flex: 1;
  min-width: 0;
  position: relative;
  z-index: 1;
}

.popular-item .item-title {
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--space-sm) 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: var(--line-height-tight);
  transition: var(--transition-fast);
}

.popular-item:hover .item-title {
  color: var(--brand-color);
}

.popular-item .item-stats {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.popular-item .stat-item {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  padding: var(--space-xs) var(--space-sm);
  background: var(--bg-tertiary);
  border-radius: var(--radius-full);
  border: 1px solid var(--border-color-light);
  font-weight: var(--font-weight-medium);
  color: var(--text-secondary);
  transition: var(--transition-fast);
}

.popular-item:hover .stat-item {
  background: var(--bg-elevated);
  border-color: var(--border-color-hover);
  transform: scale(1.05);
}

/* 热门内容列表容器优化 */
.popular-content-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  max-height: 400px; /* 新增 */
  overflow-y: auto; /* 新增 */
  padding-right: var(--space-sm); /* 为滚动条留出空间 */
}

.popular-content-list::before {
  content: '';
  position: absolute;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 80%;
  height: 1px;
  background: linear-gradient(90deg, 
    transparent 0%, 
    var(--border-color-light) 50%, 
    transparent 100%);
}

.popular-content-list::-webkit-scrollbar,
.activity-list::-webkit-scrollbar {
  width: 6px;
}

.popular-content-list::-webkit-scrollbar-track,
.activity-list::-webkit-scrollbar-track {
  background: var(--bg-secondary);
  border-radius: var(--radius-full);
}

.popular-content-list::-webkit-scrollbar-thumb,
.activity-list::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: var(--radius-full);
  transition: var(--transition-fast);
}

.popular-content-list::-webkit-scrollbar-thumb:hover,
.activity-list::-webkit-scrollbar-thumb:hover {
  background: var(--brand-color);
}

/* 深色主题适配 */
:global(:root.dark) .popular-item {
  background: linear-gradient(135deg, 
    var(--bg-elevated) 0%, 
    var(--bg-secondary) 100%);
}

:global(:root.dark) .popular-item::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%);
}

:global(:root.dark) .popular-item:hover {
  background: linear-gradient(135deg, 
    var(--bg-hover) 0%, 
    var(--bg-secondary) 100%);
}

/* 响应式优化 */
@media (max-width: 1024px) {
  .popular-item {
    padding: var(--space-md) var(--space-lg);
    gap: var(--space-md);
  }
  
  .rank-number {
    width: 24px;
    height: 24px;
    font-size: var(--font-size-xs);
  }
  
  .popular-item .item-icon {
    width: 32px;
    height: 32px;
  }
  
  .popular-item .item-title {
    font-size: var(--font-size-sm);
  }
}

@media (max-width: 768px) {
  .popular-item {
    padding: var(--space-sm) var(--space-md);
    margin-bottom: var(--space-sm);
  }
  
  .popular-item:hover {
    transform: translateX(4px) translateY(-1px);
  }
  
  .item-header {
    gap: var(--space-xs);
    margin-bottom: var(--space-xs);
  }
  
  .item-type {
    padding: var(--space-xs);
    font-size: var(--font-size-2xs);
  }
  
  .popular-item .stat-item {
    padding: var(--space-xs);
    font-size: var(--font-size-2xs);
  }
}

@media (max-width: 480px) {
  .popular-item {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-sm);
    padding: var(--space-md);
  }
  
  .item-rank {
    align-self: flex-end;
  }
  
  .item-content {
    width: 100%;
  }
  
  .item-header {
    justify-content: space-between;
    width: 100%;
  }
  
  .popular-item .item-stats {
    flex-wrap: wrap;
    gap: var(--space-xs);
  }
}

/* 社区动态项样式优化 */
.activity-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md); /* 调整卡片间距 */
  max-height: 400px;
  overflow-y: auto;
  padding: var(--space-sm) var(--space-lg) var(--space-sm) var(--space-sm); /* 调整内边距 */
}

.activity-item {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  padding: var(--space-lg);
  border-radius: var(--radius-xl);
  background: linear-gradient(135deg, var(--bg-elevated) 0%, var(--bg-secondary) 100%);
  border: 1px solid var(--border-color-light);
  transition: all 0.3s ease-in-out;
  cursor: pointer;
  box-shadow: var(--shadow-sm);
}

.activity-item:hover {
  transform: translateY(-2px) scale(1.02);
  box-shadow: var(--shadow-lg);
  border-color: var(--brand-color);
  background: linear-gradient(135deg, var(--bg-hover) 0%, var(--bg-secondary) 100%);
}

.activity-avatar .el-avatar {
  border: 2px solid var(--border-color);
  box-shadow: var(--shadow-md);
}

.activity-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
}

.activity-content.no-avatar {
  margin-left: 0;
}

.activity-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.activity-text strong {
  font-weight: 600;
  color: var(--text-primary);
}

.activity-target {
  font-weight: 500;
  color: var(--brand-color);
  margin: 0 2px;
}

.activity-time {
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

/* ===== 热门标签面板样式 ===== */
.tags-panel .panel-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
  max-height: 400px;
  overflow-y: auto;
  padding: var(--space-sm) var(--space-lg) var(--space-sm) var(--space-sm);
}

.tag-item {
  cursor: pointer;
  transition: var(--transition-normal);
  font-weight: 500;
  border: none !important;
  border-radius: var(--radius-full) !important;
  padding: 4px 12px !important;
  font-size: var(--font-size-xs) !important;
  margin: 2px !important;
  position: relative;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.tag-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(255,255,255,0.2) 0%, rgba(255,255,255,0) 100%);
  opacity: 0;
  transition: var(--transition-normal);
}

.tag-item:hover {
  transform: translateY(-2px) scale(1.05);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.tag-item:hover::before {
  opacity: 1;
}

.tag-count {
  margin-left: var(--space-1);
  opacity: 0.8;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.2);
  padding: 1px 6px;
  border-radius: var(--radius-full);
  font-size: 10px;
  display: inline-block;
  min-width: 16px;
  text-align: center;
  line-height: 1.2;
}

.loading-state {
  padding: var(--space-4);
}

.empty-state {
  text-align: center;
  padding: var(--space-6);
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
}

.refresh-btn {
  padding: var(--space-1) var(--space-2);
  opacity: 0.7;
  transition: var(--transition-fast);
}

.refresh-btn:hover {
  opacity: 1;
  color: var(--primary-color);
}

/* ===== 热门内容样式 ===== */
.popular-content-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.popular-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-md);
  padding: var(--space-md);
  border-radius: var(--radius-lg);
  background: var(--bg-elevated);
  border: 1px solid var(--border-color);
  cursor: pointer;
  transition: var(--transition-fast);
}

.popular-item:hover {
  background: var(--primary-light);
  border-color: var(--primary-color);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

.item-rank {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.item-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.item-header {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  margin-bottom: var(--space-2);
  position: relative;
  z-index: 1;
}

.type-icon {
  font-size: var(--font-size-base);
  padding: var(--space-xs);
  border-radius: var(--radius-md);
  transition: var(--transition-fast);
}

.type-icon.resource {
  color: var(--brand-color);
  background: rgba(var(--brand-color-rgb), 0.1);
}

.type-icon.post {
  color: var(--success-color);
  background: rgba(var(--success-color-rgb), 0.1);
}

.popular-item:hover .type-icon {
  transform: scale(1.1);
  box-shadow: var(--shadow-sm);
}

.item-type {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-weight: var(--font-weight-medium);
  padding: var(--space-xs) var(--space-sm);
  background: var(--bg-tertiary);
  border-radius: var(--radius-full);
  border: 1px solid var(--border-color-light);
  transition: var(--transition-fast);
}

.popular-item:hover .item-type {
  background: var(--bg-elevated);
  color: var(--text-primary);
  border-color: var(--border-color-hover);
}

.item-title {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
  line-height: 1.4;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
}

.item-stats {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  background: none;
  border: none;
  box-shadow: none;
  padding: 0;
}

.item-stats .stat-item {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  background: none;
  border: none;
  box-shadow: none;
  padding: 0;
  line-height: 1;
}

.item-stats .el-icon {
  font-size: 12px;
  vertical-align: middle;
}

/* 暗色主题适配 */
:global(:root.dark) .popular-item {
  background: rgba(31, 41, 55, 0.8);
  border-color: rgba(75, 85, 99, 0.3);
}

:global(:root.dark) .popular-item:hover {
  background: rgba(59, 130, 246, 0.1);
  border-color: var(--primary-color);
}

/* ===== 社区帖子区域样式 ===== */
.posts-section {
  margin-bottom: var(--space-16);
}

.section-header {
  text-align: center;
  margin-bottom: var(--space-12);
  padding: var(--space-12) var(--space-6);
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.8), 
    rgba(249, 250, 251, 0.9));
  backdrop-filter: blur(20px);
  border-radius: var(--radius-2xl);
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-lg);
}

:global(:root.dark) .section-header {
  background: linear-gradient(135deg, 
    rgba(31, 41, 55, 0.8), 
    rgba(17, 24, 39, 0.9));
}

.section-header h2 {
  font-size: var(--font-size-3xl);
  font-weight: 800;
  color: var(--text-primary);
  margin: 0 0 var(--space-4) 0;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.section-header p {
  font-size: var(--font-size-lg);
  color: var(--text-secondary);
  margin: 0 0 var(--space-8) 0;
}

.posts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: var(--space-6);
  margin-bottom: var(--space-12);
}

.post-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-xl);
  overflow: hidden;
  transition: var(--transition-normal);
  cursor: pointer;
  box-shadow: var(--shadow-sm);
}

.post-card:hover {
  border-color: var(--primary-light);
  box-shadow: var(--shadow-lg);
  transform: translateY(-2px);
}

.post-header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-5);
  border-bottom: 1px solid var(--border-color);
}

.post-avatar {
  flex-shrink: 0;
}

.post-author {
  flex: 1;
}

.author-name {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
}

.post-time {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

.post-category {
  flex-shrink: 0;
}

.post-content {
  padding: var(--space-5);
}

.post-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--space-3) 0;
  line-height: 1.4;
}

.post-excerpt {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.post-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-5);
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  margin-top: auto;
}

.post-meta {
  display: flex;
  gap: var(--space-4);
  flex-wrap: wrap;
}

.post-meta .meta-item {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  color: var(--el-text-color-secondary);
  font-size: var(--font-size-xs);
}

.post-meta .meta-item .el-icon {
  font-size: var(--font-size-base);
}

.post-date {
  color: var(--el-text-color-secondary);
  font-size: var(--font-size-xs);
}

.empty-posts {
  margin-top: var(--space-16);
  text-align: center;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .posts-section {
    padding: var(--space-16);
  }
  
  .posts-container {
    padding: 0 var(--space-8);
  }
  
  .posts-grid {
    grid-template-columns: 1fr;
    gap: var(--space-8);
  }
  
  .post-card {
    padding: var(--space-8);
  }
  
  .post-header {
    flex-direction: column;
    gap: var(--space-4);
  }
  
  .post-status {
    margin-left: 0;
  }
  
  .post-footer {
    flex-direction: column;
    align-items: flex-start;
  }
}

/* ===== 三栏布局核心样式 ===== */
.three-column-layout {
  display: grid;
  grid-template-columns: 280px 1fr 280px;
  grid-template-rows: 1fr;
  gap: var(--space-16);
  width: 100%;
  min-height: 600px;
  animation: fadeInUp 0.8s ease-out;
}

/* 左侧栏：公告区域 + 社区内容分类栏 */
.left-sidebar {
  grid-column: 1;
  grid-row: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-12);
  animation: slideInLeft 0.8s ease-out;
}

/* 中间主内容区域：社区轮播图区 + 内容展示区域 */
.main-content-area {
  grid-column: 2;
  grid-row: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-12);
  animation: fadeInUp 0.8s ease-out 0.2s both;
}

/* 右侧栏：社区通知 */
.right-sidebar {
  grid-column: 3;
  grid-row: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-12);
  animation: slideInRight 0.8s ease-out 0.4s both;
}

/* 三栏布局响应式设计 */
@media (max-width: 1200px) {
  .three-column-layout {
    grid-template-columns: 260px 1fr 260px;
    gap: var(--space-12);
  }
}

@media (max-width: 1024px) {
  .three-column-layout {
    grid-template-columns: 1fr;
    grid-template-rows: auto auto auto;
    gap: var(--space-12);
  }
  
  .left-sidebar {
    grid-column: 1;
    grid-row: 1;
  }
  
  .main-content-area {
    grid-column: 1;
    grid-row: 2;
  }
  
  .right-sidebar {
    grid-column: 1;
    grid-row: 3;
  }
}

@media (max-width: 768px) {
  .three-column-layout {
    gap: var(--space-12);
  }
  
  .left-sidebar,
  .main-content-area,
  .right-sidebar {
    gap: var(--space-12);
  }
}

/* 公告区域 */
.announcement-section {
  flex: 1;
  min-width: 0;
  min-height: 200px;
  position: relative;
}

/* 社区内容分类栏 */
.category-section {
  flex: 1;
  min-width: 0;
  min-height: 200px;
  position: relative;
}

/* 社区轮播图区 */
.carousel-section {
  flex: 1;
  min-width: 0;
  min-height: 300px;
  position: relative;
}

/* 内容图标 + 内容卡片区域 */
.content-display-section {
  flex: 1;
  min-width: 0;
  min-height: 500px;
  position: relative;
}

/* 筛选区域 */
.filter-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-16) 0;
  gap: var(--space-8);
}

/* 筛选区域左侧 */
.filter-left {
  flex: 1;
}

/* 筛选区域右侧 */
.filter-right {
  display: flex;
  gap: var(--space-8);
  align-items: center;
}

/* 筛选区域左侧输入框 */
.filter-search {
  flex: 1;
}

/* 筛选区域右侧选择框 */
.filter-select {
  width: 140px;
}

/* 筛选区域按钮 */
.filter-actions {
  display: flex;
  gap: var(--space-8);
}

/* 高级筛选面板 */
.advanced-filters {
  margin-bottom: var(--space-16);
}

/* 轮播为空时的状态 */
.carousel-empty {
  text-align: center;
  padding: var(--space-12) var(--space-6);
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(10px);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-xl);
  margin-top: var(--space-16);
}

.carousel-empty .empty-icon {
  font-size: 48px;
  margin-bottom: var(--space-8);
}

.carousel-empty h4 {
  margin: 0 0 var(--space-4) 0;
  font-size: var(--font-size-xl);
  color: var(--text-primary);
}

.carousel-empty p {
  margin: 0 0 var(--space-8) 0;
  color: var(--text-secondary);
}

/* 内容卡片 */
.section-card {
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(20px);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-xl);
  padding: var(--space-16);
  box-shadow: var(--shadow-lg);
  transform: translateZ(0);
}

.section-card:hover {
  transform: translateY(-4px) scale(1.02);
  box-shadow: var(--shadow-xl);
  border-color: var(--primary-light);
  background: var(--bg-hover);
}

/* 内容卡片头部 */
.content-card .card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-4) var(--space-6);
  background: rgba(255, 255, 255, 0.5);
  border-bottom: 1px solid var(--border-color);
}

.card-title-area {
  flex: 1;
}

.card-title {
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
}

.card-subtitle {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}

.card-content {
  padding: var(--space-6);
}

/* 内容卡片内容 */
.card-content .notice-list,
.card-content .category-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.card-content .notice-item,
.card-content .category-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-4);
  border-radius: var(--radius-lg);
  font-size: var(--font-size-sm);
  font-weight: 500;
  justify-content: space-between;
  transition: var(--transition-fast);
  cursor: pointer;
}

.card-content .notice-item:hover,
.card-content .category-item:hover {
  background: var(--bg-hover);
  transform: translateX(4px);
}

.card-content .notice-item.active,
.card-content .category-item.active {
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  color: white;
  box-shadow: var(--shadow-md);
}

.card-content .notice-item-content,
.card-content .category-item-content {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex: 1;
}

.card-content .notice-item-content .notice-dot,
.card-content .category-item-content .category-icon {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  backdrop-filter: blur(5px);
  border-radius: 50%;
}

.card-content .notice-item-content .notice-text,
.card-content .category-item-content .category-name {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.card-content .notice-item-content .notice-time,
.card-content .category-item-content .category-count {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
  background: rgba(255, 255, 255, 0.1);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  min-width: 24px;
  text-align: center;
}

/* 通知区域样式 */
.notification-section {
  margin-bottom: var(--space-16);
}

.notification-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-8);
}

.notification-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  padding: var(--space-4);
  background: rgba(255, 255, 255, 0.05);
  border-radius: var(--radius-lg);
  border-left: 4px solid transparent;
  transition: var(--transition-fast);
}

.notification-item:hover {
  background: var(--bg-hover);
  transform: translateX(4px);
}

.notification-item.system {
  border-left-color: #409eff;
}

.notification-item.community {
  border-left-color: #67c23a;
}

.notification-item.recommendation {
  border-left-color: #e6a23c;
}

.notification-item.interaction {
  border-left-color: #f56c6c;
}

.notification-icon {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.notification-item.system .notification-icon {
  background: rgba(64, 158, 255, 0.2);
  color: #409eff;
}

.notification-item.community .notification-icon {
  background: rgba(103, 194, 58, 0.2);
  color: #67c23a;
}

.notification-item.recommendation .notification-icon {
  background: rgba(230, 162, 60, 0.2);
  color: #e6a23c;
}

.notification-item.interaction .notification-icon {
  background: rgba(245, 108, 108, 0.2);
  color: #f56c6c;
}

.notification-content {
  flex: 1;
}

.notification-title {
  margin: 0 0 var(--space-2) 0;
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
}

.notification-text {
  margin: 0 0 var(--space-4) 0;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.4;
}

.notification-time {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

/* 快速统计样式 */
.quick-stats {
  margin-top: var(--space-16);
  padding-top: var(--space-12);
  border-top: 1px solid var(--border-color);
}

.stats-title {
  margin: 0 0 var(--space-8) 0;
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--space-8);
}

.stat-item {
  text-align: center;
  padding: var(--space-4);
  background: rgba(255, 255, 255, 0.05);
  border-radius: var(--radius-sm);
  transition: var(--transition-fast);
}

.stat-item:hover {
  background: var(--bg-hover);
  transform: translateY(-2px);
}

.stat-number {
  display: block;
  font-size: var(--font-size-lg);
  font-weight: 700;
  color: var(--brand-color);
  margin-bottom: var(--space-2);
}

.stat-label {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

/* 浅色主题下的卡片样式 */
:global(:root:not(.dark)) .section-card {
  background: rgba(248, 250, 252, 0.5) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(148, 163, 184, 0.2) !important;
  box-shadow: 0 8px 32px rgba(71, 85, 105, 0.08) !important;
}

:global(:root:not(.dark)) .section-card:hover {
  box-shadow: 0 16px 48px rgba(71, 85, 105, 0.12) !important;
  border-color: rgba(148, 163, 184, 0.3) !important;
  background: rgba(248, 250, 252, 0.7) !important;
}

/* 内容头部样式 */
.content-header-left {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.content-icon {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-sm);
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.content-header-right {
  display: flex;
  align-items: center;
  gap: var(--space-8);
}

.view-switcher {
  display: flex;
  gap: var(--space-4);
}

/* 资源卡片新样式 */
.resource-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-8);
}

.resource-body {
  margin-bottom: var(--space-8);
}

.resource-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: var(--space-4);
  border-top: 1px solid var(--border-color);
}

.resource-date {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

/* 轮播样式优化 */
.community-carousel {
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.carousel-slide {
  padding: var(--space-12);
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 动画关键帧 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideInLeft {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes slideInRight {
  from {
    opacity: 0;
    transform: translateX(20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.three-column-section {
  animation: fadeInUp 0.8s ease-out;
}

.left-column {
  animation: slideInLeft 0.8s ease-out 0.1s both;
}

.center-column {
  animation: fadeInUp 0.8s ease-out 0.2s both;
}

.right-column {
  animation: slideInRight 0.8s ease-out 0.3s both;
}

.hero-section {
  animation: fadeInUp 0.8s ease-out;
}

.posts-section {
  animation: fadeInUp 0.8s ease-out 0.4s both;
}

/* ===== Element Plus 组件定制 ===== */
:deep(.el-button) {
  border-radius: var(--radius-lg);
  font-weight: 500;
  transition: var(--transition-fast);
}

:deep(.el-button--primary) {
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  border: none;
  box-shadow: var(--shadow-sm);
}

:deep(.el-button--primary:hover) {
  background: linear-gradient(135deg, var(--primary-dark), var(--primary-color));
  box-shadow: var(--shadow-md);
  transform: translateY(-1px);
}

:deep(.el-tag) {
  border-radius: var(--radius-sm);
  border: none;
  font-weight: 500;
}

:deep(.el-empty) {
  padding: var(--space-12) var(--space-6);
}

:deep(.el-skeleton__item) {
  border-radius: var(--radius-lg);
}

:deep(.el-carousel__indicator) {
  border-radius: var(--radius-sm);
}

:deep(.el-carousel__indicator.is-active) {
  background-color: var(--primary-color);
}

:deep(.el-dropdown-menu) {
  border-radius: var(--radius-lg);
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-xl);
}

:deep(.el-dropdown-menu__item) {
  padding: var(--space-3) var(--space-4);
  font-size: var(--font-size-sm);
}

/* ===== 分页样式优化 ===== */
.pagination-wrapper {
  display: flex;
  justify-content: center;
  padding: var(--space-4) var(--space-2);
  margin-top: var(--space-4);
}

:deep(.el-pagination) {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-wrap: wrap;
  justify-content: center;
}

:deep(.el-pagination .btn-next),
:deep(.el-pagination .btn-prev) {
  border-radius: var(--radius-lg);
  border: none;
  background: transparent;
  min-height: 36px;
  min-width: 36px;
  transition: var(--transition-fast);
}

:deep(.el-pagination .btn-next:hover),
:deep(.el-pagination .btn-prev:hover) {
  background: var(--primary-light);
  color: var(--primary-color);
}

:deep(.el-pagination .el-pager li) {
  border-radius: var(--radius-lg);
  border: none;
  background: transparent;
  min-height: 36px;
  min-width: 36px;
  margin: 0 var(--space-1);
  transition: var(--transition-fast);
  font-weight: 500;
}

:deep(.el-pagination .el-pager li:hover) {
  background: var(--primary-light);
  color: var(--primary-color);
}

:deep(.el-pagination .el-pager li.is-active) {
  background: var(--primary-color);
  color: white;
  font-weight: 600;
}

:deep(.el-pagination .el-pagination__total) {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: 500;
  margin-right: var(--space-4);
}

:deep(.el-pagination .el-pagination__sizes) {
  margin-right: var(--space-4);
}

:deep(.el-pagination .el-pagination__sizes .el-select) {
  border-radius: var(--radius-lg);
}

:deep(.el-pagination .el-pagination__jump) {
  margin-left: var(--space-4);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

:deep(.el-pagination .el-pagination__jump .el-input) {
  border-radius: var(--radius-lg);
  width: 60px;
}

/* 暗色主题下的分页样式 */
:global(:root.dark) .pagination-wrapper {
  background: rgba(31, 41, 55, 0.7);
  border-color: rgba(75, 85, 99, 0.3);
}

:global(:root.dark) :deep(.el-pagination .btn-next),
:global(:root.dark) :deep(.el-pagination .btn-prev),
:global(:root.dark) :deep(.el-pagination .el-pager li) {
  background: rgba(31, 41, 55, 0.8);
  border-color: rgba(75, 85, 99, 0.3);
  color: var(--text-primary);
}

/* ===== 微调和优化 ===== */
.resource-card.list {
  display: flex;
  align-items: center;
  padding: var(--space-4);
}

.resource-card.list .card-header {
  border-bottom: none;
  padding: 0 0 0 20px;
  margin-right: var(--space-4);
}

.resource-card.list .card-body {
  flex: 1;
  padding: 0;
  margin-right: var(--space-4);
}

.resource-card.list .card-footer {
  border-top: none;
  background: transparent;
  padding: 0;
  flex-shrink: 0;
}

/* 加载状态优化 */
.loading-state :deep(.el-skeleton__item) {
  background: linear-gradient(90deg, var(--gray-200) 25%, var(--gray-100) 50%, var(--gray-200) 75%);
  background-size: 200% 100%;
  animation: loading 1.5s infinite;
}

@keyframes loading {
  0% { background-position: 200% 0; }
  100% { background-position: -200% 0; }
}

:global(:root.dark) .loading-state :deep(.el-skeleton__item) {
  background: linear-gradient(90deg, var(--gray-700) 25%, var(--gray-600) 50%, var(--gray-700) 75%);
  background-size: 200% 100%;
}

/* ===== 现代化轮播图样式全面重设计 ===== */
.carousel-section {
  margin-bottom: var(--space-3xl);
  position: relative;
  padding: 0 var(--space-2xl);
}

.carousel-section::before {
  content: '';
  position: absolute;
  top: -80px;
  left: 50%;
  transform: translateX(-50%);
  width: 200%;
  height: 160px;
  background: radial-gradient(ellipse, 
    rgba(var(--brand-color-rgb), 0.1) 0%, 
    transparent 70%);
  z-index: 0;
}

/* 轮播图响应式间距优化 */
@media (max-width: 1024px) {
  .carousel-section {
    padding: 0 var(--space-lg);
  }
}

@media (max-width: 768px) {
  .carousel-section {
    padding: 0 var(--space-lg);
    margin-bottom: var(--space-2xl);
  }
}

@media (max-width: 480px) {
  .carousel-section {
    padding: 0 var(--space-md);
    margin-bottom: var(--space-xl);
  }
}

.carousel-container {
  border-radius: var(--radius-3xl);
  overflow: hidden;
  box-shadow: var(--shadow-2xl);
  border: 1px solid var(--border-color-light);
  background: linear-gradient(135deg, 
    var(--bg-elevated) 0%, 
    var(--bg-secondary) 100%);
  position: relative;
  z-index: 1;
}

.carousel-container::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    rgba(255, 255, 255, 0.05) 50%,
    rgba(255, 255, 255, 0.02) 100%);
  z-index: 1;
  pointer-events: none;
}

.hero-carousel :deep(.el-carousel__container) {
  height: 380px;
}

.hero-carousel :deep(.el-carousel__indicators) {
  bottom: 24px;
  z-index: 10;
}

.hero-carousel :deep(.el-carousel__indicator) {
  width: 16px;
  height: 8px;
  border-radius: var(--radius-full);
  background: rgba(255, 255, 255, 0.4);
  transition: var(--transition-fast);
  border: 1px solid rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(10px);
}

.hero-carousel :deep(.el-carousel__indicator.is-active) {
  background: linear-gradient(135deg, 
    var(--brand-color) 0%, 
    var(--brand-color-light) 100%);
  width: 32px;
  border-radius: var(--radius-full);
  box-shadow: var(--shadow-md);
  border-color: var(--brand-color);
}

.hero-carousel :deep(.el-carousel__arrow) {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.9) 0%, 
    rgba(255, 255, 255, 0.8) 100%);
  backdrop-filter: blur(15px);
  border: 1px solid rgba(255, 255, 255, 0.4);
  color: var(--text-primary);
  width: 48px;
  height: 48px;
  border-radius: var(--radius-full);
  transition: var(--transition-fast);
  box-shadow: var(--shadow-md);
  z-index: 10;
}

.hero-carousel :deep(.el-carousel__arrow:hover) {
  background: linear-gradient(135deg, 
    white 0%, 
    rgba(255, 255, 255, 0.95) 100%);
  box-shadow: var(--shadow-xl);
  transform: scale(1.1);
  border-color: var(--border-color-hover);
}

.carousel-slide {
  position: relative;
  width: 100%;
  height: 100%;
  cursor: pointer;
  overflow: hidden;
  z-index: 2;
}

.carousel-slide::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(0, 0, 0, 0.1) 0%, 
    transparent 50%,
    rgba(0, 0, 0, 0.05) 100%);
  z-index: 3;
  pointer-events: none;
  transition: var(--transition-fast);
}

.carousel-slide:hover::after {
  background: linear-gradient(135deg, 
    rgba(0, 0, 0, 0.15) 0%, 
    transparent 50%,
    rgba(0, 0, 0, 0.1) 100%);
}

.slide-background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1;
}

.background-image {
  width: 100%;
  height: 100%;
  object-fit: fill;
  transition: transform var(--transition-slow);
}

.carousel-slide:hover .background-image {
  transform: scale(1.05);
}

.background-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.3);
  z-index: 2;
}

.slide-content {
  position: relative;
  z-index: 3;
  display: flex;
  align-items: center;
  padding: var(--space-12) var(--space-16);
  height: 100%;
  gap: var(--space-8);
}

.content-left {
  flex-shrink: 0;
}

.app-icon {
  width: 80px;
  height: 80px;
  border-radius: var(--radius-xl);
  background: rgba(255, 255, 255, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: var(--shadow-lg);
  backdrop-filter: blur(10px);
  border: 2px solid rgba(255, 255, 255, 0.2);
}

.app-icon img {
  width: 60px;
  height: 60px;
  border-radius: var(--radius-lg);
  object-fit: cover;
}

.app-icon.default-icon {
  background: var(--brand-color, #409EFF);
  color: white;
  font-size: 32px;
}

.content-center {
  flex: 1;
  color: white;
  text-align: left;
}

.slide-title {
  font-size: var(--font-size-4xl);
  font-weight: 800;
  margin: 0 0 var(--space-4) 0;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
  color: #fff;
}

.slide-subtitle {
  font-size: var(--font-size-lg);
  margin: 0 0 var(--space-6) 0;
  opacity: 0.95;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
  line-height: 1.5;
}

.slide-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.feature-tag {
  background: rgba(255, 255, 255, 0.9) !important;
  color: var(--text-primary) !important;
  border: none !important;
  font-weight: 600;
  padding: var(--space-1) var(--space-3);
  border-radius: var(--radius-full);
  backdrop-filter: blur(5px);
}

.content-right {
  flex-shrink: 0;
  max-width: 300px;
}

.preview-container {
  border-radius: var(--radius-xl);
  overflow: hidden;
  box-shadow: var(--shadow-xl);
  border: 3px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
}

.preview-image {
  width: 100%;
  height: 200px;
  object-fit: cover;
  transition: transform var(--transition-normal);
}

.carousel-slide:hover .preview-image {
  transform: scale(1.02);
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .slide-content {
    padding: var(--space-8) var(--space-12);
    gap: var(--space-6);
  }
  
  .content-right {
    max-width: 200px;
  }
  
  .preview-image {
    height: 150px;
  }
  
  .slide-title {
    font-size: var(--font-size-3xl);
  }
  
  .app-icon {
    width: 60px;
    height: 60px;
  }
  
  .app-icon img {
    width: 40px;
    height: 40px;
  }
}

@media (max-width: 768px) {
  .carousel-container {
    border-radius: var(--radius-xl);
  }
  
  .slide-content {
    flex-direction: column;
    text-align: center;
    padding: var(--space-6) var(--space-8);
    gap: var(--space-4);
  }
  
  .content-right {
    max-width: 100%;
  }
  
  .preview-container {
    max-width: 250px;
    margin: 0 auto;
  }
  
  .slide-title {
    font-size: var(--font-size-2xl);
  }
  
  .slide-subtitle {
    font-size: var(--font-size-base);
  }
}

/* ===== 页脚样式 ===== */
.site-footer {
  background: linear-gradient(135deg, 
    var(--bg-elevated) 0%, 
    var(--bg-secondary) 100%);
  backdrop-filter: blur(20px);
  border-top: 1px solid var(--border-color);
  margin-top: var(--space-24);
  padding: var(--space-20) 0 var(--space-12) 0;
  position: relative;
  overflow: hidden;
  transition: var(--transition-normal);
}

.site-footer::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 20% 80%, var(--brand-color) 0.1, transparent 50%),
    radial-gradient(circle at 80% 20%, var(--brand-color-light) 0.08, transparent 50%);
  pointer-events: none;
  z-index: -1;
  transition: var(--transition-normal);
}

/* ===== 深色模式页脚适配 ===== */
html.dark .site-footer {
  background: linear-gradient(135deg, 
    var(--bg-elevated) 0%, 
    var(--bg-secondary) 100%);
}

html.dark .site-footer::before {
  background: 
    radial-gradient(circle at 20% 80%, var(--brand-color) 0.08, transparent 50%),
    radial-gradient(circle at 80% 20%, var(--brand-color-light) 0.05, transparent 50%);
}

/* ===== 多彩主题页脚适配 ===== */
html.blue .site-footer::before {
  background: 
    radial-gradient(circle at 20% 80%, #1890FF 0.1, transparent 50%),
    radial-gradient(circle at 80% 20%, #40A9FF 0.08, transparent 50%);
}

html.green .site-footer::before {
  background: 
    radial-gradient(circle at 20% 80%, #52C41A 0.1, transparent 50%),
    radial-gradient(circle at 80% 20%, #73D13D 0.08, transparent 50%);
}

html.purple .site-footer::before {
  background: 
    radial-gradient(circle at 20% 80%, #722ED1 0.1, transparent 50%),
    radial-gradient(circle at 80% 20%, #9254DE 0.08, transparent 50%);
}

html.orange .site-footer::before {
  background: 
    radial-gradient(circle at 20% 80%, #FA8C16 0.1, transparent 50%),
    radial-gradient(circle at 80% 20%, #FFA940 0.08, transparent 50%);
}

html.red .site-footer::before {
  background: 
    radial-gradient(circle at 20% 80%, #F5222D 0.1, transparent 50%),
    radial-gradient(circle at 80% 20%, #FF4D4F 0.08, transparent 50%);
}

.footer-content {
  max-width: 1440px;
  margin: 0 auto;
  padding: 0 var(--space-6);
}

.footer-info {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: var(--space-16);
  margin-bottom: var(--space-16);
}

.footer-brand {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
}

.footer-brand .brand-icon {
  font-size: var(--font-size-3xl);
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light));
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  flex-shrink: 0;
  transition: var(--transition-normal);
}

.footer-brand h3 {
  font-size: var(--font-size-2xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 var(--space-2) 0;
  line-height: 1.2;
}

.footer-brand p {
  font-size: var(--font-size-base);
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

.footer-links {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-12);
}

.link-group h4 {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--space-6) 0;
  position: relative;
  padding-bottom: var(--space-2);
}

.link-group h4::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  width: 30px;
  height: 2px;
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light));
  border-radius: var(--radius-sm);
  transition: var(--transition-normal);
}

.link-group a {
  display: block;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  text-decoration: none;
  margin-bottom: var(--space-3);
  padding: var(--space-2) 0;
  border-radius: var(--radius-sm);
  transition: var(--transition-fast);
  position: relative;
}

.link-group a:hover {
  color: var(--brand-color);
  padding-left: var(--space-3);
  background: var(--bg-hover);
}

.link-group a:hover::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 2px;
  height: 60%;
  background: var(--brand-color);
  border-radius: var(--radius-sm);
}

.footer-bottom {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-top: var(--space-8);
  border-top: 1px solid var(--border-color);
  flex-wrap: wrap;
  gap: var(--space-4);
}

.footer-bottom p {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  margin: 0;
}

.footer-social {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.footer-social .el-button {
  width: 40px;
  height: 40px;
  border-radius: var(--radius-lg);
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  transition: var(--transition-fast);
}

.footer-social .el-button:hover {
  background: var(--brand-color);
  border-color: var(--brand-color);
  color: var(--text-inverse);
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

/* 轮播图响应式设计 */
@media (max-width: 1024px) {
  .hero-content {
    grid-template-columns: 1fr;
    gap: var(--space-6);
    padding: var(--space-6);
    text-align: center;
  }
  
  .hero-text {
    order: 2;
  }
  
  .hero-carousel {
    order: 1;
  }
  
  .featured-carousel {
    height: 350px;
  }
  
  .card-image img {
    height: 150px;
  }
}

@media (max-width: 768px) {
  .hero-content {
    padding: var(--space-4);
  }
  
  .featured-carousel {
    height: 300px;
  }
  
  .card-content {
    padding: var(--space-4);
  }
  
  .card-main {
    min-height: 200px;
  }
  
  .card-image img {
    height: 120px;
  }
  
  .card-title {
    font-size: var(--font-size-lg);
  }
  
  .card-description {
    font-size: var(--font-size-xs);
  }
}

/* 页脚响应式设计 */
@media (max-width: 1024px) {
  .footer-info {
    grid-template-columns: 1fr;
    gap: var(--space-12);
  }
  
  .footer-links {
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-8);
  }
}

@media (max-width: 768px) {
  .site-footer {
    padding: var(--space-16) 0 var(--space-8) 0;
  }
  
  .footer-links {
    grid-template-columns: 1fr;
    gap: var(--space-6);
  }
  
  .footer-brand {
    flex-direction: column;
    text-align: center;
    gap: var(--space-3);
  }
  
  .footer-bottom {
    flex-direction: column;
    text-align: center;
    gap: var(--space-6);
  }
}

/* ===== 页脚主题适配增强 ===== */
html.dark .footer-brand .brand-icon {
  box-shadow: var(--shadow-xl);
}

html.blue .footer-brand .brand-icon {
  background: linear-gradient(135deg, #1890FF, #40A9FF);
}

html.green .footer-brand .brand-icon {
  background: linear-gradient(135deg, #52C41A, #73D13D);
}

html.purple .footer-brand .brand-icon {
  background: linear-gradient(135deg, #722ED1, #9254DE);
}

html.orange .footer-brand .brand-icon {
  background: linear-gradient(135deg, #FA8C16, #FFA940);
}

html.red .footer-brand .brand-icon {
  background: linear-gradient(135deg, #F5222D, #FF4D4F);
}

/* 为每个主题的链接标题下划线定制颜色 */
html.blue .link-group h4::after {
  background: linear-gradient(135deg, #1890FF, #40A9FF);
}

html.green .link-group h4::after {
  background: linear-gradient(135deg, #52C41A, #73D13D);
}

html.purple .link-group h4::after {
  background: linear-gradient(135deg, #722ED1, #9254DE);
}

html.orange .link-group h4::after {
  background: linear-gradient(135deg, #FA8C16, #FFA940);
}

html.red .link-group h4::after {
  background: linear-gradient(135deg, #F5222D, #FF4D4F);
}

/* 帖子网格视图兼容 */
.posts-grid {
  display: grid;
  gap: var(--space-8);
  padding: var(--space-4);
}

.posts-grid.grid {
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: var(--space-6);
}

.posts-grid.list {
  grid-template-columns: 1fr;
  gap: var(--space-5);
}

.posts-grid .post-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-xl);
  overflow: hidden;
  transition: var(--transition-normal);
  cursor: pointer;
  box-shadow: var(--shadow-sm);
  height: fit-content;
  margin-bottom: var(--space-2);
}

.posts-grid .post-card:hover {
  border-color: var(--primary-light);
  box-shadow: var(--shadow-lg);
  transform: translateY(-4px);
  scale: 1.02;
}

.posts-grid .post-header {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-5) var(--space-6);
  border-bottom: 1px solid var(--border-color);
  background: rgba(255, 255, 255, 0.02);
}

.posts-grid .post-content {
  padding: var(--space-6);
  min-height: 120px;
}

.posts-grid .post-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--space-3) 0;
  line-height: 1.4;
}

.posts-grid .post-excerpt {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.posts-grid .post-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-6);
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  margin-top: auto;
}

.posts-grid .post-stats {
  display: flex;
  gap: var(--space-4);
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

.posts-grid .post-stats .stat-item {
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

/* 帖子区域响应式优化 */
@media (max-width: 768px) {
  .posts-grid {
    padding: var(--space-2);
    gap: var(--space-4);
  }
  
  .posts-grid.grid {
    grid-template-columns: 1fr;
    gap: var(--space-4);
  }
  
  .posts-grid .post-card {
    margin-bottom: var(--space-1);
  }
  
  .posts-grid .post-header {
    padding: var(--space-4);
  }
  
  .posts-grid .post-content {
    padding: var(--space-4);
    min-height: 100px;
  }
  
  .posts-grid .post-footer {
    padding: var(--space-3) var(--space-4);
  }
}

@media (min-width: 1200px) {
  .posts-grid.grid {
    grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  }
}

/* 分页响应式优化 */
@media (max-width: 768px) {
  .pagination-wrapper {
    padding: var(--space-4) var(--space-2);
    margin-top: var(--space-4);
  }
  
  :deep(.el-pagination) {
    gap: var(--space-1);
    font-size: var(--font-size-sm);
  }
  
  :deep(.el-pagination .btn-next),
  :deep(.el-pagination .btn-prev),
  :deep(.el-pagination .el-pager li) {
    min-height: 36px;
    min-width: 36px;
    margin: 0;
  }
  
  :deep(.el-pagination .el-pagination__total),
  :deep(.el-pagination .el-pagination__jump) {
    font-size: var(--font-size-xs);
    margin: var(--space-1);
  }
  
  :deep(.el-pagination .el-pagination__sizes) {
    margin: var(--space-1);
  }
  
  :deep(.el-pagination .el-pagination__jump .el-input) {
    width: 50px;
  }
}

@media (max-width: 480px) {
  .pagination-wrapper {
    padding: var(--space-3) var(--space-1);
  }
  
  :deep(.el-pagination) {
    /* 移动端简化布局 */
    .el-pagination__total,
    .el-pagination__sizes {
      display: none;
    }
  }
  
  :deep(.el-pagination .btn-next),
  :deep(.el-pagination .btn-prev),
  :deep(.el-pagination .el-pager li) {
    min-height: 32px;
    min-width: 32px;
    font-size: var(--font-size-xs);
  }
}

.popular-content-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.popular-item-new {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  padding: var(--space-lg);
  border-radius: var(--radius-xl);
  background: var(--bg-elevated);
  border: 1px solid var(--border-color-light);
  transition: all 0.3s ease;
  cursor: pointer;
}

.popular-item-new:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-lg);
  border-color: var(--brand-color);
}

.popular-item-new .item-rank {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.popular-item-new .rank-number {
  font-size: var(--font-size-sm);
  font-weight: bold;
  color: var(--text-secondary);
}

.popular-item-new .item-content {
  flex: 1;
  min-width: 0;
}

.popular-item-new .item-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: var(--space-sm);
}

.popular-item-new .item-stats {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  font-size: var(--font-size-xs);
  color: var(--text-tertiary);
}

.popular-item-new .stat-item {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
}

.popular-item-new .stat-item .el-icon {
  font-size: 14px;
}

.hot-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-md);
  padding: var(--space-md);
  justify-content: center;
  align-items: center;
  background: linear-gradient(135deg, var(--bg-elevated) 0%, var(--bg-secondary) 100%);
  border-radius: var(--radius-lg);
  box-shadow: inset 0 0 8px rgba(0, 0, 0, 0.05);
}

.hot-tags .el-tag.tag-item {
  cursor: pointer;
  transition: var(--transition-normal);
  font-weight: 500;
  border: none !important;
  border-radius: var(--radius-full) !important;
  padding: 4px 12px !important;
  font-size: var(--font-size-xs) !important;
  margin: 2px !important;
  position: relative;
  overflow: hidden;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.hot-tags .el-tag.tag-item::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(255,255,255,0.2) 0%, rgba(255,255,255,0) 100%);
  opacity: 0;
  transition: var(--transition-normal);
}

.hot-tags .el-tag.tag-item:hover {
  transform: translateY(-2px) scale(1.05);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.hot-tags .el-tag.tag-item:hover::before {
  opacity: 1;
}

.hot-tags .el-tag.tag-item .tag-count {
  margin-left: var(--space-1);
  opacity: 0.8;
  font-weight: 600;
  background: rgba(255, 255, 255, 0.2);
  padding: 1px 6px;
  border-radius: var(--radius-full);
  font-size: 10px;
  display: inline-block;
  min-width: 16px;
  text-align: center;
  line-height: 1.2;
}

.post-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-md);
}

.post-title-row .post-title {
  margin: 0;
  flex: 1;
}

.post-title-row .status-badges {
  display: flex;
  gap: var(--space-xs);
}

.post-card .status-badges .el-tag {
  font-size: var(--font-size-xs);
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

.post-card .status-badges .el-icon {
  font-size: 12px;
  margin-right: 2px;
}
</style>
