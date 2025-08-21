// 通用组件类型定义

import type { Component } from 'vue'

// ===== 表格组件类型定义 =====

export interface TableColumn {
  /** 列标识符 */
  prop: string
  /** 列标题 */
  label: string
  /** 列宽度 */
  width?: string | number
  /** 最小列宽度 */
  minWidth?: string | number
  /** 列固定位置 */
  fixed?: boolean | 'left' | 'right'
  /** 是否可排序 */
  sortable?: boolean | 'custom'
  /** 列对齐方式 */
  align?: 'left' | 'center' | 'right'
  /** 表头对齐方式 */
  headerAlign?: 'left' | 'center' | 'right'
  /** 列类名 */
  className?: string
  /** 是否显示溢出提示 */
  showOverflowTooltip?: boolean
  /** 列类型 */
  type?: 'text' | 'tag' | 'actions' | 'custom'
  /** 格式化函数 */
  formatter?: (value: any, row: any, column: TableColumn) => string
  /** 空值显示文本 */
  emptyText?: string
  /** 列提示信息 */
  tooltip?: string
  /** 标签类型映射 */
  tagTypeMap?: Record<string, 'success' | 'info' | 'warning' | 'danger'>
  /** 标签效果 */
  tagEffect?: 'dark' | 'light' | 'plain'
  /** 列内操作 */
  actions?: TableAction[]
  /** 是否可见（用于列配置） */
  visible?: boolean
}

export interface TableAction {
  /** 操作标识符 */
  key: string
  /** 操作标签 */
  label: string
  /** 操作类型 */
  type?: 'primary' | 'success' | 'warning' | 'danger' | 'info' | 'text'
  /** 操作图标 */
  icon?: Component
  /** 是否禁用 */
  disabled?: boolean | ((row: any, index?: number) => boolean)
  /** 是否显示 */
  visible?: boolean | ((row: any, index?: number) => boolean)
  /** 是否加载中 */
  loading?: boolean
  /** 操作处理函数 */
  handler?: (row: any, index: number) => void
}

export interface SortParams {
  column: any
  prop: string
  order: 'ascending' | 'descending' | null
}

export interface SelectionChangeParams {
  selection: any[]
}

export interface TableProps {
  /** 表格数据 */
  data: any[]
  /** 表格列配置 */
  columns: TableColumn[]
  /** 加载状态 */
  loading?: boolean
  /** 表格高度 */
  tableHeight?: string | number
  /** 最大高度 */
  maxHeight?: string | number
  /** 是否显示斑马纹 */
  stripe?: boolean
  /** 是否显示边框 */
  border?: boolean
  /** 表格尺寸 */
  size?: 'large' | 'default' | 'small'
  /** 是否高亮当前行 */
  highlightCurrentRow?: boolean
  /** 是否可选择 */
  selectable?: boolean
  /** 选择函数 */
  selectableFunction?: (row: any, index: number) => boolean
  /** 是否显示序号 */
  showIndex?: boolean
  /** 序号列标题 */
  indexLabel?: string
  /** 序号列宽度 */
  indexWidth?: number
  /** 是否可搜索 */
  searchable?: boolean
  /** 搜索占位符 */
  searchPlaceholder?: string
  /** 是否可配置列 */
  columnConfigurable?: boolean
  /** 是否可刷新 */
  refreshable?: boolean
  /** 是否显示工具栏 */
  showToolbar?: boolean
  /** 表格标题 */
  title?: string
  /** 表格描述 */
  description?: string
  /** 是否分页 */
  pagination?: boolean
  /** 每页条数 */
  pageSize?: number
  /** 总条数 */
  total?: number
  /** 每页条数选项 */
  pageSizes?: number[]
  /** 分页布局 */
  paginationLayout?: string
  /** 空数据文本 */
  emptyText?: string
  /** 默认排序 */
  defaultSort?: { prop: string; order: 'ascending' | 'descending' }
  /** 行类名 */
  rowClassName?: string | ((params: { row: any; rowIndex: number }) => string)
  /** 单元格类名 */
  cellClassName?: string | ((params: { row: any; column: any; rowIndex: number; columnIndex: number }) => string)
  /** 表头单元格类名 */
  headerCellClassName?: string | ((params: { row: any; column: any; rowIndex: number; columnIndex: number }) => string)
  /** 操作列配置 */
  actions?: TableAction[]
  /** 操作列标题 */
  actionsLabel?: string
  /** 操作列宽度 */
  actionsWidth?: number
  /** 操作列固定 */
  actionsFixed?: boolean | 'left' | 'right'
  /** 是否可展开 */
  expandable?: boolean
}

