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
              class="main-search"
            >
              <template #prefix>
                <el-icon><Search /></el-icon>
              </template>
              <template #suffix>
                <el-button type="primary" text @click="handleSearch">
                  搜索
                </el-button>
              </template>
            </el-input>
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
      <div class="content-container">
        
        <!-- 焦点轮播区域 - 全宽度 -->
        <section class="hero-section">
          <div class="hero-wrapper">
            <div class="hero-content">
              <div class="hero-text">
                <h2>{{ systemSettings.hero_title }}</h2>
                <p>{{ systemSettings.hero_subtitle }}</p>
                <div class="hero-stats">
                  <div class="stat-item">
                    <span class="stat-number">{{ totalResources }}</span>
                    <span class="stat-label">资源</span>
                  </div>
                  <div class="stat-item">
                    <span class="stat-number">{{ totalUsers }}</span>
                    <span class="stat-label">用户</span>
                  </div>
                  <div class="stat-item">
                    <span class="stat-number">{{ totalDownloads }}</span>
                    <span class="stat-label">下载</span>
                  </div>
                </div>
              </div>
              
              <div class="hero-carousel" v-if="featuredResources.length > 0">
                <el-carousel 
                  :interval="6000" 
                  indicator-position="outside"
                  height="320px"
                  class="featured-carousel"
                >
                  <el-carousel-item 
                    v-for="resource in featuredResources.slice(0, 5)" 
                    :key="resource.id"
                    class="carousel-item"
                  >
                    <div class="featured-card" @click="viewResource(resource.id)">
                      <div class="card-cover">
                        <div class="cover-gradient"></div>
                        <div class="cover-content">
                          <h3>{{ resource.name }}</h3>
                          <p>{{ resource.description }}</p>
                          <div class="card-meta">
                            <span class="meta-author">{{ resource.author }}</span>
                            <span class="meta-downloads">{{ resource.download_count }} 下载</span>
                          </div>
                        </div>
                      </div>
                    </div>
                  </el-carousel-item>
                </el-carousel>
              </div>
            </div>
          </div>
        </section>

        <!-- 三栏内容布局 -->
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
                        style="width: 120px; margin-right: 12px;"
                      >
                        <el-option label="资源" value="resources" />
                        <el-option label="帖子" value="posts" />
                      </el-select>
                      
                      <!-- 筛选器 -->
                      <el-select 
                        v-model="sortBy" 
                        placeholder="排序方式"
                        @change="applySorting"
                        style="width: 140px;"
                      >
                        <el-option label="最新发布" value="created_at" />
                        <el-option label="下载最多" value="download_count" v-if="contentType === 'resources'" />
                        <el-option label="最多查看" value="view_count" v-if="contentType === 'posts'" />
                        <el-option label="最多点赞" value="like_count" v-if="contentType === 'posts'" />
                        <el-option label="评分最高" value="rating" v-if="contentType === 'resources'" />
                        <el-option label="名称排序" value="name" v-if="contentType === 'resources'" />
                        <el-option label="标题排序" value="title" v-if="contentType === 'posts'" />
                      </el-select>
                      
                      <!-- 视图切换 -->
                      <div class="view-toggle">
                        <el-button-group>
                          <el-button 
                            :type="viewMode === 'grid' ? 'primary' : 'default'"
                            @click="viewMode = 'grid'"
                          >
                            <el-icon><Grid /></el-icon>
                          </el-button>
                          <el-button 
                            :type="viewMode === 'list' ? 'primary' : 'default'"
                            @click="viewMode = 'list'"
                          >
                            <el-icon><List /></el-icon>
                          </el-button>
                        </el-button-group>
                      </div>

                      <!-- 发布按钮 -->
                      <el-button 
                        v-if="contentType === 'posts'" 
                        type="primary" 
                        @click="goToCreatePost"
                        style="margin-left: 12px;"
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
                          <h3 class="post-title">{{ post.title }}</h3>
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
                      class="popular-item"
                      @click="goToContent(item)"
                    >
                      <div class="item-rank">
                        <span class="rank-number" :class="getRankClass(index)">{{ index + 1 }}</span>
                      </div>
                      <div class="item-content">
                        <div class="item-header">
                          <el-icon class="type-icon" :class="item.type">
                            <Document v-if="item.type === 'resource'" />
                            <ChatLineRound v-else />
                          </el-icon>
                          <span class="item-type">{{ item.type === 'resource' ? '资源' : '帖子' }}</span>
                        </div>
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
                      :type="tag.type"
                      size="small"
                      class="tag-item"
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
  List,
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
  loadResources() // 重新加载资源
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

  // 排序
  switch (advancedSortBy.value) {
    case 'newest':
      filtered.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
      break
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
  type: string
  color?: string
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
      // 为标签添加随机类型用于展示
      const tagTypes = ['success', 'primary', 'info', 'warning', 'danger']
      hotTags.value = response.data.map((tag, index) => ({
        id: tag.id,
        name: tag.name,
        count: tag.use_count || 0,
        type: tagTypes[index % tagTypes.length],
        color: tag.color,
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
/* ===== 优化的CSS变量系统 ===== */
:global(:root) {
  /* 主题色彩系统 - 优化对比度和可访问性 */
  --color-primary: #2563eb;
  --color-primary-light: #3b82f6;
  --color-primary-dark: #1d4ed8;
  --color-secondary: #6366f1;
  --color-accent: #8b5cf6;
  --color-success: #059669;
  --color-warning: #d97706;
  --color-error: #dc2626;
  --color-info: #0891b2;
  
  /* 颜色别名 - 向后兼容 */
  --primary-color: var(--color-primary);
  --primary-light: var(--color-primary-light);
  --primary-dark: var(--color-primary-dark);
  --secondary-color: var(--color-secondary);
  --brand-color: var(--color-primary);
  
  /* 灰色系别名 */
  --gray-50: var(--color-gray-50);
  --gray-100: var(--color-gray-100);
  --gray-200: var(--color-gray-200);
  --gray-300: var(--color-gray-300);
  --gray-400: var(--color-gray-400);
  --gray-500: var(--color-gray-500);
  --gray-600: var(--color-gray-600);
  --gray-700: var(--color-gray-700);
  --gray-800: var(--color-gray-800);
  --gray-900: var(--color-gray-900);
  
  /* 中性色系 - 增强对比度 */
  --color-white: #ffffff;
  --color-gray-50: #f8fafc;
  --color-gray-100: #f1f5f9;
  --color-gray-200: #e2e8f0;
  --color-gray-300: #cbd5e1;
  --color-gray-400: #94a3b8;
  --color-gray-500: #64748b;
  --color-gray-600: #475569;
  --color-gray-700: #334155;
  --color-gray-800: #1e293b;
  --color-gray-900: #0f172a;
  
  /* 语义化文字颜色 */
  --text-primary: var(--color-gray-900);
  --text-secondary: var(--color-gray-600);
  --text-tertiary: var(--color-gray-500);
  --text-disabled: var(--color-gray-400);
  --text-inverse: var(--color-white);
  --text-muted: var(--color-gray-500);
  
  /* 语义化背景色 */
  --bg-primary: var(--color-white);
  --bg-secondary: var(--color-gray-50);
  --bg-tertiary: var(--color-gray-100);
  --bg-elevated: var(--color-white);
  --bg-hover: rgba(99, 102, 241, 0.04);
  --bg-active: rgba(99, 102, 241, 0.08);
  --bg-glass: rgba(255, 255, 255, 0.6);
  --bg-glass-strong: rgba(255, 255, 255, 0.8);
  
  /* 边框色彩 */
  --border-light: var(--color-gray-200);
  --border-medium: var(--color-gray-300);
  --border-strong: var(--color-gray-400);
  --border-focus: var(--color-primary);
  --border-color: var(--color-gray-200);
  
  /* 优化的阴影系统 - 减少重绘 */
  --shadow-xs: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow-sm: 0 1px 3px 0 rgba(0, 0, 0, 0.1);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
  --shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  --shadow-glass: 0 8px 32px rgba(0, 0, 0, 0.06);
  
  /* 毛玻璃效果 */
  --glass-backdrop: blur(16px) saturate(180%);
  --glass-border: 1px solid rgba(255, 255, 255, 0.2);
  
  /* 间距系统 - 8pt网格 */
  --space-0: 0;
  --space-1: 0.25rem;  /* 4px */
  --space-2: 0.5rem;   /* 8px */
  --space-3: 0.75rem;  /* 12px */
  --space-4: 1rem;     /* 16px */
  --space-5: 1.25rem;  /* 20px */
  --space-6: 1.5rem;   /* 24px */
  --space-8: 2rem;     /* 32px */
  --space-10: 2.5rem;  /* 40px */
  --space-12: 3rem;    /* 48px */
  --space-16: 4rem;    /* 64px */
  --space-20: 5rem;    /* 80px */
  --space-24: 6rem;    /* 96px */
  
  /* 现代化圆角系统 */
  --radius-xs: 0.125rem;  /* 2px */
  --radius-sm: 0.375rem;  /* 6px */
  --radius-md: 0.5rem;    /* 8px */
  --radius-lg: 0.75rem;   /* 12px */
  --radius-xl: 1rem;      /* 16px */
  --radius-2xl: 1.5rem;   /* 24px */
  --radius-full: 9999px;
  
  /* 字体系统 - 使用系统字体栈 */
  --font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  --font-family-mono: "SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, monospace;
  
  /* 字体大小 - 增强可读性 */
  --text-xs: 0.75rem;    /* 12px */
  --text-sm: 0.875rem;   /* 14px */
  --text-base: 1rem;     /* 16px */
  --text-lg: 1.125rem;   /* 18px */
  --text-xl: 1.25rem;    /* 20px */
  --text-2xl: 1.5rem;    /* 24px */
  --text-3xl: 1.875rem;  /* 30px */
  --text-4xl: 2.25rem;   /* 36px */
  
  /* 字体大小别名 */
  --font-size-xs: var(--text-xs);
  --font-size-sm: var(--text-sm);
  --font-size-base: var(--text-base);
  --font-size-lg: var(--text-lg);
  --font-size-xl: var(--text-xl);
  --font-size-2xl: var(--text-2xl);
  --font-size-3xl: var(--text-3xl);
  --font-size-4xl: var(--text-4xl);
  
  /* 字重 */
  --font-weight-light: 300;
  --font-weight-normal: 400;
  --font-weight-medium: 500;
  --font-weight-semibold: 600;
  --font-weight-bold: 700;
  
  /* 行高 */
  --line-height-tight: 1.25;
  --line-height-normal: 1.5;
  --line-height-relaxed: 1.75;
  
  /* 优化的过渡动画 */
  --transition-fast: 0.15s cubic-bezier(0.4, 0, 0.2, 1);
  --transition-normal: 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  --transition-slow: 0.35s cubic-bezier(0.4, 0, 0.2, 1);
  
  /* Z-index层级管理 */
  --z-dropdown: 1000;
  --z-sticky: 1020;
  --z-fixed: 1030;
  --z-modal: 1050;
  --z-toast: 1060;
  
  /* 布局断点 */
  --breakpoint-sm: 640px;
  --breakpoint-md: 768px;
  --breakpoint-lg: 1024px;
  --breakpoint-xl: 1280px;
  --breakpoint-2xl: 1536px;
}

/* 暗色主题 */
:global(:root.dark) {
  --text-primary: var(--color-gray-100);
  --text-secondary: var(--color-gray-300);
  --text-muted: var(--color-gray-400);
  --text-disabled: var(--color-gray-600);
  
  --bg-primary: var(--color-gray-900);
  --bg-secondary: var(--color-gray-800);
  --bg-elevated: var(--color-gray-800);
  --bg-hover: var(--color-gray-700);
  
  --border-color: var(--color-gray-700);
  --border-hover: var(--color-gray-600);
}

/* ===== 全局容器样式 ===== */
.home-container {
  min-height: 100vh;
  position: relative;
  overflow-x: hidden;
  background: linear-gradient(135deg, 
    hsl(220, 20%, 97%) 0%, 
    hsl(220, 20%, 95%) 100%);
}

:global(:root.dark) .home-container {
  background: linear-gradient(135deg, 
    hsl(220, 20%, 8%) 0%, 
    hsl(220, 20%, 12%) 100%);
}

/* ===== 背景层样式 ===== */
.background-layer {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: -10;
  pointer-events: none;
}

.gradient-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(
    135deg,
    rgba(99, 102, 241, 0.1) 0%,
    rgba(168, 85, 247, 0.05) 25%,
    rgba(236, 72, 153, 0.05) 50%,
    rgba(245, 158, 11, 0.1) 100%
  );
}

.pattern-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-image: 
    radial-gradient(circle at 20% 20%, rgba(99, 102, 241, 0.1) 0%, transparent 50%),
    radial-gradient(circle at 80% 80%, rgba(168, 85, 247, 0.08) 0%, transparent 50%),
    radial-gradient(circle at 60% 40%, rgba(236, 72, 153, 0.06) 0%, transparent 50%);
  animation: patternFloat 20s ease-in-out infinite;
}

@keyframes patternFloat {
  0%, 100% { transform: translate(0, 0) rotate(0deg); }
  33% { transform: translate(-10px, -10px) rotate(1deg); }
  66% { transform: translate(10px, -5px) rotate(-1deg); }
}

/* ===== 顶部导航栏样式 ===== */
.top-header {
  position: sticky;
  top: 0;
  z-index: 100;
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--border-color);
  transition: var(--transition-normal);
}

:global(:root.dark) .top-header {
  background: rgba(17, 24, 39, 0.9);
}

.header-wrapper {
  max-width: 1440px;
  margin: 0 auto;
  padding: var(--space-4) var(--space-6);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-8);
}

