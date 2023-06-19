use std::time::{Duration, Instant};

// タスクを 2 つ生成してみる
// ランタイムの設定：マルチスレッド（メインスレッド + ワーカースレッド 1 つ）
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let now = Instant::now();
    println!("===== ex3 =====");
    let cpus = num_cpus::get();
    println!("Number of logical CPU cores: {}", cpus);

    // task 1
    tokio::spawn(async move {
        println!("Start task 1...");
        std::thread::sleep(Duration::from_secs(3)); // 同期スリープ 3 秒
        println!("Task 1 woke up!"); // 3秒後に実行されるよ
    });
    // この時点で task 1 は走っている

    // task 2
    tokio::spawn(async move {
        println!("Start task 2...");
        std::thread::sleep(Duration::from_secs(1)); // 同期スリープ 1 秒
        println!("Task 2 woke up!"); // 3秒後に実行されるよ
    });
    // この時点で task 2 は走らない。ワーカースレッドの数に空きがない。

    std::thread::sleep(Duration::from_secs(5));
    println!("Done!");
    println!("Elapsed: {:.4?} seconds", now.elapsed().as_secs_f64());
}

/* 出力
===== ex3 =====
Number of logical CPU cores: 10 （論理コア数。マシンによって変わる。）
Start task 1...
Task 1 woke up!
Start task 2...
Task 2 woke up!
Done!
Elapsed: 5.0049 seconds
 */

/* 解説
===== ex3 =====
[Main Thread] Number of logical CPU cores: 10 （論理コア数。マシンによって変わる。）
[Worker Thread 1] Start task 1...
(3 秒後)
[Worker Thread 1] Task 1 woke up!
--> タスク 1 は同期スリープなのでタスク 2 にワーカースレッドを譲らない。
    そして、同期スリープが終わると、タスク 2 がワーカースレッドに割り当てられる。
[Worker Thread 1] Start task 2...
(1 秒後)
[Worker Thread 1] Task 2 woke up!
Done!
 */

/*
 * tokio::spawn は OS スレッドではなく、グリーンスレッドを生成する。
 * OS スレッドを生成する場合は、std::thread::spawn を使用する。
 * Golang はプリエンプティブなスケジューリング (1.14から) のため、tokioのようなタスク (goroutine) がスレッドを占有 (10ms) する問題はない。
 */