export interface TableEmits {
  /** 搜索事件 */
  'search': [query: string]
  /** 刷新事件 */
  'refresh': []
  /** 选择变化事件 */
  'selection-change': [selection: any[]]
  /** 排序变化事件 */
  'sort-change': [params: SortParams]
  /** 行点击事件 */
  'row-click': [row: any, column: any, event: Event]
  /** 行双击事件 */
  'row-dblclick': [row: any, column: any, event: Event]
  /** 操作事件 */
  'action': [key: string, row: any, index: number]
  /** 页面大小变化事件 */
  'size-change': [size: number]
  /** 当前页变化事件 */
  'current-change': [page: number]
  /** 列配置变化事件 */
  'column-change': [columns: TableColumn[]]
}

// ===== 表单组件类型定义 =====

export interface FormField {
  /** 字段名 */
  name: string
  /** 字段标签 */
  label: string
  /** 字段类型 */
  type: 'input' | 'textarea' | 'select' | 'radio' | 'checkbox' | 'date' | 'datetime' | 'number' | 'switch' | 'upload' | 'custom'
  /** 字段值 */
  value?: any
  /** 默认值 */
  defaultValue?: any
  /** 占位符 */
  placeholder?: string
  /** 是否必填 */
  required?: boolean
  /** 是否禁用 */
  disabled?: boolean
  /** 是否只读 */
  readonly?: boolean
  /** 验证规则 */
  rules?: any[]
  /** 选项列表（用于select、radio、checkbox） */
  options?: Array<{ label: string; value: any; disabled?: boolean }>
  /** 字段属性 */
  props?: Record<string, any>
  /** 字段样式类 */
  className?: string
  /** 字段提示 */
  tooltip?: string
  /** 是否显示 */
  visible?: boolean | ((formData: any) => boolean)
  /** 字段跨度 */
  span?: number
  /** 自定义渲染组件 */
  component?: Component
}

export interface FormProps {
  /** 表单字段配置 */
  fields: FormField[]
  /** 表单数据 */
  modelValue: Record<string, any>
  /** 表单布局 */
  layout?: 'horizontal' | 'vertical' | 'inline'
  /** 标签宽度 */
  labelWidth?: string | number
  /** 标签位置 */
  labelPosition?: 'left' | 'right' | 'top'
  /** 表单尺寸 */
  size?: 'large' | 'default' | 'small'
  /** 是否禁用 */
  disabled?: boolean
  /** 是否只读 */
  readonly?: boolean
  /** 是否显示验证图标 */
  statusIcon?: boolean
  /** 栅格间距 */
  gutter?: number
  /** 每行列数 */
  columns?: number
  /** 是否显示提交按钮 */
  showSubmit?: boolean
  /** 是否显示重置按钮 */
  showReset?: boolean
  /** 提交按钮文本 */
  submitText?: string
  /** 重置按钮文本 */
  resetText?: string
  /** 提交按钮加载状态 */
  submitLoading?: boolean
}

export interface FormEmits {
  /** 表单值更新 */
  'update:modelValue': [value: Record<string, any>]
  /** 表单提交 */
  'submit': [value: Record<string, any>]
  /** 表单重置 */
  'reset': []
  /** 字段值变化 */
  'field-change': [name: string, value: any]
}

// ===== 对话框组件类型定义 =====

export interface DialogProps {
  /** 是否显示 */
  modelValue: boolean
  /** 对话框标题 */
  title?: string
  /** 对话框宽度 */
  width?: string | number
  /** 是否全屏 */
  fullscreen?: boolean
  /** 是否可拖拽 */
  draggable?: boolean
  /** 是否显示关闭按钮 */
  showClose?: boolean
  /** 点击遮罩是否关闭 */
  closeOnClickModal?: boolean
  /** 按ESC是否关闭 */
  closeOnPressEscape?: boolean
  /** 是否在关闭前确认 */
  beforeClose?: (done: () => void) => void
  /** 对话框类名 */
  customClass?: string
  /** 是否居中 */
  center?: boolean
  /** 是否显示确认按钮 */
  showConfirm?: boolean
  /** 是否显示取消按钮 */
  showCancel?: boolean
  /** 确认按钮文本 */
  confirmText?: string
  /** 取消按钮文本 */
  cancelText?: string
  /** 确认按钮类型 */
  confirmType?: 'primary' | 'success' | 'warning' | 'danger' | 'info'
  /** 确认按钮加载状态 */
  confirmLoading?: boolean
}

export interface DialogEmits {
  /** 显示状态更新 */
  'update:modelValue': [value: boolean]
  /** 对话框打开 */
  'open': []
  /** 对话框关闭 */
  'close': []
  /** 确认事件 */
  'confirm': []
  /** 取消事件 */
  'cancel': []
}

// ===== 卡片组件类型定义 =====

export interface CardProps {
  /** 卡片标题 */
  title?: string
  /** 卡片描述 */
  description?: string
  /** 是否显示阴影 */
  shadow?: 'always' | 'hover' | 'never'
  /** 卡片体样式 */
  bodyStyle?: Record<string, any>
  /** 卡片类名 */
  className?: string
  /** 是否可点击 */
  clickable?: boolean
  /** 是否禁用 */
  disabled?: boolean
  /** 是否显示边框 */
  bordered?: boolean
  /** 卡片尺寸 */
  size?: 'large' | 'default' | 'small'
}

