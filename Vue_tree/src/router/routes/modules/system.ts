import type { AppRouteModule } from '/@/router/types'

import { LAYOUT } from '/@/router/constant'
import { RoleEnum } from '/@/enums/roleEnum'

const system: AppRouteModule = {
  path: '/system',
  name: 'System',
  component: LAYOUT,
  redirect: '/system/user',
  meta: {
    orderNo: 20,
    icon: 'ion:settings-outline',
    title: '系统管理',
    roles: [RoleEnum.ADMIN, RoleEnum.MODERATOR],
  },
  children: [
    {
      path: 'user',
      name: 'UserManage',
      component: () => import('/@/views/system/UserManage.vue'),
      meta: {
        title: '用户管理',
        icon: 'ion:person-outline',
        roles: [RoleEnum.ADMIN, RoleEnum.MODERATOR],
        ignoreKeepAlive: true,
      },
    },
    {
      path: 'post',
      name: 'PostManage',
      component: () => import('/@/views/system/PostManage.vue'),
      meta: {
        title: '帖子管理',
        icon: 'ion:document-text-outline',
        roles: [RoleEnum.ADMIN, RoleEnum.MODERATOR],
      },
    },
    {
      path: 'tag',
      name: 'TagManage',
      component: () => import('/@/views/system/TagManage.vue'),
      meta: {
        title: '标签管理',
        icon: 'ion:pricetags-outline',
        roles: [RoleEnum.ADMIN, RoleEnum.MODERATOR],
      },
    },
    {
      path: 'package',
      name: 'SystemPackage',
      component: () => import('/@/views/system/PackageManage.vue'),
      meta: {
        title: '资源包管理',
        icon: 'ion:archive-outline',
        roles: [RoleEnum.ADMIN, RoleEnum.MODERATOR],
      },
    },
    {
      path: 'category',
      name: 'SystemCategory',
      component: () => import('/@/views/system/CategoryManage.vue'),
      meta: {
        title: '分类管理',
        icon: 'ion:folder-outline',
        roles: [RoleEnum.ADMIN],
      },
    },
    {
      path: 'announcement',
      name: 'AnnouncementManage',
      component: () => import('/@/views/system/AnnouncementManage.vue'),
      meta: {
        title: '公告管理',
        icon: 'ion:notifications-outline',
        roles: [RoleEnum.ADMIN, RoleEnum.MODERATOR],
      },
    },
    {
      path: 'mail',
      name: 'MailConfig',
      component: () => import('/@/views/system/MailConfig.vue'),
      meta: {
        title: '邮箱配置',
        icon: 'ion:mail-outline',
        roles: [RoleEnum.ADMIN],
      },
    },
    {
      path: 'storage',
      name: 'StorageConfig',
      component: () => import('/@/views/system/StorageConfig.vue'),
      meta: {
        title: '存储配置',
        icon: 'ion:cloud-outline',
        roles: [RoleEnum.ADMIN],
      },
    },
    {
      path: 'backup',
      name: 'SystemBackup',
      component: () => import('/@/views/system/BackupManage.vue'),
      meta: {
        title: '备份管理',
        icon: 'ion:cloud-download-outline',
        roles: [RoleEnum.ADMIN],
      },
    },
    {
      path: 'security',
      name: 'SecurityManage',
      component: () => import('/@/views/system/SecurityManage.vue'),
      meta: {
        title: '安全管理',
        icon: 'ion:shield-checkmark-outline',
        roles: [RoleEnum.ADMIN],
      },
    },
    {
      path: 'logs',
      name: 'SystemLogs',
      component: () => import('/@/views/system/LogsManage.vue'),
      meta: {
        title: '系统日志',
        icon: 'ion:document-text-outline',
        roles: [RoleEnum.ADMIN],
      },
    },
    {
      path: 'moderation',
      name: 'ModerationCenter',
      component: () => import('/@/views/system/ModerationCenter.vue'),
      meta: {
        title: '内容审核',
        icon: 'ion:checkmark-done-circle-outline',
        roles: [RoleEnum.ADMIN, RoleEnum.MODERATOR],
      },
    },
    {
      path: 'settings',
      name: 'SystemSettings',
      component: () => import('/@/views/setup/index.vue'),
      meta: {
        title: '系统设置',
        icon: 'ion:cog-outline',
        roles: [RoleEnum.ADMIN],
      },
    },
  ],
}

export default system 