import { watch, watchEffect } from 'vue';

// 这是一个Vue watch示例代码文件，用于解决ESLint的未使用导入警告
// 实际项目中应该在组件内使用这些函数

/**
 * 监听一个或多个响应式数据源，并在数据源变化时调用回调函数
 * @param {*} source - 要监听的响应式数据源
 * @param {Function} callback - 数据源变化时调用的回调函数
 * @param {Object} options - 监听选项
 */
export function useWatch(source, callback, options) {
  return watch(source, callback, options);
}

/**
 * 立即运行一个函数，同时响应式地追踪其依赖项，并在依赖项变化时重新执行
 * @param {Function} effect - 要运行的副作用函数
 * @param {Object} options - 效果选项
 */
export function useWatchEffect(effect, options) {
  return watchEffect(effect, options);
}

// 示例：如何在组件中使用
/*
import { ref } from 'vue';
import { useWatch, useWatchEffect } from './watch';

// 在组件中
const counter = ref(0);

// 使用 watch
useWatch(counter, (newValue, oldValue) => {
  console.log(`Counter changed from ${oldValue} to ${newValue}`);
}, { immediate: true });

// 使用 watchEffect
useWatchEffect(() => {
  console.log(`Current counter value: ${counter.value}`);
});
*/ 