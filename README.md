# 🎮 Discord Activity Monitor

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](.)

一个用Rust编写的跨平台轻量级工具，可将你当前活动的窗口信息实时同步到Discord Rich Presence，让你的朋友看到你正在使用什么应用程序。支持 Windows 和 macOS。

## ✨ 特性

- 🪟 **实时窗口监控** - 自动检测活动窗口变化
- 🎯 **智能解析** - 从窗口标题中智能提取应用名称和详细信息
- 💬 **Discord集成** - 无缝集成Discord Rich Presence
- 🔐 **数据加密** - 可选的AES-256-GCM加密保护隐私数据
- ⚡ **高性能** - 仅在窗口变化时更新，节省系统资源
- 🔧 **模块化设计** - 清晰的代码架构，易于扩展和维护
- 📦 **开箱即用** - 简单配置即可开始使用
- 🧪 **测试覆盖** - 完整的单元测试和文档测试
- 🌐 **跨平台支持** - 支持 Windows 和 macOS

## 📸 效果展示

当你在使用不同应用时，Discord会显示：

```plaintext
🎮 使用: Visual Studio Code
   📄 main.rs - cur-win-discord-rust
   ⏱️ 已运行 15分钟
```

## 🚀 快速开始

### 前置要求

- Windows 10/11 或 macOS 10.13+
- Rust 1.70.0 或更高版本
- Discord应用

### 安装

1. **克隆仓库**

```bash
git clone https://github.com/yourusername/cur-win-discord-rust.git
cd cur-win-discord-rust
```

