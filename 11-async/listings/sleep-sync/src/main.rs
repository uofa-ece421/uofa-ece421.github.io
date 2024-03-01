use std::thread::{sleep};
use std::time::Duration;

fn sleeping() {
    for i in 1..=5 {
        println!("Sleeping {}", i);
        sleep(Duration::from_millis(1000));
    }
}
fn interrupting() {
    for i in 1..=10 {
        println!("Interrupting {}", i);
        sleep(Duration::from_millis(500));
    }
}

fn main() {
    sleeping();
    interrupting();
}
