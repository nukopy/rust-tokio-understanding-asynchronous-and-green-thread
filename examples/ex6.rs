use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));
    let d = Arc::clone(&data);

    let handle = tokio::spawn(async move {
        println!("Start task...");
        {
            let mut guard: MutexGuard<i32> = d.lock().unwrap();
            *guard += 1;
        }
        tokio::time::sleep(Duration::from_secs(3)).await;
    });

    handle.await.unwrap();
    println!("data: {}", data.lock().unwrap());
    println!("Done!");
}
