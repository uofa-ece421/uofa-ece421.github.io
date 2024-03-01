use async_std::task::{sleep, spawn};

use std::time::Duration;

async fn sleeping() {
    for i in 1..=5 {
        println!("Sleeping {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

async fn interrupting() {
    for i in 1..=10 {
        println!("Interrupting {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

#[async_std::main]
async fn main() {
    let sleeping = spawn(sleeping());
    interrupting().await;
    sleeping.await;
}
