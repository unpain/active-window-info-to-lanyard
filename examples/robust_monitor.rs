//! 增强版窗口监控程序
//!
//! 改进：
//! - 添加心跳检测和健康检查
//! - Discord RPC 断线自动重连
//! - 更详细的日志和错误处理
//! - 防止长时间运行后卡住

use active_window_info_to_lanyard_lib::{Config, DiscordManager, WindowInfo, WindowMonitor};
use std::time::{Duration, Instant};
use std::{fs::File, io::Read, thread};

// 更新间隔（秒）
const UPDATE_INTERVAL: u64 = 5;
// 心跳间隔（秒）- 每隔这个时间打印一次状态
const HEARTBEAT_INTERVAL: u64 = 60;
// Discord 重连间隔（秒）
const RECONNECT_INTERVAL: u64 = 30;

fn main() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  Discord Activity Monitor (增强版)           ║");
    println!("║  Enhanced Robust Monitor with Auto-Recovery  ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();

    // 读取配置
    let (discord_app_id, encryption_key) = read_env_config();
    println!("🔧 Discord应用ID: {}", discord_app_id);
    println!("⏱️  更新间隔: {} 秒", UPDATE_INTERVAL);
    println!("💓 心跳间隔: {} 秒", HEARTBEAT_INTERVAL);
    
    if encryption_key.is_some() {
        println!("🔐 加密: 已启用");
    } else {
        println!("🔓 加密: 未启用");
    }
    println!();

    // 创建配置
    let config = create_config(&discord_app_id, encryption_key);

    // 创建窗口监控器
    let mut window_monitor = WindowMonitor::new();

    // 主循环状态
    let mut discord: Option<DiscordManager> = None;
    let mut last_successful_update = Instant::now();
    let mut last_heartbeat = Instant::now();
    let mut update_count = 0u64;
    let mut error_count = 0u64;
    let mut reconnect_count = 0u64;

    // 初始连接
    match connect_discord(&config) {
        Ok(manager) => {
            discord = Some(manager);
            println!("✅ 已连接到Discord RPC\n");
        }
        Err(e) => {
            eprintln!("❌ 初始连接失败: {}", e);
            eprintln!("   将在 {} 秒后重试...\n", RECONNECT_INTERVAL);
        }
    }

    println!("👀 开始监控活动窗口...\n");

    // 主循环
    loop {
        // 心跳检测
        if last_heartbeat.elapsed().as_secs() >= HEARTBEAT_INTERVAL {
            println!("💓 [心跳] 运行中 | 更新: {} 次 | 错误: {} 次 | 重连: {} 次 | 距上次成功: {} 秒",
                update_count,
                error_count,
                reconnect_count,
                last_successful_update.elapsed().as_secs()
            );
            last_heartbeat = Instant::now();
        }

        // 检查是否需要重连（超过重连间隔没有成功更新）
        if discord.is_none() || last_successful_update.elapsed().as_secs() > RECONNECT_INTERVAL * 2 {
            if discord.is_some() {
                println!("⚠️  检测到可能的连接问题，尝试重新连接...");
            }
            
            match connect_discord(&config) {
                Ok(manager) => {
                    discord = Some(manager);
                    reconnect_count += 1;
                    last_successful_update = Instant::now();
                    println!("✅ Discord 重新连接成功（第 {} 次）\n", reconnect_count);
                }
                Err(e) => {
                    eprintln!("❌ 重连失败: {}", e);
                    discord = None;
                    thread::sleep(Duration::from_secs(RECONNECT_INTERVAL));
                    continue;
                }
            }
        }

        // 检查窗口变化
        if let Some(window_title) = window_monitor.check_for_change() {
            println!("🔄 [{}] 窗口变化: {}", 
                chrono::Local::now().format("%H:%M:%S"),
                window_title
            );

            // 解析窗口信息
            let window_info = WindowInfo::parse(&window_title);

            // 尝试更新Discord状态
            if let Some(ref mut discord_manager) = discord {
                match discord_manager.update_activity(&window_info, &window_title) {
                    Ok(_) => {
                        update_count += 1;
                        last_successful_update = Instant::now();
                        println!("✅ Discord状态已更新（第 {} 次）", update_count);
                    }
                    Err(e) => {
                        error_count += 1;
                        eprintln!("⚠️  更新Discord失败: {}", e);
                        eprintln!("   将在下次心跳时尝试重连");
                        
                        // 标记需要重连
                        discord = None;
                    }
                }
            } else {
                println!("⏸️  Discord未连接，跳过更新");
            }
        }

        // 等待指定时间后再次检查
        thread::sleep(config.update_interval);
    }
}

/// 连接到Discord RPC
fn connect_discord(config: &Config) -> Result<DiscordManager, String> {
    DiscordManager::connect(config)
}

/// 创建配置
fn create_config(discord_app_id: &str, encryption_key: Option<String>) -> Config {
    if let Some(key) = encryption_key {
        let cfg = Config::new_with_encryption(
            discord_app_id.parse().expect("无效的Discord应用ID"),
            UPDATE_INTERVAL,
            key,
        );
        
        if let Err(e) = cfg.validate() {
            eprintln!("❌ 配置验证失败: {}", e);
            std::process::exit(1);
        }
        cfg
    } else {
        match Config::from_str(discord_app_id, UPDATE_INTERVAL) {
            Ok(cfg) => {
                if let Err(e) = cfg.validate() {
                    eprintln!("❌ 配置验证失败: {}", e);
                    std::process::exit(1);
                }
                cfg
            }
            Err(e) => {
                eprintln!("❌ 配置创建失败: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// 从.env文件读取配置
fn read_env_config() -> (String, Option<String>) {
    let mut file = File::open(".env").unwrap_or_else(|_| {
        eprintln!("❌ 未找到.env文件");
        eprintln!("   请在项目根目录创建.env文件");
        std::process::exit(1);
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("无法读取.env文件");

    let mut app_id = None;
    let mut encryption_key = None;

    for line in contents.lines() {
        let line = line.trim();
        
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

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

