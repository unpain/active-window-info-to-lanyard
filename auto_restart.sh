#!/bin/bash

# Discord Activity Monitor 自动重启脚本
# 用于在程序意外退出时自动重启

echo "╔════════════════════════════════════════════════╗"
echo "║  Discord Monitor 自动重启守护脚本            ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

RESTART_COUNT=0
MAX_FAST_RESTARTS=5
FAST_RESTART_WINDOW=60  # 秒

last_start_time=$(date +%s)

while true; do
    echo "🚀 [$(date '+%Y-%m-%d %H:%M:%S')] 启动程序（第 $((RESTART_COUNT + 1)) 次）"
    
    # 运行程序（优先使用增强版）
    if [ -f "target/debug/examples/robust_monitor" ] || [ -f "target/release/examples/robust_monitor" ]; then
        echo "   使用增强版监控器..."
        cargo run --example robust_monitor --release 2>&1
    else
        echo "   使用标准版..."
        cargo run --release 2>&1
    fi
    
    EXIT_CODE=$?
    current_time=$(date +%s)
    runtime=$((current_time - last_start_time))
    
    echo ""
    echo "⏸️  [$(date '+%Y-%m-%d %H:%M:%S')] 程序已退出"
    echo "   退出代码: $EXIT_CODE"
    echo "   运行时长: ${runtime} 秒"
    
    # 检查是否快速重启（可能是配置错误）
    if [ $runtime -lt $FAST_RESTART_WINDOW ]; then
        RESTART_COUNT=$((RESTART_COUNT + 1))
        
        if [ $RESTART_COUNT -ge $MAX_FAST_RESTARTS ]; then
            echo ""
            echo "❌ 错误：程序在 ${FAST_RESTART_WINDOW} 秒内重启了 ${MAX_FAST_RESTARTS} 次"
            echo "   可能存在配置问题，请检查："
            echo "   1. .env 文件是否正确"
            echo "   2. Discord 应用ID是否有效"
            echo "   3. Discord 客户端是否运行"
            echo ""
            echo "   按 Enter 继续重试，或 Ctrl+C 退出..."
            read
            RESTART_COUNT=0
        fi
    else
        # 运行时间足够长，重置计数器
        RESTART_COUNT=0
    fi
    
    last_start_time=$(date +%s)
    
    # 等待一段时间再重启
    WAIT_TIME=5
    echo "   ${WAIT_TIME} 秒后重启..."
    sleep $WAIT_TIME
    echo ""
done