.header-left {
  flex-shrink: 0;
}

.brand {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  text-decoration: none;
}

.brand-icon {
  font-size: var(--font-size-2xl);
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
}

.brand-text h1 {
  font-size: var(--font-size-xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.2;
}

.brand-text span {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  line-height: 1;
}

.header-center {
  flex: 1;
  max-width: 600px;
}

.search-container {
  position: relative;
}

.main-search {
  width: 100%;
}

.main-search :deep(.el-input__wrapper) {
  border-radius: var(--radius-2xl);
  box-shadow: var(--shadow-sm);
  border: 2px solid transparent;
  transition: var(--transition-normal);
}

.main-search :deep(.el-input__wrapper):hover {
  border-color: var(--primary-light);
  box-shadow: var(--shadow-md);
}

.main-search :deep(.el-input__wrapper.is-focus) {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.header-right {
  flex-shrink: 0;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.user-menu {
  position: relative;
}

.user-trigger {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: var(--transition-fast);
}

.user-trigger:hover {
  background: var(--bg-hover);
}

.user-name {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-primary);
}

.dropdown-arrow {
  color: var(--text-muted);
  transition: var(--transition-fast);
}

/* ===== 主内容区域样式 ===== */
.main-content {
  position: relative;
  z-index: 1;
  padding-top: var(--space-6);
}

.content-container {
  max-width: 1440px;
  margin: 0 auto;
  padding: 0 var(--space-6);
}

/* ===== 焦点轮播区域样式 ===== */
.hero-section {
  margin-bottom: var(--space-12);
}

.hero-wrapper {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.8), 
    rgba(249, 250, 251, 0.9));
  backdrop-filter: blur(20px);
  border-radius: var(--radius-2xl);
  border: 1px solid var(--border-color);
  box-shadow: var(--shadow-xl);
  overflow: hidden;
}

