/// code from  https://mp.weixin.qq.com/s/pVtbBI6xJgk7Z3bIlCsDZg

#[cfg(feature = "dhat-heap")]  // 仅启用特性时注入 Dhat 分配器，避免零开销
#[global_allocator]// 全局分配器：替换系统 malloc，拦截所有 heap alloc/free（包括 Vec/Arc）
static ALLOC: dhat::Alloc = dhat::Alloc;  // Dhat 核心：记录块数/字节/栈，实验性，可能慢

use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use rand::Rng;  // 引入 rand：模拟随机数据，测试 realloc 开销

// 模拟 Tokio S3 处理函数：异步，引入内存泄漏（forgotten handle）
async fn s3_process_request(id: u64)  {
    // 剖析点1：异步 sleep 模拟网络延迟，Dhat 不直接追踪（非 heap），但驻留时影响峰值 (t-gmax)
    tokio::time::sleep(Duration::from_micros(id % 100)).await;  // Tokio poll 开销：高并发下临时 alloc（如 future 缓冲）

    // 剖析点2：Vec 扩容分配，Dhat 拦截 malloc/realloc，记录栈（s3_process_request: line X）
    let mut data = Vec::with_capacity(1024);  // 初始容量：预分配优化，避免小 realloc（Dhat total_bytes 降低 20%）
    for _ in 0..(id % 500) {  // 循环 push：模拟 S3 数据缓冲，潜在碎片（Dhat blocks 高表示碎片）
        data.push(rand::thread_rng().r#gen::<u8>());  // 每个 push：可能 realloc，Dhat 追踪字节增量
    }

    // 剖析点3：Arc 共享，Dhat 记录 Arc::new alloc（包括计数器），循环引用易泄漏
    let shared_data = Arc::new(data);  // Arc clone：在 Tokio 多任务下计数飙升，Dhat t-end 非零即泄漏

    // 剖析点4：模拟泄漏 - forgotten future，未 await/spawn handle 未存（Tokio 常见坑）
    let leaked_future = async move {  // move：捕获 shared_data，驻留 Arc<Vec>（Dhat 栈显示此 lambda）
        // 内部任务：模拟未完成工作，驻留内存（Dhat t-gmax 高峰值）
        tokio::time::sleep(Duration::from_secs(1)).await;  // 等待：放大驻留时间，Dhat 捕获未 free
        println!("Leaked task {} completed with data len {}.", id, shared_data.len());  // 未执行：泄漏确认
    };
    tokio::spawn(leaked_future);  // spawn 未存储：Dhat 报告 t-end blocks 非零（优化：存 handles vec，await all）

    // 剖析点5：注册分配到 Dhat（可选，ad hoc 模式），但 heap 模式自动
    // 注意：Dhat 忽略 pre-profiler alloc；优化前 drop shared_data 验证 t-end=0

    return ; // 返回：生产中 error 可释放资源，Dhat free 追踪
}

// Tokio 运行时：current_thread 确保 Dhat thread-local 准确（避免迁移混淆栈）
#[tokio::main(flavor = "current_thread")]// 单线程：Dhat 兼容性高，但生产用 multi_thread + 测试模式
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dhat 初始化：堆剖析，lifetime 覆盖 main（drop 时输出 stats/JSON）
    #[cfg(feature = "dhat-heap")]// 特性门控：零成本禁用
    let _profiler = dhat::Profiler::new_heap();  // new_heap：启动追踪，记录 total/t-gmax/t-end

    // 剖析范围：10k 请求，模拟高负载泄漏（Dhat total_blocks 预期 ~10k+）
    let mut tasks = vec![];  // 存储 handle：避免全局泄漏（优化点：若 forget，Dhat t-end 高）
    for i in 0..10_000 {
        tasks.push(tokio::spawn(s3_process_request(i)));  // spawn：Dhat 追踪 JoinHandle alloc（~100 bytes/task）
    }

    // 等待任务：检查泄漏（Dhat free 点），error unwrap 模拟 panic 泄漏
    for task in tasks {
        task.await?;  // await：drop handle，Dhat free 记录（未 await 即泄漏）
    }

    // Dhat 测试模式补充：获取 stats 断言（生产中移除）
    #[cfg(feature = "dhat-heap")]// 测试扩展：验证零泄漏
    let stats = dhat::HeapStats::get();  // get：当前堆 stats（curr_bytes 应=0）
    #[cfg(feature = "dhat-heap")]
    dhat::assert_eq!(stats.curr_bytes, 0);  // 断言：失败时 dump profile，便于调试

    println!("Tokio S3 simulation completed. Check dhat-heap.json for leaks (use DHAT viewer).");
    Ok(())  // 退出：_profiler drop，输出 stats/JSON（若 exit(0)，手动 drop _profiler）
}
