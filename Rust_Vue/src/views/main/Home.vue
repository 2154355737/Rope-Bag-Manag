<template>
  <div class="home-container">
    <!-- 动态背景系统 -->
    <div class="dynamic-background">
      <!-- 主背景渐变 -->
      <div class="gradient-layer"></div>
      
      <!-- 几何形状 -->
      <div class="geometric-shapes">
        <div class="shape triangle-1"></div>
        <div class="shape triangle-2"></div>
        <div class="shape circle-1"></div>
        <div class="shape circle-2"></div>
        <div class="shape square-1"></div>
        <!-- 为页脚区域添加更多形状 -->
        <div class="shape circle-3"></div>
        <div class="shape triangle-3"></div>
        <div class="shape square-2"></div>
      </div>
      
      <!-- 浮动粒子 -->
      <div class="floating-particles">
        <div class="particle" v-for="n in 20" :key="n" :style="getParticleStyle(n)"></div>
      </div>
      
      <!-- 波纹效果 -->
      <div class="ripple-effects">
        <div class="ripple ripple-1"></div>
        <div class="ripple ripple-2"></div>
        <div class="ripple ripple-3"></div>
        <!-- 为页脚区域添加更多波纹 -->
        <div class="ripple ripple-4"></div>
        <div class="ripple ripple-5"></div>
      </div>
    </div>

    <!-- 顶部导航栏 -->
    <header class="header">
      <div class="header-content">
        <div class="logo">
          <div class="logo-icon">📚</div>
          <div class="logo-text">
            <h1>资源社区</h1>
            <p>分享、发现、学习</p>
          </div>
        </div>
        
        <div class="search">
          <el-input
            v-model="searchQuery"
            placeholder="搜索资源..."
            size="large"
            clearable
            @keyup.enter="handleSearch"
          >
            <template #prefix>
              <el-icon><Search /></el-icon>
            </template>
          </el-input>
        </div>
        
        <div class="actions">
          <ThemeSwitcher />
          <el-button 
            v-if="!isLoggedIn" 
            type="primary" 
            size="large"
            @click="goToLogin"
          >
            <el-icon><User /></el-icon>
            登录
          </el-button>
          <div v-if="isLoggedIn" class="user-section">
            <el-dropdown trigger="click" placement="bottom-end">
              <div class="user-info">
                <el-avatar :size="32" class="user-avatar" :src="userInfo.avatar_url" v-if="userInfo.avatar_url">
                  <img :src="userInfo.avatar_url" />
                </el-avatar>
                <el-avatar :size="32" class="user-avatar" v-else>
                  <el-icon><User /></el-icon>
                </el-avatar>
                <span class="user-name">{{ userInfo.nickname || userInfo.username }}</span>
                <el-icon class="dropdown-icon"><Bottom /></el-icon>
              </div>
              <template #dropdown>
                <el-dropdown-menu class="user-dropdown">
                  <el-dropdown-item @click="goToUserProfile">
                    <el-icon><User /></el-icon>
                    个人资料
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
    </header>

    <!-- 主内容区 -->
    <main class="main">
      <!-- 内容包装器 -->
      <div class="content-wrapper">
                <div class="main-content">
          <!-- 整合后的主要内容区域 -->
          <div class="unified-content">
            <!-- 侧边信息栏 -->
            <div class="sidebar">
              <div class="notice-card">
                <h3 class="notice-title">社区公告</h3>
                <div v-if="notices.length === 0" class="empty-notice">
                  <p>暂无公告信息</p>
                </div>
                <ul v-else class="notice-list">
                  <li v-for="item in notices" :key="item.id" class="notice-item">
                    {{ item.text }}
                  </li>
                </ul>
              </div>
              
              <!-- 分类导航 -->
              <div class="category-card">
                <h3 class="category-title">
                  <el-icon><Collection /></el-icon>
                  资源分类
                </h3>
                <div class="category-list">
                  <div 
                    class="category-item"
                    :class="{ 'active': activeCategory === 'all' }"
                    @click="handleCategoryChange({ props: { name: 'all' } })"
                  >
                    <div class="category-item-content">
                      <el-icon><Grid /></el-icon>
                      <span>全部</span>
                    </div>
                    <span class="category-count">{{ totalResources }}</span>
                  </div>
                  <div 
                    v-for="category in categories" 
                    :key="category.id"
                    class="category-item"
                    :class="{ 'active': activeCategory === category.id.toString() }"
                    @click="handleCategoryChange({ props: { name: category.id.toString() } })"
                  >
                    <div class="category-item-content">
                      <el-icon><Document /></el-icon>
                      <span>{{ category.name }}</span>
                    </div>
                    <span class="category-count">{{ getCategoryCount(category.id) }}</span>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- 主要内容区域 -->
            <div class="main-area">
          <!-- 精选资源轮播 -->
          <div class="featured-resources">
            <div class="section-header">
              <h2 class="section-title">
                <el-icon><Star /></el-icon>
                精选资源
              </h2>
              <p class="section-subtitle">发现社区最受欢迎的优质资源</p>
            </div>
            
            <el-carousel 
              v-if="featuredResources.length > 0"
              :interval="5000" 
              :arrow="resources.length > 1 ? 'hover' : 'never'"
              indicator-position="outside"
              class="resource-carousel"
              height="280px"
            >
              <el-carousel-item 
                v-for="(resourceGroup, index) in featuredResourceGroups" 
                :key="index"
                class="carousel-item"
              >
                <div class="featured-grid">
                  <div 
                    v-for="resource in resourceGroup" 
                    :key="resource.id"
                    class="featured-card"
                    @click="viewResource(resource.id)"
                  >
                    <div class="featured-cover">
                      <div class="featured-icon">
                        <el-icon size="32" :color="getCategoryColor(resource.category_id)">
                          <Document />
                        </el-icon>
                      </div>
                      <div class="featured-badge">
                        {{ getCategoryLabel(resource.category_id) }}
                      </div>
                    </div>
                    <div class="featured-content">
                      <h3 class="featured-title">{{ resource.name }}</h3>
                      <p class="featured-desc">{{ resource.description }}</p>
                      <div class="featured-meta">
                        <span class="meta-author">
                          <el-icon><User /></el-icon>
                          {{ resource.author }}
                        </span>
                        <span class="meta-downloads">
                          <el-icon><Download /></el-icon>
                          {{ resource.download_count }}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              </el-carousel-item>
            </el-carousel>
            
            <!-- 轮播为空时的状态 -->
            <div v-else class="featured-empty">
              <div class="empty-icon">⭐</div>
              <h3>暂无精选资源</h3>
              <p>快来上传优质资源，成为精选内容吧！</p>
              <el-button type="primary" @click="showUploadDialog = true">
                <el-icon><Upload /></el-icon>
                上传资源
              </el-button>
            </div>
          </div>

          <!-- 资源列表 -->
          <div class="resources">
            <div class="section-header">
              <h2 class="section-title">
                <el-icon><Collection /></el-icon>
                全部资源
              </h2>
              <div class="section-actions">
                <span class="resource-count">共 {{ filteredResources.length }} 个资源</span>
                <div class="view-switcher">
                  <el-button-group>
                    <el-button 
                      :type="viewMode === 'grid' ? 'primary' : 'default'" 
                      size="small"
                      @click="viewMode = 'grid'"
                    >
                      <el-icon><Grid /></el-icon>
                    </el-button>
                    <el-button 
                      :type="viewMode === 'list' ? 'primary' : 'default'" 
                      size="small"
                      @click="viewMode = 'list'"
                    >
                      <el-icon><List /></el-icon>
                    </el-button>
                    <el-button 
                      :type="viewMode === 'card' ? 'primary' : 'default'" 
                      size="small"
                      @click="viewMode = 'card'"
                    >
                  <el-icon><Document /></el-icon>
                    </el-button>
                  </el-button-group>
                </div>
                <el-button 
                  type="primary" 
                  size="default"
                  @click="showUploadDialog = true"
                >
                  <el-icon><Upload /></el-icon>
                  上传资源
                </el-button>
                </div>
              </div>
              
            <!-- 高级筛选区域 -->
            <div class="advanced-filters">
              <div class="filter-row">
                <div class="filter-group">
                  <label class="filter-label">作者</label>
                  <el-select 
                    v-model="authorFilter" 
                    placeholder="选择作者" 
                    clearable
                    size="small"
                    class="filter-input"
                  >
                    <el-option label="全部作者" value="" />
                    <el-option 
                      v-for="author in uniqueAuthors" 
                      :key="author" 
                      :label="author" 
                      :value="author" 
                    />
                  </el-select>
                </div>
                
                <div class="filter-group">
                  <label class="filter-label">下载量</label>
                  <el-select 
                    v-model="downloadFilter" 
                    placeholder="下载量范围" 
                    clearable
                    size="small"
                    class="filter-input"
                  >
                    <el-option label="全部" value="" />
                    <el-option label="0-10次" value="0-10" />
                    <el-option label="11-50次" value="11-50" />
                    <el-option label="51-100次" value="51-100" />
                    <el-option label="100次以上" value="100+" />
                  </el-select>
                </div>
                
                <div class="filter-group">
                  <label class="filter-label">上传时间</label>
                  <el-select 
                    v-model="dateFilter" 
                    placeholder="时间范围" 
                    clearable
                    size="small"
                    class="filter-input"
                  >
                    <el-option label="全部时间" value="" />
                    <el-option label="今天" value="today" />
                    <el-option label="本周" value="week" />
                    <el-option label="本月" value="month" />
                    <el-option label="三个月内" value="quarter" />
                  </el-select>
              </div>
              
                <div class="filter-group">
                  <label class="filter-label">关键词</label>
                  <el-input 
                    v-model="keywordFilter" 
                    placeholder="搜索关键词"
                    clearable
                    size="small"
                    class="filter-input"
                  >
                    <template #prefix>
                      <el-icon><Search /></el-icon>
                    </template>
                  </el-input>
                </div>
                
                <div class="filter-actions">
                  <el-button 
                    size="small" 
                    @click="clearFilters"
                    :disabled="!hasActiveFilters"
                  >
                    <el-icon><RefreshLeft /></el-icon>
                    重置
                  </el-button>
                  <el-button 
                    type="primary" 
                    size="small"
                    @click="toggleAdvancedFilters"
                  >
                    <el-icon><Filter /></el-icon>
                    {{ showAdvancedFilters ? '收起' : '高级' }}
                  </el-button>
                </div>
              </div>
              
              <!-- 高级筛选面板 -->
              <el-collapse-transition>
                <div v-show="showAdvancedFilters" class="advanced-panel">
                  <div class="advanced-row">
                    <div class="filter-group">
                      <label class="filter-label">文件大小</label>
                      <el-select 
                        v-model="sizeFilter" 
                        placeholder="文件大小" 
                        clearable
                        size="small"
                        class="filter-input"
                      >
                        <el-option label="全部大小" value="" />
                        <el-option label="小于1MB" value="small" />
                        <el-option label="1MB-10MB" value="medium" />
                        <el-option label="大于10MB" value="large" />
                      </el-select>
                </div>
                    
                    <div class="filter-group">
                      <label class="filter-label">标签</label>
                      <el-input 
                        v-model="tagFilter" 
                        placeholder="搜索标签"
                        clearable
                        size="small"
                        class="filter-input"
                      >
                        <template #prefix>
                          <el-icon><PriceTag /></el-icon>
                        </template>
                      </el-input>
                </div>
                    
                    <div class="filter-group">
                      <label class="filter-label">排序方式</label>
                      <el-select 
                        v-model="advancedSortBy" 
                        placeholder="排序方式" 
                        size="small"
                        class="filter-input"
                      >
                        <el-option label="最新上传" value="newest" />
                        <el-option label="最早上传" value="oldest" />
                        <el-option label="下载最多" value="most_downloaded" />
                        <el-option label="下载最少" value="least_downloaded" />
                        <el-option label="名称A-Z" value="name_asc" />
                        <el-option label="名称Z-A" value="name_desc" />
                      </el-select>
              </div>
            </div>
                </div>
              </el-collapse-transition>
          </div>

            <div v-if="resources.length === 0" class="empty-state">
              <div class="empty-icon">📦</div>
              <h3>暂无资源</h3>
              <p>还没有资源被上传，快来分享你的第一个资源吧！</p>
              <el-button type="primary" @click="showUploadDialog = true">
                <el-icon><Upload /></el-icon>
                上传资源
              </el-button>
            </div>
            
            <!-- 网格视图 -->
            <div v-else-if="viewMode === 'grid'" class="resources-grid">
              <div 
                v-for="resource in paginatedResources" 
                :key="resource.id"
                class="resource-card"
                @click="viewResource(resource.id)"
              >
                <div class="resource-icon">
                  <el-icon size="28" :color="getCategoryColor(resource.category_id)">
                    <Document />
                  </el-icon>
                </div>
                <div class="resource-content">
                  <div class="resource-header">
                    <h3 class="resource-title">{{ resource.name }}</h3>
                    <span class="resource-badge">{{ getCategoryLabel(resource.category_id) }}</span>
                  </div>
                  <p class="resource-desc">{{ resource.description }}</p>
                  <div class="resource-footer">
                    <div class="resource-meta">
                      <span class="meta-item">
                        <el-icon><User /></el-icon>
                        {{ resource.author }}
                      </span>
                      <span class="meta-item">
                        <el-icon><Calendar /></el-icon>
                        {{ formatDateUtil(resource.created_at) }}
                      </span>
                    </div>
                    <div class="resource-actions">
                      <el-button 
                        type="primary" 
                        size="small"
                        @click.stop="downloadResource(resource.id)"
                      >
                        <el-icon><Download /></el-icon>
                        下载
                      </el-button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            
            <!-- 列表视图 -->
            <div v-else-if="viewMode === 'list'" class="resources-list">
              <div 
                v-for="resource in paginatedResources" 
                :key="resource.id"
                class="resource-list-item"
                @click="viewResource(resource.id)"
              >
                <div class="list-icon">
                  <el-icon size="24" :color="getCategoryColor(resource.category_id)">
                    <Document />
                  </el-icon>
                </div>
                <div class="list-content">
                  <div class="list-main">
                    <h3 class="list-title">{{ resource.name }}</h3>
                    <p class="list-desc">{{ resource.description }}</p>
                  </div>
                  <div class="list-meta">
                    <span class="list-category">{{ getCategoryLabel(resource.category_id) }}</span>
                    <span class="list-author">{{ resource.author }}</span>
                    <span class="list-date">{{ formatDateUtil(resource.created_at) }}</span>
                  </div>
                </div>
                <div class="list-actions">
                  <span class="download-count">
                    <el-icon><Download /></el-icon>
                    {{ resource.download_count }}
                  </span>
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
            
            <!-- 卡片视图 -->
            <div v-else-if="viewMode === 'card'" class="resources-cards">
              <div 
                v-for="resource in paginatedResources" 
                :key="resource.id"
                class="resource-card-item"
                @click="viewResource(resource.id)"
              >
                <div class="card-header">
                  <div class="card-icon">
                    <el-icon size="32" :color="getCategoryColor(resource.category_id)">
                      <Document />
                    </el-icon>
                  </div>
                  <div class="card-title-area">
                    <h3 class="card-title">{{ resource.name }}</h3>
                    <span class="card-category">{{ getCategoryLabel(resource.category_id) }}</span>
                  </div>
                </div>
                <p class="card-desc">{{ resource.description }}</p>
                <div class="card-footer">
                  <div class="card-meta">
                    <div class="meta-row">
                      <el-icon><User /></el-icon>
                      <span>{{ resource.author }}</span>
                    </div>
                    <div class="meta-row">
                      <el-icon><Calendar /></el-icon>
                      <span>{{ formatDateUtil(resource.created_at) }}</span>
                    </div>
                    <div class="meta-row">
                      <el-icon><Download /></el-icon>
                      <span>{{ resource.download_count }} 次下载</span>
                    </div>
                  </div>
                  <el-button 
                    type="primary" 
                    @click.stop="downloadResource(resource.id)"
                  >
                    <el-icon><Download /></el-icon>
                    下载资源
                  </el-button>
                </div>
              </div>
            </div>
          </div>
          

            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- 页脚 -->
    <footer class="footer">
      <div class="footer-content">
        <div class="footer-sections">
          <!-- 网站信息 -->
          <div class="footer-section">
            <div class="footer-logo">
              <div class="footer-logo-icon">📚</div>
              <div class="footer-logo-text">
                <h3>资源社区</h3>
                <p>分享、发现、学习</p>
              </div>
            </div>
            <p class="footer-desc">
              一个致力于知识分享和资源交流的开放平台，汇聚优质学习资源，助力每个人的成长之路。
            </p>
            <div class="footer-stats">
              <div class="stat-item">
                <span class="stat-number">{{ totalResources }}</span>
                <span class="stat-label">资源</span>
              </div>
              <div class="stat-item">
                <span class="stat-number">{{ totalUsers }}</span>
                <span class="stat-label">用户</span>
              </div>
              <div class="stat-item">
                <span class="stat-number">{{ formatNumber(totalDownloads) }}</span>
                <span class="stat-label">下载</span>
              </div>
            </div>
          </div>

          <!-- 快速链接 -->
          <div class="footer-section">
            <h4 class="footer-title">快速链接</h4>
            <ul class="footer-links">
              <li><a href="#" @click.prevent="scrollToTop">回到顶部</a></li>
              <li><a href="#" @click.prevent="handleCategoryChange({ props: { name: 'all' } })">全部资源</a></li>
              <li><a href="#" @click.prevent="showUploadDialog = true">上传资源</a></li>
              <li><a href="#" @click.prevent="goToLogin" v-if="!isLoggedIn">用户登录</a></li>
              <li><a href="#" @click.prevent="goToUserProfile" v-if="isLoggedIn">个人中心</a></li>
            </ul>
          </div>

          <!-- 分类导航 -->
          <div class="footer-section">
            <h4 class="footer-title">资源分类</h4>
            <ul class="footer-links">
              <li v-for="category in categories.slice(0, 5)" :key="category.id">
                <a href="#" @click.prevent="handleCategoryChange({ props: { name: category.id.toString() } })">
                  {{ category.name }}
                </a>
              </li>
            </ul>
          </div>

          <!-- 联系信息 -->
          <div class="footer-section">
            <h4 class="footer-title">联系我们</h4>
            <div class="footer-contact">
              <div class="contact-item">
                <el-icon><Message /></el-icon>
                <span>support@example.com</span>
              </div>
              <div class="contact-item">
                <el-icon><Phone /></el-icon>
                <span>400-123-4567</span>
              </div>
              <div class="contact-item">
                <el-icon><Location /></el-icon>
                <span>北京市朝阳区</span>
              </div>
            </div>
            <div class="social-links">
              <a href="#" class="social-link" title="微信">
                <span>💬</span>
              </a>
              <a href="#" class="social-link" title="QQ">
                <span>🐧</span>
              </a>
              <a href="#" class="social-link" title="GitHub">
                <span>⚡</span>
              </a>
              <a href="#" class="social-link" title="邮箱">
                <span>📧</span>
              </a>
            </div>
          </div>
        </div>

        <!-- 底部版权信息 -->
        <div class="footer-bottom">
          <div class="footer-copyright">
            <p>&copy; {{ new Date().getFullYear() }} 资源社区. All rights reserved.</p>
            <p>Built with ❤️ using Vue 3 + Rust</p>
          </div>
          <div class="footer-legal">
            <a href="#" @click.prevent>隐私政策</a>
            <span class="divider">|</span>
            <a href="#" @click.prevent>服务条款</a>
            <span class="divider">|</span>
            <a href="#" @click.prevent>帮助中心</a>
          </div>
        </div>
      </div>
    </footer>

    <!-- 上传对话框 -->
    <el-dialog
      v-model="showUploadDialog"
      title="上传资源"
      width="600px"
      :before-close="handleUploadClose"
      destroy-on-close
    >
      <el-form 
        :model="uploadForm" 
        :rules="uploadRules" 
        ref="uploadFormRef" 
        label-width="100px"
        size="large"
      >
        <el-form-item label="资源标题" prop="title">
          <el-input 
            v-model="uploadForm.title" 
            placeholder="请输入资源标题"
            clearable
          />
        </el-form-item>
        
        <el-form-item label="资源描述" prop="description">
          <el-input
            v-model="uploadForm.description"
            type="textarea"
            :rows="4"
            placeholder="请输入资源描述"
            show-word-limit
            maxlength="500"
          />
        </el-form-item>
        
        <el-form-item label="资源分类" prop="category">
          <el-select v-model="uploadForm.category" placeholder="选择分类" style="width: 100%">
            <el-option label="教程" value="tutorial" />
            <el-option label="工具" value="tool" />
            <el-option label="模板" value="template" />
            <el-option label="其他" value="other" />
          </el-select>
        </el-form-item>
        
        <el-form-item label="资源标签">
          <el-input
            v-model="uploadForm.tagsInput"
            placeholder="输入标签，用逗号分隔"
            @keyup.enter="addTag"
            clearable
          />
          <div class="tags-container">
            <el-tag
              v-for="tag in uploadForm.tags"
              :key="tag"
              closable
              @close="removeTag(tag)"
              effect="light"
            >
              {{ tag }}
            </el-tag>
          </div>
        </el-form-item>
        
        <el-form-item label="资源文件" prop="file">
          <el-upload
            ref="uploadRef"
            :auto-upload="false"
            :on-change="handleFileChange"
            :limit="1"
            accept=".zip,.rar,.7z,.pdf,.doc,.docx"
            drag
          >
            <el-icon class="el-icon--upload"><Upload /></el-icon>
            <div class="el-upload__text">
              将文件拖到此处，或<em>点击上传</em>
            </div>
            <template #tip>
              <div class="el-upload__tip">
                支持 zip, rar, 7z, pdf, doc, docx 格式，文件大小不超过100MB
              </div>
            </template>
          </el-upload>
        </el-form-item>
        
        <el-form-item label="封面图片">
          <el-upload
            ref="coverUploadRef"
            :auto-upload="false"
            :on-change="handleCoverChange"
            :limit="1"
            accept="image/*"
            drag
          >
            <el-icon class="el-icon--upload"><Picture /></el-icon>
            <div class="el-upload__text">
              将图片拖到此处，或<em>点击上传</em>
            </div>
            <template #tip>
              <div class="el-upload__tip">
                支持 jpg, png, gif 格式，建议尺寸 300x200
              </div>
            </template>
          </el-upload>
        </el-form-item>
      </el-form>
      
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="showUploadDialog = false" size="large">取消</el-button>
          <el-button 
            type="primary" 
            @click="submitUpload" 
            :loading="uploading"
            size="large"
          >
            上传资源
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUpdated, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Search,
  User,
  Setting,
  Download,
  Star,
  Calendar,
  Upload,
  Document,
  Collection,
  Picture,
  Bottom,
  Switch,
  Message,
  Phone,
  Location,
  Grid,
  List,
  RefreshLeft,
  Filter,
  PriceTag
} from '@element-plus/icons-vue'
import type { FormInstance, UploadFile } from 'element-plus'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import { communityApi } from '@/api/community'
import type { Resource, UploadForm } from '@/types'
import { getUserInfo } from '@/utils/auth'
import { formatDate as formatDateUtil, formatFileSize } from '@/utils/format'
import { packageApi, type Package } from '@/api/packages'
import { categoryApi, type Category } from '@/api/categories'
import { getActiveAnnouncements, type Announcement } from '@/api/announcements'