:global(:root.dark) .hero-wrapper {
  background: linear-gradient(135deg, 
    rgba(31, 41, 55, 0.8), 
    rgba(17, 24, 39, 0.9));
}

.hero-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-12);
  padding: var(--space-12);
  align-items: center;
}

.hero-text h2 {
  font-size: var(--font-size-4xl);
  font-weight: 800;
  color: var(--text-primary);
  margin: 0 0 var(--space-4) 0;
  line-height: 1.2;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hero-text p {
  font-size: var(--font-size-lg);
  color: var(--text-secondary);
  margin: 0 0 var(--space-8) 0;
  line-height: 1.6;
}

.hero-stats {
  display: flex;
  gap: var(--space-8);
}

.stat-item {
  text-align: center;
}

.stat-number {
  display: block;
  font-size: var(--font-size-2xl);
  font-weight: 700;
  color: var(--primary-color);
  line-height: 1;
}

.stat-label {
  display: block;
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  margin-top: var(--space-1);
}

.hero-carousel {
  position: relative;
}

.featured-carousel :deep(.el-carousel__container) {
  border-radius: var(--radius-xl);
  overflow: hidden;
  box-shadow: var(--shadow-lg);
}

.carousel-item {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
}

.featured-card {
  width: 100%;
  height: 280px;
  border-radius: var(--radius-xl);
  overflow: hidden;
  cursor: pointer;
  transition: var(--transition-normal);
  position: relative;
}

.featured-card:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-xl);
}

