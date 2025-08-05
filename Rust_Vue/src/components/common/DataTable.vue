<template>
  <div class="data-table-container">
    <!-- 表格工具栏 -->
    <div v-if="showToolbar" class="table-toolbar">
      <div class="toolbar-left">
        <slot name="toolbar-left">
          <h3 v-if="title" class="table-title">{{ title }}</h3>
          <p v-if="description" class="table-description">{{ description }}</p>
        </slot>
      </div>
      
      <div class="toolbar-right">
        <slot name="toolbar-right">
          <!-- 搜索框 -->
          <el-input
            v-if="searchable"
            v-model="searchQuery"
            :placeholder="searchPlaceholder"
            :prefix-icon="Search"
            clearable
            class="search-input"
            @input="handleSearch"
          />
          
          <!-- 列配置 -->
          <el-dropdown v-if="columnConfigurable" trigger="click" placement="bottom-end">
            <el-button :icon="Setting" circle />
            <template #dropdown>
              <el-dropdown-menu class="column-config-menu">
                <div class="column-config-header">
                  <span>显示列</span>
                  <el-button 
                    text 
                    size="small" 
                    @click="resetColumns"
                  >
                    重置
                  </el-button>
                </div>
                <div class="column-config-list">
                  <div 
                    v-for="column in configurableColumns" 
                    :key="column.prop"
                    class="column-config-item"
                  >
                    <el-checkbox 
                      v-model="column.visible"
                      @change="updateColumnVisibility"
                    >
                      {{ column.label }}
                    </el-checkbox>
                  </div>
                </div>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
          
          <!-- 刷新按钮 -->
          <el-button 
            v-if="refreshable"
            :icon="Refresh" 
            circle 
            :loading="loading"
            @click="handleRefresh"
          />
        </slot>
      </div>
    </div>
    
    <!-- 表格主体 -->
    <div class="table-wrapper">
      <el-table
        ref="tableRef"
        v-loading="loading"
        :data="displayData"
        :height="tableHeight"
        :max-height="maxHeight"
        :stripe="stripe"
        :border="border"
        :size="size"
        :highlight-current-row="highlightCurrentRow"
        :row-class-name="getRowClassName"
        :cell-class-name="getCellClassName"
        :header-cell-class-name="getHeaderCellClassName"
        :empty-text="emptyText"
        :default-sort="defaultSort"
        class="data-table"
        @selection-change="handleSelectionChange"
        @sort-change="handleSortChange"
        @row-click="handleRowClick"
        @row-dblclick="handleRowDoubleClick"
      >
        <!-- 选择列 -->
        <el-table-column
          v-if="selectable"
          type="selection"
          width="55"
          align="center"
          :selectable="selectableFunction"
        />
        
        <!-- 序号列 -->
        <el-table-column
          v-if="showIndex"
          type="index"
          :label="indexLabel"
          :width="indexWidth"
          align="center"
          :index="getIndexMethod"
        />
        
        <!-- 数据列 -->
        <el-table-column
          v-for="column in visibleColumns"
          :key="column.prop"
          :prop="column.prop"
          :label="column.label"
          :width="column.width"
          :min-width="column.minWidth"
          :fixed="column.fixed"
          :sortable="column.sortable"
          :align="column.align || 'left'"
          :header-align="column.headerAlign || column.align"
          :class-name="column.className"
          :show-overflow-tooltip="column.showOverflowTooltip !== false"
        >
          <template #header="{ column: col }">
            <slot :name="`header-${column.prop}`" :column="col">
              <span class="column-header">
                {{ column.label }}
                <el-tooltip
                  v-if="column.tooltip"
                  :content="column.tooltip"
                  placement="top"
                >
                  <el-icon class="header-tooltip-icon">
                    <QuestionFilled />
                  </el-icon>
                </el-tooltip>
              </span>
            </slot>
          </template>
          
          <template #default="{ row, $index }">
            <slot 
              :name="column.prop" 
              :row="row" 
              :value="getNestedValue(row, column.prop)"
              :index="$index"
              :column="column"
            >
              <span 
                v-if="column.type === 'text'"
                class="cell-text"
              >
                {{ formatCellValue(row, column) }}
              </span>
              
              <el-tag
                v-else-if="column.type === 'tag'"
                :type="getTagType(row, column)"
                :size="size"
                :effect="column.tagEffect || 'light'"
              >
                {{ formatCellValue(row, column) }}
              </el-tag>
              
              <div
                v-else-if="column.type === 'actions'"
                class="cell-actions"
              >
                <template v-for="action in getActions(row, column)" :key="action.key">
                  <el-button
                    v-if="action.visible !== false"
                    :type="action.type || 'primary'"
                    :size="size"
                    :icon="action.icon"
                    :disabled="action.disabled"
                    :loading="action.loading"
                    link
                    @click="handleAction(action, row, $index)"
                  >
                    {{ action.label }}
                  </el-button>
                </template>
              </div>
              
              <span v-else class="cell-text">
                {{ formatCellValue(row, column) }}
              </span>
            </slot>
          </template>
        </el-table-column>
        
        <!-- 操作列 -->
        <el-table-column
          v-if="actions && actions.length > 0"
          :label="actionsLabel"
          :width="actionsWidth"
          :fixed="actionsFixed"
          align="center"
          class-name="actions-column"
        >
          <template #default="{ row, $index }">
            <div class="table-actions">
              <template v-for="action in getRowActions(row, $index)" :key="action.key">
                <el-button
                  v-if="action.visible !== false"
                  :type="action.type || 'primary'"
                  :size="size"
                  :icon="action.icon"
                  :disabled="action.disabled"
                  :loading="action.loading"
                  link
                  @click="handleAction(action, row, $index)"
                >
                  {{ action.label }}
                </el-button>
              </template>
            </div>
          </template>
        </el-table-column>
        
        <!-- 展开行 -->
        <template v-if="expandable" #expand="{ row, $index }">
          <slot name="expand" :row="row" :index="$index" />
        </template>
      </el-table>
    </div>
    
    <!-- 分页器 -->
    <div v-if="pagination && totalCount > 0" class="table-pagination">
      <div class="pagination-info">
        <span class="info-text">
          共 {{ totalCount }} 条记录
          <template v-if="selectedRows.length > 0">
            ，已选择 {{ selectedRows.length }} 条
          </template>
        </span>
      </div>
      
      <el-pagination
        v-model:current-page="currentPage"
        v-model:page-size="pageSize"
        :total="totalCount"
        :page-sizes="pageSizes"
        :layout="paginationLayout"
        :background="true"
        :small="size === 'small'"
        @size-change="handleSizeChange"
        @current-change="handleCurrentChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted } from 'vue'
