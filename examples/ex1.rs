use std::time::Duration;

// ランタイムの設定：マルチスレッド
#[tokio::main]
async fn main() {
    println!("===== ex1 =====");
    tokio::task::spawn(async {
        // tokio::spawn で OK
        println!("Start tokio task");
        tokio::time::sleep(Duration::from_secs(3)).await; // ネットワーク呼び出しのフリ
        println!("Woke up!"); // 3秒後に実行されるよ
    });
    // この時点でタスクはすでに走り始めている

    std::thread::sleep(Duration::from_secs(5)); // プログラムが終了しないように
    println!("Done");
}
