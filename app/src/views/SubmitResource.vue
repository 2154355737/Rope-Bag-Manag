<template>
  <div class="submit-resource-page">
    <!-- 顶部导航栏 -->
    <van-nav-bar
      title="上传资源"
      left-arrow
      @click-left="onBack"
      fixed
    />
    
    <div class="content">
      <!-- 表单 -->
      <van-form @submit="onSubmit">
        <van-cell-group inset>
          <van-field
            v-model="form.name"
            name="name"
            label="资源名称"
            placeholder="请输入资源名称"
            :rules="[{ required: true, message: '请填写资源名称' }]"
          />
          
          <van-field
            v-model="form.description"
            rows="3"
            autosize
            type="textarea"
            name="description"
            label="资源描述"
            placeholder="请输入资源描述"
            :rules="[{ required: true, message: '请填写资源描述' }]"
          />
          
          <van-field
            v-model="form.version"
            name="version"
            label="版本号"
            placeholder="请输入版本号，如1.0.0"
          />
          
          <van-field
            name="category"
            label="资源分类"
            :value="form.category_id"
            :rules="categoryRules"
          >
            <template #input>
              <van-dropdown-menu>
                <van-dropdown-item 
                  v-model="form.category_id" 
                  :options="categoryOptions" 
                  @change="onCategoryChange"
                />
              </van-dropdown-menu>
            </template>
          </van-field>
          
          <van-field
            name="tags"
            label="资源标签"
          >
            <template #input>
              <div class="tag-wrapper">
                <div class="selected-tags" v-if="form.tags.length > 0">
                  <van-tag
                    v-for="tag in form.tags"
                    :key="tag"
                    closeable
                    type="primary"
                    plain
                    class="mr-2 mb-2"
                    @close="removeTag(tag)"
                  >
                    {{ tag }}
                  </van-tag>
                </div>
                <van-field
                  v-model="tagInput"
                  placeholder="输入标签后按回车添加"
                  @keypress.enter.prevent="addTag"
                />
              </div>
            </template>
          </van-field>
          
          <!-- 文件上传组件 -->
          <van-field
            name="file"
            label="资源文件"
            :rules="fileRules"
          >
            <template #input>
              <van-uploader
                v-model="fileList"
                :after-read="afterRead"
                :max-count="1"
                accept="*"
                upload-text="选择文件"
                :max-size="100 * 1024 * 1024"
                @oversize="onOversize"
                @delete="onDelete"
              />
            </template>
          </van-field>
          
          <van-field name="agreement">
            <template #input>
              <van-checkbox v-model="agreedToTerms">
                我已阅读并同意 <span class="link-text" @click="showAgreement">《资源上传协议》</span>
              </van-checkbox>
            </template>
          </van-field>
        </van-cell-group>
        
        <div class="form-actions">
          <van-button 
            round 
            block 
            type="primary" 
            native-type="submit"
            :disabled="!agreedToTerms || submitting"
            :loading="submitting"
          >
            提交资源
          </van-button>
        </div>
      </van-form>
    </div>
    
    <!-- 协议弹窗 -->
    <van-popup
      v-model:show="showAgreementPopup"
      position="bottom"
      round
      style="height: 70%"
    >
      <div class="agreement-content">
        <div class="agreement-header">
          <div class="agreement-title">资源上传协议</div>
          <van-icon name="cross" @click="showAgreementPopup = false" />
        </div>
        
        <div class="agreement-text">
          <h3>资源上传协议</h3>
          <p>请您在上传资源前仔细阅读本协议，上传资源表示您已同意本协议的全部内容。</p>
          
          <h4>一、上传规范</h4>
          <p>1. 您上传的资源必须是您自己创作或拥有合法授权的内容。</p>
          <p>2. 上传内容不得包含任何违法、侵权或不良信息。</p>
          <p>3. 资源描述应当准确、清晰，不得有虚假宣传。</p>
          
          <h4>二、审核机制</h4>
          <p>1. 所有上传的资源需经过平台审核后才能发布。</p>
          <p>2. 审核时间通常为1-3个工作日。</p>
          <p>3. 平台有权拒绝发布不符合规范的资源。</p>
          
          <h4>三、版权声明</h4>
          <p>1. 您保证对上传的资源拥有合法的知识产权或已获得相应授权。</p>
          <p>2. 因侵犯他人知识产权导致的法律责任由上传者自行承担。</p>
          
          <h4>四、收益分配</h4>
          <p>1. 平台可能会对优质资源设置付费下载机制。</p>
          <p>2. 付费资源的收益将按照平台规则进行分配。</p>
          
          <h4>五、免责声明</h4>
          <p>平台不对用户上传的资源内容负责，由上传者自行承担相应的法律责任。</p>
        </div>
        
        <div class="agreement-footer">
          <van-button block type="primary" @click="agreeToTerms">同意并继续</van-button>
        </div>
      </div>
    </van-popup>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';
import { showToast, showDialog } from 'vant';
import { useUserStore } from '../store/user';
import { resourceApi, categoryApi } from '../api/resource';

const router = useRouter();
const userStore = useUserStore();

// 表单数据
const form = ref({
  name: '',
  description: '',
  version: '',
  category_id: '',
  tags: []
});

// 文件上传相关
const fileList = ref([]);
const uploadedFile = ref(null);