import { ElTable } from 'element-plus'
import { 
  Search, 
  Setting, 
  Refresh, 
  QuestionFilled 
} from '@element-plus/icons-vue'
import { debounce } from '@/utils/debounce'
import type { 
  TableColumn, 
  TableAction, 
  TableProps, 
  TableEmits,
  SortParams,
  SelectionChangeParams 
} from './types'

// Props 定义
const props = withDefaults(defineProps<TableProps>(), {
  data: () => [],
  columns: () => [],
  loading: false,
  stripe: true,
  border: false,
  size: 'default',
  highlightCurrentRow: false,
  selectable: false,
  showIndex: false,
  indexLabel: '#',
  indexWidth: 60,
  searchable: false,
  searchPlaceholder: '请输入搜索关键词',
  columnConfigurable: false,
  refreshable: false,
  showToolbar: true,
  pagination: false,
  pageSize: 20,
  pageSizes: () => [10, 20, 50, 100],
  paginationLayout: 'total, sizes, prev, pager, next, jumper',
  emptyText: '暂无数据',
  actionsLabel: '操作',
  actionsWidth: 150,
  actionsFixed: 'right'
})

// Emits 定义
const emit = defineEmits<TableEmits>()

// 响应式数据
const tableRef = ref<InstanceType<typeof ElTable>>()
const searchQuery = ref('')
const currentPage = ref(1)
const selectedRows = ref<any[]>([])
const configurableColumns = ref<TableColumn[]>([])

