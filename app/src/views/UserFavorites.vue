<template>
  <div class="user-favorites-page">
    <van-nav-bar title="我的点赞" left-arrow @click-left="onBack" fixed />
    <div class="content" :style="{ paddingTop: '46px' }">
      <van-tabs v-model:active="activeTab" sticky>
        <van-tab title="总览">
          <div class="cards">
            <div class="kpi-card">
              <div class="kpi-num">{{ totalLikes }}</div>
              <div class="kpi-label">累计点赞（资源+帖子）</div>
            </div>
            <div class="kpi-card">
              <div class="kpi-num">{{ totalViews }}</div>
              <div class="kpi-label">累计浏览（帖子）</div>
            </div>
          </div>
          <div class="chart" ref="likesChartRef"></div>
          <div class="chart" ref="viewsChartRef"></div>
        </van-tab>
        <van-tab title="资源">
          <van-list v-model:loading="loadingPkg" :finished="finishedPkg" finished-text="没有更多了" @load="loadPackages">
            <div class="favorite-item" v-for="item in likesPackages" :key="item.id" @click="goToResource(item.id)">
              <div class="resource">
                <div class="resource-cover">
                  <van-image width="80" height="80" radius="4" fit="cover" :src="item.cover_url || '/favicon.ico'" />
                </div>
                <div class="resource-info">
                  <div class="resource-title">{{ item.name }}</div>
                  <div class="resource-desc ellipsis-2">{{ item.description || '暂无描述' }}</div>
                  <div class="resource-meta">
                    <span>点赞 {{ item.like_count || 0 }}</span>
                    <span>下载 {{ item.download_count || 0 }}</span>
                  </div>
                </div>
              </div>
            </div>
          </van-list>
        </van-tab>
        <van-tab title="帖子">
          <van-list v-model:loading="loadingPost" :finished="finishedPost" finished-text="没有更多了" @load="loadPosts">
            <div class="favorite-item" v-for="p in likesPosts" :key="p.id" @click="goToPost(p.id)">
              <div class="resource">
                <div class="resource-info">
                  <div class="resource-title">{{ p.title }}</div>
                  <div class="resource-desc ellipsis-2">{{ p.content }}</div>
                  <div class="resource-meta">
                    <span>浏览 {{ p.view_count || 0 }}</span>
                    <span>点赞 {{ p.like_count || 0 }}</span>
                  </div>
                </div>
              </div>
            </div>
          </van-list>
        </van-tab>
      </van-tabs>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import { useRouter } from 'vue-router';
import { showToast } from 'vant';
import { useUserStore } from '../store/user';
import { get } from '../utils/request';
import { postApi } from '../api/post';

const router = useRouter();
const userStore = useUserStore();

const activeTab = ref(0);

// 概览数据
const totalLikes = ref(0);
const totalViews = ref(0);
const likesChartRef = ref(null);
const viewsChartRef = ref(null);
let likesChart, viewsChart;

// 列表数据
const likesPackages = ref([]);
const likesPosts = ref([]);
const loadingPkg = ref(false), finishedPkg = ref(false), pkgPage = ref(1);
const loadingPost = ref(false), finishedPost = ref(false), postPage = ref(1);
const pageSize = 10;

// 新增：用户行为数据（用于真实统计）
const actionLikesCount = ref(0);
const actionViewsCount = ref(0);
const actionLikeByType = ref({ Package: 0, Post: 0 });

async function fetchUserActionStats() {
  try {
    // 优先调用专用统计接口
    const stats = await get('/users/my-likes/stats', {});
    if (stats && stats.code === 0) {
      actionLikesCount.value = stats.data?.like_total || 0;
      actionViewsCount.value = stats.data?.view_total || 0;
      const byType = stats.data?.like_by_type || {};
      actionLikeByType.value = { Package: byType.package || 0, Post: byType.post || 0 };
      return;
    }
  } catch {}
  try {
    // 回退：拉取当前用户的点赞/浏览行为记录
    const [likeRes, viewRes] = await Promise.all([
      get('/user-actions', { page: 1, page_size: 1000, user_id: userStore.userId, action_type: 'Like' }),
      get('/user-actions', { page: 1, page_size: 1000, user_id: userStore.userId, action_type: 'View' }),
    ]);
    const likeActs = likeRes?.data?.actions || [];
    const viewActs = viewRes?.data?.actions || [];
    actionLikesCount.value = likeRes?.data?.total || likeActs.length;
    actionViewsCount.value = viewRes?.data?.total || viewActs.length;
    const pkgLikes = likeActs.filter(a => (a.target_type || '').toLowerCase() === 'package').length;
    const postLikes = likeActs.filter(a => (a.target_type || '').toLowerCase() === 'post').length;
    actionLikeByType.value = { Package: pkgLikes, Post: postLikes };
  } catch {}
}

