//! 跨平台窗口监控模块
//! 
//! 提供获取当前活动窗口标题的功能
//! 支持 Windows 和 macOS 平台

// 忽略 objc 宏的 clippy 警告
#![allow(unexpected_cfgs)]

use std::sync::Mutex;
use std::time::{Duration, Instant};

#[cfg(windows)]
use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, IsWindow},
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
#[cfg(target_os = "macos")]
use std::sync::LazyLock;

/// macOS 窗口查询互斥锁
/// 用于防止快速切换窗口时的并发访问问题
#[cfg(target_os = "macos")]
static WINDOW_QUERY_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

/// macOS 窗口查询超时时间（毫秒）
#[cfg(target_os = "macos")]
const WINDOW_QUERY_TIMEOUT_MS: u64 = 100;

/// 获取当前活动窗口的标题 (Windows版本)
///
/// # 返回值
/// * `Some(String)` - 窗口标题
/// * `None` - 无法获取窗口标题或没有活动窗口
///
/// # 平台支持
/// 仅支持Windows平台
///
/// # 实现说明
/// 使用 Windows API 获取前台窗口标题
/// 添加了窗口句柄验证，确保长时间运行的稳定性
/// 
/// # 长时间运行优化
/// 每次重新获取前台窗口句柄，不缓存任何状态，确保能够检测到所有窗口变化
#[cfg(windows)]
pub fn get_active_window_title() -> Option<String> {
    unsafe {
        // 每次都重新获取前台窗口句柄（不使用缓存）
        // 这确保了即使长时间未切换窗口，后续的切换也能被正确检测到
        let hwnd: HWND = GetForegroundWindow();
        
        // 验证句柄是否有效
        if hwnd.0 == 0 {
            return None;
        }
        
        // 验证窗口是否仍然存在（长时间运行时可能窗口已关闭）
        if !IsWindow(hwnd).as_bool() {
            return None;
        }

        // 使用较大的缓冲区以支持长标题
        // 每次都使用新的缓冲区，避免任何潜在的数据残留
        let mut buffer = [0u16; 512];
        let length = GetWindowTextW(hwnd, &mut buffer);

        // 检查是否成功获取文本
        if length == 0 {
            // 窗口可能没有标题，或者获取失败
            return None;
        }

        // 转换为 Rust 字符串
        let title = String::from_utf16_lossy(&buffer[0..length as usize]);
        
        // 确保标题不为空
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
/// 1. 使用互斥锁防止快速切换窗口时的并发访问问题
/// 2. 使用 autorelease pool 确保内存及时释放，防止长时间运行后内存泄漏
/// 3. 超时机制避免长时间阻塞
#[cfg(target_os = "macos")]
pub fn get_active_window_title() -> Option<String> {
    // 尝试获取锁，使用超时机制避免死锁
    let start = Instant::now();
    let lock_result = loop {
        if let Ok(guard) = WINDOW_QUERY_LOCK.try_lock() {
            break Some(guard);
        }
        
        // 检查是否超时
        if start.elapsed() > Duration::from_millis(WINDOW_QUERY_TIMEOUT_MS) {
            #[cfg(debug_assertions)]
            eprintln!("[警告] 获取窗口查询锁超时，跳过此次查询");
            break None;
        }
        
        // 短暂休眠后重试
        std::thread::sleep(Duration::from_millis(1));
    };
    
    // 如果无法获取锁，返回 None
    let _guard = lock_result?;
    
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
    /// 上次成功查询的时间（用于避免过于频繁的查询）
    last_query_time: Option<Instant>,
    /// 最小查询间隔（毫秒）
    min_query_interval_ms: u64,
}

impl WindowMonitor {
    /// 创建新的窗口监控器实例
    pub fn new() -> Self {
        Self {
            last_window_title: String::new(),
            last_query_time: None,
            min_query_interval_ms: 50, // 默认最小50ms间隔
        }
    }
    
    /// 创建新的窗口监控器实例，并指定最小查询间隔
    ///
    /// # 参数
    /// * `min_query_interval_ms` - 最小查询间隔（毫秒），避免过于频繁查询
    pub fn new_with_interval(min_query_interval_ms: u64) -> Self {
        Self {
            last_window_title: String::new(),
            last_query_time: None,
            min_query_interval_ms,
        }
    }

    /// 检查窗口标题是否发生变化
    ///
    /// # 返回值
    /// * `Some(String)` - 新的窗口标题（如果发生变化）
    /// * `None` - 窗口标题未变化或无法获取
    ///
    /// # 改进说明
    /// 1. 每次都获取当前窗口标题，即使长时间未切换也能正确检测到后续的窗口变化
    /// 2. 添加详细的调试信息，帮助诊断长时间运行后的问题
    /// 3. 添加查询间隔限制，避免过于频繁查询导致性能问题
    pub fn check_for_change(&mut self) -> Option<String> {
        // 检查是否满足最小查询间隔
        if let Some(last_time) = self.last_query_time {
            let elapsed = last_time.elapsed();
            if elapsed < Duration::from_millis(self.min_query_interval_ms) {
                // 还没到查询时间，跳过此次查询
                #[cfg(debug_assertions)]
                {
                    let remaining = self.min_query_interval_ms - elapsed.as_millis() as u64;
                    if remaining > 10 {
                        println!("[调试] 查询间隔限制，剩余 {}ms", remaining);
                    }
                }
                return None;
            }
        }
        
        // 更新查询时间
        self.last_query_time = Some(Instant::now());
        
        // 每次都尝试获取当前活动窗口标题
        let current_title = get_active_window_title();
        
        match current_title {
            Some(window_title) => {
                // 成功获取到窗口标题
                if window_title != self.last_window_title {
                    // 窗口标题发生变化
                    #[cfg(debug_assertions)]
                    println!("[调试] 检测到窗口变化: {} -> {}", self.last_window_title, window_title);
                    
                    self.last_window_title = window_title.clone();
                    return Some(window_title);
                } else {
                    // 窗口标题未变化（这是正常情况，不输出日志避免刷屏）
                }
                None
            }
            None => {
                // 无法获取窗口标题（可能没有活动窗口或获取失败）
                #[cfg(debug_assertions)]
                if !self.last_window_title.is_empty() {
                    println!("[调试] 无法获取窗口标题，之前的窗口: {}", self.last_window_title);
                }
                
                if !self.last_window_title.is_empty() {
                    // 之前有窗口，现在没有了，清空状态
                    self.last_window_title.clear();
                }
                None
            }
        }
    }

    /// 获取最后记录的窗口标题
    pub fn last_title(&self) -> &str {
        &self.last_window_title
    }

    /// 重置监控状态
    pub fn reset(&mut self) {
        self.last_window_title.clear();
        self.last_query_time = None;
    }
    
    /// 设置最小查询间隔（毫秒）
    pub fn set_min_query_interval(&mut self, interval_ms: u64) {
        self.min_query_interval_ms = interval_ms;
    }
    
    /// 获取最小查询间隔（毫秒）
    pub fn min_query_interval(&self) -> u64 {
        self.min_query_interval_ms
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
