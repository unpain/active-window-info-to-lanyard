# 快速入门指南

## 10分钟上手

### 步骤1：获取Discord Application ID

1. 访问 <https://discord.com/developers/applications>
2. 登录你的Discord账号
3. 点击右上角 "New Application"
4. 给应用起个名字，比如 "Window Monitor"
5. 创建后，在 "General Information" 页面找到 "Application ID"
6. 复制这个ID

### 步骤2：配置代码

打开 `src/main.rs`，找到第12行：

```rust
const DISCORD_APP_ID: &str = "YOUR_DISCORD_APP_ID";
```

将 `YOUR_DISCORD_APP_ID` 替换为你刚才复制的Application ID，例如：

```rust
const DISCORD_APP_ID: &str = "1234567890123456789";
```

### 步骤3：运行Discord

确保Discord桌面客户端正在运行，并且你已登录。

### 步骤4：运行程序

在项目目录下打开终端，运行：

```bash
cargo run
```

或者使用提供的启动脚本（Windows）：

```bash
start.bat
```

### 步骤5：查看效果

1. 打开Discord
2. 查看你的个人资料
3. 你应该能看到 "Windows Activity Monitor" 的状态
4. 切换窗口，状态会自动更新

## 与Lanyard集成

### 加入Lanyard服务器

1. 访问 <https://discord.gg/UrXF2cfJ7F>
2. 加入服务器
3. 你的Discord状态会自动被Lanyard追踪

### 获取你的Discord User ID

1. 在Discord中启用开发者模式：
   - 用户设置 → 高级 → 开发者模式（开启）
2. 右键点击你的用户名
3. 点击 "复制用户ID"

### 访问Lanyard API

在浏览器中访问：

```
https://api.lanyard.rest/v1/users/YOUR_DISCORD_USER_ID
```

（将 YOUR_DISCORD_USER_ID 替换为你的用户ID）

你会看到包含你当前Discord状态的JSON数据，其中 `activities` 数组包含了窗口活动信息。

## 常见问题

### Q: 连接Discord RPC失败

**A:** 确保：

- Discord桌面客户端正在运行
- Discord设置中允许显示活动状态：
  - 用户设置 → 活动状态 → "将您正在进行的活动作为状态消息展示" （开启）

### Q: Lanyard API显示空的activities

**A:** 检查：

- 是否已加入Lanyard Discord服务器
- Discord状态是否为在线（不是隐身）
- 程序是否正在运行

### Q: 窗口标题无法获取

**A (Windows):** 某些系统窗口和管理员权限窗口可能无法读取，尝试以管理员身份运行程序。

**A (macOS):** 首次运行时，系统会提示授予辅助功能权限：
- 打开"系统设置"
- 进入"隐私与安全性" → "辅助功能"
- 将本程序添加到允许列表

### Q: 想要自定义更新间隔

**A:** 在 `src/main.rs` 中修改：

```rust
const UPDATE_INTERVAL: u64 = 5;  // 单位：秒
```

## 进阶：使用配置文件版本

如果你想要更灵活的配置，可以使用高级版本：

1. 将 `src/main_advanced.rs` 重命名为 `src/main.rs`（备份原文件）
2. 复制 `config.example.txt` 为 `config.txt`
3. 在 `config.txt` 中设置你的配置：

```
DISCORD_APP_ID=1234567890123456789
UPDATE_INTERVAL=5
SHOW_DETAILS=true
SHOW_TIMESTAMPS=true
SHOW_ICONS=true
```

1. 运行 `cargo run`

高级版本特性：

- 📁 配置文件支持
- 🎨 自动检测应用类型（编程、浏览、游戏等）
- 🖼️ 应用图标映射
- 📝 更详细的日志输出
- ✂️ 文本截断（避免超长标题）

## 下一步

- 🎨 在Discord Developer Portal上传自定义图标
- 🌐 创建个人网站展示你的实时状态
- 🤖 开发Discord机器人集成Lanyard API
- 📊 统计你的应用使用时间

祝使用愉快！ 🎉
