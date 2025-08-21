/// <reference types="vite/client" />

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

interface ImportMetaEnv {
  readonly VITE_API_BASE_URL: string;
  // 更多环境变量...
  [key: string]: string | boolean | undefined;
}

// 声明模块，让Vue单文件组件可以被TypeScript识别
declare module '*.vue' {
  import type { DefineComponent } from 'vue';
  const component: DefineComponent<{}, {}, any>;
  export default component;
} 