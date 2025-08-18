import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import router from './router';
import { appApi } from './api/app';
import { initSafeArea } from './utils/safeAreaHelper';

// 导入Vant样式
import 'vant/lib/index.css';

// 导入全局样式
import './assets/styles/main.css';
import './assets/styles/safe-area.css';

// 初始化安全区域辅助工具
initSafeArea();

// 创建Vue应用实例
const app = createApp(App);

// 注册Pinia和Router
app.use(createPinia());
app.use(router);

// 挂载应用
app.mount('#app');

// 上报应用启动（不阻塞）
appApi.reportLaunch(); 