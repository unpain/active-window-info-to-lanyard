# 更新日志

## [未发布]

### 修复
- **[重要] 修复了 macOS 上窗口切换检测问题**
  - 原因：NSWorkspace 的 `frontmostApplication` 在某些情况下会缓存结果
  - 解决方案：改用 `CGWindowListCopyWindowInfo` API 直接获取窗口列表
  - 现在能够实时检测窗口切换，无需重启程序
  
- **修复了 Discord Rich Presence 时间戳更新问题**
  - 现在每次窗口切换时，时间戳会重置为当前时间
  - Discord 显示的时间正确反映了在当前窗口停留的时长

### 改进
- 移除了对 NSWorkspace API 的依赖
- 简化了 macOS 窗口检测逻辑
- 添加了详细的调试信息输出
- 更新了权限说明文档

### 技术细节
- macOS 实现现在使用 Core Graphics 的 `CGWindowListCopyWindowInfo` 
- 每次调用都重新创建 NSAutoreleasePool，确保获取最新状态
- 返回第一个 `layer=0` 的窗口作为活动窗口

## 之前的版本

详见 CHANGELOG.md

