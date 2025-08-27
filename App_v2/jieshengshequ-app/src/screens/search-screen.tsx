import React, { useEffect, useState } from 'react'
import { useSearchParams, useNavigate } from 'react-router-dom'
import { searchAll } from '../api/search'
import { getPosts } from '../api/posts'
import { getResources } from '../api/resources'  
import { getAnnouncements } from '../api/announcements'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Separator } from '@/components/ui/separator'
import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { SkeletonCard } from '@/components/ui/skeleton'
import { motion, AnimatePresence } from 'framer-motion'
import { Search, Filter, X, FileText, Megaphone, Package, TrendingUp, Flame, Eye, Heart, RotateCcw, Hash } from 'lucide-react'
import TopNavigation from '@/components/ui/top-navigation'

const SearchScreen: React.FC = () => {
	const [params, setParams] = useSearchParams()
	const navigate = useNavigate()
	const [items, setItems] = useState<any[]>([])
	const [loading, setLoading] = useState(false)
	const [searchValue, setSearchValue] = useState(params.get('q') || '')
	const [selectedType, setSelectedType] = useState<string>('all')
	const [hotItems, setHotItems] = useState<any[]>([])
	const [hotLoading, setHotLoading] = useState(false)
	const [hotSelectedType, setHotSelectedType] = useState<string>('all')
	const q = params.get('q') || ''

	const contentTypes = [
		{ key: 'all', label: '全部', icon: Search },
		{ key: 'post', label: '帖子', icon: FileText },
		{ key: 'resource', label: '资源', icon: Package },
	]

	const getTypeIcon = (type: string) => {
		switch (type) {
			case 'post': return FileText
			case 'resource': return Package  
			case 'announcement': return Megaphone
			default: return FileText
		}
	}

	const getTypeColor = (type: string) => {
		switch (type) {
			case 'post': return 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200'
			case 'resource': return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
			case 'announcement': return 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200'
			default: return 'bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-200'
		}
	}

	const handleSearch = (query: string) => {
		if (query.trim()) {
			setParams({ q: query.trim() })
		}
	}

	const handleTypeFilter = (type: string) => {
		setSelectedType(type)
	}

	const filteredItems = selectedType === 'all' 
		? items 
		: items.filter(item => item.type === selectedType)

	const filteredHotItems = hotSelectedType === 'all' 
		? hotItems 
		: hotItems.filter(item => item.type === hotSelectedType)

	// 加载搜索结果
	useEffect(() => {
		const load = async () => {
			setLoading(true)
			try {
				const data = await searchAll({ query: q, page: 1, pageSize: 20 })
				setItems(data.items || [])
			} finally { setLoading(false) }
		}
		if (q) load()
	}, [q])

	// 计算热度分数的函数
	const calculateHotScore = (item: any) => {
		const views = item.views || 0
		const likes = item.likes || 0
		const comments = item.comments || 0
		const downloads = item.downloads || 0
		
		// 创建时间权重（越新越好，最近7天内有加成）
		const createdAt = item.created_at || item.publishDate || item.date
		let timeWeight = 1
		
		if (createdAt) {
			const now = new Date().getTime()
			const itemTime = new Date(createdAt).getTime()
			const daysDiff = (now - itemTime) / (1000 * 60 * 60 * 24)
			
			if (daysDiff <= 1) timeWeight = 2.0      // 1天内
			else if (daysDiff <= 3) timeWeight = 1.8 // 3天内  
			else if (daysDiff <= 7) timeWeight = 1.5 // 1周内
			else if (daysDiff <= 30) timeWeight = 1.2 // 1月内
			else timeWeight = 0.8                    // 超过1月
		}

		// 计算基础分数 - 权重调整
		let score = 0
		score += views * 0.5        // 浏览量权重提高
		score += likes * 5          // 点赞权重提高 
		score += comments * 8       // 评论权重最高（互动性强）
		score += downloads * 3      // 下载量权重适中
		
		// 应用时间权重
		score *= timeWeight
		
		// 置顶和精华加成
		if (item.isPinned || item.is_pinned) score *= 1.5
		if (item.isHot || item.is_featured) score *= 1.3
		
		// 确保最低分数为浏览量，避免无互动内容分数为0
		score = Math.max(score, views * 0.1)
		
		return Math.round(score)
	}

	// 加载热门榜数据
	const loadHotList = async () => {
		setHotLoading(true)
		try {
			// 并行获取各类型内容（不包括公告）
			const [postsData, resourcesData] = await Promise.all([
				getPosts({ page: 1, pageSize: 20 }).catch(() => ({ list: [] })),
				getResources({ page: 1, page_size: 20 }).catch(() => ({ list: [] }))
			])

			// 统一数据格式并添加类型标识
			const allItems = [
				...(postsData.list || []).map((item: any) => ({
					...item,
					type: 'post',
					description: item.content || item.summary || '',
					views: item.view_count || item.views || 0,
					likes: item.like_count || item.likes || 0,
					comments: item.comment_count || item.comments || 0
				})),
				...(resourcesData.list || []).map((item: any) => ({
					...item,
					type: 'resource',
					title: item.title || item.name || '未命名资源',
					description: item.description || item.summary || '',
					views: item.view_count || item.views || 0,
					likes: item.like_count || item.likes || 0,
					comments: item.comment_count || item.comments || 0,
					downloads: item.download_count || item.downloads || 0
				}))
			]

			// 计算热度分数并排序
			const itemsWithScore = allItems.map(item => ({
				...item,
				hotScore: calculateHotScore(item)
			}))

			// 按热度分数排序，取前10名
			const topHotItems = itemsWithScore
				.sort((a, b) => b.hotScore - a.hotScore)
				.slice(0, 10)

			console.log('热门榜数据加载完成:', {
				totalItems: allItems.length,
				postsCount: postsData.list?.length || 0,
				resourcesCount: resourcesData.list?.length || 0,
				topHotItems: topHotItems.map(item => ({ 
					title: item.title, 
					type: item.type, 
					score: item.hotScore,
					views: item.views,
					likes: item.likes,
					comments: item.comments
				}))
			})

			setHotItems(topHotItems)
		} catch (error) {
			console.error('Failed to load hot list:', error)
			setHotItems([])
		} finally { 
			setHotLoading(false) 
		}
	}

	useEffect(() => {
		loadHotList()
	}, [])

	return (
		<div className="min-h-screen bg-background">
			{/* 顶部导航栏 */}
			<TopNavigation
				title="搜索"
				subtitle={q ? `搜索"${q}"的结果` : "搜索资源、话题、用户"}
				showBackButton
			/>

			{/* 内容区域 */}
			<div className="p-4 pt-nav max-w-4xl mx-auto">
			{/* 搜索框 */}
			<div className="mb-6">
				<div className="relative">
					<Input
						placeholder="搜索资源、话题、用户..."
						className="pl-10 pr-12 py-3 rounded-full"
						value={searchValue}
						onChange={(e) => setSearchValue(e.target.value)}
						onKeyPress={(e) => {
							if (e.key === 'Enter') {
								handleSearch(searchValue)
							}
						}}
					/>
					<Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-muted-foreground" />
					{searchValue && (
						<Button
							variant="ghost"
							size="sm"
							className="absolute right-2 top-1/2 transform -translate-y-1/2 h-8 w-8 p-0"
							onClick={() => setSearchValue('')}
						>
							<X className="h-4 w-4" />
						</Button>
					)}
				</div>
			</div>

			{q && (
				<>
					{/* 搜索结果标题和类型过滤 */}
					<div className="mb-4">
						<h2 className="text-lg font-medium mb-3">搜索"{q}"的结果</h2>
						<div className="flex flex-wrap gap-2">
							{contentTypes.map((type) => {
								const Icon = type.icon
								return (
									<Button
										key={type.key}
										variant="ghost"
										size="sm"
										onClick={() => handleTypeFilter(type.key)}
										className={`flex items-center gap-1 ${
											selectedType === type.key 
												? '!bg-green-500/3 !text-green-600 dark:!text-green-400 hover:!bg-green-500/5' 
												: 'text-muted-foreground hover:text-foreground'
										}`}
									>
										<Icon className="h-4 w-4" />
										{type.label}
									</Button>
								)
							})}
						</div>
					</div>

					{/* 搜索结果 */}
					{loading ? (
						<div className="space-y-4">
							{[...Array(3)].map((_, index) => (
								<SkeletonCard key={index} />
							))}
						</div>
					) : (
						<div className="space-y-3">
							{filteredItems.length > 0 ? (
								filteredItems.map((it, idx) => {
									const Icon = getTypeIcon(it.type)
									return (
										<motion.div
											key={`${it.type}-${it.id}-${idx}`}
											initial={{ opacity: 0, y: 20 }}
											animate={{ opacity: 1, y: 0 }}
											transition={{ delay: idx * 0.1 }}
										>
											<Card 
												className="cursor-pointer hover:shadow-md transition-shadow duration-200" 
												onClick={() => {
													if (it.type === 'post') navigate(`/post/${it.id}`)
													else if (it.type === 'resource') navigate(`/resource/${it.id}`)
													else if (it.type === 'announcement') navigate(`/announcement/${it.id}`)
												}}
											>
												<CardContent className="p-4">
													<div className="flex items-start gap-3">
														<div className="flex-shrink-0 mt-1">
															<div className={`p-2 rounded-lg ${getTypeColor(it.type)}`}>
																<Icon className="h-4 w-4" />
															</div>
														</div>
														<div className="flex-1 min-w-0">
															<h3 className="font-medium text-foreground mb-1 line-clamp-1">
																{it.title}
															</h3>
															{it.description && (
																<p className="text-sm text-muted-foreground line-clamp-2">
																	{it.description}
																</p>
															)}
														</div>
													</div>
												</CardContent>
											</Card>
										</motion.div>
									)
								})
							) : (
								<div className="text-center py-12">
									<Search className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
									<h3 className="text-lg font-medium mb-2">没有找到相关内容</h3>
									<p className="text-muted-foreground">
										试试其他关键词或检查拼写
									</p>
								</div>
							)}
						</div>
					)}
				</>
			)}

			{!q && (
				<div className="space-y-6">
					{/* 排行榜 - 重新设计 */}
					<div className="space-y-4">
											{/* 排行榜标题区域 */}
					<div className="flex items-center justify-between">
						<div className="flex items-center gap-2 sm:gap-3">
							<div className="flex items-center gap-2">
								<TrendingUp className="h-5 w-5 text-muted-foreground" />
								<h2 className="text-lg sm:text-xl font-semibold">热门排行</h2>
							</div>
							<Badge variant="secondary" className="text-xs hidden sm:inline-flex">
								基于热度实时更新
							</Badge>
						</div>
						<Button
							variant="ghost"
							size="sm"
							onClick={loadHotList}
							disabled={hotLoading}
							className="flex items-center gap-2 hover:bg-accent"
						>
							<RotateCcw className={`h-4 w-4 ${hotLoading ? 'animate-spin' : ''}`} />
							<span className="hidden sm:inline">刷新</span>
						</Button>
					</div>

						{/* 类型过滤器 - 重新设计 */}
						<div className="flex flex-wrap gap-2">
							{contentTypes.map((type) => {
								const Icon = type.icon
								const isActive = hotSelectedType === type.key
								return (
									<Button
										key={type.key}
										variant="ghost"
										size="sm"
										onClick={() => setHotSelectedType(type.key)}
										className={`flex items-center gap-2 transition-colors ${
											isActive 
												? '!bg-green-500/3 !text-green-600 dark:!text-green-400 hover:!bg-green-500/5' 
												: 'text-muted-foreground hover:text-foreground'
										}`}
									>
										<Icon className="h-4 w-4" />
										<span>{type.label}</span>
									</Button>
								)
							})}
						</div>

						{/* 排行榜内容 - 重新设计 */}
						<div className="space-y-2">
							{hotLoading ? (
								<div className="space-y-3">
									{[...Array(5)].map((_, index) => (
										<div key={index} className="flex items-center gap-4 p-4 rounded-lg border bg-card animate-pulse">
											<div className="w-8 h-8 bg-muted rounded-full shrink-0" />
											<div className="flex-1 space-y-2">
												<div className="h-4 bg-muted rounded w-3/4" />
												<div className="h-3 bg-muted rounded w-1/2" />
											</div>
											<div className="w-12 h-4 bg-muted rounded" />
										</div>
									))}
								</div>
							) : (
								<div className="space-y-2">
									{filteredHotItems.length > 0 ? (
										filteredHotItems.map((item, index) => {
											const Icon = getTypeIcon(item.type)
											const isTopThree = index < 3
											return (
												<motion.div
													key={`hot-${item.type}-${item.id}-${index}`}
													initial={{ opacity: 0, y: 10 }}
													animate={{ opacity: 1, y: 0 }}
													transition={{ delay: index * 0.05 }}
													className="group flex items-center gap-3 sm:gap-4 p-3 sm:p-4 rounded-lg border transition-all duration-200 cursor-pointer bg-card hover:bg-green-500/5 hover:border-green-200"
													onClick={() => navigate(`/${item.type}/${item.id}`)}
												>
													{/* 排名指示器 */}
													<div className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold shrink-0 transition-all duration-200 group-hover:scale-105
														${index === 0 
															? 'bg-amber-100 text-amber-700 border border-amber-200' :
														  index === 1 
															? 'bg-slate-100 text-slate-700 border border-slate-200' :
														  index === 2 
															? 'bg-orange-100 text-orange-700 border border-orange-200' :
														  'bg-muted text-muted-foreground border border-border'
														}`}
													>
														{index + 1}
													</div>

													{/* 内容信息 */}
													<div className="flex-1 min-w-0">
														<div className="flex items-start justify-between gap-3">
															<div className="flex-1 min-w-0">
																<div className="flex items-center gap-2 mb-1">
																	<Icon className={`h-4 w-4 shrink-0 ${getTypeColor(item.type).includes('blue') ? 'text-blue-500' : 
																		getTypeColor(item.type).includes('green') ? 'text-green-500' : 'text-orange-500'}`} />
																	<h3 className="font-medium text-foreground line-clamp-1 group-hover:text-primary transition-colors">
																		{item.title}
																	</h3>
																</div>
																<div className="flex items-center gap-3 sm:gap-4 text-xs text-muted-foreground">
																	<div className="flex items-center gap-1">
																		<Eye className="h-3 w-3" />
																		<span>{item.views >= 1000 ? `${(item.views / 1000).toFixed(1)}k` : item.views}</span>
																	</div>
																	<div className="flex items-center gap-1">
																		<Heart className="h-3 w-3" />
																		<span>{item.likes >= 1000 ? `${(item.likes / 1000).toFixed(1)}k` : item.likes}</span>
																	</div>
																	{item.type === 'resource' && item.downloads > 0 && (
																		<div className="flex items-center gap-1 hidden sm:flex">
																			<Package className="h-3 w-3" />
																			<span>{item.downloads >= 1000 ? `${(item.downloads / 1000).toFixed(1)}k` : item.downloads}</span>
																		</div>
																	)}
																</div>
															</div>
															{/* 热度分数 */}
															<div className="flex items-center gap-1 px-2 py-1 bg-muted/50 rounded-md shrink-0">
																<Flame className="h-3 w-3 text-muted-foreground" />
																<span className="text-xs font-medium text-muted-foreground">
																	{item.hotScore >= 1000 ? `${(item.hotScore / 1000).toFixed(1)}k` : item.hotScore}
																</span>
															</div>
														</div>
													</div>
												</motion.div>
											)
										})
									) : (
										<Card className="border-dashed">
											<CardContent className="flex flex-col items-center justify-center py-12">
												<div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center mb-4">
													<TrendingUp className="h-8 w-8 text-muted-foreground" />
												</div>
												<h3 className="text-lg font-medium text-foreground mb-2">暂无排行数据</h3>
												<p className="text-sm text-muted-foreground text-center max-w-sm">
													当前还没有足够的数据来生成热门排行榜，请稍后再试
												</p>
											</CardContent>
										</Card>
									)}
								</div>
							)}
						</div>
					</div>

					{/* 搜索提示 - 优化设计 */}
					<div className="flex flex-col items-center justify-center py-12">
						<div className="w-16 h-16 bg-muted/20 rounded-full flex items-center justify-center mb-4">
							<Search className="h-8 w-8 text-muted-foreground" />
						</div>
						<h3 className="text-lg font-medium mb-2">开始探索发现</h3>
						<p className="text-muted-foreground text-sm text-center max-w-sm">
							在上方搜索框中输入关键词，搜索资源、话题或用户内容
						</p>
					</div>
				</div>
			)}
			</div>
		</div>
	)
}

export default SearchScreen 