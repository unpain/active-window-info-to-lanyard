# 示例程序

本目录包含了一些实用的示例程序，帮助您更好地使用本项目的加密功能。

## 可用示例

### 1. 批量导出应用图标 (export_icons.rs)

批量导出 macOS 系统中所有已安装应用的图标。

**运行方式：**

```bash
cargo run --example export_icons
```

**功能：**

- 自动扫描 /Applications 和 /System/Applications 目录
- 读取应用的 Info.plist 获取图标文件信息
- 将 .icns 格式的图标转换为 PNG 格式
- 导出为 512x512 像素的高质量图标
- 自动创建 exported_icons 输出目录

**输出示例：**

```
╔════════════════════════════════════════════════╗
║  macOS 应用图标批量导出工具                  ║
║  Icon Exporter for macOS Applications        ║
╚════════════════════════════════════════════════╝

📁 输出目录: ./exported_icons

🔍 正在扫描: /Applications
   📦 Safari -> ✅ ./exported_icons/Safari.png
   📦 Chrome -> ✅ ./exported_icons/Chrome.png
   📦 Visual Studio Code -> ✅ ./exported_icons/Visual_Studio_Code.png
   ✅ 找到 50 个应用，成功导出 48 个图标

╔════════════════════════════════════════════════╗
║  导出完成！                                   ║
╚════════════════════════════════════════════════╝

📊 统计:
   - 扫描应用: 50 个
   - 成功导出: 48 个
   - 失败数量: 2 个

📁 图标已保存到: ./exported_icons
```

**高级版本 (export_icons_advanced.rs)：**

支持更多自定义选项：

```bash
# 查看所有选项
cargo run --example export_icons_advanced -- --help

# 导出为 1024x1024 的图标
cargo run --example export_icons_advanced -- -s 1024

# 导出为 JPEG 格式
cargo run --example export_icons_advanced -- -f jpeg

# 自定义输出目录
cargo run --example export_icons_advanced -- -o ~/Desktop/my_icons

# 组合使用多个选项
cargo run --example export_icons_advanced -- -s 256 -f png -o ./icons
```

**高级版功能：**
- 🎨 自定义图标尺寸（支持任意像素值）
- 📁 自定义输出目录
- 🖼️  多格式支持（PNG、JPEG、TIFF）
- 📂 自定义扫描目录
- ⚡ 并行处理（可选）
- 📊 详细的统计信息

**注意事项：**

- 仅支持 macOS 系统
- 需要系统自带的 `sips` 和 `plutil` 命令
- 图标文件保存在项目根目录的 `exported_icons` 文件夹（或自定义目录）
- 某些系统应用可能因权限问题无法导出

---

### 2. 生成加密密钥 (generate_key.rs)

生成一个安全的随机加密密钥，用于加密Discord状态数据。

**运行方式：**

```bash
cargo run --example generate_key
```

**功能：**

- 生成32字节（256位）的随机加密密钥
- 将密钥转换为64字符的十六进制格式
- 自动测试加密和解密功能
- 提供使用说明和安全建议

**输出示例：**

```
╔════════════════════════════════════════════════╗
║  Discord Activity Monitor - 密钥生成工具      ║
╚════════════════════════════════════════════════╝

🔑 正在生成加密密钥...
✅ 密钥生成成功！

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
您的加密密钥:
a1b2c3d4e5f6789012345678901234567890abcdefabcdefabcdefabcdef1234
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### 3. 测试加密功能 (test_encryption.rs)

交互式工具，用于测试加密和解密功能。

**运行方式：**

```bash
cargo run --example test_encryption
```

**功能：**

- 使用随机密钥测试加密/解密
- 使用自定义密钥测试加密/解密
- 解密已加密的数据
- 交互式命令行界面

**使用流程：**

1. 选择测试模式（随机密钥/自定义密钥/解密数据）
2. 输入要测试的消息或数据
3. 查看加密/解密结果和性能信息

### 4. 带加密功能的完整示例 (with_encryption.rs)

展示如何在实际应用中集成加密功能的完整示例程序。

**运行方式：**

```bash
cargo run --example with_encryption
```

**功能：**

- 从.env文件读取配置（包括加密密钥）
- 自动检测并启用加密功能
- 实时监控窗口并更新Discord状态
- 显示详细的加密状态信息

**配置要求：**
在项目根目录创建`.env`文件：

```env
DISCORD_APP_ID=你的应用ID
ENCRYPTION_KEY=生成的64字符密钥
```

## 快速开始

### 导出应用图标

如果你想批量导出系统中所有应用的图标：

```bash
cargo run --example export_icons
```

图标会自动保存到 `./exported_icons/` 目录，每个图标为 512x512 像素的 PNG 格式。

---

### 使用加密功能

### 步骤1：生成密钥

```bash
cargo run --example generate_key
```

复制生成的密钥并保存到 `.env` 文件：

```env
DISCORD_APP_ID=你的应用ID
ENCRYPTION_KEY=a1b2c3d4e5f6789012345678901234567890abcdefabcdefabcdefabcdef1234
```

### 步骤2：测试加密

```bash
cargo run --example test_encryption
```

选择选项2（使用自定义密钥测试），输入刚才生成的密钥，然后输入一些测试消息，验证加密和解密功能正常工作。

### 步骤3：运行带加密的程序

```bash
cargo run --example with_encryption
```

这个示例程序会自动读取.env文件中的加密密钥，并在更新Discord状态时自动加密窗口标题数据。

### 步骤4：在主程序中使用

修改 `src/main.rs`，参考 `examples/with_encryption.rs` 的实现，添加对加密密钥的支持。

## 注意事项

1. **密钥安全**：
   - 生成的密钥应妥善保管
   - 不要将密钥提交到版本控制系统
   - 确保 `.env` 已添加到 `.gitignore`

2. **密钥格式**：
   - 密钥必须是64个十六进制字符（0-9, a-f, A-F）
   - 对应32字节的二进制数据
   - 大小写不敏感

3. **加密范围**：
   - 仅加密Discord状态的state字段（窗口标题）
   - 其他字段（如应用名称）不加密

## 更多信息

详细的加密功能说明和API文档，请参阅 [docs/ENCRYPTION.md](../docs/ENCRYPTION.md)。
