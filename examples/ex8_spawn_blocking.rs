use std::time::Duration;
use tokio::task;

// シングルスレッド環境
#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(async {
        println!("Start task 1...");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("woke up! 1");
    });

    let handle = task::spawn_blocking(move || {
        // ブロッキングスレッドで動くよ
        println!("Start task 2...");
        std::thread::sleep(Duration::from_secs(1)); // 同期的に記述する
        println!("woke up! 2");
    });

    std::thread::sleep(Duration::from_secs(3));
    println!("woke up! 3");

    handle.await.expect("TODO: panic message");
}
