//! 应用程序配置管理模块

use std::time::Duration;

/// 应用程序配置
#[derive(Debug, Clone)]
pub struct Config {
    /// Discord Application ID
    pub discord_app_id: u64,
    /// 状态更新间隔
    pub update_interval: Duration,
    /// 加密密钥（可选，32字节十六进制字符串）
    pub encryption_key: Option<String>,
}

impl Config {
    /// 创建新的配置实例
    ///
    /// # 参数
    /// * `discord_app_id` - Discord应用ID
    /// * `update_interval_secs` - 更新间隔（秒）
    pub fn new(discord_app_id: u64, update_interval_secs: u64) -> Self {
        Self {
            discord_app_id,
            update_interval: Duration::from_secs(update_interval_secs),
            encryption_key: None,
        }
    }

    /// 创建带加密的配置实例
    ///
    /// # 参数
    /// * `discord_app_id` - Discord应用ID
    /// * `update_interval_secs` - 更新间隔（秒）
    /// * `encryption_key` - 64字符的十六进制加密密钥
    pub fn new_with_encryption(
        discord_app_id: u64,
        update_interval_secs: u64,
        encryption_key: String,
    ) -> Self {
        Self {
            discord_app_id,
            update_interval: Duration::from_secs(update_interval_secs),
            encryption_key: Some(encryption_key),
        }
    }

    /// 从字符串创建配置
    ///
    /// # 参数
    /// * `discord_app_id` - Discord应用ID字符串
    /// * `update_interval_secs` - 更新间隔（秒）
    ///
    /// # 错误
    /// 如果Discord应用ID无法解析为u64，返回错误
    pub fn from_str(discord_app_id: &str, update_interval_secs: u64) -> Result<Self, String> {
        let app_id = discord_app_id
            .parse::<u64>()
            .map_err(|e| format!("无法解析Discord应用ID: {}", e))?;

        Ok(Self::new(app_id, update_interval_secs))
    }

    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        if self.discord_app_id == 0 {
            return Err("Discord应用ID不能为0".to_string());
        }

        if self.update_interval.as_secs() < 1 {
            return Err("更新间隔不能小于1秒".to_string());
        }

        // 验证加密密钥格式（如果提供）
        if let Some(ref key) = self.encryption_key {
            if key.len() != 64 {
                return Err("加密密钥必须是64个十六进制字符（32字节）".to_string());
            }
            if !key.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err("加密密钥必须只包含十六进制字符（0-9, a-f, A-F）".to_string());
            }
        }

        Ok(())
    }

    /// 检查是否启用了加密
    pub fn is_encryption_enabled(&self) -> bool {
        self.encryption_key.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new(123456789, 5);
        assert_eq!(config.discord_app_id, 123456789);
        assert_eq!(config.update_interval, Duration::from_secs(5));
    }

    #[test]
    fn test_config_from_str() {
        let config = Config::from_str("123456789", 10).unwrap();
        assert_eq!(config.discord_app_id, 123456789);
    }

    #[test]
    fn test_config_validation() {
        let valid_config = Config::new(123456789, 5);
        assert!(valid_config.validate().is_ok());

        let invalid_config = Config::new(0, 5);
        assert!(invalid_config.validate().is_err());
    }
}
