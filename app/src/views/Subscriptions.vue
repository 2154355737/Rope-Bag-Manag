<template>
  <div class="subs-page">
    <van-nav-bar title="分类订阅" left-arrow @click-left="onBack" fixed />
    <div class="content" :style="{ paddingTop: '46px' }">
      <van-skeleton title :row="8" :loading="loading" />
      <div v-if="!loading">
        <van-cell-group inset>
          <van-cell v-for="c in categories" :key="c.id">
            <template #title>
              <div class="row">
                <div class="name">{{ c.name }}</div>
                <div class="desc">{{ c.description || '—' }}</div>
              </div>
            </template>
            <template #right-icon>
              <van-switch v-model="subsMap[c.id]" size="20" @change="(val)=>onToggle(c, val)" />
            </template>
          </van-cell>
        </van-cell-group>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { showToast } from 'vant';
import { categoryApi } from '@/api/resource';
import { subscriptionApi } from '@/api/subscription';

const router = useRouter();
const onBack = () => router.back();

const categories = ref([]);
const subsMap = ref({});
const loading = ref(false);

const fetchData = async () => {
  loading.value = true;
  try {
    const [catRes, subRes] = await Promise.all([
      categoryApi.getAllCategories(),
      subscriptionApi.getUserSubscriptions()
    ]);
    const rawCats = catRes?.data?.list || catRes?.data || [];
    categories.value = Array.isArray(rawCats) ? rawCats : [];
    subsMap.value = subRes?.data || {};
  } catch (e) {
    showToast('加载失败');
  } finally {
    loading.value = false;
  }
};

const onToggle = async (c, val) => {
  try {
    await subscriptionApi.setSubscription(c.id, !!val);
    showToast(val ? '已订阅' : '已取消');
  } catch (e) {
    showToast('操作失败');
    subsMap.value[c.id] = !val; // 回滚
  }
};

onMounted(fetchData);
</script>

<style scoped>
.subs-page { min-height: 100vh; background: var(--background-color); }
.content { padding: 12px; }
.row { display: flex; flex-direction: column; }
.name { font-weight: 600; color: var(--text-color); }
.desc { font-size: 12px; color: var(--text-color-lighter); margin-top: 2px; }
</style> 