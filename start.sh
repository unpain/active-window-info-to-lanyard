#!/bin/bash
# 快速启动脚本 - macOS

echo "🚀 启动 Discord 活动窗口监控..."
echo ""

# 检查是否有 .env 文件
if [ ! -f ".env" ]; then
    echo "❌ 未找到 .env 文件"
    echo "   请创建 .env 文件并添加以下内容："
    echo ""
    echo "   DISCORD_APP_ID=你的Discord应用ID"
    echo "   ENCRYPTION_KEY=你的加密密钥（可选）"
    echo ""
    exit 1
fi

# 编译并运行（Release 模式性能更好）
cargo run --release --bin main

