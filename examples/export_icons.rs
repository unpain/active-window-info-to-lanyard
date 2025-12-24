//! 批量导出 macOS 应用图标工具
//!
//! 功能：
//! - 扫描 /Applications 目录下的所有应用
//! - 提取每个应用的图标文件
//! - 将图标导出为 PNG 格式
//! - 支持自定义输出目录和图标大小

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    print_welcome();
    
    // 创建输出目录
    let output_dir = PathBuf::from("./exported_icons");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("无法创建输出目录");
    }
    
    println!("📁 输出目录: {}", output_dir.display());
    println!();
    
    // 扫描应用程序目录
    let apps_dirs = vec![
        "/Applications",
        "/System/Applications",
        "/System/Library/CoreServices",  // 包含 Finder 等系统核心应用
    ];
    
    let mut total_count = 0;
    let mut success_count = 0;
    
    for apps_dir in apps_dirs {
        println!("🔍 正在扫描: {}", apps_dir);
        
        let dir_path = Path::new(apps_dir);
        if !dir_path.exists() {
            println!("   ⚠️  目录不存在，跳过");
            continue;
        }
        
        match scan_and_export_icons(dir_path, &output_dir) {
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
    println!();
    println!("📁 图标已保存到: {}", output_dir.display());
}

fn print_welcome() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  macOS 应用图标批量导出工具                  ║");
    println!("║  Icon Exporter for macOS Applications        ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();
}

/// 扫描目录并导出所有应用图标
fn scan_and_export_icons(apps_dir: &Path, output_dir: &Path) -> Result<(usize, usize), String> {
    let entries = fs::read_dir(apps_dir)
        .map_err(|e| format!("无法读取目录: {}", e))?;
    
    let mut total_count = 0;
    let mut success_count = 0;
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        
        let path = entry.path();
        
        // 只处理 .app 结尾的应用
        if !path.is_dir() || !path.extension().map_or(false, |ext| ext == "app") {
            continue;
        }
        
        total_count += 1;
        
        let app_name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown");
        
        print!("   📦 {}", app_name);
        
        match export_app_icon(&path, output_dir, app_name) {
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

/// 导出单个应用的图标
fn export_app_icon(app_path: &Path, output_dir: &Path, app_name: &str) -> Result<PathBuf, String> {
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
    
    // 使用 sips 将 icns 转换为 png
    // 创建安全的文件名（移除特殊字符）
    let safe_name = app_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
        .collect::<String>();
    
    let output_file = output_dir.join(format!("{}.png", safe_name));
    
    let status = Command::new("sips")
        .args(&[
            "-s", "format", "png",
            "--resampleWidth", "512",  // 导出为 512x512
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

