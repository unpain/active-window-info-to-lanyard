use active_window_info_to_lanyard_lib::{ Config, DiscordManager, WindowInfo, WindowMonitor };
/// 跨平台 Discord Activity Monitor - 主入口
///
/// 监控活动窗口并将其同步到Discord Rich Presence
/// 支持 Windows 和 macOS 平台
use std::{ fs::File, io::Read, thread };

// 更新间隔（秒）
const UPDATE_INTERVAL: u64 = 5;

fn main() {
    // 读取并解析.env文件
    let (discord_app_id, encryption_key) = read_env_config();
    
    // 打印欢迎信息
    print_welcome(&discord_app_id, encryption_key.is_some());
    
    // 创建配置
    let config = if let Some(key) = encryption_key {
        println!("🔐 加密功能已启用");
        println!();
        
        let cfg = Config::new_with_encryption(
            discord_app_id.parse().expect("无效的Discord应用ID"),
            UPDATE_INTERVAL,
            key,
        );
        
        if let Err(e) = cfg.validate() {
            eprintln!("❌ 配置验证失败: {}", e);
            return;
        }
        cfg
    } else {
        println!("⚠️  加密功能未启用（明文传输）");
        println!("   提示：在.env中添加ENCRYPTION_KEY可启用加密");
        println!();
        
        match Config::from_str(&discord_app_id, UPDATE_INTERVAL) {
            Ok(cfg) => {
                if let Err(e) = cfg.validate() {
                    eprintln!("❌ 配置验证失败: {}", e);
                    eprintln!("   请在 https://discord.com/developers/applications 获取应用ID");
                    return;
                }
                cfg
            }
            Err(e) => {
                eprintln!("❌ 配置创建失败: {}", e);
                return;
            }
        }
    };

    // 连接到Discord RPC
    let mut discord = match DiscordManager::connect(&config) {
        Ok(manager) => {
            println!("✅ 已连接到Discord RPC");
            manager
        }
        Err(e) => {
            eprintln!("❌ 连接Discord失败: {}", e);
            return;
        }
    };

    // 创建窗口监控器
    let mut window_monitor = WindowMonitor::new();

    println!("👀 开始监控活动窗口...\n");

    // 主循环
    loop {
        if let Some(window_title) = window_monitor.check_for_change() {
            println!("🔄 窗口变化: {}", window_title);

            // 解析窗口信息
            let window_info = WindowInfo::parse(&window_title);

            // 更新Discord状态
            match discord.update_activity(&window_info, &window_title) {
                Ok(_) => println!("✅ Discord状态已更新"),
                Err(e) => eprintln!("⚠️  更新Discord失败: {}", e),
            }
        }

        // 等待指定时间后再次检查
        thread::sleep(config.update_interval);
    }
}

/// 打印欢迎信息
fn print_welcome(discord_app_id: &str, encryption_enabled: bool) {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  Discord Activity Monitor                     ║");
    println!("║  活动窗口监控 → Discord Rich Presence         ║");
    println!("║  支持: Windows & macOS                        ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
    println!("📝 版本: {}", active_window_info_to_lanyard_lib::VERSION);
    println!("⏱️  更新间隔: {} 秒", UPDATE_INTERVAL);
    println!("🔧 Discord应用ID: {}", discord_app_id);
    if encryption_enabled {
        println!("🔐 加密: 已启用");
    } else {
        println!("🔓 加密: 未启用");
    }
    println!();
}

/// 从.env文件读取配置
fn read_env_config() -> (String, Option<String>) {
    let mut file = File::open(".env").unwrap_or_else(|_| {
        eprintln!("❌ 未找到.env文件");
        eprintln!("   请在项目根目录创建.env文件");
        eprintln!("   格式:");
        eprintln!("   DISCORD_APP_ID=你的应用ID");
        eprintln!("   ENCRYPTION_KEY=你的加密密钥（可选）");
        std::process::exit(1);
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("无法读取.env文件");

    let mut app_id = None;
    let mut encryption_key = None;

    // 逐行解析
    for line in contents.lines() {
        let line = line.trim();
        
        // 跳过空行和注释
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