.card-cover {
  position: relative;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  display: flex;
  align-items: center;
  justify-content: center;
}

.cover-gradient {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, 
    rgba(0, 0, 0, 0.3) 0%, 
    rgba(0, 0, 0, 0.1) 100%);
}

.cover-content {
  position: relative;
  z-index: 2;
  text-align: center;
  color: white;
  padding: var(--space-6);
}

.cover-content h3 {
  font-size: var(--font-size-xl);
  font-weight: 600;
  margin: 0 0 var(--space-3) 0;
}

.cover-content p {
  font-size: var(--font-size-sm);
  opacity: 0.9;
  margin: 0 0 var(--space-4) 0;
  line-height: 1.5;
}

.card-meta {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-xs);
  opacity: 0.8;
}

/* ===== 三栏布局样式 ===== */
.three-column-section {
  margin-bottom: var(--space-16);
}

.column-wrapper {
  display: grid;
  grid-template-columns: 280px 1fr 280px;
  gap: var(--space-8);
  align-items: start;
}

/* 左侧栏样式 */
.left-column {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
  position: sticky;
  top: 120px;
}

/* 中间主内容样式 */
.center-column {
  min-height: 600px;
}

/* 右侧栏样式 */
.right-column {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
  position: sticky;
  top: 120px;
}