// 计算属性
const visibleColumns = computed(() => {
  return props.columns.filter(column => {
    const configColumn = configurableColumns.value.find(c => c.prop === column.prop)
    return configColumn ? configColumn.visible : true
  })
})

const displayData = computed(() => {
  let data = [...props.data]
  
  // 搜索过滤
  if (searchQuery.value && props.searchable) {
    const query = searchQuery.value.toLowerCase()
    data = data.filter(row => {
      return props.columns.some(column => {
        const value = getNestedValue(row, column.prop)
        return String(value).toLowerCase().includes(query)
      })
    })
  }
  
  // 分页处理
  if (props.pagination) {
    const start = (currentPage.value - 1) * props.pageSize
    const end = start + props.pageSize
    return data.slice(start, end)
  }
  
  return data
})

const totalCount = computed(() => {
  if (props.pagination && props.total !== undefined) {
    return props.total
  }
  return props.data.length
})

// 方法
const getNestedValue = (obj: any, path: string) => {
  return path.split('.').reduce((current, key) => current?.[key], obj)
}

const formatCellValue = (row: any, column: TableColumn) => {
  const value = getNestedValue(row, column.prop)
  
  if (column.formatter) {
    return column.formatter(value, row, column)
  }
  
  if (value === null || value === undefined) {
    return column.emptyText || '-'
  }
  
  return value
}

const getTagType = (row: any, column: TableColumn) => {
  if (column.tagTypeMap) {
    const value = getNestedValue(row, column.prop)
    return column.tagTypeMap[value] || 'info'
  }
  return 'info'
}

const getActions = (row: any, column: TableColumn) => {
  return column.actions?.filter(action => {
    if (typeof action.visible === 'function') {
      return action.visible(row)
    }
    return action.visible !== false
  }) || []
}

const getRowActions = (row: any, index: number) => {
  return props.actions?.filter(action => {
    if (typeof action.visible === 'function') {
      return action.visible(row, index)
    }
    return action.visible !== false
  }) || []
}

const getRowClassName = ({ row, rowIndex }: { row: any; rowIndex: number }) => {
  let className = ''
  
  if (props.rowClassName) {
    if (typeof props.rowClassName === 'function') {
      className += props.rowClassName({ row, rowIndex })
    } else {
      className += props.rowClassName
    }
  }
  
  return className
}

const getCellClassName = ({ row, column, rowIndex, columnIndex }: any) => {
  let className = ''
  
  if (props.cellClassName) {
    if (typeof props.cellClassName === 'function') {
      className += props.cellClassName({ row, column, rowIndex, columnIndex })
    } else {
      className += props.cellClassName
    }
  }
  
  return className
}

const getHeaderCellClassName = ({ row, column, rowIndex, columnIndex }: any) => {
  let className = 'table-header-cell'
  
  if (props.headerCellClassName) {
    if (typeof props.headerCellClassName === 'function') {
      className += ' ' + props.headerCellClassName({ row, column, rowIndex, columnIndex })
    } else {
      className += ' ' + props.headerCellClassName
    }
  }
  
  return className
}

const getIndexMethod = (index: number) => {
  if (props.pagination) {
    return (currentPage.value - 1) * props.pageSize + index + 1
  }
  return index + 1
}

const selectableFunction = (row: any, index: number) => {
  if (props.selectableFunction) {
    return props.selectableFunction(row, index)
  }
  return true
}

// 事件处理
const handleSearch = debounce(() => {
  currentPage.value = 1
  emit('search', searchQuery.value)
}, 300)

const handleRefresh = () => {
  emit('refresh')
}

const handleSelectionChange = (selection: any[]) => {
  selectedRows.value = selection
  emit('selection-change', selection)
}

const handleSortChange = (params: SortParams) => {
  emit('sort-change', params)
}