const router = useRouter()

// 响应式数据
const searchQuery = ref('')
const activeCategory = ref('all')
const sortBy = ref('latest')
const filterType = ref('all')

const totalResources = ref(0)
const showUploadDialog = ref(false)
const uploading = ref(false)
const loading = ref(false)

// 统计数据
const totalDownloads = ref(0)
const totalUsers = ref(0)
const totalLikes = ref(0)

// 公告数据
const notices = ref<{ id: number, text: string }[]>([])

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
    const res = await getActiveAnnouncements()
    console.log('公告数据结构:', res)
    
    if (res.code === 0 && res.data) {
      // 处理不同的数据结构可能性
      if (Array.isArray(res.data)) {
        // 如果是数组，直接使用
        notices.value = res.data.map(announcement => ({
          id: announcement.id,
          text: announcement.title + ': ' + announcement.content
        }))
      } else if (res.data.list && Array.isArray(res.data.list)) {
        // 如果是 {list: []} 格式
        notices.value = res.data.list.map((announcement: any) => ({
          id: announcement.id,
          text: announcement.title + ': ' + announcement.content
        }))
      } else {
        // 如果是单个对象
        console.warn('公告数据不是预期的数组格式:', res.data)
        notices.value = [{ id: 1, text: '欢迎来到资源社区！' }]
      }
    } else {
      console.error('获取公告失败:', res.message)
      // 设置默认公告
      notices.value = [
        { id: 1, text: '欢迎来到资源社区！' },
        { id: 2, text: '请遵守社区规范，文明发言。' },
        { id: 3, text: '资源上传请确保无版权争议。' },
      ]
    }
  } catch (error) {
    console.error('加载公告出错:', error)
    // 设置默认公告
    notices.value = [{ id: 0, text: '欢迎来到资源社区！请遵守社区规范，文明发言。' }]
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

// 上传表单
const uploadForm = reactive<UploadForm>({
  title: '',
  description: '',
  category: '',
  tags: [],
  tagsInput: '',
  file: undefined,
  cover: undefined
})

const uploadRules = {
  title: [
    { required: true, message: '请输入资源标题', trigger: 'blur' },
    { min: 2, max: 50, message: '标题长度在2到50个字符', trigger: 'blur' }
  ],
  description: [
    { required: true, message: '请输入资源描述', trigger: 'blur' },
    { min: 10, max: 500, message: '描述长度在10到500个字符', trigger: 'blur' }
  ],
  category: [
    { required: true, message: '请选择资源分类', trigger: 'change' }
  ],
  file: [
    { required: true, message: '请选择资源文件', trigger: 'change' }
  ]
}

// 计算属性
const filteredResources = computed(() => {
  let filtered = resources.value

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

  // 排序
  switch (sortBy.value) {
    case 'latest':
      filtered.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
      break
    case 'downloads':
      filtered.sort((a, b) => b.download_count - a.download_count)
      break
    case 'likes':
      filtered.sort((a, b) => (b.like_count || 0) - (a.like_count || 0))
      break
    case 'favorites':
      filtered.sort((a, b) => (b.favorite_count || 0) - (a.favorite_count || 0))
      break
  }

  return filtered
})

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
    
    console.log("请求参数:", params)
    const res = await packageApi.getPackages(params)
    
    if (res.code === 0 && res.data) {
      resources.value = res.data.list
      totalResources.value = res.data.total
      
      // 计算统计数据
      totalDownloads.value = resources.value.reduce((sum, resource) => sum + resource.download_count, 0)
      totalLikes.value = resources.value.reduce((sum, resource) => sum + resource.like_count, 0)
      
      // 统计数据可能通过API获取更准确
      totalUsers.value = Math.floor(Math.random() * 1000) + 500
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
      loadResources()
    } else {
      ElMessage.error(res.msg || '下载失败')
    }
  } catch (error) {
    console.error('下载失败:', error)
    ElMessage.error('下载失败')
  }
}

