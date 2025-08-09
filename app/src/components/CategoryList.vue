<template>
  <div class="category-list">
    <van-skeleton title :row="8" :loading="loading" v-if="loading" />
    <template v-else>
      <van-grid class="category-grid" :column-num="gridColumnNum" :border="false" :gutter="10">
        <van-grid-item 
          v-for="category in categories" 
          :key="category.id" 
          @click="onCategoryClick(category)"
        >
          <div class="category-item">
            <svg class="svg-icon" aria-hidden="true">
              <use :href="getCategoryIcon(category)"></use>
            </svg>
            <span class="category-name">{{ category.name }}</span>
          </div>
        </van-grid-item>
      </van-grid>
    </template>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import spriteContent from '@/assets/category-icons.svg?raw';

// 在页面挂载时将 sprite 注入（仅注入一次）
let spriteInjected = false;
const injectSprite = () => {
  if (spriteInjected) return;
  const el = document.createElement('div');
  el.style.display = 'none';
  el.innerHTML = spriteContent;
  document.body.prepend(el);
  spriteInjected = true;
};

// 定义属性
const props = defineProps({
  categories: {
    type: Array,
    default: () => []
  },
  loading: {
    type: Boolean,
    default: false
  },
  columnNum: {
    type: Number,
    default: 4
  }
});

// 事件
const emit = defineEmits(['category-click']);

const router = useRouter();

onMounted(() => injectSprite());

const gridColumnNum = computed(() => {
  return props.columnNum || 4;
});

// 获取分类图标（动态选择：后端字段优先，其次哈希到内置集合）
const getCategoryIcon = (category) => {
  // 后端若提供 icon 或 icon_name，直接使用（如 'knot' -> '#icon-knot'）
  const iconKey = category.icon || category.icon_name;
  if (iconKey && typeof iconKey === 'string') {
    return `#icon-${iconKey.replace(/^#icon-/, '')}`;
  }

  // 退化策略：根据名称/ID 计算哈希，稳定映射到内置图标集合
  const icons = ['#icon-knot', '#icon-tools', '#icon-video', '#icon-article', '#icon-advanced', '#icon-safety', '#icon-community'];
  const text = `${category.name || ''}|${category.id ?? ''}`;
  let hash = 0;
  for (let i = 0; i < text.length; i += 1) {
    hash = ((hash << 5) - hash) + text.charCodeAt(i);
    hash |= 0; // 转 32 位
  }
  const idx = Math.abs(hash) % icons.length;
  return icons[idx];
};

// 点击分类项
const onCategoryClick = (category) => {
  emit('category-click', category);
  router.push(`/category/${category.id}`);
};
</script>

<style scoped>
.category-list { margin: 12px 0; }
.category-grid :deep(.van-grid-item__content) { background: transparent; padding: 0 !important; }
.category-item { display: flex; flex-direction: column; align-items: center; padding: 12px; background: #fff; border-radius: 12px; box-shadow: 0 1px 4px rgba(0,0,0,0.04); }
.category-name { font-size: 13px; color: var(--text-color); margin-top: 8px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 100%; }
.svg-icon { width: 40px; height: 40px; border-radius: 8px; background: #f7f8fa; }
</style> 