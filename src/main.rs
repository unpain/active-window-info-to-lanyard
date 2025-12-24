use cur_win_discord_rust::{ Config, DiscordManager, WindowInfo, WindowMonitor };
/// Windows Discord Activity Monitor - 主入口
///
/// 监控Windows活动窗口并将其同步到Discord Rich Presence
use std::{ fs::File, io::Read, thread };

// 更新间隔（秒）
const UPDATE_INTERVAL: u64 = 5;

fn main() {
    let mut file = File::open(".env").expect("没有检测到.env文件，请在项目根目录下创建.env文件,并设置Discord Application ID,格式为DISCORD_APP_ID=你的Discord Application ID,Discord Application ID的获取方式请查看docs的QUICKSTART.md");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("无法读取.env文件");
    let discord_arr_id: &str = contents.split("=").nth(1).expect("没有检测到Discord Application ID，请在.env文件中设置Discord Application ID");
    // 打印欢迎信息
    print_welcome(discord_arr_id);
    // 创建并验证配置
    let config = match Config::from_str(discord_arr_id, UPDATE_INTERVAL) {
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
fn print_welcome(discord_arr_id: &str) {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  Windows Discord Activity Monitor            ║");
    println!("║  Windows活动窗口监控 → Discord Rich Presence  ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
    println!("📝 版本: {}", cur_win_discord_rust::VERSION);
    println!("⏱️  更新间隔: {} 秒", UPDATE_INTERVAL);
    println!("🔧 Discord应用ID: {}", discord_arr_id);
    println!();
}