const goToLogin = () => {
  router.push('/login')
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
  return num.toLocaleString()
}

const addTag = () => {
  const tag = uploadForm.tagsInput?.trim()
  if (tag && tag.length > 0 && !uploadForm.tags.includes(tag)) {
    uploadForm.tags.push(tag)
    uploadForm.tagsInput = ''
  }
}

const removeTag = (tag: string) => {
  const index = uploadForm.tags.indexOf(tag)
  if (index > -1) {
    uploadForm.tags.splice(index, 1)
  }
}

const handleFileChange = (file: UploadFile) => {
  uploadForm.file = file.raw as File | undefined
}

const handleCoverChange = (file: UploadFile) => {
  uploadForm.cover = file.raw as File | undefined
}

const submitUpload = async () => {
  if (!uploadForm.file) {
    ElMessage.error('请选择资源文件')
    return
  }

  try {
    uploading.value = true
    const res = await communityApi.createResource({
      title: uploadForm.title,
      description: uploadForm.description,
      category: uploadForm.category,
      tags: uploadForm.tags,
      file_url: uploadForm.file_url || '',
      cover_url: uploadForm.cover_url
    })

    if (res.code === 0) {
      ElMessage.success('资源上传成功')
      showUploadDialog.value = false
      Object.assign(uploadForm, {
        title: '',
        description: '',
        category: '',
        tags: [],
        tagsInput: '',
        file: undefined,
        cover: undefined
      })
      loadResources()
    } else {
      ElMessage.error(res.msg || '上传失败')
    }
  } catch (error) {
    console.error('上传失败:', error)
    ElMessage.error('上传失败')
  } finally {
    uploading.value = false
  }
}

