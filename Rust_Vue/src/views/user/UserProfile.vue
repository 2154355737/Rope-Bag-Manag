<template>
  <div class="user-profile">
    <h2>个人信息</h2>
    <p>这里可以查看和编辑您的个人资料。</p>

    <h3 style="margin-top:24px;">资源订阅</h3>
    <el-table :data="categoryList" style="width:100%;max-width:600px;" v-loading="loading">
      <el-table-column prop="name" label="类别" />
      <el-table-column label="订阅">
        <template #default="{row}">
          <el-switch v-model="subscribed[row.id]" @change="val=>onToggle(row.id,val)" />
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { categoryApi, subscriptionApi } from '@/api'

interface Category { id:number, name:string }
const categoryList = ref<Category[]>([])
const subscribed = reactive<Record<number, boolean>>({})
const loading = ref(false)

async function load(){
  loading.value=true
  const res = await categoryApi.getCategories()
  loading.value=false
  if(res.code===0){
    categoryList.value = res.data?.list || []
    // 默认全部关闭；如果后端提供获取订阅的API可在此初始化
    categoryList.value.forEach(c=>{ if(subscribed[c.id]===undefined) subscribed[c.id]=false })
  }
}

async function onToggle(catId:number, enabled:boolean){
  const res = await subscriptionApi.setSubscription({ category_id:catId, enabled })
  if(res.code===0){ ElMessage.success(enabled?'已订阅':'已取消订阅') } else { ElMessage.error(res.message||'操作失败'); subscribed[catId]=!enabled }
}

onMounted(load)
</script>

<style scoped>
.user-profile{ padding:24px; }
</style> 