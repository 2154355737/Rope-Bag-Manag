import React from 'react'
import { cn } from '@/lib/utils'

// Markdown相关导入
import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import rehypeHighlight from 'rehype-highlight'
import rehypeRaw from 'rehype-raw'

// 导入highlight.js样式
import 'highlight.js/styles/github.css'

interface MarkdownRendererProps {
  content: string
  className?: string
  onImageClick?: (images: string[], index: number) => void
}

const MarkdownRenderer: React.FC<MarkdownRendererProps> = ({ 
  content, 
  className,
  onImageClick 
}) => {
  // 检测内容类型
  const isHtmlContent = content.includes('<') && content.includes('>')
  const isMarkdownContent = content.includes('#') || 
                           content.includes('```') || 
                           content.includes('**') || 
                           content.includes('*') ||
                           content.includes('[') ||
                           content.includes('|')

  // 提取内容中的图片URL
  const extractImages = (text: string): string[] => {
    const imageRegex = /!\[.*?\]\((.*?)\)|<img[^>]+src="([^"]*)"[^>]*>/g
    const images: string[] = []
    let match
    
    while ((match = imageRegex.exec(text)) !== null) {
      images.push(match[1] || match[2])
    }
    
    return images
  }

  // 自定义组件渲染
  const components = {
    img: ({ node, src, alt, ...props }: any) => {
      const allImages = extractImages(content)
      const imageIndex = allImages.findIndex(img => img === src)
      
      return (
        <img
          src={src}
          alt={alt}
          className="max-w-full h-auto rounded-lg cursor-pointer hover:opacity-80 transition-opacity my-4"
          onClick={() => onImageClick && onImageClick(allImages, Math.max(0, imageIndex))}
          {...props}
        />
      )
    },
    
    a: ({ node, href, children, ...props }: any) => (
      <a
        href={href}
        target="_blank"
        rel="noopener noreferrer"
        className="text-primary hover:text-primary/80 underline break-all"
        {...props}
      >
        {children}
      </a>
    ),
    
    code: ({ node, inline, className, children, ...props }: any) => {
      const match = /language-(\w+)/.exec(className || '')
      
      if (inline) {
        return (
          <code 
            className="bg-muted px-1.5 py-0.5 rounded text-sm font-mono text-foreground"
            {...props}
          >
            {children}
          </code>
        )
      }
      
      return (
        <code 
          className={cn("block bg-muted p-4 rounded-lg text-sm font-mono overflow-x-auto whitespace-pre-wrap break-all my-4", className)}
          {...props}
        >
          {children}
        </code>
      )
    },
    
    pre: ({ node, children, ...props }: any) => (
      <pre className="bg-muted p-4 rounded-lg overflow-x-auto whitespace-pre-wrap break-all my-4" {...props}>
        {children}
      </pre>
    ),
    
    table: ({ node, children, ...props }: any) => (
      <div className="overflow-x-auto my-4">
        <table className="min-w-full border-collapse border border-border" {...props}>
          {children}
        </table>
      </div>
    ),
    
    th: ({ node, children, ...props }: any) => (
      <th className="border border-border px-4 py-2 bg-muted font-semibold text-left" {...props}>
        {children}
      </th>
    ),
    
    td: ({ node, children, ...props }: any) => (
      <td className="border border-border px-4 py-2" {...props}>
        {children}
      </td>
    ),
    
    blockquote: ({ node, children, ...props }: any) => (
      <blockquote className="border-l-4 border-primary/30 pl-4 my-4 italic text-muted-foreground" {...props}>
        {children}
      </blockquote>
    ),
    
    h1: ({ node, children, ...props }: any) => (
      <h1 className="text-2xl font-bold mt-8 mb-4 pb-2 border-b border-border" {...props}>
        {children}
      </h1>
    ),
    
    h2: ({ node, children, ...props }: any) => (
      <h2 className="text-xl font-semibold mt-6 mb-3" {...props}>
        {children}
      </h2>
    ),
    
    h3: ({ node, children, ...props }: any) => (
      <h3 className="text-lg font-semibold mt-5 mb-2" {...props}>
        {children}
      </h3>
    ),
    
    h4: ({ node, children, ...props }: any) => (
      <h4 className="text-base font-semibold mt-4 mb-2" {...props}>
        {children}
      </h4>
    ),
    
    p: ({ node, children, ...props }: any) => (
      <p className="mb-4 leading-relaxed break-words" {...props}>
        {children}
      </p>
    ),
    
    ul: ({ node, children, ...props }: any) => (
      <ul className="list-disc list-inside mb-4 space-y-1" {...props}>
        {children}
      </ul>
    ),
    
    ol: ({ node, children, ...props }: any) => (
      <ol className="list-decimal list-inside mb-4 space-y-1" {...props}>
        {children}
      </ol>
    ),
    
    li: ({ node, children, ...props }: any) => (
      <li className="ml-4" {...props}>
        {children}
      </li>
    )
  }

  // 如果内容为空
  if (!content || content.trim() === '') {
    return (
      <div className={cn("text-muted-foreground text-center py-8", className)}>
        <p>暂无内容</p>
      </div>
    )
  }

  try {
    // 根据内容类型选择渲染策略
    if (isHtmlContent || isMarkdownContent) {
      return (
        <div className={cn("prose prose-sm max-w-none dark:prose-invert break-words overflow-hidden", className)}>
          <ReactMarkdown
            components={components}
            remarkPlugins={[remarkGfm]}
            rehypePlugins={isHtmlContent ? [rehypeRaw, rehypeHighlight] : [rehypeHighlight]}
          >
            {content}
          </ReactMarkdown>
        </div>
      )
    }
    
    // 纯文本渲染
    return (
      <div className={cn("whitespace-pre-wrap text-foreground break-words", className)}>
        {content}
      </div>
    )
    
  } catch (error) {
    console.warn('Markdown渲染出错，回退到基础渲染:', error)
    
    // 回退到基础文本渲染
    return (
      <div className={cn("whitespace-pre-wrap text-foreground break-words", className)}>
        {content}
      </div>
    )
  }
}

export default MarkdownRenderer 