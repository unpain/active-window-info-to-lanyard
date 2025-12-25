# 变更日志

本项目的所有重要变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 修复
- **macOS 快速窗口切换优化** (2024-12-25)
  - 修复 macOS 平台上快速切换窗口时程序卡住的问题
  - 添加全局互斥锁（`LazyLock<Mutex<()>>`）保护 Core Graphics API 调用
  - 实现 100ms 超时机制防止死锁
  - 在 `WindowMonitor` 中添加最小查询间隔限制（默认 50ms）
  - 优化内存管理，确保 autorelease pool 正确释放
  - 添加 `try_lock()` + 超时重试机制，避免永久阻塞
  - 支持自定义查询间隔（`new_with_interval()` 和 `set_min_query_interval()`）
  - 详见 [docs/MACOS_RAPID_SWITCH_OPTIMIZATION.md](docs/MACOS_RAPID_SWITCH_OPTIMIZATION.md)

- **Windows长时间运行问题修复** (2024-12-25)
  - 修复Windows平台上长时间运行后推送自动停止的问题
  - 在 `get_active_window_title()` 函数中添加窗口句柄验证（使用 `IsWindow` API）
  - 修复长时间不切换窗口后无法检测到窗口切换的问题
  - 改进 `check_for_change()` 方法，使用 `match` 语句使逻辑更清晰
  - 添加调试模式下的详细日志输出
  - 明确文档说明：每次都重新获取前台窗口句柄，不使用任何缓存
  - 对比 macOS 版本（使用 autorelease pool 防止内存泄漏），Windows 版本通过句柄验证确保稳定性

### 新增
- **快速窗口切换测试工具** (2024-12-25)
  - 添加 `examples/test_rapid_switch.rs` 测试程序
  - 验证互斥锁和查询间隔限制机制
  - 提供三种测试场景：极快速、正常速度、自定义间隔
  - 显示详细的性能统计信息
  - 用于验证快速切换窗口时的稳定性

- **调试工具** (2024-12-25)
  - 添加 `examples/debug_window_monitor.rs` 调试工具
  - 用于测试长时间不切换窗口后的行为
  - 提供详细的心跳信息和窗口变化检测
  - 详见 [docs/WINDOWS_LONG_IDLE_FIX.md](docs/WINDOWS_LONG_IDLE_FIX.md)

### 计划功能
- [ ] 支持自定义窗口过滤规则
- [ ] 添加配置文件支持
- [ ] 支持多显示器
- [ ] 添加系统托盘图标

## [0.1.0] - 2024-12-24

### 新增
- 初始版本发布
- Windows活动窗口监控功能
- Discord Rich Presence集成
- 自动解析窗口标题中的应用名称
- 模块化代码架构：
  - `config` - 配置管理模块
  - `window` - 窗口监控模块
  - `parser` - 标题解析模块
  - `discord` - Discord RPC模块
- 单元测试覆盖
- 完整的文档注释

### 特性
- 实时监控活动窗口变化
- 仅在窗口变化时更新Discord状态（节省资源）
- 可配置的更新间隔
- 友好的中文界面输出
- 完整的错误处理

### 技术栈
- Rust 2021 Edition
- discord-rpc-client 0.4
- windows-rs 0.52
- 支持 Windows 10/11

[Unreleased]: https://github.com/yourusername/cur-win-discord-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/cur-win-discord-rust/releases/tag/v0.1.0