/* ===== 面板通用样式 ===== */
.nav-panel,
.announcement-panel,
.stats-panel,
.activity-panel,
.tags-panel,
.content-panel {
  background: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(20px);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
  transition: var(--transition-normal);
}

:global(:root.dark) .nav-panel,
:global(:root.dark) .announcement-panel,
:global(:root.dark) .stats-panel,
:global(:root.dark) .activity-panel,
:global(:root.dark) .tags-panel,
:global(:root.dark) .content-panel {
  background: rgba(31, 41, 55, 0.7);
}

.panel-header {
  padding: var(--space-5) var(--space-6);
  border-bottom: 1px solid var(--border-color);
  background: rgba(255, 255, 255, 0.5);
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  transition: var(--transition-fast);
}

.panel-header:hover {
  background: rgba(255, 255, 255, 0.7);
}

:global(:root.dark) .panel-header {
  background: rgba(31, 41, 55, 0.5);
}

:global(:root.dark) .panel-header:hover {
  background: rgba(31, 41, 55, 0.7);
}

.panel-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

/* 折叠图标样式 */
.collapse-icon {
  transition: transform var(--transition-fast);
  color: var(--text-muted);
}

.collapse-icon.collapsed {
  transform: rotate(-90deg);
}

.panel-header h3 {
  font-size: var(--font-size-base);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.panel-content {
  padding: var(--space-6);
}

/* ===== 快速导航样式 ===== */
.quick-nav {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.nav-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: var(--transition-fast);
  border: 2px solid transparent;
}

.nav-item:hover {
  background: var(--bg-hover);
  border-color: var(--primary-light);
}

.nav-item.active {
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  color: white;
  box-shadow: var(--shadow-md);
}

.nav-item span {
  flex: 1;
  font-size: var(--font-size-sm);
  font-weight: 500;
}

.nav-count {
  background: rgba(255, 255, 255, 0.2);
  color: currentColor;
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
  font-weight: 600;
  min-width: 24px;
  text-align: center;
}

.nav-item:not(.active) .nav-count {
  background: var(--gray-100);
  color: var(--text-muted);
}

/* ===== 公告列表样式 ===== */
.announcement-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.announcement-item {
  display: flex;
  gap: var(--space-3);
  padding: var(--space-4);
  background: var(--bg-secondary);
  border-radius: var(--radius-lg);
  border-left: 4px solid var(--primary-color);
  transition: var(--transition-fast);
}

.announcement-item:hover {
  background: var(--bg-hover);
  transform: translateX(2px);
}

.announcement-dot {
  width: 8px;
  height: 8px;
  background: var(--primary-color);
  border-radius: 50%;
  flex-shrink: 0;
  margin-top: 6px;
}

.announcement-content {
  flex: 1;
}

.announcement-text {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: 1.4;
  margin-bottom: var(--space-1);
}

.announcement-time {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

/* ===== 内容面板样式 ===== */
.content-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-6) var(--space-6) var(--space-4) var(--space-6);
  border-bottom: 1px solid var(--border-color);
  background: rgba(255, 255, 255, 0.5);
}

