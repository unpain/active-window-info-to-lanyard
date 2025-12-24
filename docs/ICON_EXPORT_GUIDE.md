# macOS 应用图标批量导出工具

## 📖 简介

这是一套用于批量导出 macOS 系统应用图标的工具，能够自动扫描应用程序目录并将图标转换为常见的图片格式。

## 🚀 快速开始

### 基础版本

最简单的使用方式，使用默认设置导出所有应用图标：

```bash
cargo run --example export_icons
```

这将：
- 扫描 `/Applications` 和 `/System/Applications` 目录
- 扫描 `/System/Library/CoreServices` 目录（包含 Finder、Dock、Siri 等系统核心应用）
- 将图标导出为 512x512 像素的 PNG 格式
- 保存到 `./exported_icons/` 目录

### 高级版本

支持更多自定义选项：

```bash
# 查看所有可用选项
cargo run --example export_icons_advanced -- --help

# 导出为 1024x1024 的高清图标
cargo run --example export_icons_advanced -- -s 1024

# 导出为 JPEG 格式（文件更小）
cargo run --example export_icons_advanced -- -f jpeg

# 自定义输出目录
cargo run --example export_icons_advanced -- -o ~/Desktop/app_icons

# 组合多个选项
cargo run --example export_icons_advanced -- -s 256 -f png -o ./my_icons
```

## 🎨 支持的格式

- **PNG** - 默认格式，支持透明度，质量高
- **JPEG** - 文件更小，但不支持透明度
- **TIFF** - 专业级格式，文件较大

## 📋 常见用法示例

### 1. 导出所有应用的高清图标

```bash
cargo run --example export_icons_advanced -- -s 1024 -o ~/Desktop/hd_icons
```

### 2. 快速预览图标（小尺寸）

```bash
cargo run --example export_icons_advanced -- -s 128 -f jpeg
```

### 3. 导出特定目录的应用图标

```bash
cargo run --example export_icons_advanced -- -d ~/Applications
```

### 4. 串行处理（某些情况下更稳定）

```bash
cargo run --example export_icons_advanced -- --no-parallel
```

## 📊 输出示例

```
╔════════════════════════════════════════════════╗
║  macOS 应用图标批量导出工具（高级版）        ║
║  Advanced Icon Exporter for macOS            ║
╚════════════════════════════════════════════════╝

📁 输出目录: ./exported_icons
📐 图标尺寸: 512x512 像素
🖼️  导出格式: PNG
⚡ 并行处理: 开启

🔍 正在扫描: /Applications
   📦 Safari -> ✅ ./exported_icons/Safari.png
   📦 Google Chrome -> ✅ ./exported_icons/Google Chrome.png
   📦 Visual Studio Code -> ✅ ./exported_icons/Visual Studio Code.png
   ...
   ✅ 找到 50 个应用，成功导出 48 个图标

╔════════════════════════════════════════════════╗
║  导出完成！                                   ║
╚════════════════════════════════════════════════╝

📊 统计:
   - 扫描应用: 50 个
   - 成功导出: 48 个
   - 失败数量: 2 个
   - 成功率: 96.0%

📁 图标已保存到: ./exported_icons
```

## ⚙️ 技术细节

### 工作原理

1. **扫描应用目录** - 遍历指定目录，查找所有 `.app` 文件
2. **读取 Info.plist** - 使用 `plutil` 命令获取图标文件名
3. **定位图标文件** - 在 `Contents/Resources/` 目录查找 `.icns` 文件
4. **转换格式** - 使用 macOS 自带的 `sips` 命令转换图标
5. **保存输出** - 将转换后的图标保存到指定目录

### 系统要求

- **操作系统**: macOS（仅支持 macOS）
- **系统工具**: `sips` 和 `plutil`（macOS 自带）
- **Rust**: 1.70 或更高版本

### 性能优化

- **并行处理**: 默认启用多线程并行处理，大幅提升速度
- **智能文件名**: 自动处理特殊字符，确保文件名安全
- **错误处理**: 单个应用失败不影响其他应用的导出

## 🔧 高级选项说明

| 选项 | 短格式 | 说明 | 默认值 |
|------|--------|------|--------|
| `--output` | `-o` | 输出目录路径 | `./exported_icons` |
| `--size` | `-s` | 图标尺寸（像素） | `512` |
| `--format` | `-f` | 输出格式 (png/jpeg/tiff) | `png` |
| `--dir` | `-d` | 扫描目录（可多次使用） | `/Applications` 和 `/System/Applications` |
| `--no-parallel` | - | 禁用并行处理 | 默认启用并行 |
| `--help` | `-h` | 显示帮助信息 | - |

## ⚠️ 注意事项

1. **权限问题**: 某些系统应用可能因权限限制无法访问图标文件
2. **图标格式**: 仅支持标准的 `.icns` 格式图标
3. **文件覆盖**: 如果输出目录已存在同名文件，将会被覆盖
4. **中文支持**: 完美支持中文应用名称（如"哔哩哔哩"、"网易云音乐"等）

## 🐛 常见问题

### Q: 为什么某些应用无法导出？

A: 可能的原因：
- 应用没有标准的 Info.plist 文件
- 图标文件不是标准的 .icns 格式
- 缺少权限访问系统应用

### Q: 如何导出更大的图标？

A: 使用 `-s` 参数指定更大的尺寸：

```bash
cargo run --example export_icons_advanced -- -s 1024
# 或者
cargo run --example export_icons_advanced -- -s 2048
```

### Q: 导出速度慢怎么办？

A: 默认已启用并行处理。如果还是觉得慢，可以：
- 减小图标尺寸（使用 `-s 256` 等）
- 使用 JPEG 格式（处理更快）
- 只扫描特定目录（使用 `-d` 指定）

### Q: 可以导出到桌面吗？

A: 当然可以：

```bash
cargo run --example export_icons_advanced -- -o ~/Desktop/icons
```

## 📝 更多信息

详细的项目文档请查看：
- [示例程序说明](./README.md)
- [项目主 README](../README.md)

## 📄 许可证

MIT License - 详见项目根目录的 LICENSE 文件

