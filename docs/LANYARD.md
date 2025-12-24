# Lanyard 集成详细说明

本文档详细说明如何将Windows活动监视器与Lanyard集成。

## 什么是Lanyard？

Lanyard是一个开源的Discord状态API服务，可以让你通过REST API或WebSocket实时访问Discord用户的状态信息，包括：

- 在线状态 (online/idle/dnd/offline)
- 正在玩的游戏
- Rich Presence活动
- Spotify播放状态
- 自定义状态

项目地址：<https://github.com/Phineas/lanyard>

## 集成步骤

### 1. 连接到Lanyard

1. 加入Lanyard Discord服务器
   - 访问：<https://discord.gg/UrXF2cfJ7F>
   - 点击加入服务器

2. 你的Discord账号会自动被Lanyard追踪

3. 获取你的Discord User ID
   - 在Discord中启用开发者模式：设置 → 高级 → 开发者模式
   - 右键点击你的用户名 → 复制ID

### 2. 运行Windows Activity Monitor

1. 配置Discord Application ID（见主README）
2. 运行程序：`cargo run`
3. 程序会将当前窗口信息发送到Discord Rich Presence

### 3. 通过Lanyard API访问状态

#### REST API

```bash
# 获取你的Discord状态
curl https://api.lanyard.rest/v1/users/YOUR_DISCORD_USER_ID
```

返回示例：

```json
{
  "success": true,
  "data": {
    "discord_user": {
      "id": "YOUR_USER_ID",
      "username": "your_username",
      "avatar": "avatar_hash",
      "discriminator": "0",
      "bot": false,
      "global_name": "Your Display Name",
      "avatar_decoration_data": null,
      "display_name": "Your Display Name",
      "public_flags": 0
    },
    "discord_status": "online",
    "activities": [
      {
        "type": 0,
        "state": "Visual Studio Code - main.rs",
        "name": "Windows Activity Monitor",
        "id": "custom",
        "details": "Using: Visual Studio Code",
        "timestamps": {
          "start": 1234567890
        },
        "assets": {
          "large_image": "windows",
          "large_text": "Windows Activity"
        },
        "application_id": "YOUR_APP_ID"
      }
    ],
    "spotify": null,
    "kv": {}
  }
}
```

#### WebSocket (实时更新)

```javascript
const ws = new WebSocket('wss://api.lanyard.rest/socket');

ws.onopen = () => {
  // 订阅特定用户
  ws.send(JSON.stringify({
    op: 2,
    d: {
      subscribe_to_id: "YOUR_DISCORD_USER_ID"
    }
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Discord状态更新:', data);
};
```

## 使用场景

### 1. 个人网站状态展示

在你的个人网站上实时显示你正在使用的应用：

```html
<!DOCTYPE html>
<html>
<head>
    <title>我的状态</title>
</head>
<body>
    <div id="status">加载中...</div>

    <script>
        async function updateStatus() {
            const response = await fetch('https://api.lanyard.rest/v1/users/YOUR_USER_ID');
            const data = await response.json();
            
            if (data.success && data.data.activities.length > 0) {
                const activity = data.data.activities[0];
                document.getElementById('status').innerHTML = `
                    <h3>${activity.name}</h3>
                    <p>${activity.state}</p>
                    <p>${activity.details}</p>
                `;
            }
        }

        // 每10秒更新一次
        setInterval(updateStatus, 10000);
        updateStatus();
    </script>
</body>
</html>
```

### 2. Discord机器人集成

创建一个Discord机器人来显示其他用户的当前活动：

```python
import discord
import aiohttp

client = discord.Client()

@client.command()
async def activity(ctx, user_id):
    async with aiohttp.ClientSession() as session:
        async with session.get(f'https://api.lanyard.rest/v1/users/{user_id}') as resp:
            data = await resp.json()
            
            if data['success'] and data['data']['activities']:
                activity = data['data']['activities'][0]
                await ctx.send(f"正在使用: {activity['state']}")
            else:
                await ctx.send("该用户当前没有活动")
```

### 3. 自动化工作流

使用Lanyard API触发自动化任务：

```python
import requests
import time

def check_if_coding():
    response = requests.get('https://api.lanyard.rest/v1/users/YOUR_USER_ID')
    data = response.json()
    
    if data['success'] and data['data']['activities']:
        for activity in data['data']['activities']:
            if 'Visual Studio Code' in activity.get('state', ''):
                return True
    return False

while True:
    if check_if_coding():
        print("检测到正在编程，启动专注模式...")
        # 执行自动化任务
    time.sleep(60)
```

## 隐私设置

要控制Lanyard可以访问的信息：

1. Discord设置 → 隐私与安全
2. 调整 "活动状态" 设置
3. 选择哪些应用可以显示活动

要停止Lanyard追踪：

- 退出Lanyard Discord服务器即可

## API限制

Lanyard REST API限制：

- 无认证限制（公开访问）
- 建议不要过于频繁请求（推荐间隔 ≥ 10秒）

WebSocket连接：

- 每个连接需要心跳维持（30秒间隔）
- 自动重连机制建议实现

## 进阶功能

### KV存储

Lanyard提供键值存储功能（需要API密钥）：

```bash
# 设置自定义数据
curl -X PUT https://api.lanyard.rest/v1/users/YOUR_USER_ID/kv/custom_key \
  -H "Authorization: YOUR_API_KEY" \
  -d "custom_value"
```

可以用来存储：

- 工作时长统计
- 应用使用频率
- 自定义状态消息

## 故障排除

### Lanyard未显示活动

1. 确保已加入Lanyard Discord服务器
2. 检查Discord隐私设置
3. 确认Discord RPC连接成功
4. 等待几秒钟让Lanyard更新

### API返回空activities

- Discord状态必须为在线（online/idle/dnd）
- 确认Discord允许显示活动状态
- Windows Activity Monitor必须正在运行

## 资源链接

- Lanyard GitHub: <https://github.com/Phineas/lanyard>
- Discord Developer Portal: <https://discord.com/developers>
- Lanyard API文档: <https://github.com/Phineas/lanyard#readme>
- Discord RPC文档: <https://discord.com/developers/docs/topics/rpc>

## 社区项目

基于Lanyard的开源项目：

- lanyard-profile-readme: GitHub README状态卡片
- react-use-lanyard: React Hook
- vue-lanyard: Vue组件
- lanyard-visualizer: 状态可视化工具