:global(:root.dark) .content-header {
  background: rgba(31, 41, 55, 0.5);
}

.header-left h2 {
  font-size: var(--font-size-2xl);
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 var(--space-1) 0;
}

.resource-count {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}

.content-controls {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.view-toggle :deep(.el-button-group) {
  border-radius: var(--radius-lg);
  overflow: hidden;
}

/* ===== 资源网格样式 ===== */
.resource-content {
  padding: var(--space-6);
}

.resource-grid {
  display: grid;
  gap: var(--space-8);
  padding: var(--space-4);
}

.resource-grid.grid {
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: var(--space-6);
}

.resource-grid.list {
  grid-template-columns: 1fr;
  gap: var(--space-5);
}

.resource-card {
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

.resource-card:hover {
  border-color: var(--primary-light);
  box-shadow: var(--shadow-lg);
  transform: translateY(-4px);
  scale: 1.02;
}

.card-header {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  padding: var(--space-5) var(--space-6);
  border-bottom: 1px solid var(--border-color);
  background: rgba(255, 255, 255, 0.02);
}

.card-icon {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.card-title {
  flex: 1;
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
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.3;
}

.card-category {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
  background: var(--bg-secondary);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
}

.card-actions {
  flex-shrink: 0;
}

.card-body {
  padding: var(--space-6);
  min-height: 100px;
}

.card-description {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0 0 var(--space-4) 0;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.card-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.card-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-6);
  background: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  margin-top: auto;
}

.footer-meta {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

.footer-meta span {
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

/* 响应式优化 */
@media (max-width: 768px) {
  .resource-grid {
    padding: var(--space-2);
    gap: var(--space-4);
  }
  
  .resource-grid.grid {
    grid-template-columns: 1fr;
    gap: var(--space-4);
  }
  
  .resource-card {
    margin-bottom: var(--space-1);
  }
  
  .card-header {
    padding: var(--space-4);
  }
  
  .card-body {
    padding: var(--space-4);
    min-height: 80px;
  }
  
  .card-footer {
    padding: var(--space-3) var(--space-4);
  }
}

@media (min-width: 1200px) {
  .resource-grid.grid {
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  }
}

/* ===== 统计面板样式 ===== */
.stats-container {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.stat-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4);
  background: linear-gradient(135deg, var(--bg-elevated) 0%, var(--bg-secondary) 100%);
  border-radius: var(--radius-xl);
  border: 1px solid var(--border-color);
  transition: var(--transition-normal);
  position: relative;
  overflow: hidden;
  animation: slideInUp 0.6s ease-out;
  animation-fill-mode: both;
}

.stat-item::before {
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
  transition: var(--transition-normal);
  pointer-events: none;
}

:global(:root.dark) .stat-item::before {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.05) 0%, 
    rgba(255, 255, 255, 0.02) 100%);
}

.stat-item:nth-child(1) { animation-delay: 0.1s; }
.stat-item:nth-child(2) { animation-delay: 0.2s; }
.stat-item:nth-child(3) { animation-delay: 0.3s; }
.stat-item:nth-child(4) { animation-delay: 0.4s; }

@keyframes slideInUp {
  0% {
    opacity: 0;
    transform: translateY(20px);
  }
  100% {
    opacity: 1;
    transform: translateY(0);
  }
}



.stat-item:hover {
  border-color: var(--primary-light);
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.stat-item:hover::before {
  opacity: 1;
}

.stat-header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.stat-icon-wrapper {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-secondary);
  border: 2px solid transparent;
  transition: var(--transition-normal);
  position: relative;
}

.stat-trend {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-2);
  background: rgba(103, 194, 58, 0.1);
  border-radius: var(--radius-md);
  font-size: var(--font-size-xs);
  color: #67c23a;
  font-weight: 500;
}

