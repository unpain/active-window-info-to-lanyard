# 变更日志

本项目的所有重要变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 修复
- **Windows长时间运行问题修复** (2024-12-25)
  - 修复Windows平台上长时间运行后推送自动停止的问题
  - 添加Discord RPC连接健康检查机制
  - 实现自动重连功能（连续失败3次或5分钟无更新时触发）
  - 添加连接状态跟踪（失败次数、最后成功时间）
  - 增强错误诊断日志输出
  - 详见 [WINDOWS_FIX.md](WINDOWS_FIX.md)

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

