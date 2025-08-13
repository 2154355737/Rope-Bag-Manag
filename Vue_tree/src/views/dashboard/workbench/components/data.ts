interface GroupItem {
  title: string
  icon: string
  color: string
  desc: string
  date: string
  group: string
}

interface NavItem {
  title: string
  icon: string
  color: string
}

interface DynamicInfoItem {
  avatar: string
  name: string
  date: string
  desc: string
}

export const navItems: NavItem[] = [
  {
    title: '数据分析',
    icon: 'ion:analytics-outline',
    color: '#1fdaca',
  },
  {
    title: '用户管理',
    icon: 'ion:person-outline',
    color: '#bf0c2c',
  },
  {
    title: '资源管理',
    icon: 'ion:archive-outline',
    color: '#e18525',
  },
  {
    title: '分类管理',
    icon: 'ion:folder-outline',
    color: '#3fb27f',
  },
  {
    title: '备份管理',
    icon: 'ion:cloud-download-outline',
    color: '#4daf1bc9',
  },
  {
    title: '系统日志',
    icon: 'ion:document-text-outline',
    color: '#00d8ff',
  },
]

export const dynamicInfoItems: DynamicInfoItem[] = [
  {
    avatar: 'dynamic-avatar-1|svg',
    name: 'admin',
    date: '刚刚',
    desc: `在 <a>系统管理</a> 创建了 <a>数据备份</a>`,
  },
  {
    avatar: 'dynamic-avatar-2|svg',
    name: 'user123',
    date: '1个小时前',
    desc: `上传了新的 <a>资源包</a> `,
  },
  {
    avatar: 'dynamic-avatar-3|svg',
    name: 'moderator',
    date: '1天前',
    desc: `审核通过了 <a>用户提交</a> `,
  },
  {
    avatar: 'dynamic-avatar-4|svg',
    name: 'elder',
    date: '2天前',
    desc: `发布了 <a>社区公告</a> `,
  },
  {
    avatar: 'dynamic-avatar-5|svg',
    name: '皮特',
    date: '3天前',
    desc: `回复了 <a>杰克</a> 的问题 <a>如何进行项目优化？</a>`,
  },
  {
    avatar: 'dynamic-avatar-6|svg',
    name: '杰克',
    date: '1周前',
    desc: `关闭了问题 <a>如何运行项目</a> `,
  },
  {
    avatar: 'dynamic-avatar-1|svg',
    name: '威廉',
    date: '1周前',
    desc: `发布了 <a>个人动态</a> `,
  },
  {
    avatar: 'dynamic-avatar-1|svg',
    name: '威廉',
    date: '2021-04-01 20:00',
    desc: `推送了代码到 <a>Github</a>`,
  },
]

export const groupItems: GroupItem[] = [
  {
    title: 'Github',
    icon: 'carbon:logo-github',
    color: '',
    desc: '不要等待机会，而要创造机会。',
    group: '开源组',
    date: '2021-04-01',
  },
  {
    title: 'Vue',
    icon: 'ion:logo-vue',
    color: '#3fb27f',
    desc: '现在的你决定将来的你。',
    group: '算法组',
    date: '2021-04-01',
  },
  {
    title: 'Html5',
    icon: 'ion:logo-html5',
    color: '#e18525',
    desc: '没有什么才能比努力更重要。',
    group: '上班摸鱼',
    date: '2021-04-01',
  },
  {
    title: 'Angular',
    icon: 'ion:logo-angular',
    color: '#bf0c2c',
    desc: '热情和欲望可以突破一切难关。',
    group: 'UI',
    date: '2021-04-01',
  },
  {
    title: 'React',
    icon: 'bx:bxl-react',
    color: '#00d8ff',
    desc: '健康的身体是实目标的基石。',
    group: '技术牛',
    date: '2021-04-01',
  },
  {
    title: 'Js',
    icon: 'ion:logo-javascript',
    color: '#4daf1bc9',
    desc: '路是走出来的，而不是空想出来的。',
    group: '架构组',
    date: '2021-04-01',
  },
]
