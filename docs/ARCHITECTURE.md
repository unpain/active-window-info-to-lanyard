# 项目架构文档

## 概述

Windows Discord Activity Monitor 是一个用Rust编写的轻量级工具，用于监控Windows活动窗口并将信息同步到Discord Rich Presence。

## 架构设计

### 模块化设计

项目采用模块化架构，将不同功能分离到独立模块中：

```plain
src/
├── lib.rs          # 库入口，导出公共API
├── main.rs         # 可执行程序入口
├── config.rs       # 配置管理
├── window.rs       # Windows API交互
├── parser.rs       # 数据解析
└── discord.rs      # Discord RPC集成
```

### 模块职责

#### 1. config.rs - 配置管理模块

**职责：**

- 管理应用程序配置参数
- 提供配置验证功能
- 支持从不同来源加载配置

**主要类型：**

- `Config` - 配置结构体
  - `discord_app_id: u64` - Discord应用ID
  - `update_interval: Duration` - 更新间隔

**主要方法：**

- `new()` - 创建配置实例
- `from_str()` - 从字符串创建配置
- `validate()` - 验证配置有效性

#### 2. window.rs - 窗口监控模块

**职责：**

- 调用Windows API获取活动窗口信息
- 跟踪窗口变化
- 提供跨平台兼容性（仅Windows实现）

**主要类型：**

- `WindowMonitor` - 窗口监控器
  - `last_window_title: String` - 上次窗口标题

**主要函数：**

- `get_active_window_title()` - 获取当前活动窗口标题
- `WindowMonitor::check_for_change()` - 检查窗口是否变化

**Windows API调用：**

- `GetForegroundWindow()` - 获取前台窗口句柄
- `GetWindowTextW()` - 获取窗口标题

#### 3. parser.rs - 窗口标题解析模块

**职责：**

- 从窗口标题中提取结构化信息
- 识别应用名称和详细信息
- 清理和格式化文本

**主要类型：**

- `WindowInfo` - 窗口信息结构
  - `app_name: String` - 应用名称
  - `details: String` - 详细信息

**主要函数：**

- `extract_app_name()` - 提取应用名称
- `sanitize_title()` - 清理标题文本
- `WindowInfo::parse()` - 解析窗口标题

**解析规则：**

- 格式："详细信息 - 应用名称"
- 使用最右侧的 " - " 作为分隔符
- 支持多级分隔符

#### 4. discord.rs - Discord RPC模块

**职责：**

- 管理Discord RPC连接
- 更新Rich Presence状态
- 处理连接错误和重试

**主要类型：**

- `DiscordManager` - Discord RPC管理器
  - `client: DiscordClient` - RPC客户端
  - `start_time: u64` - 启动时间戳
- `UpdateResult` - 更新结果枚举

**主要方法：**

- `connect()` - 连接到Discord
- `update_activity()` - 更新活动状态
- `clear_activity()` - 清除活动状态

#### 5. lib.rs - 库入口

**职责：**

- 声明和导出所有公共模块
- 提供统一的API接口
- 定义库级别的常量和版本信息

#### 6. main.rs - 程序入口

**职责：**

- 初始化应用程序
- 协调各模块工作
- 实现主事件循环
- 处理程序生命周期

## 数据流

```plain
┌─────────────────┐
│  Windows系统     │
│  (活动窗口)      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  window.rs      │
│  获取窗口标题    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  parser.rs      │
│  解析标题信息    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  discord.rs     │
│  更新RPC状态     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Discord应用     │
│  显示Rich        │
│  Presence       │
└─────────────────┘
```

## 主循环流程

```plain
1. 初始化配置 (config.rs)
   ↓
2. 连接Discord RPC (discord.rs)
   ↓
3. 创建窗口监控器 (window.rs)
   ↓
4. 进入主循环:
   ├─ 检查窗口变化 (window.rs)
   │  ├─ 如果窗口变化:
   │  │  ├─ 解析窗口信息 (parser.rs)
   │  │  └─ 更新Discord状态 (discord.rs)
   │  └─ 如果未变化:
   │     └─ 跳过更新
   ├─ 等待指定间隔 (config.rs)
   └─ 返回步骤4
```

## 错误处理策略

### 1. 配置错误

- 在启动时验证配置
- 提供清晰的错误信息
- 失败时退出程序

### 2. Discord连接错误

- 记录错误但不中断程序
- 在下次循环时继续尝试
- 提供用户友好的错误提示

### 3. 窗口API错误

- 返回 `None` 表示无法获取窗口
- 不抛出panic，保持程序稳定
- 仅在Windows平台上实现

## 性能考虑

### 1. 资源使用

- 仅在窗口变化时更新Discord状态
- 可配置的更新间隔（默认5秒）
- 最小化Windows API调用

### 2. 内存管理

- 使用 `String` 而非 `Vec<u8>` 简化内存管理
- 复用 `WindowMonitor` 实例避免重复分配
- 字符串操作使用引用减少克隆

### 3. 线程模型

- 单线程设计，简单可靠
- 使用 `thread::sleep()` 避免忙等待
- Discord RPC库内部处理异步通信

## 扩展性

### 支持的扩展点

1. **新的窗口信息提取器**
   - 实现新的解析规则
   - 添加到 `parser.rs`

2. **配置文件支持**
   - 扩展 `config.rs`
   - 添加文件读取功能

3. **过滤规则**
   - 在 `window.rs` 或 `parser.rs` 中添加过滤逻辑
   - 支持忽略特定窗口

4. **其他平台支持**
   - 为 macOS/Linux 实现 `get_active_window_title()`
   - 使用条件编译保持代码整洁

## 测试策略

### 单元测试

- 每个模块包含独立的测试
- 使用 `#[cfg(test)]` 模块
- 测试公共API的正确性

### 集成测试

- 可在 `tests/` 目录添加集成测试
- 测试模块间的交互

### 测试覆盖

- 配置解析和验证
- 窗口标题解析逻辑
- 错误处理路径

## 依赖管理

### 核心依赖

- `discord-rpc-client` - Discord RPC客户端
- `windows` - Windows API绑定

### 开发依赖

- Rust标准库测试框架
- 文档生成工具

## 构建和发布

### 构建配置

- 支持库和可执行文件双模式
- 条件编译Windows特定代码
- 优化发布构建大小

### 文档生成

```bash
cargo doc --open
```

### 发布检查清单

- [ ] 更新版本号
- [ ] 更新 CHANGELOG.md
- [ ] 运行所有测试
- [ ] 运行 clippy
- [ ] 格式化代码
- [ ] 更新文档
