<template>
  <van-popup v-model:show="visible" position="bottom" round :style="{ height: '86vh' }">
    <div class="sheet">
      <div class="sheet-header">
        <div class="title">编辑资源</div>
        <div class="actions">
          <van-button size="small" @click="close">取消</van-button>
          <van-button size="small" type="primary" :loading="saving" @click="onSubmit">保存</van-button>
        </div>
      </div>

      <van-form ref="formRef">
        <van-cell-group inset>
          <van-field
            v-model="form.name"
            label="名称"
            placeholder="请输入资源名称"
            required
          />
          <van-field
            v-model="form.description"
            type="textarea"
            label="描述"
            placeholder="请输入资源描述"
            autosize
            rows="3"
          />


          <!-- 可选：直接上传替换文件（将覆盖直链） -->
          <van-field label="替换文件">
            <template #input>
              <van-uploader
                v-model="uploadFileList"
                :after-read="afterRead"
                :max-count="1"
                accept="*"
                upload-text="选择文件"
                :max-size="100 * 1024 * 1024"
                @oversize="onOversize"
                @delete="onDelete"
              />
            </template>
            <template #label>
              <div>替换文件<span class="label-desc">（可选）</span></div>
            </template>
          </van-field>

          <!-- 分类选择器（名称展示，保存写入 category_id） -->
          <van-field
            is-link
            readonly
            label="分类"
            :value="selectedCategoryName"
            placeholder="请选择分类"
            @click="showCategoryPicker = true"
          />

          <van-popup v-model:show="showCategoryPicker" position="bottom">
            <van-picker
              :columns="categoryColumns"
              @confirm="onCategoryConfirm"
              @cancel="showCategoryPicker = false"
            />
          </van-popup>

          <!-- 标签多选（热门标签 + 自定义） -->
          <van-field label="标签" placeholder="选择或输入，用逗号分隔">
            <template #input>
              <div class="tag-box">
                <van-checkbox-group v-model="selectedTags" direction="horizontal">
                  <van-checkbox v-for="t in hotTags" :key="t" :name="t">{{ t }}</van-checkbox>
                </van-checkbox-group>
                <van-field v-model="customTags" placeholder="自定义标签，逗号分隔" />
              </div>
            </template>
          </van-field>
        </van-cell-group>
      </van-form>
    </div>
  </van-popup>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { showToast } from 'vant';
import { resourceApi, categoryApi, tagApi } from '../api/resource';

const emit = defineEmits(['update:modelValue', 'submit']);

const props = defineProps({
  modelValue: { type: Boolean, default: false },
  resource: { type: Object, default: null },
  saving: { type: Boolean, default: false },
});

const visible = ref(false);
const formRef = ref(null);
const form = ref({ name: '', description: '', category_id: null, tags: [] });

const downloading = ref(false);

// 新增：上传文件（可选）
const uploadFileList = ref([]);
const selectedFile = ref(null);


// 分类
const showCategoryPicker = ref(false);
const categories = ref([]);
// 简化：单列直接数组（对象项），避免外层 { values } 结构
const categoryColumns = computed(() => categories.value.map(c => ({ text: c.name, value: c.id })));
const selectedCategoryName = computed(() => {
  const id = form.value.category_id;
  const found = categories.value.find(c => Number(c.id) === Number(id));
  return found ? found.name : '';
});

// 标签
const hotTags = ref([]);
const selectedTags = ref([]);
const customTags = ref('');

watch(
  () => props.modelValue,
  async (val) => {
    visible.value = val;
    if (val) {
      if (props.resource) {
        form.value = {
          name: props.resource.name || '',
          description: props.resource.description || '',
          category_id: props.resource.category_id ?? null,
          tags: Array.isArray(props.resource.tags) ? props.resource.tags : [],
        };
        selectedTags.value = [...form.value.tags];
      }
      // 预加载分类/标签（无论是否有 resource 都加载）
      try { await Promise.all([fetchCategories(), fetchHotTags()]); } catch (e) { /* ignore */ }
      // 清理上传状态
      uploadFileList.value = [];
      selectedFile.value = null;
    }
  },
  { immediate: true }
);

const close = () => {
  emit('update:modelValue', false);
};


const fetchCategories = async () => {
  try {
    const res = await categoryApi.getAllCategories();
    // 兼容多种返回结构：{ data: { list: [] } } 或 { data: [] }
    const raw = res?.data;
    const list = Array.isArray(raw) ? raw : (raw?.list || raw?.data?.list || []);
    categories.value = Array.isArray(list) ? list : [];
  } catch (e) {
    categories.value = [];
  }
};

const fetchHotTags = async () => {
  try {
    const res = await tagApi.getHotTags(20);
    const list = res.data || res.data?.list;
    hotTags.value = Array.isArray(list)
      ? list.map(t => (typeof t === 'string' ? t : (t.name || ''))).filter(Boolean)
      : [];
  } catch (e) {
    hotTags.value = [];
  }
};

const onCategoryConfirm = (arg1, arg2) => {
  let picked = null;
  // 新版事件对象：{ selectedOptions: [...] }
  if (arg1 && typeof arg1 === 'object' && 'selectedOptions' in arg1) {
    const arr = arg1.selectedOptions;
    picked = Array.isArray(arr) ? arr[0] : arr;
  } else if (Array.isArray(arg1)) {
    // 旧版：selectedValues 或 selectedOptions
    const v = arg1[0];
    if (v && typeof v === 'object' && ('value' in v || 'text' in v)) {
      picked = v;
    } else {
      // v 可能是名称字符串
      const options = categoryColumns.value || [];
      picked = options.find(o => o.text === v || o.value === v) || null;
    }
  } else if (arg1 && typeof arg1 === 'object' && ('value' in arg1 || 'text' in arg1)) {
    picked = arg1;
  }

  if (picked) {
    if (picked.value != null) {
      form.value.category_id = Number(picked.value);
    } else if (picked.text) {
      const found = categories.value.find(c => c.name === picked.text);
      if (found) form.value.category_id = found.id;
    }
  }
  showCategoryPicker.value = false;
};

// 选择文件
const afterRead = (file) => {
  selectedFile.value = file.file;
};
const onOversize = () => showToast('文件大小不能超过100MB');
const onDelete = () => { selectedFile.value = null; };

// 调整：提交前进行表单校验
const onSubmit = async () => {
  try {
    await formRef.value?.validate();
  } catch (e) {
    return;
  }
  // 合成标签：热门选择 + 自定义
  const custom = customTags.value
    .split(',')
    .map(s => s.trim())
    .filter(Boolean);
  const tags = Array.from(new Set([...(selectedTags.value || []), ...custom]));
  // 将选中文件一起回传
  emit('submit', { ...form.value, tags, __file: selectedFile.value || null });
};
</script>

<style scoped>
.sheet { padding: 12px; }
.sheet-header { display: flex; justify-content: space-between; align-items: center; padding: 4px 4px 12px; }
.title { font-size: 16px; font-weight: 500; }
.actions { display: flex; gap: 8px; }
.flex { display: flex; align-items: center; }
.gap-8 { gap: 8px; }
.tag-box { display: flex; flex-direction: column; gap: 8px; }
.label-desc { color: var(--text-color-lighter); font-size: 12px; margin-left: 4px; }
</style>