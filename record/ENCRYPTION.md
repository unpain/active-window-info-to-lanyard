# State数据加密功能

## 概述

本程序现在支持对发送到Discord的state数据（窗口标题）进行AES-256-GCM加密，以保护您的隐私。启用加密后，Discord上显示的state内容将是加密后的Base64字符串，而不是明文窗口标题。

## 功能特性

- **AES-256-GCM加密**: 使用工业级加密算法保护数据
- **自动加密/解密**: 发送时自动加密，需要时可以解密
- **可选功能**: 默认不启用，需要手动配置
- **密钥管理**: 支持自定义32字节（256位）加密密钥

## 快速开始

### 1. 生成加密密钥

您可以使用以下任意方式生成一个安全的加密密钥：

#### 方法1：使用Rust代码生成

创建一个临时文件 `generate_key.rs`：

```rust
use active_window_info_to_lanyard_lib::CryptoManager;

fn main() {
    let key = CryptoManager::generate_key();
    let hex_key = CryptoManager::key_to_hex(&key);
    println!("生成的加密密钥:");
    println!("{}", hex_key);
    println!("\n请将此密钥添加到.env文件中");
}
```

编译并运行：
```bash
cargo run --example generate_key
```

#### 方法2：使用在线工具

访问 https://www.random.org/bytes/ 生成32字节随机数据，并转换为十六进制。

#### 方法3：使用OpenSSL

```bash
openssl rand -hex 32
```

### 2. 配置加密密钥

在项目根目录的 `.env` 文件中添加加密密钥：

```env
DISCORD_APP_ID=你的Discord应用ID
ENCRYPTION_KEY=你的64字符十六进制密钥
```

示例：
```env
DISCORD_APP_ID=1234567890123456789
ENCRYPTION_KEY=a1b2c3d4e5f6789012345678901234567890abcdefabcdefabcdefabcdef1234
```

### 3. 修改主程序代码

在 `src/main.rs` 中添加加密密钥的读取：

```rust
// 读取.env文件
let mut file = File::open(".env").expect("未找到.env文件");
let mut contents = String::new();
file.read_to_string(&mut contents).expect("无法读取.env文件");

// 解析配置
let mut discord_app_id = None;
let mut encryption_key = None;

for line in contents.lines() {
    let parts: Vec<&str> = line.split('=').collect();
    if parts.len() == 2 {
        match parts[0].trim() {
            "DISCORD_APP_ID" => discord_app_id = Some(parts[1].trim()),
            "ENCRYPTION_KEY" => encryption_key = Some(parts[1].trim().to_string()),
            _ => {}
        }
    }
}

let app_id = discord_app_id.expect("未设置DISCORD_APP_ID");

// 创建配置
let config = if let Some(key) = encryption_key {
    println!("🔐 已启用加密功能");
    Config::new_with_encryption(
        app_id.parse().expect("无效的Discord应用ID"),
        UPDATE_INTERVAL,
        key,
    )
} else {
    println!("⚠️  未启用加密（明文传输）");
    Config::from_str(app_id, UPDATE_INTERVAL).expect("创建配置失败")
};
```

## API 使用说明

### 创建带加密的配置

```rust
use active_window_info_to_lanyard_lib::Config;

// 不启用加密
let config = Config::new(1234567890, 5);

// 启用加密
let config = Config::new_with_encryption(
    1234567890,
    5,
    "a1b2c3d4...".to_string(), // 64字符的十六进制密钥
);

// 检查是否启用加密
if config.is_encryption_enabled() {
    println!("加密已启用");
}
```

### 加密管理器直接使用

```rust
use active_window_info_to_lanyard_lib::CryptoManager;

// 从十六进制字符串创建
let crypto = CryptoManager::from_hex("a1b2c3d4...").unwrap();

// 加密数据
let encrypted = crypto.encrypt("敏感信息").unwrap();
println!("加密后: {}", encrypted);

// 解密数据
let decrypted = crypto.decrypt(&encrypted).unwrap();
println!("解密后: {}", decrypted);
```

### Discord管理器使用

