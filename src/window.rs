//! Windows窗口监控模块
//! 
//! 提供获取当前活动窗口标题的功能

#[cfg(windows)]
use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
};

/// 获取当前活动窗口的标题
///
/// # 返回值
/// * `Some(String)` - 窗口标题
/// * `None` - 无法获取窗口标题或没有活动窗口
///
/// # 平台支持
/// 仅支持Windows平台
#[cfg(windows)]
pub fn get_active_window_title() -> Option<String> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0 == 0 {
            return None;
        }

        let mut buffer = [0u16; 512];
        let length = GetWindowTextW(hwnd, &mut buffer);

        if length == 0 {
            return None;
        }

        let title = String::from_utf16_lossy(&buffer[0..length as usize]);
        if title.is_empty() {
            None
        } else {
            Some(title)
        }
    }
}

/// 非Windows平台的占位实现
#[cfg(not(windows))]
pub fn get_active_window_title() -> Option<String> {
    eprintln!("⚠️  窗口监控仅在Windows平台上受支持");
    None
}

/// 窗口监控器
///
/// 封装窗口监控逻辑，跟踪窗口标题变化
pub struct WindowMonitor {
    last_window_title: String,
}

impl WindowMonitor {
    /// 创建新的窗口监控器实例
    pub fn new() -> Self {
        Self {
            last_window_title: String::new(),
        }
    }

    /// 检查窗口标题是否发生变化
    ///
    /// # 返回值
    /// * `Some(String)` - 新的窗口标题（如果发生变化）
    /// * `None` - 窗口标题未变化或无法获取
    pub fn check_for_change(&mut self) -> Option<String> {
        if let Some(window_title) = get_active_window_title() {
            if window_title != self.last_window_title {
                self.last_window_title = window_title.clone();
                return Some(window_title);
            }
        } else if !self.last_window_title.is_empty() {
            // 没有活动窗口，但之前有窗口
            self.last_window_title.clear();
        }
        None
    }

    /// 获取最后记录的窗口标题
    pub fn last_title(&self) -> &str {
        &self.last_window_title
    }

    /// 重置监控状态
    pub fn reset(&mut self) {
        self.last_window_title.clear();
    }
}

impl Default for WindowMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_monitor_creation() {
        let monitor = WindowMonitor::new();
        assert_eq!(monitor.last_title(), "");
    }

    #[test]
    fn test_window_monitor_reset() {
        let mut monitor = WindowMonitor::new();
        monitor.last_window_title = "test".to_string();
        monitor.reset();
        assert_eq!(monitor.last_title(), "");
    }
}