const handleUploadClose = () => {
  showUploadDialog.value = false
  Object.assign(uploadForm, {
    title: '',
    description: '',
    category: '',
    tags: [],
    tagsInput: '',
    file: undefined,
    cover: undefined
  })
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

// 初始化
onMounted(async () => {
  await loadCategories()
  await loadResources()
  await fetchAnnouncements()
})

const scrollToTop = () => {
  window.scrollTo({ top: 0, behavior: 'smooth' })
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

const paginatedResources = computed(() => {
  let filtered = filteredResources.value

  // 作者筛选
  if (authorFilter.value) {
    filtered = filtered.filter(resource => resource.author.toLowerCase().includes(authorFilter.value.toLowerCase()))
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

  // 日期筛选
  if (dateFilter.value) {
    const today = new Date()
    const startDate = new Date()
    const endDate = new Date()

    switch (dateFilter.value) {
      case 'today':
        startDate.setHours(0, 0, 0, 0)
        endDate.setHours(23, 59, 59, 999)
        break
      case 'week':
        startDate.setDate(today.getDate() - today.getDay())
        endDate.setDate(today.getDate() + (6 - today.getDay()))
        startDate.setHours(0, 0, 0, 0)
        endDate.setHours(23, 59, 59, 999)
        break
      case 'month':
        startDate.setDate(1)
        endDate.setMonth(today.getMonth() + 1)
        endDate.setDate(0)
        startDate.setHours(0, 0, 0, 0)
        endDate.setHours(23, 59, 59, 999)
        break
      case 'quarter':
        startDate.setMonth(Math.floor(today.getMonth() / 3) * 3)
        endDate.setMonth(startDate.getMonth() + 2)
        endDate.setDate(31)
        startDate.setDate(1)
        startDate.setHours(0, 0, 0, 0)
        endDate.setHours(23, 59, 59, 999)
        break
    }

    filtered = filtered.filter(resource => {
      const createdAt = new Date(resource.created_at)
      return createdAt >= startDate && createdAt <= endDate
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

  // 移除分页，直接返回所有筛选结果
  return filtered
})

const clearFilters = () => {
  authorFilter.value = ''
  downloadFilter.value = ''
  dateFilter.value = ''
  keywordFilter.value = ''
  sizeFilter.value = ''
  tagFilter.value = ''
  advancedSortBy.value = 'newest'
  showAdvancedFilters.value = false
}

const toggleAdvancedFilters = () => {
  showAdvancedFilters.value = !showAdvancedFilters.value
}

// 动态设置背景高度以覆盖整个页面
const updateBackgroundHeight = () => {
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
  })
}

// 监听内容变化和窗口大小变化
onMounted(() => {
  updateBackgroundHeight()
  window.addEventListener('resize', updateBackgroundHeight)
  
  // 监听DOM变化
  const observer = new MutationObserver(updateBackgroundHeight)
  const homeContainer = document.querySelector('.home-container')
  if (homeContainer) {
    observer.observe(homeContainer, {
      childList: true,
      subtree: true,
      attributes: true
    })
  }
})

onUpdated(() => {
  updateBackgroundHeight()
})
</script>

<style scoped>
.home-container {
  min-height: 100vh;
  height: auto; /* 确保容器高度随内容自适应 */
  background: transparent;
  position: relative;
  z-index: 0; /* 确保容器不会遮挡背景 */
  transition: background-color var(--transition-base);
  width: 100%;
  overflow-x: hidden; /* 防止水平滚动 */
}

/* 动态背景系统 */
.dynamic-background {
  position: absolute; /* 相对于父容器定位 */
  top: 0;
  left: 0;
  width: 100%;
  height: 100%; /* 跟随父容器高度 */
  min-width: 100%;
  min-height: 100vh; /* 至少覆盖视口高度 */
  z-index: -10;
  overflow: hidden;
}

.gradient-layer {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  width: 100%;
  height: 100%;
  min-height: 100vh;
  min-width: 100%;
  background: 
    radial-gradient(ellipse at top left, rgba(59, 130, 246, 0.3) 0%, transparent 50%),
    radial-gradient(ellipse at top right, rgba(236, 72, 153, 0.2) 0%, transparent 50%),
    radial-gradient(ellipse at center left, rgba(168, 85, 247, 0.2) 0%, transparent 50%),
    radial-gradient(ellipse at center right, rgba(34, 197, 94, 0.2) 0%, transparent 50%),
    linear-gradient(135deg, #0a0f23 0%, #1a1d3a 25%, #2d1b69 50%, #1e293b 75%, #0f172a 100%);
  z-index: -10;
}

/* 浅色主题下的渐变层 */
:root:not(.dark) .gradient-layer {
  background: 
    radial-gradient(ellipse at top left, rgba(59, 130, 246, 0.4) 0%, transparent 60%),
    radial-gradient(ellipse at top right, rgba(236, 72, 153, 0.3) 0%, transparent 60%),
    radial-gradient(ellipse at center left, rgba(168, 85, 247, 0.3) 0%, transparent 60%),
    radial-gradient(ellipse at center right, rgba(34, 197, 94, 0.3) 0%, transparent 60%),
    radial-gradient(ellipse at bottom center, rgba(245, 158, 11, 0.2) 0%, transparent 50%),
    linear-gradient(135deg, #f0f9ff 0%, #e0e7ff 25%, #fdf4ff 50%, #f0fdf4 75%, #fffbeb 100%);
}

.geometric-shapes {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 200vh;
  min-height: 200vh;
  min-width: 100vw;
  pointer-events: none; /* 允许背景元素被点击穿透 */
  z-index: -9;
}

.shape {
  position: absolute;
  background: rgba(59, 130, 246, 0.08);
  border-radius: 50%;
  opacity: 0.1;
  filter: blur(1px);
  animation: float 10s infinite ease-in-out;
}

/* 浅色主题下的几何形状 */
:root:not(.dark) .shape {
  background: rgba(59, 130, 246, 0.15);
  opacity: 0.3;
  filter: blur(0.5px);
  box-shadow: 0 0 20px rgba(59, 130, 246, 0.2);
}

.shape.triangle-1 {
  width: 100px;
  height: 100px;
  top: 20%;
  left: 10%;
  transform: rotate(45deg);
  background: rgba(236, 72, 153, 0.08);
}

:root:not(.dark) .shape.triangle-1 {
  background: rgba(236, 72, 153, 0.2);
  box-shadow: 0 0 25px rgba(236, 72, 153, 0.3);
}

.shape.triangle-2 {
  width: 80px;
  height: 80px;
  top: 60%;
  right: 10%;
  transform: rotate(-45deg);
  background: rgba(168, 85, 247, 0.08);
}

:root:not(.dark) .shape.triangle-2 {
  background: rgba(168, 85, 247, 0.2);
  box-shadow: 0 0 25px rgba(168, 85, 247, 0.3);
}

.shape.circle-1 {
  width: 120px;
  height: 120px;
  bottom: 20%;
  left: 40%;
  background: rgba(34, 197, 94, 0.08);
}

:root:not(.dark) .shape.circle-1 {
  background: rgba(34, 197, 94, 0.2);
  box-shadow: 0 0 30px rgba(34, 197, 94, 0.3);
}

.shape.circle-2 {
  width: 150px;
  height: 150px;
  top: 30%;
  right: 30%;
  background: rgba(245, 158, 11, 0.08);
}

:root:not(.dark) .shape.circle-2 {
  background: rgba(245, 158, 11, 0.2);
  box-shadow: 0 0 35px rgba(245, 158, 11, 0.3);
}

.shape.square-1 {
  width: 100px;
  height: 100px;
  bottom: 15%; /* 在更高的背景中调整位置 */
  left: 70%;
  background: rgba(139, 92, 246, 0.08);
}

:root:not(.dark) .shape.square-1 {
  background: rgba(139, 92, 246, 0.2);
  box-shadow: 0 0 25px rgba(139, 92, 246, 0.3);
}

/* 为页脚区域添加的新几何形状 */
.shape.circle-3 {
  width: 90px;
  height: 90px;
  bottom: 25%;
  left: 15%;
  background: rgba(245, 158, 11, 0.08);
}

:root:not(.dark) .shape.circle-3 {
  background: rgba(245, 158, 11, 0.2);
  box-shadow: 0 0 25px rgba(245, 158, 11, 0.3);
}

.shape.triangle-3 {
  width: 70px;
  height: 70px;
  bottom: 8%;
  right: 25%;
  transform: rotate(90deg);
  background: rgba(34, 197, 94, 0.08);
}

:root:not(.dark) .shape.triangle-3 {
  background: rgba(34, 197, 94, 0.2);
  box-shadow: 0 0 25px rgba(34, 197, 94, 0.3);
}

.shape.square-2 {
  width: 85px;
  height: 85px;
  bottom: 35%;
  right: 5%;
  background: rgba(236, 72, 153, 0.08);
}

:root:not(.dark) .shape.square-2 {
  background: rgba(236, 72, 153, 0.2);
  box-shadow: 0 0 25px rgba(236, 72, 153, 0.3);
}

.floating-particles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 200vh;
  min-height: 200vh;
  min-width: 100vw;
  pointer-events: none;
  z-index: -8;
}

.particle {
  position: absolute;
  width: 5px;
  height: 5px;
  background: rgba(59, 130, 246, 0.4);
  border-radius: 50%;
  opacity: 0.5;
  animation: float 10s infinite ease-in-out;
}

/* 浅色主题下的粒子 */
:root:not(.dark) .particle {
  background: rgba(59, 130, 246, 0.6);
  opacity: 0.8;
  box-shadow: 0 0 8px rgba(59, 130, 246, 0.4);
}

.ripple-effects {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 200vh;
  min-height: 200vh;
  min-width: 100vw;
  pointer-events: none;
  z-index: -7;
}

.ripple {
  position: absolute;
  border-radius: 50%;
  background: rgba(59, 130, 246, 0.1);
  border: 2px solid rgba(59, 130, 246, 0.2);
  opacity: 0;
  animation: ripple-pulse 3s infinite;
}

/* 浅色主题下的波纹 */
:root:not(.dark) .ripple {
  background: rgba(59, 130, 246, 0.15);
  border: 2px solid rgba(59, 130, 246, 0.3);
  box-shadow: 0 0 40px rgba(59, 130, 246, 0.2);
}

.ripple-1 {
  width: 100px;
  height: 100px;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.ripple-2 {
  width: 150px;
  height: 150px;
  top: 20%;
  left: 20%;
  transform: translate(-50%, -50%);
}

.ripple-3 {
  width: 200px;
  height: 200px;
  bottom: 5%; /* 调整位置以适应更高的背景 */
  right: 10%;
  transform: translate(-50%, -50%);
}

/* 为页脚区域添加的新波纹 */
.ripple-4 {
  width: 120px;
  height: 120px;
  bottom: 20%;
  left: 30%;
  transform: translate(-50%, -50%);
  animation-delay: 1s;
}

.ripple-5 {
  width: 160px;
  height: 160px;
  bottom: 12%;
  right: 35%;
  transform: translate(-50%, -50%);
  animation-delay: 2s;
}

@keyframes float {
  0% {
    transform: translateY(0) translateX(0) rotate(0deg);
  }
  25% {
    transform: translateY(-10px) translateX(10px) rotate(5deg);
  }
  50% {
    transform: translateY(0) translateX(0) rotate(0deg);
  }
  75% {
    transform: translateY(10px) translateX(-10px) rotate(-5deg);
  }
  100% {
    transform: translateY(0) translateX(0) rotate(0deg);
  }
}

@keyframes ripple-pulse {
  0% {
    opacity: 0.5;
    transform: scale(0.5);
  }
  50% {
    opacity: 0.2;
    transform: scale(1);
  }
  100% {
    opacity: 0;
    transform: scale(1.5);
  }
}

/* 轮播卡片浮动动画 */
@keyframes cardFloat {
  0%, 100% {
    transform: translateY(0px) rotateX(0deg);
  }
  33% {
    transform: translateY(-3px) rotateX(1deg);
  }
  66% {
    transform: translateY(2px) rotateX(-0.5deg);
  }
}

/* 图标脉冲动画 */
@keyframes iconPulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.9;
  }
  50% {
    transform: scale(1.1);
    opacity: 1;
  }
}

/* 标签滑入动画 */
@keyframes slideInBadge {
  0% {
    transform: translateX(100%);
    opacity: 0;
  }
  100% {
    transform: translateX(0);
    opacity: 1;
  }
}

/* 顶部导航 */
.header {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(15px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  padding: 16px 0;
  position: sticky;
  top: 0;
  z-index: 100;
  transition: all var(--transition-base);
}

/* 浅色主题下的导航栏 */
:root:not(.dark) .header {
  background: rgba(255, 255, 255, 0.4) !important;
  backdrop-filter: blur(25px) !important;
  border-bottom: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 2px 20px rgba(59, 130, 246, 0.1) !important;
}

.header-content {
  max-width: min(1400px, 95vw); /* 响应式最大宽度 */
  margin: 0 auto;
  padding: 0 clamp(16px, 4vw, 32px); /* 流体内边距 */
  display: flex;
  align-items: center;
  gap: 32px;
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.logo-icon {
  font-size: 32px;
}

.logo-text h1 {
  margin: 0;
  font-size: clamp(18px, 3.5vw, 24px); /* 流体字体 */
  font-weight: 600;
  color: var(--text-primary);
}

.logo-text p {
  margin: 4px 0 0 0;
  font-size: clamp(12px, 2.5vw, 16px);
  color: var(--text-secondary);
}

.search {
  flex: 1;
  max-width: 400px;
}

.actions {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-shrink: 0;
  margin-left: auto;
  padding: 0 8px;
}

/* 用户信息区域 */
.user-section {
  display: flex;
  align-items: center;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

/* 浅色主题下的用户信息 */
:root:not(.dark) .user-info {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 4px 15px rgba(59, 130, 246, 0.1) !important;
}

.user-info:hover {
  background: rgba(255, 255, 255, 0.2);
  border-color: rgba(255, 255, 255, 0.3);
}

:root:not(.dark) .user-info:hover {
  background: rgba(255, 255, 255, 0.5) !important;
  border-color: rgba(59, 130, 246, 0.3) !important;
}

.user-avatar {
  border: 2px solid var(--border-color);
  transition: all 0.3s ease;
}

.user-info:hover .user-avatar {
  border-color: var(--brand-color);
  transform: scale(1.05);
}

.user-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dropdown-icon {
  font-size: 12px;
  color: var(--text-secondary);
  transition: transform 0.3s ease;
}

.user-info:hover .dropdown-icon {
  transform: rotate(180deg);
}

/* 用户下拉菜单 */
.user-dropdown {
  min-width: 180px;
  padding: 8px 0;
}

.user-dropdown .el-dropdown-menu__item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  border-radius: 8px;
  margin: 4px 8px;
  transition: all 0.2s ease;
}

.user-dropdown .el-dropdown-menu__item:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.user-dropdown .el-dropdown-menu__item.divided {
  border-top: 1px solid var(--border-color);
  margin-top: 8px;
  padding-top: 16px;
}

/* 统一导航栏 */
.navigation-bar {
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(15px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.15);
  transition: all var(--transition-base);
}

/* 浅色主题下的导航栏 */
:root:not(.dark) .navigation-bar {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(25px) !important;
  border-bottom: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 2px 15px rgba(59, 130, 246, 0.08) !important;
}

.navigation-content {
  max-width: min(1400px, 95vw);
  margin: 0 auto;
  padding: 0 clamp(16px, 4vw, 32px);
}

/* 分类区域 */
.category-section {
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.category-wrapper {
  padding: 8px 0;
}

.category-tabs :deep(.el-tabs__header) {
  margin: 0;
}

.category-tabs :deep(.el-tabs__nav-wrap) {
  padding: 0;
}

.category-tabs :deep(.el-tabs__item) {
  font-weight: 500;
  color: var(--text-secondary);
  transition: color var(--transition-base);
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
  padding: 12px 16px;
}

.category-tabs :deep(.el-tabs__item.is-active) {
  color: var(--brand-color);
  font-weight: 600;
}

.category-tabs :deep(.el-tabs__item:hover) {
  color: var(--brand-color-light);
}

/* 筛选区域 */
.filter-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
  gap: 16px;
}

.filter-controls {
  display: flex;
  gap: 12px;
  align-items: center;
}

.filter-select {
  width: 140px;
}

.upload-btn {
  flex-shrink: 0;
  font-weight: 500;
}

/* 主内容区 */
.main {
  width: 100%;
  min-height: 0;
  height: auto;
  box-sizing: border-box;
  padding: 32px 0;
  position: relative; /* 确保内容区域在背景之上 */
  z-index: 5; /* 更高的z-index确保内容在背景之上 */
  background: transparent; /* 主内容区域透明背景 */
}

/* 内容包装器 */
.content-wrapper {
  position: relative;
  width: 100%;
  max-width: min(1400px, 95vw);
  margin: 0 auto;
  padding: 0 clamp(16px, 4vw, 32px);
  background: transparent;
  backdrop-filter: none;
  border: none;
  border-radius: 0;
  box-shadow: none;
  min-height: calc(100vh - 200px);
  overflow: visible;
}

/* 浅色主题下的包装器 */
:root:not(.dark) .content-wrapper {
  background: transparent !important;
  backdrop-filter: none !important;
  border: none !important;
  box-shadow: none !important;
}

.main-content {
  display: flex;
  width: 100%;
  box-sizing: border-box;
  position: relative;
  min-height: 600px;
  padding: 24px 0;
}

/* 整合后的内容区域 */
.unified-content {
  display: flex;
  gap: 24px;
  width: 100%;
  align-items: flex-start;
}

/* 侧边栏样式 */
.sidebar {
  width: 280px;
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 主要内容区域 */
.main-area {
  flex: 1;
  min-width: 0;
  min-height: 500px;
  position: relative;
}

/* 统计卡片 */
.stats {
  margin-bottom: 32px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(200px, 100%), 1fr));
  gap: 16px;
  width: 100%;
}

/* 超宽屏幕优化 */
@media (min-width: 1600px) {
  .stats-grid {
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 20px;
  }
}

.stat-card {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  padding: 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: var(--shadow-light);
  transition: all 0.2s ease;
}

/* 浅色主题下的统计卡片 */
:root:not(.dark) .stat-card {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.1) !important;
}

.stat-card:hover {
  box-shadow: var(--shadow-base);
  transform: translateY(-2px);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-dark));
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 20px;
}

.stat-info {
  flex: 1;
}

.stat-number {
  font-size: clamp(20px, 4vw, 28px); /* 流体字体 */
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
}

.stat-label {
  font-size: clamp(12px, 2.5vw, 16px);
  color: var(--text-secondary);
  margin-top: 4px;
}

/* 资源列表 */
.resources {
  margin-bottom: 32px;
}

.empty-state {
  text-align: center;
  padding: 80px 20px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  box-shadow: var(--shadow-light);
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.empty-state h3 {
  margin: 0 0 8px 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.empty-state p {
  margin: 0 0 24px 0;
  color: var(--text-secondary);
}

.resources-grid {
  width: 100%;
  box-sizing: border-box;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(300px, 100%), 1fr));
  gap: 12px;
}

/* 不同屏幕尺寸下的资源网格 */
@media (min-width: 1600px) {
  .resources-grid {
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 16px;
  }
}

@media (min-width: 1200px) and (max-width: 1599px) {
  .resources-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (min-width: 900px) and (max-width: 1199px) {
  .resources-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 899px) {
  .resources-grid {
    grid-template-columns: 1fr;
  }
}

.resource-card {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  overflow: hidden;
  box-shadow: var(--shadow-light);
  transition: all 0.2s ease;
  cursor: pointer;
}

/* 浅色主题下的资源卡片 */
:root:not(.dark) .resource-card {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.1) !important;
}

.resource-card:hover {
  box-shadow: var(--shadow-base);
  transform: translateY(-4px);
}

.resource-icon {
  position: relative;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(5px);
  border-radius: 8px 8px 0 0;
}

/* 浅色主题下的资源图标 */
:root:not(.dark) .resource-icon {
  background: rgba(255, 255, 255, 0.2) !important;
  backdrop-filter: blur(15px) !important;
  border-bottom: 1px solid rgba(59, 130, 246, 0.1) !important;
}

.resource-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  font-size: 12px;
  padding: 2px 6px;
  background: rgba(0, 0, 0, 0.6);
  color: #fff;
  border-radius: 4px;
}

.resource-content {
  padding: 16px;
}

.resource-actions {
  margin-top: 12px;
  display: flex;
  justify-content: center;
}

.resource-title {
  margin: 0 0 8px 0;
  font-size: clamp(14px, 3vw, 18px);
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.resource-desc {
  font-size: clamp(11px, 2vw, 14px);
  color: var(--text-secondary);
  margin: 0 0 2px 0;
  line-height: 1.4;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 50%;
}

.resource-meta {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.meta-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-muted);
}

.resource-stats {
  display: flex;
  gap: 16px;
  margin-bottom: 12px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.resource-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}



/* 公告区 */
.notice-card {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 16px;
  padding: 18px 20px;
  box-shadow: var(--shadow-light);
}

/* 浅色主题下的公告卡片 */
:root:not(.dark) .notice-card {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.1) !important;
}

