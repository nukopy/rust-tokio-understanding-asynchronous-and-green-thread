async fn my_async_task() {
    println!("Start task...");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    println!("Woke up!");
}

// tokio ランタイムの設定のカスタマイズ
// マクロを使わず、ランタイムビルダーを使用してランタイムを生成する。
// block_on にプログラムを本体を渡す
fn main() {
    let cpus = num_cpus::get();
    println!("Number of logical CPU cores: {}", cpus);

    // ref: https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html#examples
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4) // ワーカースレッド数
        .max_blocking_threads(1) // ブロッキングスレッド数
        .enable_all() // 全ての機能を有効化（ネットワーク I/O など）
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .on_thread_start(|| {
            println!("thread started");
        })
        .on_thread_stop(|| {
            println!("thread stopped");
        })
        .build()
        .unwrap()
        .block_on(async {
            // ルートのタスク
            my_async_task().await;
        });
}
