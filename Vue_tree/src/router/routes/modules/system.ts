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