<template>
  <div class="tag-manage">
    <el-card>
      <template #header>
        <div class="header">
          <el-input
            v-model="search"
            placeholder="搜索标签"
            clearable
            style="max-width: 220px"
            @change="loadTags"
          />
          <div class="header-buttons">
            <el-button @click="refreshTags">刷新</el-button>
            <el-button type="primary" @click="showAddDialog">新增标签</el-button>
          </div>
        </div>
      </template>

      <el-table :data="tags" style="width: 100%" :loading="loading">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="name" label="名称" />
        <el-table-column prop="description" label="描述" />
        <el-table-column prop="color" label="颜色" width="120">
          <template #default="{ row }">
            <div v-if="row.color" :style="{ background: row.color, width: '24px', height: '24px', borderRadius: '4px' }" />
          </template>
        </el-table-column>
        <el-table-column prop="use_count" label="使用次数" width="120" />
        <el-table-column prop="created_at" label="创建时间" />
        <el-table-column label="操作" width="180">
          <template #default="{ row }">
            <el-button size="small" @click="editTag(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="deleteTag(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <!-- 新增标签对话框 -->
    <el-dialog v-model="addDialogVisible" title="新增标签">
      <el-form ref="addFormRef" :model="newTag" :rules="rules" label-width="80px">
        <el-form-item label="名称" prop="name">
          <el-input v-model="newTag.name" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="newTag.description" />
        </el-form-item>
        <el-form-item label="颜色" prop="color">
          <el-input v-model="newTag.color" placeholder="#409EFF" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="addDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleAdd">确定</el-button>
      </template>
    </el-dialog>

    <!-- 编辑标签对话框 -->
    <el-dialog v-model="editDialogVisible" title="编辑标签">
      <el-form ref="editFormRef" :model="editTagData" :rules="rules" label-width="80px">
        <el-form-item label="名称" prop="name">
          <el-input v-model="editTagData.name" />
        </el-form-item>
        <el-form-item label="描述" prop="description">
          <el-input v-model="editTagData.description" />
        </el-form-item>
        <el-form-item label="颜色" prop="color">
          <el-input v-model="editTagData.color" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="editDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleUpdate">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import * as tagApi from '@/api/tags'
import type { Tag, CreateTagRequest, UpdateTagRequest } from '@/api/tags'

const tags = ref<Tag[]>([])
const search = ref('')
const loading = ref(false)

// 加载标签
const loadTags = async () => {
  loading.value = true
  try {
    console.log('🔍 [TagManage] 开始加载标签, search:', search.value)
    
    const res = search.value
      ? await tagApi.getTags({ search: search.value })
      : await tagApi.getAllTags()

    console.log('🔍 [TagManage] API响应:', res)

    // 检查是否有嵌套的data结构（axios包装的响应）
    const apiData = (res as any).data || res
    console.log('🔍 [TagManage] 实际API数据:', apiData)

    if ((apiData as any).code === 0 && (apiData as any).data) {
      const tagsList = Array.isArray((apiData as any).data) ? (apiData as any).data : (apiData as any).data.list || []
      console.log('🔍 [TagManage] 解析后的标签列表:', tagsList)
      tags.value = tagsList
      
      if (tagsList.length === 0) {
        console.warn('⚠️ [TagManage] 标签列表为空')
      }
    } else {
      console.error('❌ [TagManage] API返回错误:', apiData)
      ElMessage.error((apiData as any).msg || '获取标签失败')
    }
  } catch (error) {
    console.error('❌ [TagManage] 请求异常:', error)
    ElMessage.error('获取标签失败')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadTags()
})

// 手动刷新功能
const refreshTags = () => {
  console.log('🔄 [TagManage] 手动刷新标签')
  loadTags()
}

// 新增标签逻辑
const addDialogVisible = ref(false)
const newTag = ref<CreateTagRequest>({ name: '', description: '', color: '' })
const addFormRef = ref()

const rules = {
  name: [{ required: true, message: '请输入名称', trigger: 'blur' }]
}

const showAddDialog = () => {
  newTag.value = { name: '', description: '', color: '' }
  addDialogVisible.value = true
}

const handleAdd = async () => {
  if (!addFormRef.value) return
  await addFormRef.value.validate()
  try {
    const res = await tagApi.createTag(newTag.value)
    if (res.code === 0) {
      ElMessage.success('创建成功')
      addDialogVisible.value = false
      loadTags()
    } else {
      ElMessage.error(res.msg || '创建失败')
    }
  } catch (error) {
    console.error('创建标签失败', error)
    ElMessage.error('创建失败')
  }
}

// 编辑标签逻辑
const editDialogVisible = ref(false)
const editTagData = ref<(Tag & UpdateTagRequest) | any>({})
const editFormRef = ref()

const editTag = (row: Tag) => {
  editTagData.value = { ...row }
  editDialogVisible.value = true
}

const handleUpdate = async () => {
  if (!editFormRef.value) return
  await editFormRef.value.validate()
  try {
    const { id, ...data } = editTagData.value
    const res = await tagApi.updateTag(id, data)
    if (res.code === 0) {
      ElMessage.success('更新成功')
      editDialogVisible.value = false
      loadTags()
    } else {
      ElMessage.error(res.msg || '更新失败')
    }
  } catch (error) {
    console.error('更新标签失败', error)
    ElMessage.error('更新失败')
  }
}

// 删除标签
const deleteTag = (row: Tag) => {
  ElMessageBox.confirm(`确定删除标签 "${row.name}" ?`, '提示', { type: 'warning' })
    .then(async () => {
      const res = await tagApi.deleteTag(row.id)
      if (res.code === 0) {
        ElMessage.success('删除成功')
        loadTags()
      } else {
        ElMessage.error(res.msg || '删除失败')
      }
    })
    .catch(() => {})
}
</script>

<style scoped>
.tag-manage {
  padding: 24px;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.header-buttons {
  display: flex;
  gap: 10px; /* Adjust as needed for spacing */
}
</style> 