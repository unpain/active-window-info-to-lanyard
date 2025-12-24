/// 加密密钥生成工具
///
/// 生成一个用于加密Discord状态数据的随机密钥
use active_window_info_to_lanyard_lib::CryptoManager;

fn main() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  Discord Activity Monitor - 密钥生成工具      ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();

    // 生成随机密钥
    println!("🔑 正在生成加密密钥...");
    let key = CryptoManager::generate_key();
    let hex_key = CryptoManager::key_to_hex(&key);

    println!("✅ 密钥生成成功！");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("您的加密密钥:");
    println!("{}", hex_key);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("📝 使用说明:");
    println!("1. 将上面的密钥复制到 .env 文件中");
    println!("2. 在 .env 中添加一行: ENCRYPTION_KEY=<密钥>");
    println!("3. 重启程序即可启用加密功能");
    println!();
    println!("⚠️  安全提示:");
    println!("• 请妥善保管此密钥，不要分享给他人");
    println!("• 不要将密钥提交到版本控制系统（Git）");
    println!("• 确保 .env 文件已添加到 .gitignore");
    println!("• 如果怀疑密钥泄露，请立即生成新密钥");
    println!();

    // 测试加密和解密
    println!("🧪 测试加密功能...");
    let crypto = CryptoManager::new(&key).expect("创建加密管理器失败");
    let test_message = "Hello, Discord! 你好，Discord！";
    
    match crypto.encrypt(test_message) {
        Ok(encrypted) => {
            println!("✅ 加密测试成功");
            println!("   原文: {}", test_message);
            println!("   密文: {}...", &encrypted[..50.min(encrypted.len())]);
            
            match crypto.decrypt(&encrypted) {
                Ok(decrypted) => {
                    if decrypted == test_message {
                        println!("✅ 解密测试成功");
                    } else {
                        println!("❌ 解密测试失败: 解密后内容不匹配");
                    }
                }
                Err(e) => println!("❌ 解密测试失败: {}", e),
            }
        }
        Err(e) => println!("❌ 加密测试失败: {}", e),
    }
    
    println!();
    println!("✨ 完成！祝您使用愉快！");
}

