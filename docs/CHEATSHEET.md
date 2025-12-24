# 快速参考卡 - Windows Activity Monitor

## 一、快速配置（5分钟）

### 1. 获取Application ID

```plaintext
https://discord.com/developers/applications
→ New Application
→ 复制 Application ID
```

### 2. 编辑配置

```rust
// src/main.rs 第12行
const DISCORD_APP_ID: &str = "你的ID";
```

### 3. 运行

```bash
start.bat          # Windows一键启动
# 或
cargo run --release
```

## 二、常用命令

```bash
# 检查代码
cargo check

# 开发运行
cargo run

# 发布构建
cargo build --release

# 代码检查
cargo clippy

# 清理构建
cargo clean
```

## 三、文件位置

| 文件 | 位置 | 说明 |
|------|------|------|
| 主程序 | `src/main.rs` | 基础版本 |
| 高级版 | `src/main_advanced.rs` | 功能更多 |
| 可执行文件 | `target/release/*.exe` | 编译输出 |
| 配置示例 | `config.example.txt` | 高级版配置 |

## 四、Lanyard集成

### 步骤

1. 加入服务器: <https://discord.gg/UrXF2cfJ7F>
2. 获取User ID: Discord → 右键 → 复制ID
3. API地址: `https://api.lanyard.rest/v1/users/YOUR_ID`

### API响应

```json
{
  "data": {
    "activities": [
      {
        "name": "Windows Activity Monitor",
        "state": "窗口标题",
        "details": "应用详情"
      }
    ]
  }
}
```

## 五、配置选项

### 基础版 (src/main.rs)

```rust
const DISCORD_APP_ID: &str = "...";     // 必填
const UPDATE_INTERVAL: u64 = 5;         // 更新间隔（秒）
```

### 高级版 (config.txt)

```ini
DISCORD_APP_ID=...        # Application ID
UPDATE_INTERVAL=5         # 更新间隔
SHOW_DETAILS=true         # 显示详情
SHOW_TIMESTAMPS=true      # 显示时间
SHOW_ICONS=true           # 显示图标
```

## 六、故障排除

### ❌ 无法连接Discord

```plaintext
→ 确保Discord正在运行
→ 设置 → 活动状态 → 开启"显示当前活动"
```

### ❌ Lanyard无数据

```plaintext
→ 确认已加入Lanyard服务器
→ Discord状态不是"隐身"
→ 程序正在运行
```

### ❌ 窗口标题为空

```plaintext
→ 某些系统窗口无法读取
→ 尝试以管理员身份运行
```

### ❌ 编译失败

```bash
# 更新工具链
rustup update

# 清理重建
cargo clean
cargo build
```

## 七、Discord Rich Presence显示内容

```plaintext
┌─────────────────────────────┐
│ 🎯 Windows Activity Monitor │  ← 应用名称
│                              │
│ 📝 状态: 窗口标题           │  ← 当前窗口
│ 💻 详情: Using: 应用名      │  ← 应用信息
│ ⏱️  已持续: XX分钟           │  ← 时间戳
│                              │
│ [图标]                       │  ← 自定义图标
└─────────────────────────────┘
```

## 八、应用类型检测（高级版）

| 应用类型 | 图标 | 检测关键词 |
|---------|------|-----------|
| 编程 | 💻 | code, studio, vim |
| 浏览 | 🌐 | chrome, firefox, edge |
| 音乐 | 🎵 | spotify, music |
| 游戏 | 🎮 | steam, game |
| 办公 | 📄 | word, excel, office |
| 设计 | 🎨 | photoshop, illustrator |
| 聊天 | 💬 | discord, slack |

## 九、网页集成示例

```html
<!-- 加载Lanyard API -->
<script>
fetch('https://api.lanyard.rest/v1/users/YOUR_ID')
  .then(r => r.json())
  .then(data => {
    const activity = data.data.activities[0];
    console.log('当前活动:', activity.state);
  });
</script>
```

完整示例: `examples/lanyard-web-demo.html`

## 十、系统要求

| 项目 | 要求 |
|------|------|
| 操作系统 | Windows 7+ |
| Rust | 1.70+ |
| Discord | 桌面客户端 |
| 内存 | < 20MB |
| CPU | 极低占用 |

## 十一、文档索引

- 📖 [README.md](README.md) - 完整文档
- 🚀 [QUICKSTART.md](QUICKSTART.md) - 快速开始
- 🌐 [LANYARD.md](LANYARD.md) - Lanyard指南
- 📁 [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) - 项目结构
- 📊 [SUMMARY.md](SUMMARY.md) - 项目总结

## 十二、快捷链接

| 资源 | 链接 |
|------|------|
| Discord Developer | <https://discord.com/developers> |
| Lanyard服务器 | <https://discord.gg/UrXF2cfJ7F> |
| Lanyard API | <https://api.lanyard.rest> |
| Discord RPC文档 | <https://discord.com/developers/docs> |

---

**快速支持**: 查看README.md的"故障排除"部分
