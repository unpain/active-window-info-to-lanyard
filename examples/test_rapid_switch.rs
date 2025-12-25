/// 测试快速窗口切换场景
/// 
/// 这个示例模拟了快速切换窗口的场景，验证新的锁机制和查询间隔限制
use active_window_info_to_lanyard_lib::WindowMonitor;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 快速窗口切换测试 ===\n");
    println!("本测试会在短时间内快速查询窗口状态");
    println!("验证互斥锁和查询间隔限制是否正常工作");
    println!("提示：你可以在运行过程中快速切换窗口\n");
    
    // 创建窗口监控器（使用默认50ms最小查询间隔）
    let mut monitor = WindowMonitor::new();
    
    println!("测试 1: 极快速查询（10ms间隔，共100次）");
    println!("预期：由于最小查询间隔限制，大部分查询会被跳过\n");
    
    let start = std::time::Instant::now();
    let mut query_count = 0;
    let mut change_count = 0;
    
    for i in 0..100 {
        if let Some(title) = monitor.check_for_change() {
            change_count += 1;
            println!("  [{}] 窗口变化: {}", i, title);
        }
        query_count += 1;
        thread::sleep(Duration::from_millis(10));
    }
    
    let elapsed = start.elapsed();
    println!("\n测试结果:");
    println!("  总尝试次数: {}", query_count);
    println!("  检测到变化: {}", change_count);
    println!("  耗时: {:?}", elapsed);
    println!("  平均每次: {:?}\n", elapsed / query_count);
    
    // 重置监控器
    monitor.reset();
    
    println!("\n测试 2: 正常速度查询（100ms间隔，共30次）");
    println!("预期：大部分查询会被执行\n");
    
    let start = std::time::Instant::now();
    query_count = 0;
    change_count = 0;
    
    for i in 0..30 {
        if let Some(title) = monitor.check_for_change() {
            change_count += 1;
            println!("  [{}] 窗口变化: {}", i, title);
        }
        query_count += 1;
        thread::sleep(Duration::from_millis(100));
    }
    
    let elapsed = start.elapsed();
    println!("\n测试结果:");
    println!("  总尝试次数: {}", query_count);
    println!("  检测到变化: {}", change_count);
    println!("  耗时: {:?}", elapsed);
    println!("  平均每次: {:?}\n", elapsed / query_count);
    
    println!("\n测试 3: 自定义查询间隔（200ms最小间隔，500ms查询周期）");
    monitor.reset();
    monitor.set_min_query_interval(200);
    println!("设置最小查询间隔: {}ms\n", monitor.min_query_interval());
    
    let start = std::time::Instant::now();
    query_count = 0;
    change_count = 0;
    
    for i in 0..20 {
        if let Some(title) = monitor.check_for_change() {
            change_count += 1;
            println!("  [{}] 窗口变化: {}", i, title);
        }
        query_count += 1;
        thread::sleep(Duration::from_millis(500));
    }
    
    let elapsed = start.elapsed();
    println!("\n测试结果:");
    println!("  总尝试次数: {}", query_count);
    println!("  检测到变化: {}", change_count);
    println!("  耗时: {:?}", elapsed);
    println!("  平均每次: {:?}\n", elapsed / query_count);
    
    println!("\n=== 测试完成 ===");
    println!("\n优化说明:");
    println!("1. 互斥锁防止并发访问 Core Graphics API");
    println!("2. 查询间隔限制避免过于频繁的系统调用");
    println!("3. 超时机制防止死锁");
    println!("4. 这些优化确保即使在快速切换窗口时也能稳定运行");
}