.notice-title {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.notice-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.notice-list li {
  padding: 10px 0;
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.6;
  border-bottom: 1px solid var(--border-color-light);
}

.notice-list li:last-child {
  border-bottom: none;
}

.empty-notice {
  padding: 20px 0;
  text-align: center;
  color: var(--text-muted);
  font-size: 14px;
}

.notice-item {
  transition: all 0.2s ease;
  cursor: default;
}

.notice-item:hover {
  background-color: rgba(255, 255, 255, 0.05);
  padding-left: 8px;
}

/* 上传对话框 */
.upload-dialog :deep(.el-dialog) {
  border-radius: 12px;
}

/* 对话框头部背景透明 */
:deep(.el-dialog__header) {
  background-color: transparent !important;
}

.tags-container {
  margin-top: 8px;
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .content-wrapper {
    margin: 16px;
    padding: 16px;
  }
  
  .main-content {
    padding: 16px 0;
  }
  
  .unified-content {
    flex-direction: column;
    gap: 16px;
  }
  
  .sidebar {
    width: 100%;
    order: 2; /* 移动端将侧边栏移到主内容下方 */
  }
  
  .main-area {
    order: 1; /* 移动端主内容在上方 */
  }
  
  .notice-card {
    position: static;
  }
  
  /* 优化中等屏幕的网格 */
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .header-content {
    flex-wrap: wrap;
    gap: 12px;
    justify-content: center;
  }
  
  .search {
    order: 3;
    width: 100%;
    margin-top: 12px;
  }
  

  
  /* 分类卡片移动端优化 */
  .category-card {
    position: static;
    margin-top: 16px;
  }
  
  .category-list {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }
  
    .category-item {
    padding: 10px 12px;
    font-size: 13px;
  }
  
  /* 视图切换移动端优化 */
  .section-actions {
    flex-direction: column;
    gap: 12px;
    align-items: stretch;
  }
  
  .view-switcher {
    order: 1;
    justify-content: center;
  }
  
  .resource-count {
    order: 2;
    text-align: center;
  }
  
  /* 列表视图移动端 */
  .resource-list-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .list-content {
    width: 100%;
  }
  
  .list-meta {
    flex-wrap: wrap;
  }
  
  /* 卡片视图移动端 */
  .resources-cards {
    flex-direction: column;
  }
  
  .resource-card-item {
    width: 100%;
    min-width: auto;
  }
  
  .card-header {
    padding: 16px;
  }
  
  .card-meta {
    flex-direction: column;
    align-items: flex-start;
    gap: 4px;
  }
  
  .stats-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }
  
  /* 已在上面的资源网格断点中处理 */
}