/* 具体图标样式 - 根据统计类型设置不同颜色 */
.stat-item:nth-child(1) .stat-icon-wrapper {
  background: rgba(64, 158, 255, 0.1);
  border-color: rgba(64, 158, 255, 0.2);
}

.stat-item:nth-child(1) .stat-icon-wrapper .el-icon {
  color: #409eff;
}

.stat-item:nth-child(2) .stat-icon-wrapper {
  background: rgba(103, 194, 58, 0.1);
  border-color: rgba(103, 194, 58, 0.2);
}

.stat-item:nth-child(2) .stat-icon-wrapper .el-icon {
  color: #67c23a;
}

.stat-item:nth-child(3) .stat-icon-wrapper {
  background: rgba(230, 162, 60, 0.1);
  border-color: rgba(230, 162, 60, 0.2);
}

.stat-item:nth-child(3) .stat-icon-wrapper .el-icon {
  color: #e6a23c;
}

.stat-item:nth-child(4) .stat-icon-wrapper {
  background: rgba(245, 108, 108, 0.1);
  border-color: rgba(245, 108, 108, 0.2);
}

.stat-item:nth-child(4) .stat-icon-wrapper .el-icon {
  color: #f56c6c;
}

/* 暗色主题适配 */
:global(:root.dark) .stat-item {
  background: linear-gradient(135deg, rgba(31, 41, 55, 0.8) 0%, rgba(17, 24, 39, 0.8) 100%);
  border-color: var(--color-gray-700);
}

:global(:root.dark) .stat-item:hover {
  border-color: var(--primary-light);
  background: linear-gradient(135deg, rgba(31, 41, 55, 0.9) 0%, rgba(17, 24, 39, 0.9) 100%);
}

:global(:root.dark) .stat-number {
  color: var(--color-gray-100);
}

:global(:root.dark) .stat-title {
  color: var(--color-gray-400);
}

:global(:root.dark) .stat-trend {
  background: rgba(103, 194, 58, 0.2);
  color: #67c23a;
}



.stat-item:hover .stat-icon-wrapper {
  transform: scale(1.05);
  box-shadow: var(--shadow-md);
}

.stat-main {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  text-align: right;
  min-width: 0;
}

.stat-number {
  font-size: var(--font-size-2xl);
  font-weight: 800;
  color: var(--text-primary);
  line-height: 1;
  margin-bottom: var(--space-1);
  display: block;
  font-variant-numeric: tabular-nums;
  animation: fadeInScale 0.6s ease-out;
}

@keyframes fadeInScale {
  0% {
    opacity: 0;
    transform: scale(0.8);
  }
  100% {
    opacity: 1;
    transform: scale(1);
  }
}

.stat-title {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: 500;
  line-height: 1.2;
  white-space: nowrap;
}

/* 响应式设计 - 统计面板 */
@media (max-width: 1280px) {
  .stats-container {
    gap: var(--space-3);
  }
  
  .stat-item {
    padding: var(--space-3);
  }
  
  .stat-icon-wrapper {
    width: 40px;
    height: 40px;
  }
  
  .stat-number {
    font-size: var(--font-size-xl);
  }
  
  .stat-title {
    font-size: var(--font-size-xs);
  }
  
  .stat-trend {
    font-size: var(--font-size-xs);
    padding: var(--space-1);
  }
}

