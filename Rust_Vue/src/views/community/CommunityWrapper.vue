<template>
  <div class="community-wrapper">
    <CommunityHome v-if="!isMobile" />
    <CommunityHomeMobile v-else />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { shouldUseMobileVersion } from '@/utils/device'
import CommunityHome from './desktop/CommunityHome.vue'
import CommunityHomeMobile from './mobile/CommunityHome.vue'

const isMobile = computed(() => shouldUseMobileVersion())
</script>

<style scoped>
.community-wrapper {
  width: 100%;
  height: 100%;
}
</style> 