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
    discord_app_id: u64,
    last_successful_update: u64,
    consecutive_failures: u32,
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
            discord_app_id: config.discord_app_id,
            last_successful_update: start_time,
            consecutive_failures: 0,
        })
    }

    /// 尝试重新连接到Discord RPC
    ///
    /// # 返回值
    /// * `Ok(())` - 重连成功
    /// * `Err(String)` - 重连失败
    fn reconnect(&mut self) -> Result<(), String> {
        println!("🔄 尝试重新连接Discord RPC...");
        
        // 创建新的客户端实例
        let mut new_client = DiscordClient::new(self.discord_app_id);
        new_client.start();
        
        // 替换旧的客户端
        self.client = new_client;
        
        // 重置失败计数
        self.consecutive_failures = 0;
        
        // 更新最后成功时间
        self.last_successful_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("获取系统时间失败: {}", e))?
            .as_secs();
        
        println!("✅ Discord RPC重新连接成功");
        Ok(())
    }

    /// 检查连接健康状态并在需要时重连
    ///
    /// # 返回值
    /// * `Ok(())` - 连接健康或重连成功
    /// * `Err(String)` - 重连失败
    fn check_connection_health(&mut self) -> Result<(), String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("获取系统时间失败: {}", e))?
            .as_secs();
        
        // 如果连续失败次数超过3次，或者距离上次成功更新超过5分钟，尝试重连
        let time_since_last_success = now - self.last_successful_update;
        if self.consecutive_failures >= 3 || time_since_last_success > 300 {
            println!("⚠️  检测到连接异常 (连续失败: {}, 距上次成功: {}秒)", 
                     self.consecutive_failures, time_since_last_success);
            self.reconnect()?;
        }
        
        Ok(())
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
        // 检查连接健康状态
        if let Err(e) = self.check_connection_health() {
            eprintln!("⚠️  连接健康检查失败: {}", e);
            // 即使健康检查失败，也尝试继续更新
        }
        
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

        let result = self.client
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
            .map_err(|e| format!("更新Discord状态失败: {}", e));
        
        // 根据结果更新状态跟踪
        match result {
            Ok(_) => {
                self.consecutive_failures = 0;
                self.last_successful_update = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                Ok(())
            }
            Err(e) => {
                self.consecutive_failures += 1;
                println!("⚠️  更新失败 (连续失败次数: {})", self.consecutive_failures);
                Err(e)
            }
        }
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
