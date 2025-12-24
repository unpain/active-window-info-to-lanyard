//! 窗口标题解析模块
//! 
//! 提供从窗口标题中提取应用名称和详细信息的功能

/// 窗口信息结构体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowInfo {
    /// 应用名称
    pub app_name: String,
    /// 详细信息（如文档名、标签页标题等）
    pub details: String,
}

impl WindowInfo {
    /// 创建新的窗口信息实例
    pub fn new(app_name: String, details: String) -> Self {
        Self { app_name, details }
    }

    /// 从窗口标题解析窗口信息
    ///
    /// # 参数
    /// * `window_title` - 完整的窗口标题
    ///
    /// # 返回值
    /// 包含应用名称和详细信息的WindowInfo实例
    ///
    /// # 示例
    /// ```
    /// use active_window_info_to_lanyard_lib::parser::WindowInfo;
    ///
    /// let info = WindowInfo::parse("document.txt - Notepad");
    /// assert_eq!(info.app_name, "Notepad");
    /// assert_eq!(info.details, "document.txt");
    /// ```
    pub fn parse(window_title: &str) -> Self {
        extract_app_name(window_title)
    }
}

/// 从窗口标题中提取应用名称和详细信息
///
/// 许多窗口标题格式类似："文档名 - 应用名" 或 "标签页 - 应用名"
///
/// # 参数
/// * `window_title` - 完整的窗口标题
///
/// # 返回值
/// 包含应用名称和详细信息的WindowInfo实例
///
/// # 示例
/// ```
/// use active_window_info_to_lanyard_lib::parser::extract_app_name;
///
/// let info = extract_app_name("README.md - Visual Studio Code");
/// assert_eq!(info.app_name, "Visual Studio Code");
/// assert_eq!(info.details, "README.md");
/// ```
pub fn extract_app_name(window_title: &str) -> WindowInfo {
    // 尝试从右侧查找最后一个 " - " 分隔符
    if let Some(pos) = window_title.rfind(" - ") {
        let (detail, app) = window_title.split_at(pos);
        let app_name = app.trim_start_matches(" - ").to_string();
        WindowInfo::new(app_name, detail.to_string())
    } else {
        // 如果没有找到分隔符，将整个标题作为应用名称
        WindowInfo::new(window_title.to_string(), String::new())
    }
}

/// 清理窗口标题，移除特殊字符和多余空格
pub fn sanitize_title(title: &str) -> String {
    title
        .trim()
        .chars()
        .filter(|c| !c.is_control())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_app_name_with_separator() {
        let info = extract_app_name("document.txt - Notepad");
        assert_eq!(info.app_name, "Notepad");
        assert_eq!(info.details, "document.txt");
    }

    #[test]
    fn test_extract_app_name_without_separator() {
        let info = extract_app_name("Calculator");
        assert_eq!(info.app_name, "Calculator");
        assert_eq!(info.details, "");
    }

    #[test]
    fn test_extract_app_name_multiple_separators() {
        let info = extract_app_name("file.txt - folder - VSCode");
        assert_eq!(info.app_name, "VSCode");
        assert_eq!(info.details, "file.txt - folder");
    }

    #[test]
    fn test_sanitize_title() {
        let title = "  Test   Title  ";
        assert_eq!(sanitize_title(title), "Test Title");
    }

    #[test]
    fn test_window_info_parse() {
        let info = WindowInfo::parse("Chrome - Google");
        assert_eq!(info.app_name, "Google");
        assert_eq!(info.details, "Chrome");
    }
}