@media (max-width: 480px) {
  .header-content,
  .main-content {
    padding-left: 12px;
    padding-right: 12px;
  }
  
  .stat-card {
    padding: 14px;
    gap: 12px;
  }
  
  .resource-content {
    padding: 14px;
  }
  
  .el-button {
    min-height: 44px; /* 确保触摸友好 */
  }
  
  .user-info {
    padding: 6px 10px;
    gap: 6px;
  }
  
  .user-avatar {
    width: 28px !important;
    height: 28px !important;
  }
}

/* 页脚样式 */
.footer {
  background: transparent;
  color: var(--text-primary);
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  margin-top: 40px;
  position: relative;
  overflow: hidden;
  border-radius: 0 0 12px 12px;
  transition: all var(--transition-base);
}

/* 浅色主题下的页脚 */
:root:not(.dark) .footer {
  background: transparent !important;
  border-top: 1px solid rgba(59, 130, 246, 0.15) !important;
}

.footer::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--border-color), transparent);
}

.footer-content {
  max-width: min(1400px, 95vw);
  margin: 0 auto;
  padding: clamp(40px, 8vw, 80px) clamp(16px, 4vw, 32px) clamp(16px, 3vw, 32px);
}

.footer-sections {
  display: grid;
  grid-template-columns: minmax(300px, 2fr) repeat(3, minmax(200px, 1fr));
  gap: clamp(24px, 5vw, 48px);
  margin-bottom: 48px;
  width: 100%;
}