export interface CardEmits {
  /** 卡片点击事件 */
  'click': [event: Event]
}

// ===== 上传组件类型定义 =====

export interface UploadFile {
  /** 文件ID */
  id: string
  /** 文件名 */
  name: string
  /** 文件大小 */
  size: number
  /** 文件类型 */
  type: string
  /** 文件URL */
  url?: string
  /** 上传状态 */
  status: 'ready' | 'uploading' | 'success' | 'error'
  /** 上传进度 */
  progress?: number
  /** 错误信息 */
  error?: string
  /** 原始文件对象 */
  raw?: File
}

export interface UploadProps {
  /** 文件列表 */
  fileList: UploadFile[]
  /** 上传地址 */
  action: string
  /** 上传方法 */
  httpRequest?: (options: any) => void
  /** 接受的文件类型 */
  accept?: string
  /** 是否支持多选 */
  multiple?: boolean
  /** 最大文件数 */
  limit?: number
  /** 文件大小限制（MB） */
  maxSize?: number
  /** 是否自动上传 */
  autoUpload?: boolean
  /** 是否显示文件列表 */
  showFileList?: boolean
  /** 文件列表类型 */
  listType?: 'text' | 'picture' | 'picture-card'
  /** 是否禁用 */
  disabled?: boolean
  /** 上传前钩子 */
  beforeUpload?: (file: File) => boolean | Promise<boolean>
  /** 上传成功钩子 */
  onSuccess?: (response: any, file: UploadFile) => void
  /** 上传失败钩子 */
  onError?: (error: any, file: UploadFile) => void
  /** 上传进度钩子 */
  onProgress?: (progress: number, file: UploadFile) => void
}

export interface UploadEmits {
  /** 文件列表更新 */
  'update:fileList': [fileList: UploadFile[]]
  /** 文件选择 */
  'change': [file: UploadFile, fileList: UploadFile[]]
  /** 文件移除 */
  'remove': [file: UploadFile, fileList: UploadFile[]]
  /** 文件预览 */
  'preview': [file: UploadFile]
}

// ===== 通用工具类型 =====

export type ComponentSize = 'large' | 'default' | 'small'

export type ComponentStatus = 'success' | 'warning' | 'error' | 'info'

export interface LoadingState {
  loading: boolean
  error: string | null
  success: boolean
}

export interface PaginationState {
  currentPage: number
  pageSize: number
  total: number
  loading: boolean
}

export interface SearchState {
  query: string
  filters: Record<string, any>
}

// ===== 响应式断点类型 =====

export interface BreakpointState {
  xs: boolean  // < 576px
  sm: boolean  // >= 576px
  md: boolean  // >= 768px
  lg: boolean  // >= 992px
  xl: boolean  // >= 1200px
  xxl: boolean // >= 1400px
}

// ===== 主题类型 =====

export type ThemeType = 'light' | 'dark' | 'blue' | 'green' | 'purple' | 'orange' | 'red' | 'auto'

export interface ThemeConfig {
  name: ThemeType
  label: string
  icon: string
  description: string
}

// ===== API 响应类型 =====

export interface ApiResponse<T = any> {
  code: number
  message: string
  msg?: string
  data?: T
  success?: boolean
}

export interface ListResponse<T = any> {
  items: T[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

// ===== 用户权限类型 =====

export interface Permission {
  id: string
  name: string
  description: string
  resource: string
  action: string
}

export interface Role {
  id: string
  name: string
  label: string
  description: string
  permissions: Permission[]
}

export interface User {
  id: number
  username: string
  nickname: string
  email: string
  avatar?: string
  role: string
  status: string
  ban_status: string
  created_at: string
  updated_at: string
}

// ===== 资源类型 =====

export interface Resource {
  id: number
  title: string
  description: string
  category: string
  tags: string[]
  cover?: string
  file_url?: string
  file_size?: number
  download_count: number
  view_count: number
  like_count: number
  status: 'active' | 'inactive' | 'pending' | 'rejected'
  author_id: number
  author_name: string
  created_at: string
  updated_at: string
}

// ===== 评论类型 =====

export interface Comment {
  id: number
  content: string
  resource_id: number
  user_id: number
  user_name: string
  user_avatar?: string
  parent_id?: number
  replies?: Comment[]
  created_at: string
  updated_at: string
}

// ===== 系统设置类型 =====

export interface SystemSettings {
  site_name: string
  site_description: string
  site_logo?: string
  site_favicon?: string
  allow_registration: boolean
  require_email_verification: boolean
  max_upload_size: number
  allowed_file_types: string[]
  maintenance_mode: boolean
  maintenance_message: string
} 