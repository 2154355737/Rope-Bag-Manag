<template>
  <van-popup v-model:show="showTips" position="bottom" round>
    <div class="download-tips">
      <div class="tips-header">
        <h3>下载文件位置</h3>
        <van-icon name="cross" @click="showTips = false" />
      </div>
      
      <div class="tips-content">
        <div class="tip-item">
          <van-icon name="folder-o" />
          <div class="tip-text">
            <h4>Android手机</h4>
            <p>文件管理器 → 下载 或 Downloads文件夹</p>
            <p class="path">/storage/emulated/0/Download/</p>
          </div>
        </div>
        
        <div class="tip-item">
          <van-icon name="phone-o" />
          <div class="tip-text">
            <h4>通知栏</h4>
            <p>下拉通知栏查看下载进度和完成提醒</p>
          </div>
        </div>
        
        <div class="tip-item">
          <van-icon name="setting-o" />
          <div class="tip-text">
            <h4>浏览器下载</h4>
            <p>在系统浏览器中打开链接进行下载</p>
          </div>
        </div>
      </div>
      
      <div class="tips-footer">
        <van-button type="primary" block @click="showTips = false">
          我知道了
        </van-button>
      </div>
    </div>
  </van-popup>
</template>

<script setup>
import { ref } from 'vue'

const showTips = ref(false)

const show = () => {
  showTips.value = true
}

defineExpose({ show })
</script>

<style scoped>
.download-tips {
  padding: 20px;
  max-height: 60vh;
  overflow-y: auto;
}

.tips-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.tips-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.tips-content {
  margin-bottom: 20px;
}

.tip-item {
  display: flex;
  align-items: flex-start;
  margin-bottom: 16px;
  padding: 12px;
  background: #f7f8fa;
  border-radius: 8px;
}

.tip-item .van-icon {
  margin-right: 12px;
  margin-top: 2px;
  color: #1989fa;
  font-size: 20px;
}

.tip-text h4 {
  margin: 0 0 4px 0;
  font-size: 14px;
  font-weight: 600;
  color: #323233;
}

.tip-text p {
  margin: 0 0 4px 0;
  font-size: 12px;
  color: #646566;
  line-height: 1.4;
}

.path {
  font-family: monospace;
  background: #ebedf0;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px !important;
}

.tips-footer {
  padding-top: 16px;
  border-top: 1px solid #ebedf0;
}
</style> 