@media (max-width: 1024px) {
  .stats-container {
    gap: var(--space-3);
  }
  
  .stat-item {
    padding: var(--space-3);
  }
  
  .stat-icon-wrapper {
    width: 36px;
    height: 36px;
  }
  
  .stat-number {
    font-size: var(--font-size-lg);
  }
  
  .stat-title {
    font-size: var(--font-size-xs);
  }
  
  .stat-trend {
    font-size: var(--font-size-xs);
    padding: var(--space-1);
  }
}

@media (max-width: 768px) {
  .stats-container {
    gap: var(--space-2);
  }
  
  .stat-item {
    padding: var(--space-3);
  }
  
  .stat-icon-wrapper {
    width: 32px;
    height: 32px;
  }
  
  .stat-number {
    font-size: var(--font-size-base);
  }
  
  .stat-title {
    font-size: var(--font-size-xs);
  }
  
  .stat-trend {
    font-size: var(--font-size-xs);
    padding: var(--space-1);
  }
}

/* ===== 活动列表样式 ===== */
.activity-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.activity-item {
  display: flex;
  gap: var(--space-3);
  padding: var(--space-3);
  border-radius: var(--radius-lg);
  transition: var(--transition-fast);
  cursor: pointer;
}

.activity-item:hover {
  background: var(--primary-light);
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.activity-avatar {
  flex-shrink: 0;
}

.activity-content {
  flex: 1;
  min-width: 0;
}

.activity-content.no-avatar {
  margin-left: 0;
}

.activity-text {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  line-height: 1.4;
  margin-bottom: var(--space-1);
}

.activity-target {
  color: var(--primary-color);
  font-weight: 500;
}

.activity-time {
  font-size: var(--font-size-xs);
  color: var(--text-muted);
}

/* ===== 热门标签样式 ===== */
.hot-tags {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

.tag-item {
  cursor: pointer;
  transition: var(--transition-fast);
}

.tag-item:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

.tag-count {
  margin-left: var(--space-1);
  opacity: 0.7;
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
  gap: var(--space-3);
}

.popular-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  padding: var(--space-3);
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
}

.rank-number {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: white;
}

.rank-first {
  background: linear-gradient(135deg, #FFD700, #FFA500);
}

.rank-second {
  background: linear-gradient(135deg, #C0C0C0, #A0A0A0);
}

.rank-third {
  background: linear-gradient(135deg, #CD7F32, #B8860B);
}

.rank-normal {
  background: var(--color-gray-400);
}

.item-content {
  flex: 1;
  min-width: 0;
}

.item-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-1);
}

.type-icon {
  font-size: var(--font-size-sm);
}

.type-icon.resource {
  color: var(--color-primary);
}

.type-icon.post {
  color: var(--color-success);
}

.item-type {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
  font-weight: 500;
}

.item-title {
  font-size: var(--font-size-sm);
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--space-2);
  line-height: 1.4;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
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
}

.item-stats .el-icon {
  font-size: 12px;
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
.card-header {
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
  padding: 0;
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

/* ===== 页脚样式 ===== */
.site-footer {
  background: linear-gradient(135deg, 
    rgba(255, 255, 255, 0.9) 0%, 
    rgba(248, 250, 252, 0.95) 100%);
  backdrop-filter: blur(20px);
  border-top: 1px solid var(--border-color);
  margin-top: var(--space-24);
  padding: var(--space-20) 0 var(--space-12) 0;
  position: relative;
  overflow: hidden;
}

:global(:root.dark) .site-footer {
  background: linear-gradient(135deg, 
    rgba(17, 24, 39, 0.9) 0%, 
    rgba(31, 41, 55, 0.95) 100%);
}

.site-footer::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 20% 80%, rgba(99, 102, 241, 0.1) 0%, transparent 50%),
    radial-gradient(circle at 80% 20%, rgba(168, 85, 247, 0.08) 0%, transparent 50%);
  pointer-events: none;
  z-index: -1;
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
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  flex-shrink: 0;
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
  background: linear-gradient(135deg, var(--primary-color), var(--secondary-color));
  border-radius: var(--radius-sm);
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
  color: var(--primary-color);
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
  background: var(--primary-color);
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
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}

/* 响应式设计 */
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
</style>