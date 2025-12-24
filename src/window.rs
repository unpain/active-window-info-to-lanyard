//! 跨平台窗口监控模块
//! 
//! 提供获取当前活动窗口标题的功能
//! 支持 Windows 和 macOS 平台

// 忽略 objc 宏的 clippy 警告
#![allow(unexpected_cfgs)]

#[cfg(windows)]
use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
};

#[cfg(target_os = "macos")]
use cocoa::{
    base::nil,
    foundation::NSAutoreleasePool,
};
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl};
#[cfg(target_os = "macos")]
use core_foundation::{
    base::TCFType,
    string::CFStringRef,
};
#[cfg(target_os = "macos")]
use core_graphics::window::{kCGWindowListOptionOnScreenOnly, kCGNullWindowID};

/// 获取当前活动窗口的标题 (Windows版本)
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

/// 获取当前活动窗口的标题 (macOS版本)
///
/// # 返回值
/// * `Some(String)` - 窗口标题
/// * `None` - 无法获取窗口标题或没有活动窗口
///
/// # 平台支持
/// 仅支持macOS平台
///
/// # 实现说明
/// 使用 Core Graphics API 直接获取窗口列表，返回第一个 layer=0 的窗口（活动窗口）
/// 这种方法比使用 NSWorkspace 更可靠，能够实时检测窗口变化
/// 
/// # 性能优化
/// 使用 autorelease pool 确保内存及时释放，防止长时间运行后内存泄漏
#[cfg(target_os = "macos")]
pub fn get_active_window_title() -> Option<String> {
    unsafe {
        // 创建 autorelease pool，确保在函数结束时释放所有自动释放的对象
        let pool = NSAutoreleasePool::new(nil);
        
        // 获取结果后，确保在 return 前 drain pool
        let result = get_window_title_internal();
        
        // 显式释放 autorelease pool
        let _: () = msg_send![pool, drain];
        
        result
    }
}

/// 内部函数：实际获取窗口标题的逻辑
/// 这样设计可以确保 autorelease pool 在外层函数统一管理
#[cfg(target_os = "macos")]
unsafe fn get_window_title_internal() -> Option<String> {
    use core_graphics::window::CGWindowListCopyWindowInfo;
    use core_foundation::{
        array::CFArray,
        dictionary::CFDictionary,
        number::CFNumber,
        string::CFString,
    };
    
    // 使用 Core Graphics API 直接获取窗口列表
    // SAFETY: CGWindowListCopyWindowInfo 是 macOS 系统 API，按照文档使用是安全的
    let window_list = unsafe {
        CGWindowListCopyWindowInfo(
            kCGWindowListOptionOnScreenOnly,
            kCGNullWindowID
        )
    };
    
    if window_list.is_null() {
        return None;
    }
    
    let window_list_ref = window_list as *const _ as *const std::ffi::c_void;
    
    // SAFETY: wrap_under_create_rule 用于从 C API 获取的对象，我们拥有所有权
    let cf_array = unsafe {
        CFArray::<CFDictionary<CFString, *const std::ffi::c_void>>::wrap_under_create_rule(
            window_list_ref as core_foundation::array::CFArrayRef
        )
    };
    
    // 遍历所有窗口，找到第一个 layer=0 的窗口（这通常是活动窗口）
    for i in 0..cf_array.len() {
        let window_info = cf_array.get(i).unwrap();
        
        // 检查窗口层级
        let layer = if let Some(layer_ptr) = window_info.find(CFString::new("kCGWindowLayer").as_concrete_TypeRef()) {
            // SAFETY: wrap_under_get_rule 用于从字典获取的值，不获取所有权
            let layer_num = unsafe {
                CFNumber::wrap_under_get_rule(*layer_ptr as core_foundation::number::CFNumberRef)
            };
            layer_num.to_i32().unwrap_or(-1)
        } else {
            -1
        };
        
        // 只处理 layer=0 的窗口（普通窗口）
        if layer != 0 {
            continue;
        }
        
        // 获取窗口所属的应用名称
        let app_name = if let Some(owner_ptr) = window_info.find(CFString::new("kCGWindowOwnerName").as_concrete_TypeRef()) {
            // SAFETY: wrap_under_get_rule 用于从字典获取的值，不获取所有权
            let owner_cf = unsafe {
                CFString::wrap_under_get_rule(*owner_ptr as CFStringRef)
            };
            owner_cf.to_string()
        } else {
            String::new()
        };
        
        if app_name.is_empty() {
            continue;
        }
        
        // 获取窗口标题
        let window_title = if let Some(title_ptr) = window_info.find(CFString::new("kCGWindowName").as_concrete_TypeRef()) {
            // SAFETY: wrap_under_get_rule 用于从字典获取的值，不获取所有权
            let title_cf = unsafe {
                CFString::wrap_under_get_rule(*title_ptr as CFStringRef)
            };
            title_cf.to_string()
        } else {
            String::new()
        };
        
        // 返回第一个找到的 layer=0 窗口
        if !window_title.is_empty() {
            return Some(format!("{} - {}", window_title, app_name));
        } else {
            return Some(app_name);
        }
    }
    
    None
}

/// 非支持平台的占位实现
#[cfg(not(any(windows, target_os = "macos")))]
pub fn get_active_window_title() -> Option<String> {
    eprintln!("⚠️  窗口监控仅在 Windows 和 macOS 平台上受支持");
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
