/// Windows Discord Activity Monitor
///
/// 这个库提供了在Windows上监控活动窗口并同步到Discord Rich Presence的功能
///
/// # 模块
/// * `config` - 应用配置管理
/// * `window` - Windows窗口监控
/// * `parser` - 窗口标题解析
/// * `discord` - Discord RPC集成
/// * `crypto` - 加密/解密功能
pub mod config;
pub mod crypto;
pub mod discord;
pub mod parser;
pub mod window;

// 重新导出常用类型，方便使用
pub use config::Config;
pub use crypto::{CryptoError, CryptoManager};
pub use discord::{DiscordManager, UpdateResult};
pub use parser::{extract_app_name, sanitize_title, WindowInfo};
pub use window::{get_active_window_title, WindowMonitor};

/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_exists() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_name_exists() {
        assert!(!NAME.is_empty());
    }
}
