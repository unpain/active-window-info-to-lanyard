//! 高级应用图标批量导出工具
//!
//! 功能增强：
//! - 支持命令行参数配置
//! - 可自定义输出目录
//! - 可自定义图标尺寸
//! - 支持选择扫描目录
//! - 支持导出为多种格式（PNG, JPEG, TIFF）
//! - 并行处理提升速度

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
struct ExportConfig {
    output_dir: PathBuf,
    icon_size: u32,
    format: String,
    scan_dirs: Vec<String>,
    parallel: bool,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("./exported_icons"),
            icon_size: 512,
            format: "png".to_string(),
            scan_dirs: vec![
                "/Applications".to_string(),
                "/System/Applications".to_string(),
            ],
            parallel: true,
        }
    }
}

fn main() {
    let config = parse_args();
    
    print_welcome(&config);
    
    // 创建输出目录
    if !config.output_dir.exists() {
        fs::create_dir_all(&config.output_dir).expect("无法创建输出目录");
    }
    
    println!("📁 输出目录: {}", config.output_dir.display());
    println!("📐 图标尺寸: {}x{} 像素", config.icon_size, config.icon_size);
    println!("🖼️  导出格式: {}", config.format.to_uppercase());
    println!("⚡ 并行处理: {}", if config.parallel { "开启" } else { "关闭" });
    println!();
    
    let mut total_count = 0;
    let mut success_count = 0;
    
    for apps_dir in &config.scan_dirs {
        println!("🔍 正在扫描: {}", apps_dir);
        
        let dir_path = Path::new(apps_dir);
        if !dir_path.exists() {
            println!("   ⚠️  目录不存在，跳过");
            continue;
        }
        
        match scan_and_export_icons(dir_path, &config) {
            Ok((count, success)) => {
                total_count += count;
                success_count += success;
                println!("   ✅ 找到 {} 个应用，成功导出 {} 个图标", count, success);
            }
            Err(e) => {
                println!("   ❌ 扫描失败: {}", e);
            }
        }
        println!();
    }
    
    println!("╔════════════════════════════════════════════════╗");
    println!("║  导出完成！                                   ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
    println!("📊 统计:");
    println!("   - 扫描应用: {} 个", total_count);
    println!("   - 成功导出: {} 个", success_count);
    println!("   - 失败数量: {} 个", total_count - success_count);
    if success_count > 0 {
        let success_rate = (success_count as f64 / total_count as f64) * 100.0;
        println!("   - 成功率: {:.1}%", success_rate);
    }
    println!();
    println!("📁 图标已保存到: {}", config.output_dir.display());
}

fn print_welcome(_config: &ExportConfig) {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  macOS 应用图标批量导出工具（高级版）        ║");
    println!("║  Advanced Icon Exporter for macOS            ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
}

fn parse_args() -> ExportConfig {
    let args: Vec<String> = env::args().collect();
    let mut config = ExportConfig::default();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    config.output_dir = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("错误: {} 需要一个参数", args[i]);
                    print_usage();
                    std::process::exit(1);
                }
            }
            "-s" | "--size" => {
                if i + 1 < args.len() {
                    config.icon_size = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("错误: 无效的尺寸值");
                        std::process::exit(1);
                    });
                    i += 2;
                } else {
                    eprintln!("错误: {} 需要一个参数", args[i]);
                    print_usage();
                    std::process::exit(1);
                }
            }
            "-f" | "--format" => {
                if i + 1 < args.len() {
                    let format = args[i + 1].to_lowercase();
                    if ["png", "jpeg", "jpg", "tiff", "tif"].contains(&format.as_str()) {
                        config.format = format;
                    } else {
                        eprintln!("错误: 不支持的格式: {}", format);
                        eprintln!("支持的格式: png, jpeg, jpg, tiff, tif");
                        std::process::exit(1);
                    }
                    i += 2;
                } else {
                    eprintln!("错误: {} 需要一个参数", args[i]);
                    print_usage();
                    std::process::exit(1);
                }
            }
            "-d" | "--dir" => {
                if i + 1 < args.len() {
                    // 清空默认目录，使用用户指定的目录
                    if config.scan_dirs.len() == 2 && config.scan_dirs[0] == "/Applications" {
                        config.scan_dirs.clear();
                    }
                    config.scan_dirs.push(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("错误: {} 需要一个参数", args[i]);
                    print_usage();
                    std::process::exit(1);
                }
            }
            "--no-parallel" => {
                config.parallel = false;
                i += 1;
            }
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                eprintln!("错误: 未知的选项: {}", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
    }
    
    config
}