1. **获取Discord应用ID**

   访问 [Discord Developer Portal](https://discord.com/developers/applications)：
   - 点击 "New Application" 创建新应用
   - 在左侧选择 "Rich Presence"
   - 复制 "Application ID"
   - （可选）上传名为 "windows" 的图标

2. **配置应用**

   编辑 `src/main.rs`，替换Discord应用ID：

   ```rust
   const DISCORD_APP_ID: &str = "你的应用ID";
   ```

3. **编译运行**

```bash
cargo build --release
cargo run --release
```

### 使用方法

程序启动后会自动：

1. 连接到Discord
2. 监控活动窗口
3. 每5秒检查一次窗口变化
4. 在窗口变化时更新Discord状态

按 `Ctrl+C` 退出程序。

## 📁 项目结构

```plaintext
cur-win-discord-rust/
├── src/
│   ├── lib.rs          # 库入口，导出公共API
│   ├── main.rs         # 程序主入口
│   ├── config.rs       # 配置管理模块
│   ├── crypto.rs       # 加密/解密模块
│   ├── window.rs       # Windows窗口监控模块
│   ├── parser.rs       # 窗口标题解析模块
│   └── discord.rs      # Discord RPC集成模块
├── examples/           # 示例代码
│   ├── generate_key.rs     # 生成加密密钥工具
│   ├── test_encryption.rs  # 加密功能测试工具
│   ├── with_encryption.rs  # 带加密的完整示例
│   ├── config_demo.rs      # 配置管理示例
│   ├── parser_demo.rs      # 解析器示例
│   └── custom_monitor.rs   # 自定义监控器示例
├── web/                # 前端解密工具
│   ├── decrypt.html        # 在线解密工具
│   ├── crypto.js           # JavaScript加密模块
│   └── README.md           # 前端使用文档
├── docs/              # 文档目录
│   ├── QUICKSTART.md       # 快速开始指南
│   ├── ENCRYPTION.md       # 加密功能文档
│   ├── ARCHITECTURE.md     # 架构文档
│   └── config.example.txt  # 配置示例
├── Cargo.toml          # 项目配置
├── README.md           # 项目说明
├── CONTRIBUTING.md     # 贡献指南
├── CHANGELOG.md        # 变更日志
└── LICENSE             # MIT许可证
```

## 🎯 示例代码

### 作为库使用

```rust
use cur_win_discord_rust::{Config, WindowMonitor, DiscordManager, WindowInfo};
use std::thread;

fn main() {
    // 创建配置
    let config = Config::new(123456789, 5);
    
    // 连接Discord
    let mut discord = DiscordManager::connect(&config).unwrap();
    
    // 创建窗口监控器
    let mut monitor = WindowMonitor::new();
    
    loop {
        if let Some(window_title) = monitor.check_for_change() {
            let info = WindowInfo::parse(&window_title);
            discord.update_activity(&info, &window_title).ok();
        }
        thread::sleep(config.update_interval);
    }
}
```

### 运行示例

```bash
# 生成加密密钥
cargo run --example generate_key

# 测试加密功能
cargo run --example test_encryption

# 配置管理示例
cargo run --example config_demo

# 窗口标题解析示例
cargo run --example parser_demo

# 自定义监控器示例
cargo run --example custom_monitor
```

## 🔧 配置选项

在 `src/main.rs` 中可以配置：

```rust
// Discord应用ID
const DISCORD_APP_ID: &str = "你的应用ID";

// 更新间隔（秒）
const UPDATE_INTERVAL: u64 = 5;
```

### 🔐 启用数据加密（可选）

为了保护你的隐私，可以启用AES-256-GCM加密来加密发送到Discord的窗口标题：

1. **生成加密密钥**
   ```bash
   cargo run --example generate_key
   ```

2. **配置密钥**
   
   在项目根目录创建 `.env` 文件，添加：
   ```env
   DISCORD_APP_ID=你的应用ID
   ENCRYPTION_KEY=生成的64字符密钥
   ```

3. **运行程序**
   ```bash
   cargo run
   ```

4. **前端解密**（可选）
   
   如果你需要在网页或其他前端应用中显示解密后的数据：
   - 打开 `web/decrypt.html` 使用在线解密工具
   - 或在你的项目中集成 `web/crypto.js` 模块
   - 查看 `web/README.md` 获取详细说明

详细的加密功能说明，请查看 [docs/ENCRYPTION.md](docs/ENCRYPTION.md)  
前端解密方案，请查看 [web/README.md](web/README.md)

## 📚 API文档

生成并查看完整的API文档：

```bash
cargo doc --open
```

### 主要模块

- **`config`** - 配置管理，创建和验证应用配置
- **`crypto`** - 加密/解密功能，提供AES-256-GCM加密
- **`window`** - Windows API交互，获取活动窗口信息
- **`parser`** - 窗口标题解析，提取应用名称和详细信息
- **`discord`** - Discord RPC集成，更新Rich Presence状态

## 🧪 测试

运行测试套件：

```bash
# 运行所有测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture

# 运行特定模块的测试
cargo test --lib config

# 运行文档测试
cargo test --doc
```

当前测试覆盖：

- ✅ 17个单元测试（包括加密功能测试）
- ✅ 2个文档测试
- ✅ 所有公共API都有测试

## 🛠️ 开发

### 代码质量

```bash
# 格式化代码
cargo fmt

# 运行Clippy检查
cargo clippy -- -D warnings

# 检查编译错误
cargo check
```

### 构建发布版本

```bash
cargo build --release
```

可执行文件位于 `target/release/cur-win-discord-rust.exe`

## 🏗️ 架构

项目采用模块化设计，每个模块负责特定功能：

```plaintext
用户窗口活动
    ↓
[window.rs] 捕获窗口标题
    ↓
[parser.rs] 解析应用信息
    ↓
[discord.rs] 更新Discord状态
    ↓
Discord Rich Presence显示
```

详细架构说明请查看 [ARCHITECTURE.md](ARCHITECTURE.md)

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

### 贡献流程

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'feat: 添加某个功能'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📋 TODO

- [ ] 支持配置文件（TOML/JSON）
- [ ] 添加窗口过滤规则（忽略特定窗口）
- [ ] 支持自定义Discord图标映射
- [ ] 添加系统托盘集成
- [ ] 支持多显示器
- [ ] 添加窗口活动时间统计
- [ ] Web界面配置（可选）

## 📝 变更日志

查看 [CHANGELOG.md](CHANGELOG.md) 了解所有版本的变更详情。

## ⚠️ 注意事项

- 支持 Windows 和 macOS 平台
- 需要Discord应用运行才能更新状态
- 某些窗口可能无法正确获取标题（如管理员权限窗口）
- macOS 首次运行需要授予辅助功能访问权限
- Discord API有频率限制，建议更新间隔不小于3秒

## 🐛 故障排除

### Discord未显示状态

1. 确认Discord应用正在运行
2. 检查Discord应用ID是否正确
3. 在Discord设置中启用 "显示当前活动"
4. 检查是否有防火墙阻止连接

### 编译错误

1. 确认Rust版本 >= 1.70.0
2. 运行 `cargo clean` 清理缓存
3. 检查网络连接（下载依赖）

### 窗口标题无法获取

- Windows: 某些以管理员权限运行的窗口无法被普通权限程序读取，尝试以管理员权限运行本程序
- macOS: 首次运行时需要在"系统设置 → 隐私与安全性 → 辅助功能"中授予权限

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [discord-rpc-client](https://crates.io/crates/discord-rpc-client) - Discord RPC客户端库
- [windows-rs](https://crates.io/crates/windows) - Windows API绑定

## 📧 联系方式

如有问题或建议，请：

- 提交 [Issue](https://github.com/yourusername/cur-win-discord-rust/issues)
- 发起 [Discussion](https://github.com/yourusername/cur-win-discord-rust/discussions)

---

⭐ 如果这个项目对你有帮助，请给个Star！
