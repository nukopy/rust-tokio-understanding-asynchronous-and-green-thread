use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // チャネル
    let buffer_size = 2;
    let (tx, mut rx) = mpsc::channel(buffer_size);

    // 送信側タスク
    let handle1 = tokio::spawn(async move {
        println!("Start transmitting task...");
        let mut count: i32 = 0;

        while count < 5 {
            // チャネルにはバッファーサイズを指定できる。
            // バッファーがいっぱいになると、send().await はサスペンドし次の処理に進めない。
            // これをバックプレッシャーと呼ぶが、送りすぎを防止する機能になる。
            if let Err(e) = tx.send(count).await {
                println!("Failed to send a message: {}", e);
                break;
            }

            println!("Sent: {}", count);
            count += 1;
        }
    });
    // この時点で送信側タスクは走り始めている

    // 受信側タスク
    let handle2 = tokio::spawn(async move {
        println!("Start receiving task...");
        while let Some(msg) = rx.recv().await {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Received: {}", msg);
        }
    });

    handle1.await.unwrap();
    handle2.await.unwrap();
    println!("Done!");
}

/* 出力（毎回変わる。バッファの数を変えると挙動が変わるよ。）
チャネルにはバッファーサイズを指定できる。
バッファーがいっぱいになると、send().await はサスペンドし次の処理に進めない。
これをバックプレッシャーと呼ぶが、送りすぎを防止する機能になる。

[buffer_size = 2]
Start transmitting task...
Sent: 0
Sent: 1
Sent: 2
Start receiving task...
Sent: 3
Received: 0
Sent: 4
Received: 1
Received: 2
Received: 3
Received: 4
Done!
*/

/*
[buffer_size = 1]
Start transmitting task...
Sent: 0
Start receiving task...
Sent: 1
Received: 0
Sent: 2
Received: 1
Sent: 3
Received: 2
Sent: 4
Received: 3
Received: 4
Done!
 */

/*
[buffer_size = 5]
Start transmitting task...
Sent: 0
Sent: 1
Sent: 2
Sent: 3
Start receiving task...
Sent: 4
Received: 0
Received: 1
Received: 2
Received: 3
Received: 4
Done!
 */

/*
[buffer_size = 0]
チャネルの buffer_size を 0 に指定するとランタイムエラーになる。
thread 'main' panicked at 'mpsc bounded channel requires buffer > 0', examples/ex5_channel.rs:8:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 */
