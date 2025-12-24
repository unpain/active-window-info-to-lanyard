# macOS 窗口检测权限说明

## ✅ 已解决

macOS 窗口检测功能现已正常工作！使用 Core Graphics API 直接获取窗口列表，无需依赖 NSWorkspace。

## 可能需要的权限

在 macOS 上，应用可能需要"屏幕录制"权限才能获取其他应用的窗口信息。

### 检查步骤

1. 打开"系统设置" (System Settings/System Preferences)
2. 进入"隐私与安全性" (Privacy & Security)
3. 找到以下选项：
   - **屏幕录制** (Screen Recording)

### 添加权限

1. 在"屏幕录制"中
2. 点击 "+" 按钮
3. 添加你的终端应用（Terminal.app 或 iTerm.app）
4. 重启终端后重新运行程序

## 测试

运行主程序：

```bash
cargo run --bin main
```

或运行测试程序验证窗口检测：

```bash
cargo run --example test_window_detection
```

切换窗口，观察是否能检测到变化。

## 技术说明

- 使用 `CGWindowListCopyWindowInfo` API 实时获取窗口列表
- 返回第一个 `layer=0` 的窗口（活动窗口）
- 每次调用都重新获取，确保实时性
- 比 NSWorkspace 更可靠，能正确检测窗口切换
