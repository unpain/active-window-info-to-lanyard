/// Discord Rich Presence管理模块
///
/// 提供与Discord RPC的连接和状态更新功能
use discord_rpc_client::Client as DiscordClient;
use std::time::{ SystemTime, UNIX_EPOCH };

use crate::config::Config;
use crate::crypto::CryptoManager;
use crate::parser::WindowInfo;

/// Discord RPC管理器
pub struct DiscordManager {
    client: DiscordClient,
    start_time: u64,
    current_activity_start: u64,
    crypto: Option<CryptoManager>,
}

impl DiscordManager {
    /// 创建并连接Discord RPC客户端
    ///
    /// # 参数
    /// * `config` - 应用配置
    ///
    /// # 返回值
    /// * `Ok(DiscordManager)` - 成功创建并连接
    /// * `Err(String)` - 连接失败
    pub fn connect(config: &Config) -> Result<Self, String> {
        let mut client = DiscordClient::new(config.discord_app_id);

        client.start();

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("获取系统时间失败: {}", e))?
            .as_secs();

        // 如果配置中有加密密钥，初始化加密管理器
        let crypto = if let Some(ref key) = config.encryption_key {
            Some(
                CryptoManager::from_hex(key)
                    .map_err(|e| format!("初始化加密管理器失败: {}", e))?
            )
        } else {
            None
        };

        Ok(Self {
            client,
            start_time,
            current_activity_start: start_time,
            crypto,
        })
    }

    /// 更新Discord Rich Presence状态
    ///
    /// # 参数
    /// * `window_info` - 窗口信息
    /// * `full_title` - 完整的窗口标题
    ///
    /// # 返回值
    /// * `Ok(())` - 更新成功
    /// * `Err(String)` - 更新失败
    pub fn update_activity(
        &mut self,
        window_info: &WindowInfo,
        full_title: &str
    ) -> Result<(), String> {
        // 更新当前活动的开始时间为当前时间
        self.current_activity_start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("获取系统时间失败: {}", e))?
            .as_secs();
        
        // 如果启用了加密，加密state数据
        let state_data = if let Some(ref crypto) = self.crypto {
            crypto
                .encrypt(full_title)
                .map_err(|e| format!("加密state数据失败: {}", e))?
        } else {
            full_title.to_string()
        };

        self.client
            .set_activity(|act| {
                let mut activity = act
                    .state(&state_data)
                    .details(&window_info.app_name)
                    .timestamps(|t| t.start(self.current_activity_start));

                // 添加Windows图标（需要在Discord Developer Portal上传）
                activity = activity.assets(|a| {
                    a.large_image("windows").large_text("Windows Activity Monitor")
                });

                activity
            })
            .map(|_| ())
            .map_err(|e| format!("更新Discord状态失败: {}", e))
    }

    /// 清除Discord Rich Presence状态
    pub fn clear_activity(&mut self) -> Result<(), String> {
        self.client
            .clear_activity()
            .map(|_| ())
            .map_err(|e| format!("清除Discord状态失败: {}", e))
    }

    /// 获取启动时间戳
    pub fn start_time(&self) -> u64 {
        self.start_time
    }
    
    /// 获取当前活动的开始时间戳
    pub fn current_activity_start(&self) -> u64 {
        self.current_activity_start
    }

    /// 检查是否启用了加密
    pub fn is_encryption_enabled(&self) -> bool {
        self.crypto.is_some()
    }

    /// 解密state数据（用于调试或日志记录）
    ///
    /// # 参数
    /// * `encrypted_data` - 加密的数据
    ///
    /// # 返回值
    /// * `Ok(String)` - 解密后的数据
    /// * `Err(String)` - 解密失败或未启用加密
    pub fn decrypt_state(&self, encrypted_data: &str) -> Result<String, String> {
        if let Some(ref crypto) = self.crypto {
            crypto
                .decrypt(encrypted_data)
                .map_err(|e| format!("解密state数据失败: {}", e))
        } else {
            Err("加密未启用".to_string())
        }
    }
}

/// Discord Rich Presence更新结果
#[derive(Debug)]
pub enum UpdateResult {
    /// 成功更新
    Success,
    /// 跳过更新（窗口未变化）
    Skipped,
    /// 更新失败
    Failed(String),
}

impl UpdateResult {
    /// 检查是否成功
    pub fn is_success(&self) -> bool {
        matches!(self, UpdateResult::Success)
    }

    /// 检查是否失败
    pub fn is_failed(&self) -> bool {
        matches!(self, UpdateResult::Failed(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_result() {
        let success = UpdateResult::Success;
        assert!(success.is_success());
        assert!(!success.is_failed());

        let failed = UpdateResult::Failed("test error".to_string());
        assert!(!failed.is_success());
        assert!(failed.is_failed());
    }
}
