/// 带加密功能的Discord Activity Monitor示例
///
/// 展示如何集成加密功能保护Discord状态数据
use active_window_info_to_lanyard_lib::{Config, DiscordManager, WindowInfo, WindowMonitor};
use std::{fs::File, io::Read, thread};

const UPDATE_INTERVAL: u64 = 5;

fn main() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  Discord Activity Monitor (带加密)            ║");
    println!("╚════════════════════════════════════════════════╝\n");

    // 读取配置
    let (app_id, encryption_key) = read_env_config();

    // 创建配置
    let config = if let Some(key) = encryption_key {
        println!("🔐 加密功能已启用");
        println!("   State数据将被加密后发送到Discord\n");
        Config::new_with_encryption(app_id.parse().expect("无效的应用ID"), UPDATE_INTERVAL, key)
    } else {
        println!("⚠️  加密功能未启用（明文传输）");
        println!("   提示：在.env中添加ENCRYPTION_KEY可启用加密\n");
        Config::from_str(&app_id, UPDATE_INTERVAL).expect("创建配置失败")
    };

    // 验证配置
    if let Err(e) = config.validate() {
        eprintln!("❌ 配置验证失败: {}", e);
        return;
    }

    // 连接Discord
    let mut discord = match DiscordManager::connect(&config) {
        Ok(manager) => {
            println!("✅ 已连接到Discord RPC");
            if manager.is_encryption_enabled() {
                println!("🔒 加密管理器已初始化");
            }
            println!();
            manager
        }
        Err(e) => {
            eprintln!("❌ 连接Discord失败: {}", e);
            return;
        }
    };

    // 创建窗口监控器
    let mut window_monitor = WindowMonitor::new();
    println!("👀 开始监控活动窗口...");
    println!("⏱️  更新间隔: {} 秒", UPDATE_INTERVAL);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // 主循环
    loop {
        if let Some(window_title) = window_monitor.check_for_change() {
            println!("🔄 窗口变化检测到");
            println!("   标题: {}", window_title);

            // 解析窗口信息
            let window_info = WindowInfo::parse(&window_title);
            println!("   应用: {}", window_info.app_name);
            println!("   详情: {}", window_info.details);

            // 更新Discord状态
            match discord.update_activity(&window_info, &window_title) {
                Ok(_) => {
                    if discord.is_encryption_enabled() {
                        println!("✅ Discord状态已更新（已加密）");
                    } else {
                        println!("✅ Discord状态已更新");
                    }
                }
                Err(e) => {
                    eprintln!("❌ 更新Discord失败: {}", e);
                }
            }
            println!();
        }

        // 等待指定时间后再次检查
        thread::sleep(config.update_interval);
    }
}

/// 从.env文件读取配置
fn read_env_config() -> (String, Option<String>) {
    let mut file = match File::open(".env") {
        Ok(f) => f,
        Err(_) => {
            eprintln!("❌ 未找到.env文件");
            eprintln!("   请在项目根目录创建.env文件");
            eprintln!("   格式:");
            eprintln!("   DISCORD_APP_ID=你的应用ID");
            eprintln!("   ENCRYPTION_KEY=你的加密密钥（可选）");
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("无法读取.env文件");

    let mut app_id = None;
    let mut encryption_key = None;

    for line in contents.lines() {
        // 跳过空行和注释
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // 解析键值对
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();

            match key {
                "DISCORD_APP_ID" => app_id = Some(value.to_string()),
                "ENCRYPTION_KEY" => {
                    if !value.is_empty() {
                        encryption_key = Some(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    let app_id = app_id.unwrap_or_else(|| {
        eprintln!("❌ .env文件中未设置DISCORD_APP_ID");
        std::process::exit(1);
    });

    (app_id, encryption_key)
}

