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

    // シングルスレッド環境で同期スリープを実行すると、OS スレッドを掴んだままスリープしてしまう
    std::thread::sleep(Duration::from_secs(5));
    println!("Hello from the main thread!");
}
