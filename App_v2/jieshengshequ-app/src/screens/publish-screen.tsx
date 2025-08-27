import React, { useState, useEffect } from 'react'
import { ArrowLeft, Send, FileText, Package, Upload, Hash, Folder, Tag, Code, Image, Bold, Italic, Link, Camera, X, Eye, Edit3, Info, Clock, CheckCircle, AlertCircle, Monitor, Smartphone, Globe, Settings, Star, List, Plus } from 'lucide-react'
import { useNavigate, useSearchParams } from 'react-router-dom'
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
import { getPost, updatePost } from '@/api/posts'
import { getResource, updateResource } from '@/api/resources'

type PublishType = 'resource' | 'post'

const PublishScreen: React.FC = () => {
  const navigate = useNavigate()
  const [searchParams] = useSearchParams()
  
  // 编辑模式状态
  const editId = searchParams.get('id')
  const editType = searchParams.get('type') as PublishType || 'resource'
  const isEditMode = !!editId
  
  const [publishType, setPublishType] = useState<PublishType>(editType)
  const [isPublishing, setIsPublishing] = useState(false)
  const [isLoading, setIsLoading] = useState(isEditMode)

  // 通用字段
  const [title, setTitle] = useState('')
  const [content, setContent] = useState('')
  const [tags, setTags] = useState<string[]>([])
  const [newTag, setNewTag] = useState('')

  // 资源专用字段
  const [version, setVersion] = useState('')
  const [category, setCategory] = useState('')
  const [files, setFiles] = useState<File[]>([])
  const [fileUploadStatus, setFileUploadStatus] = useState<Record<string, 'pending' | 'uploading' | 'verifying' | 'success' | 'failed'>>({})
  const [fileVerificationAttempts, setFileVerificationAttempts] = useState<Record<string, number>>({})
  const [filePaths, setFilePaths] = useState<Record<string, string>>({})
  const [screenshots, setScreenshots] = useState<File[]>([])
  const [screenshotUploadStatus, setScreenshotUploadStatus] = useState<Record<string, 'pending' | 'uploading' | 'verifying' | 'success' | 'failed'>>({})
  const [screenshotVerificationAttempts, setScreenshotVerificationAttempts] = useState<Record<string, number>>({})
  const [screenshotPaths, setScreenshotPaths] = useState<Record<string, string>>({})
  const [requirements, setRequirements] = useState<string[]>([])
  const [newRequirement, setNewRequirement] = useState('')

  // 帖子专用字段  
  const [images, setImages] = useState<File[]>([])
  const [imageUploadStatus, setImageUploadStatus] = useState<Record<string, 'pending' | 'uploading' | 'verifying' | 'success' | 'failed'>>({})
  const [imageVerificationAttempts, setImageVerificationAttempts] = useState<Record<string, number>>({})
  const [imagePaths, setImagePaths] = useState<Record<string, string>>({})
  const [codeSnippet, setCodeSnippet] = useState('')
  const [showCodeEditor, setShowCodeEditor] = useState(false)
  
  // 最大验证尝试次数
  const MAX_VERIFICATION_ATTEMPTS = 2
  
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

  // 加载编辑数据
  useEffect(() => {
    if (isEditMode && editId) {
      loadEditData()
    }
  }, [isEditMode, editId, editType])

  const loadEditData = async () => {
    try {
      setIsLoading(true)
      
      if (editType === 'post') {
        const data = await getPost(parseInt(editId!))
        
        // 检查状态是否允许编辑
        const status = data.status
        if (status === 'reviewing' || status === 'under_review' || status === 'pending') {
          toast.error('内容正在审核中，无法编辑')
          navigate('/my-content')
          return
        }
        
        // 填充帖子数据
        setTitle(data.title || '')
        setContent(data.content || '')
        setTags(data.tags || [])
        
        // 处理现有图片 - 转换URL为显示用的路径
        if (data.images && data.images.length > 0) {
          const existingImagePaths: Record<string, string> = {}
          data.images.forEach((imageUrl, index) => {
            const fileName = `existing_image_${index}.${imageUrl.split('.').pop() || 'jpg'}`
            existingImagePaths[fileName] = imageUrl
          })
          setImagePaths(existingImagePaths)
          
          // 设置上传状态为成功
          const existingImageStatus: Record<string, 'success'> = {}
          Object.keys(existingImagePaths).forEach(fileName => {
            existingImageStatus[fileName] = 'success'
          })
          setImageUploadStatus(existingImageStatus)
        }
      } else if (editType === 'resource') {
        const data = await getResource(parseInt(editId!))
        
        // 检查状态是否允许编辑
        const status = data.status
        if (status === 'reviewing' || status === 'under_review' || status === 'pending') {
          toast.error('内容正在审核中，无法编辑')
          navigate('/my-content')
          return
        }
        
        // 填充资源数据
        setTitle(data.name || data.title || '')
        setContent(data.description || '')
        setVersion(data.version || '')
        setTags(data.tags || [])
        setRequirements(data.requirements || [])
        setCategory(data.category_id?.toString() || '')
        
        // 处理现有文件
        if (data.files && data.files.length > 0) {
          const existingFilePaths: Record<string, string> = {}
          data.files.forEach((file: any, index: number) => {
            const fileName = file.name || `existing_file_${index}`
            existingFilePaths[fileName] = file.url || file.path || ''
          })
          setFilePaths(existingFilePaths)
          
          // 设置文件上传状态为成功
          const existingFileStatus: Record<string, 'success'> = {}
          Object.keys(existingFilePaths).forEach(fileName => {
            existingFileStatus[fileName] = 'success'
          })
          setFileUploadStatus(existingFileStatus)
        }
        
        // 处理现有截图
        if (data.screenshots && data.screenshots.length > 0) {
          const existingScreenshotPaths: Record<string, string> = {}
          data.screenshots.forEach((screenshotUrl: string, index: number) => {
            const fileName = `existing_screenshot_${index}.${screenshotUrl.split('.').pop() || 'jpg'}`
            existingScreenshotPaths[fileName] = screenshotUrl
          })
          setScreenshotPaths(existingScreenshotPaths)
          
          // 设置截图上传状态为成功
          const existingScreenshotStatus: Record<string, 'success'> = {}
          Object.keys(existingScreenshotPaths).forEach(fileName => {
            existingScreenshotStatus[fileName] = 'success'
          })
          setScreenshotUploadStatus(existingScreenshotStatus)
        }
      }
      
      toast.success('数据加载成功')
    } catch (error) {
      console.error('加载编辑数据失败:', error)
      toast.error('加载数据失败，请稍后重试')
    } finally {
      setIsLoading(false)
    }
  }

  // 文件类型验证
  const validateFileType = (file: File, allowedTypes: string[], maxSizeMB: number = 50): boolean => {
    const fileExtension = file.name.split('.').pop()?.toLowerCase()
    if (!fileExtension || !allowedTypes.includes(fileExtension)) {
      toast.error(`不支持的文件格式。允许的格式：${allowedTypes.join(', ')}`)
      return false
    }
    
    const maxSizeBytes = maxSizeMB * 1024 * 1024
    if (file.size > maxSizeBytes) {
      toast.error(`文件大小超过限制（最大 ${maxSizeMB}MB）`)
      return false
    }
    
    return true
  }

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
      const validFiles: File[] = []
      
      Array.from(uploadedFiles).forEach(file => {
        // 资源文件只允许 zip 格式
        if (validateFileType(file, ['zip'], 100)) {
          validFiles.push(file)
        }
      })
      
      if (validFiles.length > 0) {
        setFiles([...files, ...validFiles])
      }
    }
  }

  const handleImageUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const uploadedImages = event.target.files
    if (uploadedImages) {
      const validImages: File[] = []
      
      Array.from(uploadedImages).forEach(file => {
        // 图片只允许常见格式
        if (validateFileType(file, ['jpg', 'jpeg', 'png', 'gif', 'webp'], 10)) {
          validImages.push(file)
        }
      })
      
      if (validImages.length > 0) {
        setImages([...images, ...validImages])
      }
    }
  }

  const handleScreenshotUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const uploadedScreenshots = event.target.files
    if (uploadedScreenshots) {
      const validScreenshots: File[] = []
      
      Array.from(uploadedScreenshots).forEach(file => {
        // 截图只允许图片格式
        if (validateFileType(file, ['jpg', 'jpeg', 'png', 'gif', 'webp'], 5)) {
          validScreenshots.push(file)
        }
      })
      
      if (validScreenshots.length > 0) {
        setScreenshots([...screenshots, ...validScreenshots])
      }
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

  // 处理文件验证重试
  const handleRetryVerification = async (fileName: string, fileType: 'file' | 'screenshot' | 'image' = 'file') => {
    const { verifyFile } = await import('@/api/publish')
    
    // 获取对应的文件路径
    let filePath = ''
    
    if (fileType === 'file') {
      // 从存储的文件路径中获取
      filePath = filePaths[fileName] || ''
      setFileUploadStatus(prev => ({ ...prev, [fileName]: 'verifying' }))
      
      // 增加验证尝试次数
      const currentAttempts = fileVerificationAttempts[fileName] || 0
      setFileVerificationAttempts(prev => ({ ...prev, [fileName]: currentAttempts + 1 }))
      
      // 如果超过最大尝试次数，提示用户重新上传
      if (currentAttempts >= MAX_VERIFICATION_ATTEMPTS) {
        toast.error(`文件 ${fileName} 验证失败次数过多，请删除后重新上传`, {
          duration: 5000,
        })
        setFileUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
        return
      }
    } else if (fileType === 'screenshot') {
      // 从存储的截图路径中获取
      filePath = screenshotPaths[fileName] || ''
      setScreenshotUploadStatus(prev => ({ ...prev, [fileName]: 'verifying' }))
      
      const currentAttempts = screenshotVerificationAttempts[fileName] || 0
      setScreenshotVerificationAttempts(prev => ({ ...prev, [fileName]: currentAttempts + 1 }))
      
      if (currentAttempts >= MAX_VERIFICATION_ATTEMPTS) {
        toast.error(`截图 ${fileName} 验证失败次数过多，请删除后重新上传`, {
          duration: 5000,
        })
        setScreenshotUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
        return
      }
    } else if (fileType === 'image') {
      // 从存储的图片路径中获取
      filePath = imagePaths[fileName] || ''
      setImageUploadStatus(prev => ({ ...prev, [fileName]: 'verifying' }))
      
      const currentAttempts = imageVerificationAttempts[fileName] || 0
      setImageVerificationAttempts(prev => ({ ...prev, [fileName]: currentAttempts + 1 }))
      
      if (currentAttempts >= MAX_VERIFICATION_ATTEMPTS) {
        toast.error(`图片 ${fileName} 验证失败次数过多，请删除后重新上传`, {
          duration: 5000,
        })
        setImageUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
        return
      }
    }
    
    // 检查是否有文件路径
    if (!filePath) {
      toast.error(`无法获取 ${fileName} 的文件路径，请删除后重新上传`)
      return
    }
    
    try {
      // 验证文件是否成功上传到存储系统
      const verifyResult = await verifyFile(filePath)
      
      if (!verifyResult.exists) {
        console.warn(`文件 ${fileName} 重新验证失败，文件可能未成功保存`)
        toast.warning(`文件 ${fileName} 验证失败，请检查网络连接`)
        
        if (fileType === 'file') {
          setFileUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
        } else if (fileType === 'screenshot') {
          setScreenshotUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
        } else {
          setImageUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
        }
      } else {
        console.log(`文件 ${fileName} 重新验证成功`)
        toast.success(`文件 ${fileName} 验证成功`)
        
        if (fileType === 'file') {
          setFileUploadStatus(prev => ({ ...prev, [fileName]: 'success' }))
        } else if (fileType === 'screenshot') {
          setScreenshotUploadStatus(prev => ({ ...prev, [fileName]: 'success' }))
        } else {
          setImageUploadStatus(prev => ({ ...prev, [fileName]: 'success' }))
        }
      }
    } catch (error) {
      console.error(`文件 ${fileName} 验证出错:`, error)
      toast.error(`文件 ${fileName} 验证出错，请重试`)
      
      if (fileType === 'file') {
        setFileUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
      } else if (fileType === 'screenshot') {
        setScreenshotUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
      } else {
        setImageUploadStatus(prev => ({ ...prev, [fileName]: 'failed' }))
      }
    }
  }

  const handlePublish = async () => {
    if (!title.trim() || !content.trim()) {
      toast.error('请填写标题和内容')
      return
    }

    // 编辑模式的处理
    if (isEditMode && editId) {
      setIsPublishing(true)
      try {
        if (editType === 'post') {
          await updatePost(parseInt(editId), {
            title: title.trim(),
            content: content.trim(),
            tags
          })
          toast.success('帖子更新成功')
        } else if (editType === 'resource') {
          if (!version.trim() || !category) {
            toast.error('请填写版本信息和选择分类')
            setIsPublishing(false)
            return
          }
          await updateResource(parseInt(editId), {
            title: title.trim(),
            name: title.trim(),
            description: content.trim(),
            version: version.trim(),
            category_id: parseInt(category),
            tags,
            requirements
          })
          toast.success('资源更新成功')
        }
        
        // 延迟跳转回我的内容页
        setTimeout(() => {
          navigate('/my-content')
        }, 2000)
        
      } catch (error: any) {
        console.error('更新失败:', error)
        toast.error(error.message || '更新失败，请重试')
      } finally {
        setIsPublishing(false)
      }
      return
    }
    
    if (isPublishing) return // 防止重复提交
    if (publishType === 'resource') {
      if (!version.trim() || !category) {
        toast.error('请填写版本信息和选择分类')
        return
      }
    }

    setIsPublishing(true)
    
    try {
      let response: any
      
      // 如果有文件，先上传所有文件（资源文件和截图）
      let uploadedFiles: any[] = []
      let uploadedScreenshots: any[] = []
      
      // 判断发布类型
      if (publishType === 'resource') {
        // 先创建资源记录，获取ID，后续上传绑定
        const { publishResource } = await import('@/api/publish')
        response = await publishResource({
          title,
          content,
          version,
          category,
          tags,
          requirements,
          files: [],
          screenshots: [],
        })
        const resourceId = (response as any)?.id
        if (!resourceId) {
          toast.error('创建资源失败')
          setIsPublishing(false)
          return
        }
        
        // 1. 先上传资源文件
        if (files.length > 0) {
          console.log('开始上传资源文件...')
          const { uploadImage, verifyFile } = await import('@/api/publish')
          let allFilesUploaded = true
          
          for (const file of files) {
            try {
              setFileUploadStatus(prev => ({ ...prev, [file.name]: 'uploading' }))
              console.log(`开始上传文件: ${file.name}`)
              
              const uploadResult = await uploadImage(file, resourceId)
              console.log(`文件 ${file.name} 上传结果:`, uploadResult)
              setFilePaths(prev => ({ ...prev, [file.name]: uploadResult.file_path }))
              
              setFileUploadStatus(prev => ({ ...prev, [file.name]: 'verifying' }))
              
              const verifyResult = await verifyFile(uploadResult.file_path)
              console.log(`文件 ${file.name} 验证结果:`, verifyResult)
              
              if (!verifyResult.exists) {
                console.warn(`文件 ${file.name} 上传验证失败 - exists: ${verifyResult.exists}`)
                toast.error(`文件 ${file.name} 上传失败，请重试`)
                setFileUploadStatus(prev => ({ ...prev, [file.name]: 'failed' }))
                allFilesUploaded = false
                break
              }
              
              setFileUploadStatus(prev => ({ ...prev, [file.name]: 'success' }))
              
              // 保存上传成功的文件信息
              uploadedFiles.push({
                name: file.name,
                size: uploadResult.file_size,
                type: file.type,
                file_path: uploadResult.file_path,
                download_url: uploadResult.download_url
              })
              
              console.log(`文件 ${file.name} 上传并验证成功`)
              
            } catch (error: any) {
              console.error(`文件 ${file.name} 上传失败:`, error)
              toast.error(`文件 ${file.name} 上传失败: ${error.message}`)
              setFileUploadStatus(prev => ({ ...prev, [file.name]: 'failed' }))
              allFilesUploaded = false
              break
            }
          }
          
          if (!allFilesUploaded) {
            console.error('文件上传失败，取消资源创建')
            toast.error('文件上传失败，请重新尝试')
            setIsPublishing(false)
            return
          }
        }
        
        // 2. 再上传截图
        if (screenshots.length > 0) {
          console.log('开始上传截图...')
          const { uploadImage, verifyFile } = await import('@/api/publish')
          let allScreenshotsUploaded = true
          
          for (const screenshot of screenshots) {
            try {
              setScreenshotUploadStatus(prev => ({ ...prev, [screenshot.name]: 'uploading' }))
              console.log(`开始上传截图: ${screenshot.name}`)
              
              const uploadResult = await uploadImage(screenshot, resourceId)
              console.log(`截图 ${screenshot.name} 上传结果:`, uploadResult)
              setScreenshotPaths(prev => ({ ...prev, [screenshot.name]: uploadResult.file_path }))
              
              setScreenshotUploadStatus(prev => ({ ...prev, [screenshot.name]: 'verifying' }))
              
              const verifyResult = await verifyFile(uploadResult.file_path)
              console.log(`截图 ${screenshot.name} 验证结果:`, verifyResult)
              
              if (!verifyResult.exists) {
                console.warn(`截图 ${screenshot.name} 上传验证失败`)
                setScreenshotUploadStatus(prev => ({ ...prev, [screenshot.name]: 'failed' }))
                toast.warning(`截图 ${screenshot.name} 上传失败`)
                allScreenshotsUploaded = false
                break
              }
              
              setScreenshotUploadStatus(prev => ({ ...prev, [screenshot.name]: 'success' }))
              
              // 保存上传成功的截图信息
              uploadedScreenshots.push({
                name: screenshot.name,
                size: uploadResult.file_size,
                type: screenshot.type,
                file_path: uploadResult.file_path,
                download_url: uploadResult.download_url
              })
              
              console.log(`截图 ${screenshot.name} 上传并验证成功`)
              
            } catch (error: any) {
              console.error(`截图 ${screenshot.name} 上传失败:`, error)
              toast.warning(`截图 ${screenshot.name} 上传失败: ${error.message}`)
              setScreenshotUploadStatus(prev => ({ ...prev, [screenshot.name]: 'failed' }))
              allScreenshotsUploaded = false
              break
            }
          }
          
          if (!allScreenshotsUploaded) {
            console.warn('部分截图上传失败，但仍继续创建资源')
          }
        }
        
        // 3. 最后更新资源记录，包含所有已上传文件的信息
        console.log('开始更新资源记录，文件数:', uploadedFiles.length, '截图数:', uploadedScreenshots.length)
        const { updateResource } = await import('@/api/resources')
        response = await updateResource(resourceId, {
          title,
          description: content,
          version,
          category_id: Number(category) || undefined,
          tags,
          requirements,
        })
        
        console.log('资源记录更新成功:', response)
        
      } else {
        // 帖子发布流程
        let uploadedImages: any[] = []
        
        // 1. 先上传图片（如果有）
        if (images.length > 0) {
          console.log('开始上传帖子图片...')
          const { uploadImage, verifyFile } = await import('@/api/publish')
          let allImagesUploaded = true
          
          for (const image of images) {
            try {
              setImageUploadStatus(prev => ({ ...prev, [image.name]: 'uploading' }))
              console.log(`开始上传图片: ${image.name}`)
              
              const uploadResult = await uploadImage(image)
              console.log(`图片 ${image.name} 上传结果:`, uploadResult)
              setImagePaths(prev => ({ ...prev, [image.name]: uploadResult.file_path }))
              
              setImageUploadStatus(prev => ({ ...prev, [image.name]: 'verifying' }))
              
              const verifyResult = await verifyFile(uploadResult.file_path)
              console.log(`图片 ${image.name} 验证结果:`, verifyResult)
              
              if (!verifyResult.exists) {
                console.warn(`图片 ${image.name} 上传验证失败`)
                setImageUploadStatus(prev => ({ ...prev, [image.name]: 'failed' }))
                toast.warning(`图片 ${image.name} 上传失败`)
                allImagesUploaded = false
                break
              }
              
              setImageUploadStatus(prev => ({ ...prev, [image.name]: 'success' }))
              
              // 保存上传成功的图片信息
              uploadedImages.push({
                name: image.name,
                size: uploadResult.file_size,
                type: image.type,
                file_path: uploadResult.file_path,
                download_url: uploadResult.download_url
              })
              
              console.log(`图片 ${image.name} 上传并验证成功`)
              
            } catch (error: any) {
              console.error(`图片 ${image.name} 上传失败:`, error)
              toast.warning(`图片 ${image.name} 上传失败: ${error.message}`)
              setImageUploadStatus(prev => ({ ...prev, [image.name]: 'failed' }))
              allImagesUploaded = false
              break
            }
          }
          
          if (!allImagesUploaded) {
            console.warn('部分图片上传失败，但仍继续创建帖子')
          }
        }
        
        // 2. 创建帖子记录
        console.log('开始创建帖子记录，图片数:', uploadedImages.length)
        const { publishPost } = await import('@/api/publish')
        response = await publishPost({
          title,
          content,
          tags,
          images: uploadedImages.map(img => ({ name: img.name, size: img.size, type: img.type })),
          code_snippet: codeSnippet
        })
        
        console.log('帖子记录创建成功:', response)
      }
      
      // 显示成功提示
      const isResource = publishType === 'resource'
      toast.success(`${isResource ? '资源' : '帖子'}发布成功！`, {
        description: isResource ? '资源已发布，用户可以查看和下载' : '帖子已发布',
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
        title={isEditMode ? "编辑内容" : "发布内容"}
        subtitle={isEditMode 
          ? (publishType === 'resource' ? '编辑资源' : '编辑帖子')
          : (publishType === 'resource' ? '分享资源' : '发布帖子')
        }
        showBackButton
        rightAction={
          <Button
            onClick={handlePublish}
            disabled={!title || !content || isPublishing || isLoading}
            className="px-6"
          >
            {isLoading ? (
              <>
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2" />
                加载中...
              </>
            ) : isPublishing ? (
              <>
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin mr-2" />
                {isEditMode ? '保存中...' : '发布中...'}
              </>
            ) : (
              <>
                <Send size={16} className="mr-2" />
                {isEditMode ? '保存' : '发布'}
              </>
            )}
          </Button>
        }
      />

      {/* 内容区域 - 为固定导航栏留出空间 */}
      <div className="pt-nav"> {/* 固定导航栏高度 + 安全区域 */}
        <div className="container max-w-2xl mx-auto p-4 space-y-6">
        
        {/* 编辑模式加载状态 */}
        {isLoading && (
          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center justify-center py-8">
                <div className="w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin mr-3" />
                <span>正在加载编辑数据...</span>
              </div>
            </CardContent>
          </Card>
        )}
        {/* 发布类型选择 - 编辑模式下不显示 */}
        {!isEditMode && (
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
        )}

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
                      {files.map((file, index) => {
                        const status = fileUploadStatus[file.name] || 'pending';
                        return (
                          <div key={index} className="flex items-center justify-between p-2 bg-muted rounded-lg">
                            <div className="flex items-center gap-2">
                              <FileText className="h-4 w-4 text-muted-foreground" />
                              <span className="text-sm">{file.name}</span>
                              <span className="text-xs text-muted-foreground">
                                ({(file.size / 1024 / 1024).toFixed(2)} MB)
                              </span>
                              {status === 'uploading' && (
                                <div className="flex items-center">
                                  <div className="w-3 h-3 border-2 border-primary border-t-transparent rounded-full animate-spin mr-1"></div>
                                  <span className="text-xs text-primary">上传中...</span>
                                </div>
                              )}
                              {status === 'verifying' && (
                                <div className="flex items-center">
                                  <div className="w-3 h-3 border-2 border-amber-500 border-t-transparent rounded-full animate-spin mr-1"></div>
                                  <span className="text-xs text-amber-500">验证中...</span>
                                </div>
                              )}
                              {status === 'success' && (
                                <div className="flex items-center">
                                  <CheckCircle className="w-3 h-3 text-green-500 mr-1" />
                                  <span className="text-xs text-green-500">已上传</span>
                                </div>
                              )}
                              {status === 'failed' && (
                              <div className="flex items-center gap-1">
                                <AlertCircle className="w-3 h-3 text-red-500 mr-1" />
                                <span className="text-xs text-red-500">上传失败</span>
                                <Button 
                                  variant="ghost" 
                                  size="sm" 
                                  className="h-5 px-1 text-xs text-red-500 hover:text-red-700"
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    handleRetryVerification(file.name);
                                  }}
                                >
                                  重试
                                </Button>
                              </div>
                              )}
                            </div>
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={() => handleRemoveFile(index)}
                              disabled={status === 'uploading' || status === 'verifying'}
                            >
                              <X className="h-4 w-4" />
                            </Button>
                          </div>
                        );
                      })}
                    </div>
                  )}
                  <label className="flex items-center justify-center w-full h-32 border-2 border-dashed border-muted-foreground/25 rounded-lg cursor-pointer hover:bg-muted/50 transition-colors">
                    <div className="text-center">
                      <Upload className="h-8 w-8 mx-auto mb-2 text-muted-foreground" />
                      <p className="text-sm text-muted-foreground">点击上传资源文件</p>
                      <p className="text-xs text-muted-foreground mt-1">仅支持ZIP格式，单文件不超过100MB</p>
                    </div>
                    <input
                      type="file"
                      accept=".zip"
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
                      {screenshots.map((screenshot, index) => {
                        const status = screenshotUploadStatus[screenshot.name] || 'pending';
                        return (
                          <div key={index} className="relative">
                            <img
                              src={URL.createObjectURL(screenshot)}
                              alt={`Screenshot ${index + 1}`}
                              className={`w-full h-32 object-cover rounded-lg border ${status === 'failed' ? 'border-red-500' : ''}`}
                            />
                            <Button
                              variant="destructive"
                              size="sm"
                              className="absolute top-1 right-1 h-6 w-6 p-0"
                              onClick={() => removeScreenshot(index)}
                              disabled={status === 'uploading' || status === 'verifying'}
                            >
                              <X className="h-3 w-3" />
                            </Button>
                            
                            {/* 上传状态指示器 */}
                            {status === 'uploading' && (
                              <div className="absolute bottom-1 left-1 right-1 bg-black/50 text-white text-xs p-1 rounded flex items-center justify-center">
                                <div className="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin mr-1"></div>
                                上传中...
                              </div>
                            )}
                            {status === 'verifying' && (
                              <div className="absolute bottom-1 left-1 right-1 bg-amber-500/70 text-white text-xs p-1 rounded flex items-center justify-center">
                                <div className="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin mr-1"></div>
                                验证中...
                              </div>
                            )}
                            {status === 'success' && (
                              <div className="absolute bottom-1 left-1 right-1 bg-green-500/70 text-white text-xs p-1 rounded flex items-center justify-center">
                                <CheckCircle className="w-3 h-3 mr-1" />
                                已上传
                              </div>
                            )}
                            {status === 'failed' && (
                              <div className="absolute bottom-1 left-1 right-1 bg-red-500/70 text-white text-xs p-1 rounded flex items-center justify-center gap-1">
                                <AlertCircle className="w-3 h-3 mr-1" />
                                上传失败
                                <Button 
                                  variant="ghost" 
                                  size="sm" 
                                  className="h-5 px-1 text-xs text-white hover:text-white/80"
                                  onClick={(e) => {
                                    e.stopPropagation();
                                    handleRetryVerification(screenshot.name, 'screenshot');
                                  }}
                                >
                                  重试
                                </Button>
                              </div>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  )}
                  <label className="flex items-center justify-center w-full h-32 border-2 border-dashed border-muted-foreground/25 rounded-lg cursor-pointer hover:bg-muted/50 transition-colors">
                    <div className="text-center">
                      <Camera className="h-8 w-8 mx-auto mb-2 text-muted-foreground" />
                      <p className="text-sm text-muted-foreground">添加预览截图</p>
                      <p className="text-xs text-muted-foreground mt-1">JPG/PNG/GIF/WebP格式，建议尺寸16:9，单文件不超过5MB</p>
                    </div>
                    <input
                      type="file"
                      multiple
                      accept="image/jpeg,image/jpg,image/png,image/gif,image/webp"
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
                {images.map((image, index) => {
                  const status = imageUploadStatus[image.name] || 'pending';
                  return (
                    <div key={index} className="relative aspect-square bg-muted rounded-lg overflow-hidden">
                      <img
                        src={URL.createObjectURL(image)}
                        alt={`上传图片 ${index + 1}`}
                        className={`w-full h-full object-cover ${status === 'failed' ? 'opacity-70' : ''}`}
                      />
                      <Button
                        variant="ghost"
                        size="sm"
                        className="absolute top-1 right-1 h-6 w-6 p-0 bg-black/50 hover:bg-black/70"
                        onClick={() => handleRemoveImage(index)}
                        disabled={status === 'uploading' || status === 'verifying'}
                      >
                        <X className="h-3 w-3 text-white" />
                      </Button>
                      
                      {/* 上传状态指示器 */}
                      {status === 'uploading' && (
                        <div className="absolute bottom-0 left-0 right-0 bg-black/50 text-white text-xs p-1 flex items-center justify-center">
                          <div className="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin mr-1"></div>
                          上传中...
                        </div>
                      )}
                      {status === 'verifying' && (
                        <div className="absolute bottom-0 left-0 right-0 bg-amber-500/70 text-white text-xs p-1 flex items-center justify-center">
                          <div className="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin mr-1"></div>
                          验证中...
                        </div>
                      )}
                      {status === 'success' && (
                        <div className="absolute bottom-0 left-0 right-0 bg-green-500/70 text-white text-xs p-1 flex items-center justify-center">
                          <CheckCircle className="w-3 h-3 mr-1" />
                          已上传
                        </div>
                      )}
                      {status === 'failed' && (
                        <div className="absolute bottom-0 left-0 right-0 bg-red-500/70 text-white text-xs p-1 flex items-center justify-center gap-1">
                          <AlertCircle className="w-3 h-3 mr-1" />
                          上传失败
                          <Button 
                            variant="ghost" 
                            size="sm" 
                            className="h-5 px-1 text-xs text-white hover:text-white/80"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleRetryVerification(image.name, 'image');
                            }}
                          >
                            重试
                          </Button>
                        </div>
                      )}
                    </div>
                  );
                })}
                {images.length < 9 && (
                  <label className="aspect-square border-2 border-dashed border-muted-foreground/25 rounded-lg flex items-center justify-center cursor-pointer hover:bg-muted/50 transition-colors">
                    <Camera className="h-6 w-6 text-muted-foreground" />
                    <input
                      type="file"
                      multiple
                      accept="image/jpeg,image/jpg,image/png,image/gif,image/webp"
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