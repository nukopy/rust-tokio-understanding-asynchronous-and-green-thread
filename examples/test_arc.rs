use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 共有するデータを用意します
    let shared_data = Arc::new(Mutex::new(0));

    // スレッド1
    let shared_data1 = Arc::clone(&shared_data);
    let thread1 = thread::spawn(move || {
        let mut data = shared_data1.lock().unwrap();
        println!("[Thread 1] before: {}", *data);
        *data += 1;
        println!("[Thread 1] after {}", *data);
    });

    // スレッド2
    let shared_data2 = Arc::clone(&shared_data);
    let thread2 = thread::spawn(move || {
        let mut data = shared_data2.lock().unwrap();
        println!("[Thread 2] before: {}", *data);
        *data += 2;
        println!("[Thread 2] after {}", *data);
    });

    // スレッドの終了を待機します
    thread1.join().unwrap();
    thread2.join().unwrap();

    // 共有データの最終結果を取得します
    let final_data = shared_data.lock().unwrap();
    println!("Final Data: {}", *final_data);
}
