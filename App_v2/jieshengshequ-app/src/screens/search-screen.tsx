import React, { useEffect, useState } from 'react'
import { useSearchParams, useNavigate } from 'react-router-dom'
import { searchAll } from '../api/search'
import { Card, CardContent } from '@/components/ui/card'

const SearchScreen: React.FC = () => {
	const [params] = useSearchParams()
	const navigate = useNavigate()
	const [items, setItems] = useState<any[]>([])
	const [loading, setLoading] = useState(false)
	const q = params.get('q') || ''

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

	return (
		<div className="p-4 pt-nav">
			<h2 className="text-lg font-medium mb-3">搜索“{q}”的结果</h2>
			{loading ? <div>加载中...</div> : (
				<div className="space-y-3">
					{items.map((it, idx) => (
						<Card key={idx} className="cursor-pointer" onClick={() => {
							if (it.type === 'post') navigate(`/post/${it.id}`)
							else if (it.type === 'resource') navigate(`/resource/${it.id}`)
							else if (it.type === 'announcement') navigate(`/announcement/${it.id}`)
						}}>
							<CardContent className="p-3">
								<div className="text-sm text-muted-foreground">{it.type}</div>
								<div className="font-medium">{it.title}</div>
								{it.description && <div className="text-sm text-muted-foreground line-clamp-2">{it.description}</div>}
							</CardContent>
						</Card>
					))}
				</div>
			)}
		</div>
	)
}

export default SearchScreen 