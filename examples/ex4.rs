use futures::{stream::FuturesUnordered, StreamExt};
use std::time::Duration;

// タスクをたくさん生成してみる
// #[tokio::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    console_subscriber::init(); // for tokio-console

    println!("===== ex4 =====");
    let mut tasks = FuturesUnordered::new();

    let h1 = tokio::spawn({
        async move {
            println!("Start task 1...");
            std::thread::sleep(Duration::from_secs(10)); // 同期スリープ 10 秒 （ここでつまる）
            println!("Task 1 woke up!"); // 3秒後に実行されるよ
        }
    });
    tasks.push(h1);

    for i in 1..100 {
        let h2 = tokio::spawn({
            async move {
                println!("Start task {}...", i);
                tokio::time::sleep(Duration::from_secs(1)).await; // 非同期スリープ 1 秒
                println!("Task {} woke up!", i);
            }
        });
        tasks.push(h2);
    }

    // join_all はおそい
    // https://github.com/tokio-rs/tokio/issues/2401
    while let Some(item) = tasks.next().await {
        match item {
            Ok(_) => {
                println!("Join successed!");
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    println!("Done!");
}

/*
===== ex4 =====
Start task 1...
Task 1 woke up!
Start task 1...
Join successed!
Start task 2...
Start task 3...
Start task 4...
Start task 5...
Start task 6...
Start task 7...
Start task 8...
Start task 9...
Start task 10...
Start task 11...
Start task 12...
Start task 13...
Start task 14...
Start task 15...
Start task 16...
Start task 17...
Start task 18...
Start task 19...
Start task 20...
Start task 21...
Start task 22...
Start task 23...
Start task 24...
Start task 25...
Start task 26...
Start task 27...
Start task 28...
Start task 29...
Start task 30...
Start task 31...
Start task 32...
Start task 33...
Start task 34...
Start task 35...
Start task 36...
Start task 37...
Start task 38...
Start task 39...
Start task 40...
Start task 41...
Start task 42...
Start task 43...
Start task 44...
Start task 45...
Start task 46...
Start task 47...
Start task 48...
Start task 49...
Start task 50...
Start task 51...
Start task 52...
Start task 53...
Start task 54...
Start task 55...
Start task 56...
Start task 57...
Start task 58...
Start task 59...
Start task 60...
Start task 61...
Start task 62...
Start task 63...
Start task 64...
Start task 65...
Start task 66...
Start task 67...
Start task 68...
Start task 69...
Start task 70...
Start task 71...
Start task 72...
Start task 73...
Start task 74...
Start task 75...
Start task 76...
Start task 77...
Start task 78...
Start task 79...
Start task 80...
Start task 81...
Start task 82...
Start task 83...
Start task 84...
Start task 85...
Start task 86...
Start task 87...
Start task 88...
Start task 89...
Start task 90...
Start task 91...
Start task 92...
Start task 93...
Start task 94...
Start task 95...
Start task 96...
Start task 97...
Start task 98...
Start task 99...
Task 1 woke up!
Join successed!
Task 6 woke up!
Join successed!
Task 2 woke up!
Join successed!
Task 3 woke up!
Join successed!
Task 4 woke up!
Join successed!
Task 5 woke up!
Join successed!
Task 9 woke up!
Join successed!
Task 7 woke up!
Join successed!
Task 8 woke up!
Join successed!
Task 12 woke up!
Join successed!
Task 10 woke up!
Join successed!
Task 11 woke up!
Join successed!
Task 16 woke up!
Join successed!
Task 13 woke up!
Join successed!
Task 14 woke up!
Join successed!
Task 21 woke up!
Join successed!
Task 15 woke up!
Join successed!
Task 17 woke up!
Join successed!
Task 18 woke up!
Join successed!
Task 19 woke up!
Join successed!
Task 20 woke up!
Join successed!
Task 31 woke up!
Join successed!
Task 22 woke up!
Join successed!
Task 23 woke up!
Join successed!
Task 24 woke up!
Join successed!
Task 25 woke up!
Join successed!
Task 26 woke up!
Join successed!
Task 27 woke up!
Join successed!
Task 28 woke up!
Join successed!
Task 29 woke up!
Join successed!
Task 30 woke up!
Join successed!
Task 43 woke up!
Join successed!
Task 32 woke up!
Join successed!
Task 33 woke up!
Join successed!
Task 34 woke up!
Join successed!
Task 35 woke up!
Join successed!
Task 36 woke up!
Join successed!
Task 37 woke up!
Join successed!
Task 38 woke up!
Join successed!
Task 39 woke up!
Join successed!
Task 40 woke up!
Join successed!
Task 41 woke up!
Join successed!
Task 42 woke up!
Join successed!
Task 49 woke up!
Join successed!
Task 44 woke up!
Join successed!
Task 45 woke up!
Join successed!
Task 46 woke up!
Join successed!
Task 47 woke up!
Join successed!
Task 48 woke up!
Join successed!
Task 61 woke up!
Join successed!
Task 50 woke up!
Join successed!
Task 51 woke up!
Join successed!
Task 52 woke up!
Join successed!
Task 53 woke up!
Join successed!
Task 54 woke up!
Task 55 woke up!
Join successed!
Join successed!
Task 56 woke up!
Join successed!
Task 57 woke up!
Join successed!
Task 58 woke up!
Join successed!
Task 59 woke up!
Join successed!
Task 60 woke up!
Join successed!
Task 68 woke up!
Join successed!
Task 62 woke up!
Task 63 woke up!
Join successed!
Join successed!
Task 64 woke up!
Join successed!
Task 65 woke up!
Task 66 woke up!
Join successed!
Join successed!
Task 67 woke up!
Join successed!
Task 84 woke up!
Join successed!
Task 69 woke up!
Join successed!
Task 70 woke up!
Join successed!
Task 71 woke up!
Task 72 woke up!
Task 73 woke up!
Join successed!
Join successed!
Join successed!
Task 74 woke up!
Task 75 woke up!
Join successed!
Join successed!
Task 76 woke up!
Task 77 woke up!
Task 78 woke up!
Join successed!
Join successed!
Join successed!
Task 79 woke up!
Task 80 woke up!
Task 81 woke up!
Task 82 woke up!
Task 83 woke up!
Join successed!
Join successed!
Join successed!
Join successed!
Join successed!
Task 92 woke up!
Task 85 woke up!
Task 86 woke up!
Task 87 woke up!
Task 88 woke up!
Task 89 woke up!
Task 90 woke up!
Join successed!
Join successed!
Join successed!
Join successed!
Join successed!
Join successed!
Join successed!
Task 91 woke up!
Join successed!
Task 99 woke up!
Task 93 woke up!
Task 94 woke up!
Task 95 woke up!
Task 96 woke up!
Task 97 woke up!
Task 98 woke up!
Join successed!
Join successed!
Join successed!
Join successed!
Join successed!
Join successed!
Join successed!
Done!
 */