const onBack = () => router.back();
const goToResource = (id) => router.push(`/resource/${id}`);
const goToPost = (id) => router.push(`/post/${id}`);

// 临时数据：后端暂无“我的点赞”专属对象列表，先用热门资源/帖子替代
async function loadPackages() {
  if (loadingPkg.value || finishedPkg.value) return;
  loadingPkg.value = true;
  try {
    // 优先调用“我的点赞列表（资源）”
    const res = await get('/users/my-likes', { page: pkgPage.value, page_size: pageSize, target: 'package' });
    const list = res?.data?.list || [];
    likesPackages.value = likesPackages.value.concat(list.map(x => ({
      id: x.id, name: x.name, description: x.description, like_count: x.like_count, download_count: x.download_count
    })));
    if (list.length < pageSize) finishedPkg.value = true; else pkgPage.value += 1;
  } catch {
    // 回退：使用 packages 列表
    try {
      const res2 = await get('/packages', { page: pkgPage.value, page_size: pageSize, sort: 'like_count_desc' });
      const list2 = res2?.data?.list || [];
      likesPackages.value = likesPackages.value.concat(list2);
      if (list2.length < pageSize) finishedPkg.value = true; else pkgPage.value += 1;
    } catch { showToast('资源加载失败'); finishedPkg.value = true; }
  } finally { loadingPkg.value = false; }
}

async function loadPosts() {
  if (loadingPost.value || finishedPost.value) return;
  loadingPost.value = true;
  try {
    // 优先调用“我的点赞列表（帖子）”
    const res = await get('/users/my-likes', { page: postPage.value, page_size: pageSize, target: 'post' });
    const list = res?.data?.list || [];
    likesPosts.value = likesPosts.value.concat(list.map(p => ({ id: p.id, title: p.title, content: p.content, like_count: p.like_count, view_count: p.view_count })));
    if (list.length < pageSize) finishedPost.value = true; else postPage.value += 1;
  } catch {
    // 回退：使用帖子列表
    try {
      const res2 = await postApi.getPosts({ page: postPage.value, page_size: pageSize });
      const list2 = res2?.data?.list || [];
      likesPosts.value = likesPosts.value.concat(list2);
      if (list2.length < pageSize) finishedPost.value = true; else postPage.value += 1;
    } catch { showToast('帖子加载失败'); finishedPost.value = true; }
  } finally { loadingPost.value = false; }
}

function initCharts() {
  const ensureEcharts = () => new Promise((resolve) => {
    if (window.echarts) return resolve(window.echarts);
    const s = document.createElement('script');
    s.src = 'https://cdn.jsdelivr.net/npm/echarts@5/dist/echarts.min.js';
    s.onload = () => resolve(window.echarts);
    document.body.appendChild(s);
  });
  ensureEcharts().then((echarts) => {
    likesChart = echarts.init(likesChartRef.value);
    viewsChart = echarts.init(viewsChartRef.value);
    renderCharts();
    window.addEventListener('resize', () => { likesChart.resize(); viewsChart.resize(); });
  });
}

