/// 加密/解密模块
///
/// 提供AES-256-GCM加密和解密功能
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;

/// 加密错误类型
#[derive(Debug)]
pub enum CryptoError {
    /// 加密失败
    EncryptionFailed(String),
    /// 解密失败
    DecryptionFailed(String),
    /// 密钥无效
    InvalidKey(String),
    /// Base64编码/解码失败
    Base64Error(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::EncryptionFailed(msg) => write!(f, "加密失败: {}", msg),
            CryptoError::DecryptionFailed(msg) => write!(f, "解密失败: {}", msg),
            CryptoError::InvalidKey(msg) => write!(f, "密钥无效: {}", msg),
            CryptoError::Base64Error(msg) => write!(f, "Base64错误: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

/// 加密管理器
pub struct CryptoManager {
    cipher: Aes256Gcm,
}

impl CryptoManager {
    /// 从32字节密钥创建加密管理器
    ///
    /// # 参数
    /// * `key` - 32字节的加密密钥
    ///
    /// # 返回值
    /// * `Ok(CryptoManager)` - 成功创建
    /// * `Err(CryptoError)` - 密钥无效
    pub fn new(key: &[u8; 32]) -> Result<Self, CryptoError> {
        let cipher = Aes256Gcm::new(key.into());
        Ok(Self { cipher })
    }

    /// 从十六进制字符串创建加密管理器
    ///
    /// # 参数
    /// * `hex_key` - 64个字符的十六进制密钥字符串
    ///
    /// # 返回值
    /// * `Ok(CryptoManager)` - 成功创建
    /// * `Err(CryptoError)` - 密钥格式无效
    pub fn from_hex(hex_key: &str) -> Result<Self, CryptoError> {
        if hex_key.len() != 64 {
            return Err(CryptoError::InvalidKey(
                "密钥必须是64个十六进制字符（32字节）".to_string(),
            ));
        }

        let mut key = [0u8; 32];
        for i in 0..32 {
            key[i] = u8::from_str_radix(&hex_key[i * 2..i * 2 + 2], 16)
                .map_err(|e| CryptoError::InvalidKey(format!("无效的十六进制字符: {}", e)))?;
        }

        Self::new(&key)
    }

    /// 生成随机的32字节密钥
    ///
    /// # 返回值
    /// 32字节的随机密钥
    pub fn generate_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        key
    }

    /// 将密钥转换为十六进制字符串
    ///
    /// # 参数
    /// * `key` - 32字节的密钥
    ///
    /// # 返回值
    /// 64个字符的十六进制字符串
    pub fn key_to_hex(key: &[u8; 32]) -> String {
        key.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    /// 加密数据
    ///
    /// # 参数
    /// * `plaintext` - 要加密的明文字符串
    ///
    /// # 返回值
    /// * `Ok(String)` - Base64编码的加密数据（包含nonce）
    /// * `Err(CryptoError)` - 加密失败
    pub fn encrypt(&self, plaintext: &str) -> Result<String, CryptoError> {
        // 生成随机nonce（12字节）
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // 加密数据
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

        // 组合nonce和密文: [nonce(12 bytes)][ciphertext]
        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        // Base64编码
        Ok(general_purpose::STANDARD.encode(&result))
    }

    /// 解密数据
    ///
    /// # 参数
    /// * `encrypted` - Base64编码的加密数据
    ///
    /// # 返回值
    /// * `Ok(String)` - 解密后的明文字符串
    /// * `Err(CryptoError)` - 解密失败
    pub fn decrypt(&self, encrypted: &str) -> Result<String, CryptoError> {
        // Base64解码
        let data = general_purpose::STANDARD
            .decode(encrypted)
            .map_err(|e| CryptoError::Base64Error(e.to_string()))?;

        if data.len() < 12 {
            return Err(CryptoError::DecryptionFailed(
                "加密数据太短".to_string(),
            ));
        }

        // 分离nonce和密文
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // 解密
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

        String::from_utf8(plaintext)
            .map_err(|e| CryptoError::DecryptionFailed(format!("无效的UTF-8数据: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = CryptoManager::generate_key();
        let crypto = CryptoManager::new(&key).unwrap();

        let plaintext = "Hello, World! 你好世界！";
        let encrypted = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_from_hex() {
        let key = CryptoManager::generate_key();
        let hex_key = CryptoManager::key_to_hex(&key);

        let crypto1 = CryptoManager::new(&key).unwrap();
        let crypto2 = CryptoManager::from_hex(&hex_key).unwrap();

        let plaintext = "测试数据";
        let encrypted = crypto1.encrypt(plaintext).unwrap();
        let decrypted = crypto2.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_invalid_key() {
        let result = CryptoManager::from_hex("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_invalid_data() {
        let key = CryptoManager::generate_key();
        let crypto = CryptoManager::new(&key).unwrap();

        let result = crypto.decrypt("invalid_base64!");
        assert!(result.is_err());
    }
}

