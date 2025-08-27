import { cn } from "@/lib/utils"

function Skeleton({
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("animate-pulse rounded-md bg-muted", className)}
      {...props}
    />
  )
}

// 预定义的常用skeleton样式
const SkeletonText = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <Skeleton className={cn("h-4 w-full", className)} {...props} />
)

const SkeletonTitle = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <Skeleton className={cn("h-6 w-3/4", className)} {...props} />
)

const SkeletonAvatar = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <Skeleton className={cn("h-10 w-10 rounded-full", className)} {...props} />
)

const SkeletonButton = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <Skeleton className={cn("h-10 w-24 rounded-md", className)} {...props} />
)

const SkeletonCard = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <div className={cn("space-y-3 p-4 border rounded-lg", className)} {...props}>
    <div className="flex items-center space-x-3">
      <SkeletonAvatar />
      <div className="space-y-2 flex-1">
        <SkeletonTitle className="w-1/2" />
        <SkeletonText className="w-1/3 h-3" />
      </div>
    </div>
    <div className="space-y-2">
      <SkeletonText />
      <SkeletonText className="w-4/5" />
    </div>
    <div className="flex space-x-2">
      <SkeletonButton className="w-16 h-8" />
      <SkeletonButton className="w-16 h-8" />
    </div>
  </div>
)

export { 
  Skeleton, 
  SkeletonText, 
  SkeletonTitle, 
  SkeletonAvatar, 
  SkeletonButton, 
  SkeletonCard 
}
