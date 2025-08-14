import { createRouter, createWebHistory } from 'vue-router';

// 路由懒加载
const SplashScreen = () => import('../views/SplashScreen.vue');
const Home = () => import('../views/Home.vue');
const Category = () => import('../views/Category.vue');
const ResourceDetail = () => import('../views/ResourceDetail.vue');
const Search = () => import('../views/Search.vue');
const Login = () => import('../views/Login.vue');
const Register = () => import('../views/Register.vue');
const Profile = () => import('../views/Profile.vue');
const UserResources = () => import('../views/UserResources.vue');
const UserComments = () => import('../views/UserComments.vue');
const UserFavorites = () => import('../views/UserFavorites.vue');
const UserPosts = () => import('../views/UserPosts.vue');
const SubmitResource = () => import('../views/SubmitResource.vue');
const NotFound = () => import('../views/NotFound.vue');
const Community = () => import('../views/Community.vue');
const PostDetail = () => import('../views/PostDetail.vue');
const PostCreate = () => import('../views/PostCreate.vue');
const Notifications = () => import('../views/Notifications.vue');
const Subscriptions = () => import('../views/Subscriptions.vue');
const About = () => import('../views/About.vue');
const Announcements = () => import('../views/Announcements.vue');

const routes = [
  {
    path: '/splash',
    name: 'SplashScreen',
    component: SplashScreen,
    meta: {
      title: '绳包社区',
      hideFromHistory: true
    }
  },
  {
    path: '/announcements',
    name: 'Announcements',
    component: Announcements,
    meta: { title: '公告', keepAlive: false }
  },
  {
    path: '/',
    name: 'Home',
    component: Home,
    meta: {
      title: '首页',
      keepAlive: true
    }
  },
  {
    path: '/category/:id?',
    name: 'Category',
    component: Category,
    props: true,
    meta: {
      title: '分类浏览',
      keepAlive: true
    }
  },
  {
    path: '/resource/:id',
    name: 'ResourceDetail',
    component: ResourceDetail,
    props: true,
    meta: {
      title: '资源详情',
      keepAlive: false
    }
  },
  {
    path: '/search',
    name: 'Search',
    component: Search,
    meta: {
      title: '搜索',
      keepAlive: false
    }
  },
  {
    path: '/community',
    name: 'Community',
    component: Community,
    meta: {
      title: '社区',
      keepAlive: true
    }
  },
  {
    path: '/post/:id',
    name: 'PostDetail',
    component: PostDetail,
    props: true,
    meta: {
      title: '帖子详情',
      keepAlive: false
    }
  },
  {
    path: '/post/create',
    name: 'PostCreate',
    component: PostCreate,
    meta: {
      title: '写帖子',
      requiresAuth: true
    }
  },
  {
    path: '/notifications',
    name: 'Notifications',
    component: Notifications,
    meta: { title: '我的通知', requiresAuth: true }
  },
  {
    path: '/subscriptions',
    name: 'Subscriptions',
    component: Subscriptions,
    meta: { title: '分类订阅', requiresAuth: true }
  },
  {
    path: '/about',
    name: 'About',
    component: About,
    meta: { title: '关于我们', keepAlive: false }
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
    meta: {
      title: '登录',
      requiresGuest: true
    }
  },
  {
    path: '/register',
    name: 'Register',
    component: Register,
    meta: {
      title: '注册',
      requiresGuest: true
    }
  },
  {
    path: '/profile',
    name: 'Profile',
    component: Profile,
    meta: {
      title: '个人中心',
      requiresAuth: true
    }
  },
  {
    path: '/my-resources',
    name: 'UserResources',
    component: UserResources,
    meta: {
      title: '我的资源',
      requiresAuth: true
    }
  },
  {
    path: '/my-posts',
    name: 'UserPosts',
    component: UserPosts,
    meta: {
      title: '我的帖子',
      requiresAuth: true
    }
  },
  {
    path: '/my-comments',
    name: 'UserComments',
    component: UserComments,
    meta: {
      title: '我的评论',
      requiresAuth: true
    }
  },
  {
    path: '/favorites',
    name: 'UserFavorites',
    component: UserFavorites,
    meta: {
      title: '我的收藏',
      requiresAuth: true
    }
  },
  {
    path: '/submit',
    name: 'SubmitResource',
    component: SubmitResource,
    meta: {
      title: '提交资源',
      requiresAuth: true
    }
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: NotFound,
    meta: {
      title: '页面不存在'
    }
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

// 全局前置守卫
router.beforeEach((to, from, next) => {
  // 设置页面标题
  document.title = to.meta.title ? `${to.meta.title} - 资源社区` : '资源社区';

  // 获取用户登录状态
  const isLoggedIn = localStorage.getItem('token');

  // 需要登录但未登录
  if (to.meta.requiresAuth && !isLoggedIn) {
    next({
      path: '/login',
      query: { redirect: to.fullPath }
    });
    return;
  }

  // 已登录用户访问游客页面
  if (to.meta.requiresGuest && isLoggedIn) {
    next({ path: '/' });
    return;
  }

  next();
});

export default router; 