import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import router from './router';

// 导入Vant样式
import 'vant/lib/index.css';

// 导入全局样式
import './assets/styles/main.css';

// 创建Vue应用实例
const app = createApp(App);

// 注册Pinia和Router
app.use(createPinia());
app.use(router);

// 挂载应用
app.mount('#app'); 