.footer-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.footer-logo {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.footer-logo-icon {
  font-size: 32px;
}

.footer-logo-text h3 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--text-primary);
}

.footer-logo-text p {
  margin: 4px 0 0 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.footer-desc {
  font-size: 14px;
  line-height: 1.6;
  color: var(--text-secondary);
  margin: 0 0 24px 0;
}

.footer-stats {
  display: flex;
  gap: 24px;
}

.footer-stats .stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
    padding: 16px;
  background: transparent;
  backdrop-filter: none;
  border: none;
  border-radius: 12px;
  min-width: 80px;
  transition: all 0.3s ease;
}

.footer-stats .stat-item:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateY(-2px);
}

.footer-stats .stat-number {
  font-size: 18px;
  font-weight: 700;
  color: var(--brand-color);
  margin-bottom: 4px;
}

.footer-stats .stat-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.footer-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 16px 0;
  padding-bottom: 8px;
  border-bottom: 2px solid var(--brand-color);
  position: relative;
}

.footer-title::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  width: 30px;
  height: 2px;
  background: var(--brand-color-light);
}

.footer-links {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.footer-links li a {
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 14px;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  padding: 4px 0;
  position: relative;
}

.footer-links li a:hover {
  color: var(--brand-color);
  padding-left: 8px;
}

.footer-links li a::before {
  content: '→';
  position: absolute;
  left: -20px;
  opacity: 0;
  transition: all 0.3s ease;
}

.footer-links li a:hover::before {
  opacity: 1;
  left: -16px;
}

.footer-contact {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.contact-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: var(--text-secondary);
}

.contact-item .el-icon {
  color: var(--brand-color);
  font-size: 16px;
}

.social-links {
  display: flex;
  gap: 12px;
  margin-top: 16px;
}

.social-link {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: transparent;
  backdrop-filter: none;
  border: none;
  border-radius: 50%;
  text-decoration: none;
  font-size: 18px;
  transition: all 0.3s ease;
}

.social-link:hover {
  background: rgba(59, 130, 246, 0.8);
  border-color: rgba(59, 130, 246, 0.5);
  transform: translateY(-3px) scale(1.1);
  box-shadow: 0 8px 25px rgba(59, 130, 246, 0.3);
}

.footer-bottom {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 24px;
  border-top: 1px solid var(--border-color);
}

.footer-copyright p {
  margin: 0;
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.footer-legal {
  display: flex;
  align-items: center;
  gap: 12px;
}

.footer-legal a {
  color: var(--text-secondary);
  text-decoration: none;
  font-size: 14px;
  transition: color 0.3s ease;
}

.footer-legal a:hover {
  color: var(--brand-color);
}

.footer-legal .divider {
  color: var(--text-muted);
}

/* 页脚响应式设计 */
@media (max-width: 1024px) {
  .footer-sections {
    grid-template-columns: 1fr 1fr;
    gap: 32px;
  }
}

@media (max-width: 768px) {
  .footer-content {
    padding: 40px 16px 16px;
  }
  
  .footer-sections {
    grid-template-columns: 1fr;
    gap: 24px;
    margin-bottom: 32px;
  }
  
  .footer-stats {
    justify-content: center;
  }
  
  .footer-bottom {
    flex-direction: column;
    gap: 16px;
    text-align: center;
  }
}

@media (max-width: 480px) {
  .footer-stats {
    flex-wrap: wrap;
    gap: 12px;
  }
  
  .footer-stats .stat-item {
    min-width: 70px;
    padding: 12px;
  }
  
  .social-links {
    justify-content: center;
  }
}

/* 让 el-input 输入框背景透明 */
:deep(.el-input__wrapper) input {
  background-color: transparent !important;
}

/* 浅色主题下的Element Plus组件亚克力效果 */
:root:not(.dark) :deep(.el-input__wrapper) {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(15px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 2px 10px rgba(59, 130, 246, 0.1) !important;
}

:root:not(.dark) :deep(.el-select__wrapper) {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(15px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 2px 10px rgba(59, 130, 246, 0.1) !important;
}

:root:not(.dark) :deep(.el-button--primary) {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.9), rgba(79, 70, 229, 0.9)) !important;
  backdrop-filter: blur(15px) !important;
  border: none !important;
  box-shadow: 0 4px 15px rgba(59, 130, 246, 0.3) !important;
}



/* 主题切换按钮图标居中 */
:deep(.theme-btn) {
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
}

:deep(.theme-icon) {
  display: flex !important;
  align-items: center !important;
  justify-content: center !important;
}

/* 浅色主题下增强页面整体亚克力效果 */
:root:not(.dark) .home-container {
  background: transparent !important;
  backdrop-filter: blur(5px) !important;
}

/* 浅色主题下增强主内容区域 */
:root:not(.dark) .main {
  background: transparent !important;
  backdrop-filter: blur(3px) !important;
}

/* 浅色主题下增强空状态卡片 */
:root:not(.dark) .empty-state {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.1) !important;
}

/* 超宽屏幕页脚优化 */
@media (min-width: 1600px) {
  .footer-sections {
    grid-template-columns: minmax(350px, 2fr) repeat(3, minmax(220px, 1fr));
    gap: 56px;
  }
}

/* 中等屏幕页脚优化 */
@media (min-width: 900px) and (max-width: 1199px) {
  .footer-sections {
    grid-template-columns: 1fr 1fr;
    gap: 32px;
  }
}

/* 小屏幕页脚优化 */
@media (max-width: 899px) {
  .footer-sections {
    grid-template-columns: 1fr;
    gap: 24px;
    margin-bottom: 32px;
  }
}

/* 增强移动端响应式 */
@media (max-width: 640px) {
  .header-content {
    gap: 16px;
    padding: 0 16px;
  }
  
  .actions {
    gap: 12px;
  }
  
  .user-name {
    display: none; /* 小屏幕隐藏用户名，只显示头像 */
  }
  

  
  .main {
    padding: 20px 0;
  }
  
  .stat-card {
    padding: 16px 12px;
  }
  
  .resource-card {
    margin-bottom: 8px;
  }
  
  .resource-content {
    padding: 12px;
  }
  
  /* 精选资源移动端优化 */
  .featured-grid {
    grid-template-columns: 1fr;
    gap: 16px;
  }
  
  .section-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .section-actions {
    width: 100%;
    justify-content: space-between;
  }
  
  .section-subtitle {
    margin-left: 0;
  }
}

/* 超小屏幕优化 */
@media (max-width: 360px) {
  .logo-text {
    display: none; /* 极小屏幕隐藏Logo文字 */
  }
  
  .search {
    max-width: 200px;
  }
  
  .content-right {
    min-width: 100%;
  }
  
  .footer-stats {
    flex-direction: column;
    gap: 12px;
  }
  
  .footer-stats .stat-item {
    width: 100%;
    min-width: auto;
  }
  
  /* 轮播在极小屏幕的优化 */
  .carousel-item {
    padding: 8px;
  }
  
  .featured-card {
    border-radius: 12px;
  }
  
  .featured-content {
    padding: 12px;
  }
}

/* 页面区块标题样式 */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 2px solid rgba(255, 255, 255, 0.1);
}

.section-title {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 0;
  font-size: clamp(20px, 4vw, 28px);
  font-weight: 600;
  color: var(--text-primary);
}

.section-subtitle {
  margin: 8px 0 0 44px;
  font-size: clamp(14px, 2.5vw, 16px);
  color: var(--text-secondary);
  font-weight: 400;
}

