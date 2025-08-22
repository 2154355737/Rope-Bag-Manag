import React, { useState } from 'react'
import { ArrowLeft, Send, FileText, Package, Upload, Hash, Folder, Tag, Code, Image, Bold, Italic, Link, Camera, X, Eye, Edit3, Info, Clock, CheckCircle, AlertCircle, Monitor, Smartphone, Globe, Settings, Star, List, Plus } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Textarea } from '@/components/ui/textarea'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Badge } from '@/components/ui/badge'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Separator } from '@/components/ui/separator'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { toast } from 'sonner'
import TopNavigation from '@/components/ui/top-navigation'
import { getCategories, Category } from '@/api/categories'

type PublishType = 'resource' | 'post'

const PublishScreen: React.FC = () => {
  const navigate = useNavigate()
  const [publishType, setPublishType] = useState<PublishType>('resource')
  const [isPublishing, setIsPublishing] = useState(false)

  // 通用字段
  const [title, setTitle] = useState('')
  const [content, setContent] = useState('')
  const [tags, setTags] = useState<string[]>([])
  const [newTag, setNewTag] = useState('')

  // 资源专用字段
  const [version, setVersion] = useState('')
  const [category, setCategory] = useState('')
  const [files, setFiles] = useState<File[]>([])
  const [screenshots, setScreenshots] = useState<File[]>([])
  const [requirements, setRequirements] = useState<string[]>([])
  const [newRequirement, setNewRequirement] = useState('')

  // 帖子专用字段  
  const [images, setImages] = useState<File[]>([])
  const [codeSnippet, setCodeSnippet] = useState('')
  const [showCodeEditor, setShowCodeEditor] = useState(false)
  
  // 富文本编辑器状态
  const [showPreview, setShowPreview] = useState(false)

  // 分类数据
  const [categories, setCategories] = useState<Category[]>([])
  const [loadingCategories, setLoadingCategories] = useState(true)

  // 加载分类数据
  React.useEffect(() => {
    const loadCategories = async () => {
      try {
        setLoadingCategories(true)
        const categoryList = await getCategories()
        setCategories(categoryList)
      } catch (error) {
        console.error('加载分类失败:', error)
        toast.error('加载分类失败')
        // 使用备用分类数据
        setCategories([
          { id: 1, name: '开发工具' },
          { id: 2, name: '库/框架' },
          { id: 3, name: '模板' },
          { id: 4, name: '插件' },
          { id: 5, name: '素材资源' },
          { id: 6, name: '文档教程' },
          { id: 7, name: '其他' }
        ])
      } finally {
        setLoadingCategories(false)
      }
    }
    
    loadCategories()
  }, [])

  const handleAddTag = () => {
    if (newTag.trim() && !tags.includes(newTag.trim())) {
      setTags([...tags, newTag.trim()])
      setNewTag('')
    }
  }

  const handleRemoveTag = (tagToRemove: string) => {
    setTags(tags.filter(tag => tag !== tagToRemove))
  }

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const uploadedFiles = event.target.files
    if (uploadedFiles) {
      setFiles([...files, ...Array.from(uploadedFiles)])
    }
  }

  const handleImageUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const uploadedImages = event.target.files
    if (uploadedImages) {
      setImages([...images, ...Array.from(uploadedImages)])
    }
  }

  const handleScreenshotUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const uploadedScreenshots = event.target.files
    if (uploadedScreenshots) {
      setScreenshots([...screenshots, ...Array.from(uploadedScreenshots)])
    }
  }

  const handleAddRequirement = () => {
    if (newRequirement.trim() && !requirements.includes(newRequirement.trim()) && requirements.length < 10) {
      setRequirements([...requirements, newRequirement.trim()])
      setNewRequirement('')
    }
  }

  const handleRemoveRequirement = (reqToRemove: string) => {
    setRequirements(requirements.filter(req => req !== reqToRemove))
  }

  const removeScreenshot = (index: number) => {
    setScreenshots(screenshots.filter((_, i) => i !== index))
  }

  const handleRemoveFile = (index: number) => {
    setFiles(files.filter((_, i) => i !== index))
  }

  const handleRemoveImage = (index: number) => {
    setImages(images.filter((_, i) => i !== index))
  }

  // Markdown工具栏功能
  const insertMarkdown = (syntax: string, placeholder = '') => {
    const textarea = document.getElementById('content-editor') as HTMLTextAreaElement
    if (!textarea) return

    const start = textarea.selectionStart
    const end = textarea.selectionEnd
    const selectedText = content.substring(start, end)
    
    let newText = ''
    switch (syntax) {
      case 'bold':
        newText = `**${selectedText || placeholder || '粗体文字'}**`
        break
      case 'italic':
        newText = `*${selectedText || placeholder || '斜体文字'}*`
        break
      case 'code':
        newText = `\`${selectedText || placeholder || '代码'}\``
        break
      case 'link':
        newText = `[${selectedText || '链接文字'}](${placeholder || 'https://'})`
        break
      case 'image':
        newText = `![${selectedText || '图片描述'}](${placeholder || '图片链接'})`
        break
      case 'codeblock':
        newText = `\`\`\`\n${selectedText || placeholder || '代码块'}\n\`\`\``
        break
      default:
        return
    }

    const newContent = content.substring(0, start) + newText + content.substring(end)
    setContent(newContent)
    
    // 重新聚焦并设置光标位置
    setTimeout(() => {
      textarea.focus()
      const newCursorPos = start + newText.length
      textarea.setSelectionRange(newCursorPos, newCursorPos)
    }, 0)
  }

  const renderMarkdownPreview = (text: string) => {
    return text
      .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
      .replace(/\*(.*?)\*/g, '<em>$1</em>')
      .replace(/`(.*?)`/g, '<code class="bg-muted px-1 rounded">$1</code>')
      .replace(/```\n([\s\S]*?)\n```/g, '<pre class="bg-muted p-3 rounded-md overflow-x-auto"><code>$1</code></pre>')
      .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" class="text-primary underline">$1</a>')
      .replace(/!\[([^\]]*)\]\(([^)]+)\)/g, '<img src="$2" alt="$1" class="max-w-full h-auto rounded" />')
      .replace(/\n/g, '<br>')
  }

  const handlePublish = async () => {
    if (!title.trim() || !content.trim()) {
      toast.error('请填写标题和内容')
      return
    }
    
    if (isPublishing) return // 防止重复提交
    if (publishType === 'resource') {
      if (!version.trim() || !category) {
        toast.error('请填写版本信息和选择分类')
        return
      }
      // 资源文件现在是可选的，不再强制要求上传
    }

    setIsPublishing(true)
    
    try {
      let response: any
      
      if (publishType === 'resource') {
        // 发布资源
        const { publishResource } = await import('@/api/publish')
        response = await publishResource({
          title,
          content,
          version,
          category,
          tags,
          requirements,
          files: files.map(f => ({ name: f.name, size: f.size, type: f.type })),
          screenshots: screenshots.map(s => ({ name: s.name, size: s.size }))
        })
        
        // 如果有文件，逐个上传
        if (files.length > 0) {
          const { uploadResourceFile } = await import('@/api/publish')
          for (const file of files) {
            try {
              await uploadResourceFile(response.id, file)
            } catch (error) {
              console.warn(`文件 ${file.name} 上传失败:`, error)
            }
          }
        }
        
        // 如果有截图，上传截图
        if (screenshots.length > 0) {
          const { uploadImage } = await import('@/api/publish')
          for (const screenshot of screenshots) {
            try {
              await uploadImage(screenshot, response.id)
            } catch (error) {
              console.warn(`截图 ${screenshot.name} 上传失败:`, error)
            }
          }
        }
      } else {
        // 发布帖子
        const { publishPost } = await import('@/api/publish')
        response = await publishPost({
          title,
          content,
          tags,
          images: images.map(img => ({ name: img.name, size: img.size })),
          code_snippet: codeSnippet
        })
        
        // 如果有图片，上传图片
        if (images.length > 0) {
          const { uploadPostImage } = await import('@/api/publish')
          for (const image of images) {
            try {
              await uploadPostImage(image, response.id)
            } catch (error) {
              console.warn(`图片 ${image.name} 上传失败:`, error)
            }
          }
        }
      }
      
      // 显示成功提示
      const isResource = publishType === 'resource'
      toast.success(`${isResource ? '资源' : '帖子'}发布成功！`, {
        description: isResource ? '内容已提交审核，审核通过后将自动发布' : '帖子已发布',
        duration: 5000,
      })
      
      // 重置表单
      setTitle('')
      setContent('')
      setTags([])
      setVersion('')
      setCategory('')
      setFiles([])
      setImages([])
      setScreenshots([])
      setRequirements([])
      setCodeSnippet('')
      
      // 延迟跳转，让用户看到成功提示
      setTimeout(() => {
        navigate('/')
      }, 2000)
      
    } catch (error: any) {
      console.error('发布失败:', error)
      toast.error(error.message || '发布失败，请重试')
    } finally {
      setIsPublishing(false)
    }
  }

  return (
    <div className="min-h-screen bg-background">
      {/* 顶部导航栏 */}
      <TopNavigation
        title="发布内容"
        subtitle={publishType === 'resource' ? '分享资源' : '发布帖子'}
        showBackButton
        rightAction={
          <Button
            onClick={handlePublish}
            disabled={!title || !content || isPublishing}
            className="px-6"
          >
            {isPublishing ? (
              <>
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2" />
                发布中...
              </>
            ) : (
              <>
                <Send size={16} className="mr-2" />
                发布
              </>
            )}
          </Button>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        <div className="container max-w-2xl mx-auto p-4 space-y-6">
        {/* 发布类型选择 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">发布类型</CardTitle>
          </CardHeader>
          <CardContent>
            <Tabs value={publishType} onValueChange={(value) => setPublishType(value as PublishType)}>
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="resource" className="flex items-center gap-2">
                  <Package className="h-4 w-4" />
                  资源
                </TabsTrigger>
                <TabsTrigger value="post" className="flex items-center gap-2">
                  <FileText className="h-4 w-4" />
                  帖子
                </TabsTrigger>
              </TabsList>
            </Tabs>
          </CardContent>
        </Card>

        {/* 温馨提示 */}
        <Alert>
          <Clock className="h-4 w-4" />
          <AlertDescription>
            <div className="space-y-2">
              <div className="font-medium text-sm">📋 发布须知</div>
              <ul className="text-xs space-y-1 text-muted-foreground">
                <li>• {publishType === 'resource' ? '资源' : '帖子'}发布后需要管理员审核，通常在1-24小时内完成</li>
                <li>• 请确保内容原创或已获得授权，避免版权纠纷</li>
                <li>• {publishType === 'resource' ? '如需上传文件请确保无病毒，建议压缩打包' : '如需上传图片建议压缩后上传，单张不超过5MB'}</li>
                <li>• 标题和内容请使用规范语言，避免敏感词汇</li>
              </ul>
            </div>
          </AlertDescription>
        </Alert>

        {publishType === 'resource' && (
          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription>
              <div className="space-y-2">
                <div className="font-medium text-sm">💡 资源发布建议</div>
                <ul className="text-xs space-y-1 text-muted-foreground">
                  <li>• 详细描述资源的功能特性和使用场景</li>
                  <li>• 提供清晰的安装或使用说明</li>
                  <li>• 如有依赖项，请在描述中说明</li>
                  <li>• 建议提供截图或演示视频链接</li>
                </ul>
              </div>
            </AlertDescription>
          </Alert>
        )}

        {publishType === 'post' && (
          <Alert>
            <Info className="h-4 w-4" />
            <AlertDescription>
              <div className="space-y-2">
                <div className="font-medium text-sm">✍️ 帖子发布建议</div>
                <ul className="text-xs space-y-1 text-muted-foreground">
                  <li>• 使用Markdown语法让内容更美观</li>
                  <li>• 代码块请标注编程语言类型</li>
                  <li>• 添加合适的标签便于其他用户发现</li>
                  <li>• 鼓励分享技术经验和学习心得</li>
                </ul>
              </div>
            </AlertDescription>
          </Alert>
        )}
      </div>

      {/* 主要内容 */}
      <div className="container max-w-2xl mx-auto px-4 space-y-6">
        {/* 标题 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base">标题</CardTitle>
          </CardHeader>
          <CardContent>
            <Input
              id="publish-title"
              name="title"
              placeholder={publishType === 'resource' ? '输入资源名称...' : '输入帖子标题...'}
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              maxLength={100}
              autoComplete="off"
            />
            <div className="text-xs text-muted-foreground mt-2">
              {title.length}/100
            </div>
          </CardContent>
        </Card>

        {/* 资源专用字段 */}
        {publishType === 'resource' && (
          <>
            <Card>
              <CardHeader>
                <CardTitle className="text-base">版本信息</CardTitle>
              </CardHeader>
              <CardContent>
                <Input
                  id="resource-version"
                  name="version"
                  placeholder="例如：v1.0.0, 2024.1, beta-1.2"
                  value={version}
                  onChange={(e) => setVersion(e.target.value)}
                  autoComplete="off"
                />
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle className="text-base flex items-center">
                  <Folder className="h-4 w-4 mr-2" />
                  分类
                </CardTitle>
              </CardHeader>
              <CardContent>
                <Select value={category} onValueChange={setCategory}>
                  <SelectTrigger>
                    <SelectValue placeholder="选择资源分类" />
                  </SelectTrigger>
                  <SelectContent>
                    {categories.map((cat) => (
                      <SelectItem key={cat.id} value={cat.id.toString()}>
                        {cat.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle className="text-base flex items-center">
                  <Upload className="h-4 w-4 mr-2" />
                  资源文件（可选）
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {files.length > 0 && (
                    <div className="space-y-2">
                      {files.map((file, index) => (
                        <div key={index} className="flex items-center justify-between p-2 bg-muted rounded-lg">
                          <div className="flex items-center gap-2">
                            <FileText className="h-4 w-4 text-muted-foreground" />
                            <span className="text-sm">{file.name}</span>
                            <span className="text-xs text-muted-foreground">
                              ({(file.size / 1024 / 1024).toFixed(2)} MB)
                            </span>
                          </div>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleRemoveFile(index)}
                          >
                            <X className="h-4 w-4" />
                          </Button>
                        </div>
                      ))}
                    </div>
                  )}
                  <label className="flex items-center justify-center w-full h-32 border-2 border-dashed border-muted-foreground/25 rounded-lg cursor-pointer hover:bg-muted/50 transition-colors">
                    <div className="text-center">
                      <Upload className="h-8 w-8 mx-auto mb-2 text-muted-foreground" />
                      <p className="text-sm text-muted-foreground">点击上传资源文件</p>
                      <p className="text-xs text-muted-foreground mt-1">支持所有文件格式，单文件不超过100MB</p>
                    </div>
                    <input
                      type="file"
                      multiple
                      onChange={handleFileUpload}
                      className="hidden"
                    />
                  </label>
                  <Alert>
                    <AlertCircle className="h-4 w-4" />
                    <AlertDescription className="text-xs">
                      <strong>文件上传提醒：</strong>
                      <ul className="mt-1 space-y-0.5">
                        <li>• 建议将多个文件压缩为ZIP/RAR格式上传</li>
                        <li>• 请确保文件安全，避免包含恶意代码</li>
                        <li>• 大文件建议使用网盘分享链接</li>
                      </ul>
                    </AlertDescription>
                  </Alert>
                </div>
              </CardContent>
            </Card>

            {/* 预览截图 */}
            <Card>
              <CardHeader>
                <CardTitle className="text-base flex items-center">
                  <Monitor className="h-4 w-4 mr-2" />
                  预览截图（可选）
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {screenshots.length > 0 && (
                    <div className="grid grid-cols-2 gap-3">
                      {screenshots.map((screenshot, index) => (
                        <div key={index} className="relative">
                          <img
                            src={URL.createObjectURL(screenshot)}
                            alt={`Screenshot ${index + 1}`}
                            className="w-full h-32 object-cover rounded-lg border"
                          />
                          <Button
                            variant="destructive"
                            size="sm"
                            className="absolute top-1 right-1 h-6 w-6 p-0"
                            onClick={() => removeScreenshot(index)}
                          >
                            <X className="h-3 w-3" />
                          </Button>
                        </div>
                      ))}
                    </div>
                  )}
                  <label className="flex items-center justify-center w-full h-32 border-2 border-dashed border-muted-foreground/25 rounded-lg cursor-pointer hover:bg-muted/50 transition-colors">
                    <div className="text-center">
                      <Camera className="h-8 w-8 mx-auto mb-2 text-muted-foreground" />
                      <p className="text-sm text-muted-foreground">添加预览截图</p>
                      <p className="text-xs text-muted-foreground mt-1">JPG/PNG格式，建议尺寸16:9</p>
                    </div>
                    <input
                      type="file"
                      multiple
                      accept="image/jpeg,image/png"
                      onChange={handleScreenshotUpload}
                      className="hidden"
                    />
                  </label>
                  <Alert>
                    <Info className="h-4 w-4" />
                    <AlertDescription className="text-xs">
                      <strong>截图建议：</strong>
                      <ul className="mt-1 space-y-0.5">
                        <li>• 展示资源的主要功能和界面</li>
                        <li>• 图片清晰，尺寸统一</li>
                        <li>• 建议3-5张截图，突出亮点</li>
                      </ul>
                    </AlertDescription>
                  </Alert>
                </div>
              </CardContent>
            </Card>

            {/* 系统要求 */}
            <Card>
              <CardHeader>
                <CardTitle className="text-base flex items-center">
                  <Settings className="h-4 w-4 mr-2" />
                  系统要求
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {requirements.length > 0 && (
                    <div className="space-y-2">
                      {requirements.map((req, index) => (
                        <div key={index} className="flex items-center justify-between p-2 bg-muted rounded-lg">
                          <div className="flex items-center gap-2">
                            <CheckCircle className="h-4 w-4 text-green-500" />
                            <span className="text-sm">{req}</span>
                          </div>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={() => handleRemoveRequirement(req)}
                          >
                            <X className="h-4 w-4" />
                          </Button>
                        </div>
                      ))}
                    </div>
                  )}
                  <div className="flex gap-2">
                    <Input
                      placeholder="例如：Node.js >= 16.0.0"
                      value={newRequirement}
                      onChange={(e) => setNewRequirement(e.target.value)}
                      onKeyPress={(e) => e.key === 'Enter' && handleAddRequirement()}
                    />
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={handleAddRequirement}
                      disabled={!newRequirement.trim()}
                    >
                      <Plus className="h-4 w-4" />
                    </Button>
                  </div>
                  <Alert>
                    <Info className="h-4 w-4" />
                    <AlertDescription className="text-xs">
                      <strong>要求说明：</strong>
                      <ul className="mt-1 space-y-0.5">
                        <li>• 列出运行环境和依赖版本</li>
                        <li>• 包括操作系统、软件版本等</li>
                        <li>• 帮助用户判断兼容性</li>
                      </ul>
                    </AlertDescription>
                  </Alert>
                </div>
              </CardContent>
            </Card>
          </>
        )}

        {/* 内容 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center justify-between">
              内容
              {publishType === 'post' && (
                <div className="flex gap-2">
                  <Button
                    variant={showPreview ? "outline" : "default"}
                    size="sm"
                    onClick={() => setShowPreview(false)}
                  >
                    <Edit3 className="h-4 w-4 mr-1" />
                    编辑
                  </Button>
                  <Button
                    variant={showPreview ? "default" : "outline"}
                    size="sm"
                    onClick={() => setShowPreview(true)}
                  >
                    <Eye className="h-4 w-4 mr-1" />
                    预览
                  </Button>
                </div>
              )}
            </CardTitle>
          </CardHeader>
          <CardContent>
            {publishType === 'post' && !showPreview && (
              <div className="space-y-3">
                {/* 富文本工具栏 */}
                <div className="flex flex-wrap gap-2 p-2 bg-muted rounded-lg">
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => insertMarkdown('bold', '粗体文字')}
                  >
                    <Bold className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => insertMarkdown('italic', '斜体文字')}
                  >
                    <Italic className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => insertMarkdown('code', '代码')}
                  >
                    <Code className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => insertMarkdown('codeblock', '代码块内容')}
                  >
                    <Code className="h-4 w-4" />
                    <span className="ml-1 text-xs">块</span>
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => insertMarkdown('link', '链接文字')}
                  >
                    <Link className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => insertMarkdown('image', '图片描述')}
                  >
                    <Image className="h-4 w-4" />
                  </Button>
                </div>
                <Separator />
              </div>
            )}
            
            {publishType === 'post' && showPreview ? (
              <div 
                className="min-h-[120px] p-3 border rounded-md prose prose-sm max-w-none"
                dangerouslySetInnerHTML={{ __html: renderMarkdownPreview(content) }}
              />
            ) : (
              <Textarea
                id="content-editor"
                placeholder={
                  publishType === 'resource' 
                    ? '详细描述资源的功能、使用方法、注意事项等...'
                    : '支持Markdown格式，可以插入代码、图片、链接等...'
                }
                value={content}
                onChange={(e) => setContent(e.target.value)}
                className="min-h-[120px] resize-none"
                maxLength={publishType === 'resource' ? 2000 : 5000}
              />
            )}
            <div className="text-xs text-muted-foreground mt-2">
              {content.length}/{publishType === 'resource' ? 2000 : 5000}
            </div>
          </CardContent>
        </Card>

        {/* 帖子专用：代码片段 */}
        {publishType === 'post' && (
          <Card>
            <CardHeader>
              <CardTitle className="text-base flex items-center justify-between">
                <div className="flex items-center">
                  <Code className="h-4 w-4 mr-2" />
                  代码片段（可选）
                </div>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => setShowCodeEditor(!showCodeEditor)}
                >
                  {showCodeEditor ? '收起' : '展开'}
                </Button>
              </CardTitle>
            </CardHeader>
            {showCodeEditor && (
              <CardContent>
                <div className="space-y-3">
                  <Textarea
                    placeholder="粘贴代码片段，支持多种编程语言..."
                    value={codeSnippet}
                    onChange={(e) => setCodeSnippet(e.target.value)}
                    className="font-mono text-sm min-h-[200px]"
                    maxLength={5000}
                  />
                  <div className="flex justify-between items-center">
                    <div className="text-xs text-muted-foreground">
                      {codeSnippet.length}/5000 字符
                    </div>
                    <div className="flex gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => insertMarkdown('codeblock', codeSnippet)}
                        disabled={!codeSnippet.trim()}
                      >
                        插入到内容
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => setCodeSnippet('')}
                        disabled={!codeSnippet.trim()}
                      >
                        清空
                      </Button>
                    </div>
                  </div>
                  <Alert>
                    <Info className="h-4 w-4" />
                    <AlertDescription className="text-xs">
                      <strong>代码建议：</strong>
                      <ul className="mt-1 space-y-0.5">
                        <li>• 添加注释说明代码功能</li>
                        <li>• 保持代码格式清晰易读</li>
                        <li>• 可以点击"插入到内容"添加到正文中</li>
                      </ul>
                    </AlertDescription>
                  </Alert>
                </div>
              </CardContent>
            )}
          </Card>
        )}

        {/* 帖子专用：图片上传 */}
        {publishType === 'post' && (
                      <Card>
              <CardHeader>
                <CardTitle className="text-base flex items-center">
                  <Image className="h-4 w-4 mr-2" />
                  图片（可选）
                </CardTitle>
              </CardHeader>
            <CardContent>
              <div className="grid grid-cols-3 gap-2 mb-4">
                {images.map((image, index) => (
                  <div key={index} className="relative aspect-square bg-muted rounded-lg overflow-hidden">
                    <img
                      src={URL.createObjectURL(image)}
                      alt={`上传图片 ${index + 1}`}
                      className="w-full h-full object-cover"
                    />
                    <Button
                      variant="ghost"
                      size="sm"
                      className="absolute top-1 right-1 h-6 w-6 p-0 bg-black/50 hover:bg-black/70"
                      onClick={() => handleRemoveImage(index)}
                    >
                      <X className="h-3 w-3 text-white" />
                    </Button>
                  </div>
                ))}
                {images.length < 9 && (
                  <label className="aspect-square border-2 border-dashed border-muted-foreground/25 rounded-lg flex items-center justify-center cursor-pointer hover:bg-muted/50 transition-colors">
                    <Camera className="h-6 w-6 text-muted-foreground" />
                    <input
                      type="file"
                      multiple
                      accept="image/*"
                      onChange={handleImageUpload}
                      className="hidden"
                    />
                  </label>
                )}
              </div>
              <p className="text-xs text-muted-foreground">
                最多可上传9张图片，支持JPG、PNG格式
              </p>
            </CardContent>
          </Card>
        )}

        {/* 标签 */}
        <Card>
          <CardHeader>
            <CardTitle className="text-base flex items-center">
              <Hash className="h-4 w-4 mr-2" />
              标签
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="flex flex-wrap gap-2 mb-3">
              {tags.map((tag) => (
                <Badge
                  key={tag}
                  variant="secondary"
                  className="cursor-pointer"
                  onClick={() => handleRemoveTag(tag)}
                >
                  #{tag} ×
                </Badge>
              ))}
            </div>
            <div className="flex gap-2">
              <Input
                id="new-tag-input"
                name="newTag"
                placeholder="添加标签"
                value={newTag}
                onChange={(e) => setNewTag(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleAddTag()}
                autoComplete="off"
              />
              <Button
                variant="outline"
                size="sm"
                onClick={handleAddTag}
                disabled={!newTag.trim() || tags.length >= 10}
              >
                添加
              </Button>
            </div>
            <div className="mt-3 p-2 bg-muted/30 rounded-lg">
              <p className="text-xs text-muted-foreground mb-1">
                💡 标签使用建议：
              </p>
              <ul className="text-xs text-muted-foreground space-y-0.5">
                <li>• 最多可添加10个标签，用于分类和搜索</li>
                <li>• {publishType === 'resource' ? '推荐使用技术栈、平台、用途等标签' : '推荐使用技术、教程、经验等标签'}</li>
                <li>• 避免使用过于宽泛的标签，如"好用"、"推荐"</li>
              </ul>
            </div>
          </CardContent>
        </Card>

        {/* 发布规范提醒 */}
        <div className="container max-w-2xl mx-auto px-4">
          <Alert>
          <CheckCircle className="h-4 w-4" />
          <AlertDescription>
            <div className="space-y-2">
              <div className="font-medium text-sm">🏛️ 社区发布规范</div>
              <div className="text-xs text-muted-foreground space-y-1">
                <p>• 遵守国家法律法规，不发布违法违规内容</p>
                <p>• 尊重知识产权，标注内容来源和版权信息</p>
                <p>• 保持友善交流，构建和谐社区氛围</p>
                <p>• 发布有价值的内容，避免灌水和广告</p>
                <div className="mt-2 pt-2 border-t border-border">
                  <p className="text-primary">📞 如有疑问，请联系管理员或查看社区帮助文档</p>
                </div>
              </div>
            </div>
          </AlertDescription>
        </Alert>
        </div>

        {/* 预览和发布区域 */}
        <div className="container max-w-2xl mx-auto px-4 pb-4">
          <div className="flex gap-3">
            <Button
              variant="outline"
              className="flex-1"
              onClick={() => {
                // 这里可以添加预览功能，比如打开模态框显示预览
                toast.success('预览功能开发中', {
                  description: '敬请期待完整的预览体验',
                  duration: 2000,
                })
              }}
              disabled={!title.trim() || !content.trim()}
            >
              <Eye className="h-4 w-4 mr-1" />
              预览
            </Button>
            <Button
              className="flex-1"
              onClick={handlePublish}
              disabled={isPublishing || !title.trim() || !content.trim()}
            >
              {isPublishing ? (
                <>
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                  发布中...
                </>
              ) : (
                <>
                  <Send className="h-4 w-4 mr-1" />
                  {publishType === 'resource' ? '发布资源' : '发布帖子'}
                </>
              )}
            </Button>
          </div>
        </div>
      </div>
      </div> {/* 结束内容区域 */}

      {/* 底部安全区域 */}
      <div className="h-16 safe-area-bottom" />
    </div>
  )
}

export default PublishScreen 