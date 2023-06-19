use std::time::Duration;

// ランタイムの設定：シングルスレッド（Node.js みたいなランタイム）
#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("===== ex2 =====");
    tokio::spawn(async {
        println!("Hello from Tokio!");
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("Woke up!");
    });

    // シングルスレッド環境で同期スリープを非同期スリープにすれば、
    // tokio のスケジューラでタスクを管理できる
    // std::thread::sleep(Duration::from_secs(5));
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Hello from the main thread!");
}
