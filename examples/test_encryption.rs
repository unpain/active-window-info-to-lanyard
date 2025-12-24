/// 加密功能测试工具
///
/// 测试加密和解密功能，支持自定义密钥和消息
use active_window_info_to_lanyard_lib::CryptoManager;
use std::io::{self, Write};

fn main() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  Discord Activity Monitor - 加密测试工具      ║");
    println!("╚════════════════════════════════════════════════╝");
    println!();

    loop {
        println!("\n请选择操作:");
        println!("1. 使用随机密钥测试");
        println!("2. 使用自定义密钥测试");
        println!("3. 解密已加密的数据");
        println!("4. 退出");
        print!("\n请输入选项 (1-4): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => test_with_random_key(),
            "2" => test_with_custom_key(),
            "3" => decrypt_data(),
            "4" => {
                println!("\n👋 再见！");
                break;
            }
            _ => println!("❌ 无效的选项，请重新选择"),
        }
    }
}

fn test_with_random_key() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔑 生成随机密钥测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let key = CryptoManager::generate_key();
    let hex_key = CryptoManager::key_to_hex(&key);
    println!("密钥: {}", hex_key);

    let crypto = CryptoManager::new(&key).expect("创建加密管理器失败");

    print!("\n请输入要加密的消息: ");
    io::stdout().flush().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();

    encrypt_and_decrypt(&crypto, message);
}

fn test_with_custom_key() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔐 使用自定义密钥测试");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    print!("\n请输入64字符的十六进制密钥: ");
    io::stdout().flush().unwrap();
    let mut hex_key = String::new();
    io::stdin().read_line(&mut hex_key).unwrap();
    let hex_key = hex_key.trim();

    let crypto = match CryptoManager::from_hex(hex_key) {
        Ok(c) => {
            println!("✅ 密钥验证成功");
            c
        }
        Err(e) => {
            println!("❌ 密钥无效: {}", e);
            return;
        }
    };

    print!("\n请输入要加密的消息: ");
    io::stdout().flush().unwrap();
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();

    encrypt_and_decrypt(&crypto, message);
}

fn decrypt_data() {
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔓 解密已加密的数据");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    print!("\n请输入64字符的十六进制密钥: ");
    io::stdout().flush().unwrap();
    let mut hex_key = String::new();
    io::stdin().read_line(&mut hex_key).unwrap();
    let hex_key = hex_key.trim();

    let crypto = match CryptoManager::from_hex(hex_key) {
        Ok(c) => {
            println!("✅ 密钥验证成功");
            c
        }
        Err(e) => {
            println!("❌ 密钥无效: {}", e);
            return;
        }
    };

    print!("\n请输入要解密的Base64数据: ");
    io::stdout().flush().unwrap();
    let mut encrypted = String::new();
    io::stdin().read_line(&mut encrypted).unwrap();
    let encrypted = encrypted.trim();

    match crypto.decrypt(encrypted) {
        Ok(plaintext) => {
            println!("\n✅ 解密成功！");
            println!("原文: {}", plaintext);
        }
        Err(e) => {
            println!("\n❌ 解密失败: {}", e);
        }
    }
}

fn encrypt_and_decrypt(crypto: &CryptoManager, message: &str) {
    if message.is_empty() {
        println!("⚠️  消息不能为空");
        return;
    }

    println!("\n📤 加密中...");
    match crypto.encrypt(message) {
        Ok(encrypted) => {
            println!("✅ 加密成功！");
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("原文: {}", message);
            println!("原文长度: {} 字节", message.len());
            println!("\n密文 (Base64):");
            println!("{}", encrypted);
            println!("密文长度: {} 字节", encrypted.len());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            println!("\n📥 解密中...");
            match crypto.decrypt(&encrypted) {
                Ok(decrypted) => {
                    if decrypted == message {
                        println!("✅ 解密成功！原文匹配！");
                        println!("解密后: {}", decrypted);
                    } else {
                        println!("❌ 解密后内容不匹配！");
                        println!("期望: {}", message);
                        println!("实际: {}", decrypted);
                    }
                }
                Err(e) => {
                    println!("❌ 解密失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 加密失败: {}", e);
        }
    }
}