.section-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.resource-count {
  font-size: 14px;
  color: var(--text-secondary);
  font-weight: 500;
}

/* 精选资源轮播 */
.featured-resources {
  margin-bottom: 40px;
}

.resource-carousel {
  margin-top: 16px;
}

/* 完全隐藏轮播指示器 */
:deep(.el-carousel__indicators) {
  display: none !important;
}

:deep(.el-carousel__indicator) {
  display: none !important;
}

.carousel-item {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 16px;
  animation: fadeInSlide 0.8s ease-out;
}

/* 轮播项淡入滑动动画 */
@keyframes fadeInSlide {
  0% {
    opacity: 0;
    transform: translateX(30px) scale(0.95);
  }
  100% {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

.featured-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  width: 100%;
  max-width: 900px;
}

/* 为每个卡片添加不同的动画延迟 */
.featured-card:nth-child(1) {
  animation-delay: 0s;
}

.featured-card:nth-child(2) {
  animation-delay: 0.2s;
}

.featured-card:nth-child(3) {
  animation-delay: 0.4s;
}

.featured-card {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(15px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 16px;
  overflow: hidden;
  transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  cursor: pointer;
  box-shadow: var(--shadow-light);
  animation: cardFloat 6s ease-in-out infinite;
}

.featured-card:hover {
  transform: translateY(-12px) scale(1.05) rotateY(5deg);
  box-shadow: var(--shadow-heavy);
  border-color: var(--brand-color);
  animation-play-state: paused; /* 悬停时暂停浮动动画 */
}

.featured-cover {
  position: relative;
  height: 120px;
  background: linear-gradient(135deg, var(--brand-color-light), var(--brand-color));
  display: flex;
  align-items: center;
  justify-content: center;
}

.featured-icon {
  color: rgba(255, 255, 255, 0.9);
  animation: iconPulse 3s ease-in-out infinite;
  transition: transform 0.3s ease;
}

.featured-card:hover .featured-icon {
  transform: scale(1.2) rotate(10deg);
}

.featured-badge {
  position: absolute;
  top: 12px;
  right: 12px;
  background: rgba(0, 0, 0, 0.7);
  color: #fff;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 500;
  transform: translateX(100%);
  animation: slideInBadge 0.8s ease-out 0.5s forwards;
}

.featured-card:hover .featured-badge {
  background: var(--brand-color);
  transform: translateX(0) scale(1.1);
}

.featured-content {
  padding: 16px;
}

.featured-title {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.featured-desc {
  margin: 0 0 12px 0;
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.featured-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  color: var(--text-muted);
}

.meta-author,
.meta-downloads {
  display: flex;
  align-items: center;
  gap: 4px;
}

.featured-empty {
  text-align: center;
  padding: 60px 20px;
  background: rgba(255, 255, 255, 0.08);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  margin-top: 16px;
}

.featured-empty .empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.featured-empty h3 {
  margin: 0 0 8px 0;
  font-size: 18px;
  color: var(--text-primary);
}

.featured-empty p {
  margin: 0 0 20px 0;
  color: var(--text-secondary);
}

/* 浅色主题下的精选卡片 */
:root:not(.dark) .featured-card {
  background: rgba(255, 255, 255, 0.4) !important;
  backdrop-filter: blur(25px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.1) !important;
}

:root:not(.dark) .featured-empty {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
}

.category-card {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 16px;
  padding: 16px 18px;
  box-shadow: var(--shadow-light);
}

/* 浅色主题下的分类卡片 */
:root:not(.dark) .category-card {
  background: rgba(255, 255, 255, 0.3) !important;
  backdrop-filter: blur(20px) !important;
  border: 1px solid rgba(59, 130, 246, 0.2) !important;
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.1) !important;
}

.category-title {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 8px;
}

.category-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.category-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  justify-content: space-between;
  transition: all 0.2s ease;
  cursor: pointer;
}

.category-item:hover {
  background-color: rgba(255, 255, 255, 0.15);
  transform: translateX(4px);
}

.category-item.active {
  background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(79, 70, 229, 0.15));
  color: var(--brand-color);
  font-weight: 600;
  border-left: 3px solid var(--brand-color);
}

.category-item-content {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.category-count {
  font-size: 12px;
  color: var(--text-secondary);
  background: rgba(255, 255, 255, 0.1);
  padding: 2px 6px;
  border-radius: 10px;
  min-width: 20px;
  text-align: center;
}

.view-switcher {
  display: flex;
  gap: 16px;
}

.view-switcher .el-button {
  flex: 1;
  min-width: 36px;
}

.resources-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.resource-list-item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: rgba(255, 255, 255, 0.18);
  backdrop-filter: blur(15px);
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  cursor: pointer;
}

.resource-list-item:hover {
  background: rgba(255, 255, 255, 0.25);
  border-color: var(--brand-color);
  transform: translateX(8px) translateY(-2px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.15);
}

.list-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, var(--brand-color), var(--brand-color-light));
  backdrop-filter: blur(10px);
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.list-content {
  flex: 1;
}

.list-main {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.list-title {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-primary);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.list-desc {
  font-size: 14px;
  color: var(--text-secondary);
}

.list-meta {
  display: flex;
  gap: 8px;
}

.list-category,
.list-author,
.list-date {
  font-size: 12px;
  color: var(--text-muted);
}

.list-actions {
  display: flex;
  gap: 16px;
}

.resources-cards {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.resource-card-item {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  overflow: hidden;
  box-shadow: var(--shadow-light);
  transition: all 0.2s ease;
  cursor: pointer;
  width: calc(33.33% - 16px);
  min-width: 280px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 12px;
  background: rgba(255, 255, 255, 0.05);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.card-icon {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(5px);
  border-radius: 8px;
}

.card-title-area {
  flex: 1;
}

.card-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.card-category {
  font-size: 12px;
  color: var(--text-muted);
}

.card-desc {
  padding: 12px;
  font-size: 14px;
  color: var(--text-secondary);
}

.card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: rgba(255, 255, 255, 0.05);
}

.card-meta {
  display: flex;
  gap: 8px;
}

.meta-row {
  display: flex;
  align-items: center;
  gap: 4px;
}

.meta-row .el-icon {
  color: var(--brand-color);
  font-size: 16px;
}

.meta-row span {
  font-size: 12px;
  color: var(--text-secondary);
}

.card-actions {
  display: flex;
  gap: 16px;
}

.card-actions .el-button {
  flex: 1;
}

.advanced-filters {
  margin-bottom: 20px;
}

.filter-row {
  display: flex;
  gap: 16px;
  align-items: center;
}

.filter-group {
  flex: 1;
}

.filter-label {
  font-weight: 500;
  margin-bottom: 8px;
}

.filter-input {
  width: 100%;
}

.filter-actions {
  display: flex;
  gap: 16px;
}

.advanced-panel {
  padding: 16px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  box-shadow: var(--shadow-light);
}

.advanced-row {
  display: flex;
  gap: 16px;
}

/* 高级筛选样式 */
.advanced-filters {
  margin-bottom: 20px;
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(15px);
  border: 2px solid rgba(255, 255, 255, 0.25);
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  transition: all var(--transition-base);
}

/* 浅色主题下的筛选区域 */
:root:not(.dark) .advanced-filters {
  background: rgba(255, 255, 255, 0.6) !important;
  backdrop-filter: blur(25px) !important;
  border: 2px solid rgba(59, 130, 246, 0.4) !important;
  box-shadow: 0 8px 32px rgba(59, 130, 246, 0.2) !important;
}

.filter-row {
  display: flex;
  gap: 16px;
  align-items: center;
  flex-wrap: wrap;
}

.filter-group {
  flex: 1;
  min-width: 120px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.filter-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}

.filter-input {
  width: 100%;
}

.filter-actions {
  display: flex;
  gap: 16px;
  flex-shrink: 0;
  align-items: flex-end;
  padding-bottom: 4px;
}

.advanced-panel {
  padding: 16px;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  box-shadow: var(--shadow-light);
  margin-top: 12px;
}

/* 浅色主题下的高级面板 */
:root:not(.dark) .advanced-panel {
  background: rgba(255, 255, 255, 0.2) !important;
  backdrop-filter: blur(15px) !important;
  border: 1px solid rgba(59, 130, 246, 0.15) !important;
  box-shadow: 0 2px 10px rgba(59, 130, 246, 0.05) !important;
}

.advanced-row {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

/* 筛选区域响应式 */
@media (max-width: 1024px) {
  .filter-row {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }
  
  .filter-group {
    min-width: auto;
  }
  
  .filter-actions {
    justify-content: center;
  }
}

@media (max-width: 768px) {
  .advanced-filters {
    padding: 12px;
  }
  
  .filter-row {
    gap: 8px;
  }
  
  .advanced-row {
    flex-direction: column;
    gap: 8px;
  }
  
  .filter-actions {
    flex-direction: column;
    gap: 8px;
  }
}
</style> 