// 表单状态
const submitting = ref(false);
const agreedToTerms = ref(false);
const showAgreementPopup = ref(false);
const tagInput = ref('');

// 分类选项
const categories = ref([]);
const categoryOptions = computed(() => {
  return categories.value.map(category => ({
    text: category.name,
    value: Number(category.id)
  }));
});

// 文件校验规则
const fileRules = [
  { validator: () => fileList.value.length > 0, message: '请选择要上传的文件' },
];
// 分类校验规则（自定义读取 form）
const categoryRules = [
  { validator: () => Number(form.value.category_id) > 0, message: '请选择资源分类' },
];

// 返回上一页
const onBack = () => {
  if (form.value.name || form.value.description || fileList.value.length > 0) {
    showDialog({
      title: '提示',
      message: '确定要放弃当前编辑吗？',
      showCancelButton: true,
    }).then((action) => {
      if (action === 'confirm') {
        router.back();
      }
    });
  } else {
    router.back();
  }
};

// 加载分类
const loadCategories = async () => {
  try {
    const res = await categoryApi.getAllCategories();
    categories.value = res.data.list || [];
    
    // 如果有分类，默认选中第一个
    if (categories.value.length > 0) {
      form.value.category_id = Number(categories.value[0].id);
    }
  } catch (error) {
    console.error('获取分类失败', error);
    showToast('获取分类失败');
  }
};

// 分类变化
const onCategoryChange = (value) => {
  form.value.category_id = Number(value);
};

// 添加标签
const addTag = () => {
  const tag = tagInput.value.trim();
  if (!tag) return;
  
  if (form.value.tags.includes(tag)) {
    showToast('标签已存在');
    return;
  }
  
  if (form.value.tags.length >= 5) {
    showToast('最多添加5个标签');
    return;
  }
  
  form.value.tags.push(tag);
  tagInput.value = '';
};

// 移除标签
const removeTag = (tag) => {
  form.value.tags = form.value.tags.filter(item => item !== tag);
};

// 显示协议
const showAgreement = () => {
  showAgreementPopup.value = true;
};

// 同意协议
const agreeToTerms = () => {
  agreedToTerms.value = true;
  showAgreementPopup.value = false;
};

// 文件选择后的处理
const afterRead = (file) => {
  uploadedFile.value = file.file;
  console.log('选择的文件:', file.file.name, '大小:', file.file.size);
};

// 文件大小超限处理
const onOversize = () => {
  showToast('文件大小不能超过100MB');
};

// 删除文件
const onDelete = () => {
  uploadedFile.value = null;
};

// 提交表单
const onSubmit = async () => {
  if (!agreedToTerms.value) {
    showToast('请阅读并同意《资源上传协议》');
    return;
  }
  
  if (!uploadedFile.value) {
    showToast('请选择要上传的文件');
    return;
  }
  
  submitting.value = true;
  
  try {
    // 第一步：创建资源记录（不包含文件）
    const categoryName = (categories.value.find(c => Number(c.id) === Number(form.value.category_id)) || {}).name;
    const resourceData = {
      title: form.value.name,
      description: form.value.description || undefined,
      category: categoryName || undefined,
      tags: form.value.tags,
      file_url: '', // 暂时为空，等待文件上传后更新
    };
    
    const createRes = await resourceApi.createResource(resourceData);
    
    if (createRes.code !== 0) {
      showToast(createRes.message || '创建资源失败');
      return;
    }
    
    const resourceId = createRes.data.id;
    showToast('资源创建成功，正在上传文件...');
    
    // 第二步：上传文件
    const formData = new FormData();
    formData.append('file', uploadedFile.value);
    
    const uploadRes = await resourceApi.uploadFile(resourceId, formData);
    
    if (uploadRes.code === 0) {
      showToast('资源上传成功，等待审核');
      router.replace('/my-resources');
    } else {
      showToast(uploadRes.message || '文件上传失败');
    }
  } catch (error) {
    console.error('资源提交失败', error);
    showToast('提交失败，请重试');
  } finally {
    submitting.value = false;
  }
};

// 页面加载时检查登录状态并加载分类
onMounted(() => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录');
    router.replace({
      path: '/login',
      query: { redirect: '/submit' }
    });
    return;
  }
  
  loadCategories();
});
</script>

<style scoped>
.submit-resource-page {
  min-height: 100vh;
  background-color: var(--background-color);
  padding-bottom: 20px;
}

.content {
  padding: 16px;
}

.form-actions {
  margin: 24px 16px;
}

.tag-wrapper {
  width: 100%;
}

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  margin-bottom: 8px;
}

.mr-2 {
  margin-right: 8px;
}

.mb-2 {
  margin-bottom: 8px;
}

.link-text {
  color: var(--primary-color);
}

.agreement-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
}

.agreement-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.agreement-title {
  font-size: 18px;
  font-weight: 500;
}

.agreement-text {
  flex: 1;
  overflow-y: auto;
  padding: 16px 0;
  font-size: 14px;
  line-height: 1.6;
}

.agreement-text h3 {
  font-size: 18px;
  font-weight: 500;
  margin-bottom: 16px;
}

.agreement-text h4 {
  font-size: 16px;
  font-weight: 500;
  margin: 16px 0 8px;
}

.agreement-text p {
  margin-bottom: 8px;
}

.agreement-footer {
  padding-top: 16px;
  border-top: 1px solid var(--border-color);
}
</style> 