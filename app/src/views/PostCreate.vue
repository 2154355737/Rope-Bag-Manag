<template>
  <div class="post-create-page">
    <van-nav-bar :title="navTitle" left-arrow @click-left="onBack" fixed />
    <div class="content">
      <van-form @submit.prevent="onSubmit">
        <van-cell-group inset>
          <van-field v-model="title" label="标题" placeholder="请输入标题" :rules="[{ required: true, message: '请输入标题' }]" />
          <van-field v-model="content" rows="6" type="textarea" label="内容" placeholder="请输入内容" :rules="[{ required: true, message: '请输入内容' }]" />
          <van-field v-model="tagInput" label="标签" placeholder="输入标签后回车添加" @keypress.enter.prevent="addTag" />
          <div class="tags" v-if="tags.length">
            <van-tag v-for="t in tags" :key="t" closeable type="primary" plain class="mr8" @close="removeTag(t)">{{ t }}</van-tag>
          </div>
        </van-cell-group>
        <div class="actions">
          <van-button round block type="primary" native-type="submit" :loading="submitting">{{ isEditing ? '保存修改' : '提交' }}</van-button>
        </div>
      </van-form>
      <p class="tip">{{ isEditing ? '修改后将重新进入审核' : '提交后将进入审核，请耐心等待' }}</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { postApi } from '../api/post';
import { showToast } from 'vant';

const router = useRouter();
const route = useRoute();
const editId = computed(() => Number(route.query.id || 0));
const isEditing = computed(() => editId.value > 0);
const navTitle = computed(() => (isEditing.value ? '编辑帖子' : '写帖子'));

const title = ref('');
const content = ref('');
const tagInput = ref('');
const tags = ref([]);
const submitting = ref(false);

const addTag = () => {
  const t = tagInput.value.trim();
  if (!t) return; if (tags.value.includes(t)) return; if (tags.value.length >= 5) return;
  tags.value.push(t); tagInput.value = '';
};
const removeTag = (t) => { tags.value = tags.value.filter(x => x !== t); };
const onBack = () => router.back();

onMounted(async () => {
  if (isEditing.value) {
    try {
      const res = await postApi.getPostDetail(editId.value);
      const p = res.data || {};
      title.value = p.title || '';
      content.value = p.content || '';
      tags.value = Array.isArray(p.tags) ? p.tags : [];
    } catch (e) {
      showToast('加载帖子失败');
    }
  }
});

const onSubmit = async () => {
  // 防止重复提交
  if (submitting.value) {
    return;
  }
  
  submitting.value = true;
  try {
    if (isEditing.value) {
      const res = await postApi.updatePost(editId.value, { title: title.value, content: content.value, tags: tags.value });
      if (res.code === 0) {
        showToast('已更新，等待审核');
        router.replace('/my-posts');
      } else {
        showToast(res.message || '更新失败');
      }
    } else {
      const res = await postApi.createPost({ title: title.value, content: content.value, tags: tags.value });
      if (res.code === 0) {
        showToast('提交成功，等待审核');
        router.replace('/my-posts');
      } else {
        showToast(res.message || '提交失败');
      }
    }
  } catch (e) {
    showToast(isEditing.value ? '更新失败' : '提交失败');
  } finally { submitting.value = false; }
};
</script>

<style scoped>
.content { padding: 12px; }
.actions { margin: 16px; }
.tags { padding: 8px 16px; }
.mr8 { margin-right: 8px; }
.tip { color: var(--text-color-lighter); font-size: 12px; text-align: center; }
</style> 