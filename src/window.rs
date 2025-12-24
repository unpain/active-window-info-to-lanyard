//! 跨平台窗口监控模块
//! 
//! 提供获取当前活动窗口标题的功能
//! 支持 Windows 和 macOS 平台

#[cfg(windows)]
use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW},
};

#[cfg(target_os = "macos")]
use cocoa::{
    appkit::NSWorkspace,
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSString},
};
#[cfg(target_os = "macos")]
use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};
#[cfg(target_os = "macos")]
use core_graphics::window::{kCGWindowListOptionOnScreenOnly, kCGWindowLayer, kCGNullWindowID};
#[cfg(target_os = "macos")]
use std::ptr;

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
#[cfg(target_os = "macos")]
pub fn get_active_window_title() -> Option<String> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        
        // 获取当前活动的应用
        let workspace: id = NSWorkspace::sharedWorkspace(nil);
        let active_app: id = msg_send![workspace, frontmostApplication];
        
        if active_app == nil {
            return None;
        }
        
        // 获取应用名称
        let app_name: id = msg_send![active_app, localizedName];
        if app_name == nil {
            return None;
        }
        
        let app_name_str = nsstring_to_string(app_name);
        
        // 尝试获取窗口标题
        // 使用 Core Graphics 获取活动窗口信息
        use core_graphics::window::CGWindowListCopyWindowInfo;
        
        let window_list = CGWindowListCopyWindowInfo(
            kCGWindowListOptionOnScreenOnly,
            kCGNullWindowID
        );
        
        if window_list.is_null() {
            return Some(app_name_str);
        }
        
        let window_list_ref = window_list as *const _ as *const std::ffi::c_void;
        let cf_array = core_foundation::array::CFArray::<core_foundation::dictionary::CFDictionary<CFString, *const std::ffi::c_void>>::wrap_under_create_rule(
            window_list_ref as core_foundation::array::CFArrayRef
        );
        
        // 查找层级为0的窗口（活动窗口）
        for i in 0..cf_array.len() {
            let window_info = cf_array.get(i).unwrap();
            
            // 获取窗口层级
            if let Some(layer) = window_info.find(CFString::new("kCGWindowLayer").as_concrete_TypeRef()) {
                use core_foundation::number::CFNumber;
                let layer_num = CFNumber::wrap_under_get_rule(*layer as core_foundation::number::CFNumberRef);
                if let Some(layer_val) = layer_num.to_i32() {
                    if layer_val == 0 {
                        // 获取窗口标题
                        if let Some(title_ptr) = window_info.find(CFString::new("kCGWindowName").as_concrete_TypeRef()) {
                            let title_cf = CFString::wrap_under_get_rule(*title_ptr as CFStringRef);
                            let title = title_cf.to_string();
                            if !title.is_empty() {
                                return Some(format!("{} - {}", title, app_name_str));
                            }
                        }
                        
                        // 如果没有窗口标题，只返回应用名
                        return Some(app_name_str);
                    }
                }
            }
        }
        
        // 如果没找到合适的窗口，返回应用名
        Some(app_name_str)
    }
}

#[cfg(target_os = "macos")]
unsafe fn nsstring_to_string(nsstring: id) -> String {
    use cocoa::foundation::NSString as NSStringTrait;
    let cstr = NSStringTrait::UTF8String(nsstring);
    if cstr.is_null() {
        String::new()
    } else {
        std::ffi::CStr::from_ptr(cstr)
            .to_string_lossy()
            .into_owned()
    }
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
