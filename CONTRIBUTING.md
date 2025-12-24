# 贡献指南

感谢你对本项目的关注！我们欢迎任何形式的贡献。

## 如何贡献

### 报告问题

如果你发现了bug或有功能建议：

1. 检查 [Issues](https://github.com/yourusername/cur-win-discord-rust/issues) 确认问题是否已被报告
2. 创建新的 Issue，清楚地描述问题或建议
3. 提供尽可能多的相关信息（操作系统版本、错误日志等）

### 提交代码

1. **Fork 本仓库**

2. **创建功能分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **遵循代码规范**
   - 使用 `cargo fmt` 格式化代码
   - 使用 `cargo clippy` 检查代码质量
   - 为新功能编写测试
   - 保持代码注释清晰

4. **提交更改**
   ```bash
   git commit -m "feat: 添加新功能描述"
   ```

   提交信息格式：
   - `feat:` 新功能
   - `fix:` 修复bug
   - `docs:` 文档更新
   - `style:` 代码格式调整
   - `refactor:` 代码重构
   - `test:` 测试相关
   - `chore:` 构建/工具链更改

5. **推送到你的 Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **创建 Pull Request**
   - 清楚地描述你的更改
   - 引用相关的 Issue
   - 确保所有测试通过

## 开发环境设置

### 必需工具

- Rust 1.70.0 或更高版本
- Windows 10/11（用于窗口监控功能）
- Discord应用（用于测试RPC功能）

### 构建项目

```bash
# 克隆仓库
git clone https://github.com/yourusername/cur-win-discord-rust.git
cd cur-win-discord-rust

# 构建项目
cargo build

# 运行测试
cargo test

# 运行程序
cargo run
```

### 代码质量检查

```bash
# 格式化代码
cargo fmt

# 运行 Clippy
cargo clippy -- -D warnings

# 运行所有测试
cargo test --all-features
```

## 项目结构

```
src/
├── lib.rs          # 库入口，导出公共API
├── main.rs         # 程序入口
├── config.rs       # 配置管理
├── window.rs       # Windows窗口监控
├── parser.rs       # 窗口标题解析
└── discord.rs      # Discord RPC集成
```

## 编码规范

### Rust代码规范

- 遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- 使用 `rustfmt` 默认配置
- 所有公共API必须有文档注释
- 为新功能添加单元测试

### 文档规范

- 使用中文编写用户文档
- 代码注释使用中文
- API文档使用标准的Rustdoc格式
- 保持README.md更新

### 测试规范

- 为所有公共函数编写测试
- 使用有意义的测试名称
- 测试应该是独立和可重复的
- 包含边界情况测试

## 发布流程

1. 更新版本号（遵循[语义化版本](https://semver.org/lang/zh-CN/)）
2. 更新 CHANGELOG.md
3. 创建发布标签
4. 发布到 crates.io

## 行为准则

- 尊重所有贡献者
- 欢迎建设性的反馈
- 保持友好和专业的态度

## 需要帮助？

如果你在贡献过程中遇到问题：

- 查看现有的 Issues 和 Pull Requests
- 在 Issue 中提问
- 联系维护者

感谢你的贡献！🎉

