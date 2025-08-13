/**
 * Stagewise配置文件
 * 用于管理应用的不同部署阶段
 */

module.exports = {
  // 项目名称
  projectName: 'resource-community-app',
  
  // 不同的阶段配置
  stages: {
    // 开发阶段
    development: {
      name: 'Development',
      description: '开发环境配置',
      environment: 'development',
      apiBaseUrl: 'http://localhost:15201/api/v1',
      buildCommand: 'npm run dev',
      outputDir: 'dist',
      port: 5173,
      variables: {
        NODE_ENV: 'development',
        VITE_API_BASE_URL: 'http://localhost:15201/api/v1',
        VITE_APP_TITLE: '资源社区 - 开发版',
        VITE_DEBUG: 'true'
      }
    },
    
    // 测试阶段
    testing: {
      name: 'Testing',
      description: '测试环境配置',
      environment: 'testing',
      apiBaseUrl: 'http://test.example.com/api/v1',
      buildCommand: 'npm run build',
      outputDir: 'dist',
      variables: {
        NODE_ENV: 'testing',
        VITE_API_BASE_URL: 'http://test.example.com/api/v1',
        VITE_APP_TITLE: '资源社区 - 测试版',
        VITE_DEBUG: 'false'
      }
    },
    
    // 预生产阶段
    staging: {
      name: 'Staging',
      description: '预生产环境配置',
      environment: 'staging',
      apiBaseUrl: 'http://staging.example.com/api/v1',
      buildCommand: 'npm run build',
      outputDir: 'dist',
      variables: {
        NODE_ENV: 'production',
        VITE_API_BASE_URL: 'http://staging.example.com/api/v1',
        VITE_APP_TITLE: '资源社区 - 预览版',
        VITE_DEBUG: 'false'
      }
    },
    
    // 生产阶段
    production: {
      name: 'Production',
      description: '生产环境配置',
      environment: 'production',
      apiBaseUrl: 'http://39.105.113.219:15201/api/v1',
      buildCommand: 'npm run build',
      outputDir: 'dist',
      variables: {
        NODE_ENV: 'production',
        VITE_API_BASE_URL: 'http://39.105.113.219:15201/api/v1',
        VITE_APP_TITLE: '资源社区',
        VITE_DEBUG: 'false'
      }
    },
    
    // Tauri应用配置
    'tauri-dev': {
      name: 'Tauri Development',
      description: 'Tauri开发环境配置',
      environment: 'development',
      apiBaseUrl: 'http://39.105.113.219:15201/api/v1',
      buildCommand: 'npm run tauri:dev',
      variables: {
        NODE_ENV: 'development',
        VITE_API_BASE_URL: 'http://39.105.113.219:15201/api/v1',
        VITE_APP_TITLE: '资源社区 - 桌面版',
        VITE_DEBUG: 'true'
      }
    },
    
    'tauri-build': {
      name: 'Tauri Production',
      description: 'Tauri生产环境构建',
      environment: 'production',
      apiBaseUrl: 'http://39.105.113.219:15201/api/v1',
      buildCommand: 'npm run tauri:build',
      variables: {
        NODE_ENV: 'production',
        VITE_API_BASE_URL: 'http://39.105.113.219:15201/api/v1',
        VITE_APP_TITLE: '资源社区',
        VITE_DEBUG: 'false'
      }
    }
  },
  
  // 部署配置
  deployment: {
    // 开发环境部署
    development: {
      type: 'local',
      autoStart: true
    },
    
    // 测试环境部署
    testing: {
      type: 'docker',
      registry: 'your-registry.com',
      image: 'resource-community-app',
      tag: 'testing'
    },
    
    // 预生产环境部署
    staging: {
      type: 'docker',
      registry: 'your-registry.com',
      image: 'resource-community-app',
      tag: 'staging'
    },
    
    // 生产环境部署
    production: {
      type: 'docker',
      registry: 'your-registry.com',
      image: 'resource-community-app',
      tag: 'latest'
    }
  },
  
  // 钩子函数
  hooks: {
    // 构建前钩子
    preBuild: [
      'echo "开始构建..."',
      'npm install'
    ],
    
    // 构建后钩子
    postBuild: [
      'echo "构建完成"',
      'npm run lint'
    ],
    
    // 部署前钩子
    preDeploy: [
      'echo "准备部署..."'
    ],
    
    // 部署后钩子
    postDeploy: [
      'echo "部署完成"'
    ]
  },
  
  // 监控配置
  monitoring: {
    healthCheck: {
      enabled: true,
      path: '/health',
      interval: '30s'
    },
    
    logging: {
      level: 'info',
      format: 'json'
    }
  }
}; 