```rust
use active_window_info_to_lanyard_lib::{Config, DiscordManager};

let config = Config::new_with_encryption(
    1234567890,
    5,
    "your_key_here".to_string(),
);

let mut discord = DiscordManager::connect(&config).unwrap();

// 检查是否启用加密
if discord.is_encryption_enabled() {
    println!("Discord状态将被加密");
}

// 解密state数据（用于调试）
let encrypted_state = "base64_encrypted_data";
match discord.decrypt_state(encrypted_state) {
    Ok(plaintext) => println!("原始内容: {}", plaintext),
    Err(e) => eprintln!("解密失败: {}", e),
}
```

## 安全建议

1. **密钥保管**: 
   - 永远不要将加密密钥提交到版本控制系统（Git）
   - 确保 `.env` 文件已添加到 `.gitignore`

2. **密钥生成**:
   - 使用加密安全的随机数生成器生成密钥
   - 密钥长度必须是64个十六进制字符（32字节）

3. **密钥轮换**:
   - 定期更换加密密钥以提高安全性
   - 如果怀疑密钥泄露，立即更换

4. **加密范围**:
   - 当前仅加密Discord状态的state字段（窗口标题）
   - details字段（应用名称）和其他元数据不加密

## 示例：完整的主程序

```rust
use active_window_info_to_lanyard_lib::{Config, DiscordManager, WindowInfo, WindowMonitor};
use std::{fs::File, io::Read, thread};

const UPDATE_INTERVAL: u64 = 5;

fn main() {
    // 读取配置
    let (app_id, encryption_key) = read_env_config();
    
    // 创建配置
    let config = if let Some(key) = encryption_key {
        println!("🔐 加密已启用");
        Config::new_with_encryption(
            app_id.parse().expect("无效的应用ID"),
            UPDATE_INTERVAL,
            key,
        )
    } else {
        println!("⚠️  未启用加密");
        Config::from_str(&app_id, UPDATE_INTERVAL).expect("创建配置失败")
    };

    config.validate().expect("配置验证失败");

    // 连接Discord
    let mut discord = DiscordManager::connect(&config).expect("连接Discord失败");
    println!("✅ 已连接到Discord RPC");

    // 监控窗口
    let mut window_monitor = WindowMonitor::new();
    println!("👀 开始监控活动窗口...\n");

    loop {
        if let Some(window_title) = window_monitor.check_for_change() {
            let window_info = WindowInfo::parse(&window_title);
            
            match discord.update_activity(&window_info, &window_title) {
                Ok(_) => {
                    if discord.is_encryption_enabled() {
                        println!("✅ 状态已更新（已加密）");
                    } else {
                        println!("✅ 状态已更新");
                    }
                }
                Err(e) => eprintln!("❌ 更新失败: {}", e),
            }
        }

        thread::sleep(config.update_interval);
    }
}

fn read_env_config() -> (String, Option<String>) {
    let mut file = File::open(".env").expect("未找到.env文件");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("无法读取.env文件");

    let mut app_id = None;
    let mut encryption_key = None;

    for line in contents.lines() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }
        
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            match parts[0].trim() {
                "DISCORD_APP_ID" => app_id = Some(parts[1].trim().to_string()),
                "ENCRYPTION_KEY" => encryption_key = Some(parts[1].trim().to_string()),
                _ => {}
            }
        }
    }

    (app_id.expect("未设置DISCORD_APP_ID"), encryption_key)
}
```

## 常见问题

### Q: 加密会影响性能吗？
A: AES-256-GCM是一个高性能的加密算法，对窗口标题这种短文本的加密几乎没有性能影响。

### Q: 如果丢失了加密密钥怎么办？
A: 如果丢失密钥，无法解密已加密的数据。但这不影响程序运行，您可以生成新密钥继续使用。

### Q: 可以在运行时更改加密密钥吗？
A: 当前版本不支持运行时更改，需要修改配置并重启程序。

### Q: Discord上会显示什么？
A: 启用加密后，Discord上的state字段会显示类似这样的Base64编码字符串：
```
Nq7x5YmK8vP... (加密的窗口标题)
```

## 测试加密功能

您可以运行内置的测试来验证加密功能：

```bash
cargo test crypto
```

这将运行 `src/crypto.rs` 中的所有单元测试。

