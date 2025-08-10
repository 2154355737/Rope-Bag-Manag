# 常见默认 SDK 目录
$sdk = "$env:LOCALAPPDATA\Android\Sdk"
if (-not (Test-Path $sdk)) {
  Write-Host "未找到 $sdk，请在 Android Studio 的 SDK Manager 中查看实际 SDK 路径，并改下面的变量"
} else {
  Write-Host "检测到 SDK: $sdk"
}

# 选择最高版本 NDK
$ndkRoot = Join-Path $sdk "ndk"
if (Test-Path $ndkRoot) {
  $ndk = (Get-ChildItem $ndkRoot -Directory | Sort-Object Name -Descending | Select-Object -First 1).FullName
  Write-Host "检测到 NDK: $ndk"
} else {
  Write-Host "未检测到 NDK，请在 Android Studio 的 SDK Manager 勾选 NDK (Side by side) 后重试"
}
Read-Host "检测完成，按回车退出..."