function renderCharts() {
  // 使用用户行为真实计数作为 KPI；若为 0 则回退到当前已加载列表的汇总
  const likePkg = actionLikeByType.value.Package || likesPackages.value.reduce((s, x) => s + (x.like_count || 0), 0);
  const likePost = actionLikeByType.value.Post || likesPosts.value.reduce((s, x) => s + (x.like_count || 0), 0);
  const viewTotal = actionViewsCount.value || likesPosts.value.reduce((s, x) => s + (x.view_count || 0), 0);

  totalLikes.value = (actionLikesCount.value || (likePkg + likePost));
  totalViews.value = viewTotal;

  const E = (typeof window !== 'undefined' && window.echarts) ? window.echarts : null;

  if (likesChart) likesChart.setOption({
    backgroundColor: 'transparent',
    title: { text: '点赞构成', left: 'center', top: 4, textStyle: { fontSize: 14, fontWeight: 600, color: '#111827' } },
    tooltip: { trigger: 'item', formatter: '{b}: {c} ({d}%)' },
    legend: { bottom: 4, icon: 'circle', itemWidth: 8, itemHeight: 8, textStyle: { color: '#6b7280' } },
    color: ['#4C6FFF', '#7BD88F'],
    series: [{
      type: 'pie',
      radius: ['52%', '72%'],
      center: ['50%', '54%'],
      avoidLabelOverlap: true,
      label: { show: false },
      labelLine: { show: false },
      itemStyle: { borderColor: '#fff', borderWidth: 2 },
      emphasis: { scale: true, scaleSize: 6 },
      data: [
        { name: '资源点赞', value: likePkg },
        { name: '帖子点赞', value: likePost },
      ],
    }],
    graphic: totalLikes.value ? [{
      type: 'text', left: 'center', top: 'middle',
      style: { text: `${totalLikes.value}\n总点赞`, textAlign: 'center', fill: '#111827', fontSize: 14, fontWeight: 600 }
    }] : [],
  });

  if (viewsChart) viewsChart.setOption({
    backgroundColor: 'transparent',
    title: { text: '帖子浏览', left: 'center', top: 4, textStyle: { fontSize: 14, fontWeight: 600, color: '#111827' } },
    grid: { left: 24, right: 16, top: 44, bottom: 28, containLabel: true },
    tooltip: { trigger: 'axis', axisPointer: { type: 'shadow' } },
    xAxis: {
      type: 'category',
      data: likesPosts.value.map(p => p.title).slice(0, 8),
      axisTick: { show: false },
      axisLine: { lineStyle: { color: '#e5e7eb' } },
      axisLabel: { color: '#6b7280', interval: 0, formatter: v => (v && v.length > 8 ? v.slice(0, 8) + '…' : v) }
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      splitLine: { lineStyle: { color: '#f3f4f6' } },
      axisLabel: { color: '#6b7280' }
    },
    series: [{
      type: 'bar',
      data: likesPosts.value.map(p => p.view_count || 0).slice(0, 8),
      barWidth: 24,
      itemStyle: {
        borderRadius: [8, 8, 0, 0],
        color: E ? new E.graphic.LinearGradient(0, 1, 0, 0, [
          { offset: 0, color: '#4C6FFF33' },
          { offset: 1, color: '#4C6FFF' },
        ]) : '#4C6FFF'
      }
    }]
  });
}

watch([likesPackages, likesPosts], renderCharts, { deep: true });

onMounted(async () => {
  if (!userStore.isLoggedIn) {
    router.replace({ path: '/login', query: { redirect: '/favorites' } });
    return;
  }
  await fetchUserActionStats();
  initCharts();
  loadPackages();
  loadPosts();
});
</script>

<style scoped>
.user-favorites-page { min-height: 100vh; background-color: var(--background-color); }
.content { padding: 12px; }
.cards { display: flex; gap: 12px; }
.kpi-card { flex: 1; background: #fff; border-radius: 8px; padding: 12px; text-align: center; }
.kpi-num { font-size: 20px; font-weight: 700; color: var(--text-color); }
.kpi-label { font-size: 12px; color: var(--text-color-light); }
.chart { height: 260px; background: #fff; border-radius: 12px; margin-top: 12px; box-shadow: 0 1px 2px rgba(0,0,0,0.04); }
.favorite-item { background-color: #fff; border-radius: 8px; padding: 12px; margin: 12px 0; }
.resource { display: flex; }
.resource-cover { margin-right: 12px; }
.resource-info { flex: 1; }
.resource-title { font-size: 16px; font-weight: 500; margin-bottom: 6px; }
.resource-desc { font-size: 14px; color: var(--text-color-light); margin-bottom: 8px; max-height: 40px; overflow: hidden; }
.resource-meta { display: flex; justify-content: space-between; font-size: 12px; color: var(--text-color-lighter); }
</style> 