fn print_usage() {
    println!("用法: cargo run --example export_icons_advanced [选项]");
    println!();
    println!("选项:");
    println!("  -o, --output <目录>    指定输出目录 (默认: ./exported_icons)");
    println!("  -s, --size <尺寸>      指定图标尺寸 (默认: 512)");
    println!("  -f, --format <格式>    指定输出格式: png, jpeg, tiff (默认: png)");
    println!("  -d, --dir <目录>       添加扫描目录 (可多次使用)");
    println!("  --no-parallel          禁用并行处理");
    println!("  -h, --help             显示此帮助信息");
    println!();
    println!("示例:");
    println!("  # 默认设置");
    println!("  cargo run --example export_icons_advanced");
    println!();
    println!("  # 导出为 1024x1024 的 PNG");
    println!("  cargo run --example export_icons_advanced -s 1024");
    println!();
    println!("  # 导出为 JPEG 格式");
    println!("  cargo run --example export_icons_advanced -f jpeg");
    println!();
    println!("  # 自定义输出目录");
    println!("  cargo run --example export_icons_advanced -o ~/Desktop/icons");
    println!();
    println!("  # 扫描自定义目录");
    println!("  cargo run --example export_icons_advanced -d ~/Applications");
}

fn scan_and_export_icons(apps_dir: &Path, config: &ExportConfig) -> Result<(usize, usize), String> {
    let entries = fs::read_dir(apps_dir)
        .map_err(|e| format!("无法读取目录: {}", e))?;
    
    // 收集所有应用路径
    let mut app_paths = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        
        let path = entry.path();
        
        // 只处理 .app 结尾的应用
        if path.is_dir() && path.extension().map_or(false, |ext| ext == "app") {
            app_paths.push(path);
        }
    }
    
    let total_count = app_paths.len();
    
    if config.parallel && total_count > 1 {
        // 并行处理
        let success_count = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for path in app_paths {
            let config = config.clone();
            let success_count = Arc::clone(&success_count);
            
            let handle = thread::spawn(move || {
                let app_name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown");
                
                print!("   📦 {}", app_name);
                
                match export_app_icon(&path, &config.output_dir, app_name, config.icon_size, &config.format) {
                    Ok(icon_path) => {
                        let mut count = success_count.lock().unwrap();
                        *count += 1;
                        println!(" -> ✅ {}", icon_path.display());
                    }
                    Err(e) => {
                        println!(" -> ❌ {}", e);
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
        
        let success_count = *success_count.lock().unwrap();
        Ok((total_count, success_count))
    } else {
        // 串行处理
        let mut success_count = 0;
        
        for path in app_paths {
            let app_name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown");
            
            print!("   📦 {}", app_name);
            
            match export_app_icon(&path, &config.output_dir, app_name, config.icon_size, &config.format) {
                Ok(icon_path) => {
                    success_count += 1;
                    println!(" -> ✅ {}", icon_path.display());
                }
                Err(e) => {
                    println!(" -> ❌ {}", e);
                }
            }
        }
        
        Ok((total_count, success_count))
    }
}

fn export_app_icon(
    app_path: &Path,
    output_dir: &Path,
    app_name: &str,
    size: u32,
    format: &str,
) -> Result<PathBuf, String> {
    // 读取 Info.plist 获取图标文件名
    let info_plist = app_path.join("Contents/Info.plist");
    if !info_plist.exists() {
        return Err("未找到 Info.plist".to_string());
    }
    
    // 使用 plutil 读取 plist 文件
    let output = Command::new("plutil")
        .args(&["-extract", "CFBundleIconFile", "raw", "-o", "-"])
        .arg(&info_plist)
        .output()
        .map_err(|e| format!("执行 plutil 失败: {}", e))?;
    
    if !output.status.success() {
        return Err("未找到图标配置".to_string());
    }
    
    let icon_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if icon_name.is_empty() {
        return Err("图标文件名为空".to_string());
    }
    
    // 尝试不同的图标文件扩展名
    let resources_dir = app_path.join("Contents/Resources");
    let possible_extensions = vec!["icns", ""];
    
    let mut icon_file: Option<PathBuf> = None;
    for ext in possible_extensions {
        let mut test_name = icon_name.clone();
        if !ext.is_empty() && !test_name.ends_with(&format!(".{}", ext)) {
            test_name = format!("{}.{}", test_name, ext);
        }
        
        let test_path = resources_dir.join(&test_name);
        if test_path.exists() {
            icon_file = Some(test_path);
            break;
        }
    }
    
    let icon_file = icon_file.ok_or_else(|| "未找到图标文件".to_string())?;
    
    // 创建安全的文件名
    let safe_name = app_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
        .collect::<String>();
    
    let output_file = output_dir.join(format!("{}.{}", safe_name, format));
    
    // 使用 sips 转换图标
    let status = Command::new("sips")
        .args(&[
            "-s", "format", format,
            "--resampleWidth", &size.to_string(),
        ])
        .arg(&icon_file)
        .args(&["--out"])
        .arg(&output_file)
        .output()
        .map_err(|e| format!("执行 sips 失败: {}", e))?;
    
    if !status.status.success() {
        return Err("图标转换失败".to_string());
    }
    
    Ok(output_file)
}