const handleRowClick = (row: any, column: any, event: Event) => {
  emit('row-click', row, column, event)
}

const handleRowDoubleClick = (row: any, column: any, event: Event) => {
  emit('row-dblclick', row, column, event)
}

const handleAction = (action: TableAction, row: any, index: number) => {
  if (action.handler) {
    action.handler(row, index)
  }
  emit('action', action.key, row, index)
}

const handleSizeChange = (size: number) => {
  currentPage.value = 1
  emit('size-change', size)
}

const handleCurrentChange = (page: number) => {
  emit('current-change', page)
}

const updateColumnVisibility = () => {
  emit('column-change', configurableColumns.value)
}

const resetColumns = () => {
  configurableColumns.value.forEach(column => {
    column.visible = true
  })
  updateColumnVisibility()
}

// 公开方法
const clearSelection = () => {
  tableRef.value?.clearSelection()
}

const toggleRowSelection = (row: any, selected?: boolean) => {
  tableRef.value?.toggleRowSelection(row, selected)
}

const toggleAllSelection = () => {
  tableRef.value?.toggleAllSelection()
}

const setCurrentRow = (row: any) => {
  tableRef.value?.setCurrentRow(row)
}

const clearSort = () => {
  tableRef.value?.clearSort()
}

const doLayout = () => {
  nextTick(() => {
    tableRef.value?.doLayout()
  })
}

// 初始化
onMounted(() => {
  // 初始化列配置
  configurableColumns.value = props.columns.map(column => ({
    ...column,
    visible: true
  }))
})

// 监听数据变化
watch(() => props.data, () => {
  if (props.pagination && currentPage.value > 1) {
    const maxPage = Math.ceil(totalCount.value / props.pageSize)
    if (currentPage.value > maxPage) {
      currentPage.value = Math.max(1, maxPage)
    }
  }
}, { deep: true })

// 暴露方法
defineExpose({
  clearSelection,
  toggleRowSelection,
  toggleAllSelection,
  setCurrentRow,
  clearSort,
  doLayout,
  tableRef
})
</script>

<style scoped>
.data-table-container {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
}

.table-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-6);
  border-bottom: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.toolbar-left {
  flex: 1;
}

.table-title {
  margin: 0 0 var(--space-1) 0;
  font-size: var(--font-size-lg);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
}

.table-description {
  margin: 0;
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.search-input {
  width: 240px;
}

.table-wrapper {
  position: relative;
}

.data-table {
  width: 100%;
}

.data-table :deep(.el-table__header-wrapper) {
  background: var(--bg-elevated);
}

.data-table :deep(.table-header-cell) {
  background: var(--bg-elevated);
  color: var(--text-primary);
  font-weight: var(--font-weight-medium);
}

.column-header {
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

.header-tooltip-icon {
  font-size: 14px;
  color: var(--text-tertiary);
  cursor: help;
}

.cell-text {
  color: var(--text-primary);
}

.cell-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.table-actions {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
}

.table-pagination {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4) var(--space-6);
  border-top: 1px solid var(--border-color);
  background: var(--bg-secondary);
}

.pagination-info {
  flex: 1;
}

.info-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.column-config-menu {
  min-width: 200px;
}

.column-config-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border-color);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.column-config-list {
  max-height: 300px;
  overflow-y: auto;
}

.column-config-item {
  padding: var(--space-2) var(--space-4);
  border-bottom: 1px solid var(--border-color-light);
}

.column-config-item:last-child {
  border-bottom: none;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .table-toolbar {
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-3);
  }
  
  .toolbar-right {
    justify-content: flex-end;
  }
  
  .search-input {
    width: 200px;
  }
  
  .table-pagination {
    flex-direction: column;
    align-items: stretch;
    gap: var(--space-3);
  }
  
  .pagination-info {
    text-align: center;
  }
}

/* 深色模式适配 */
:global(html.dark) .data-table-container {
  box-shadow: var(--shadow-dark);
}
</style> 