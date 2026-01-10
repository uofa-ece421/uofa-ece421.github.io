use chrono::Utc;
use std::io;
use tokio::time::{sleep, Duration};

use std::collections::HashMap;

async fn handle_cmd(cmd: char) {
    match cmd {
        'i' => {
            if let Ok(resp) = reqwest::get("https://httpbin.org/ip").await {
                if let Ok(ip) = resp.json::<HashMap<String, String>>().await {
                    println!("{:#?}", ip);
                }
            }
        },
        'd' => if let Ok(resp) = reqwest::get("https://httpbin.org/delay/9").await {
                 println!("{:#?}", resp);
        },
        other => println!("unknown cmd: {}", other),
    }
}

async fn read_cmd() -> Result<usize, Box<dyn std::error::Error>> {
    let mut data = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut data) {
        Ok(len) => {
            if len > 0 {
                if let Some(cmd) = data.chars().next() {
                    handle_cmd(cmd).await;
                }
            }
            Ok(len)
        },
        Err(e) => Err(Box::new(e)),
    }
}

async fn periodic_task(interval: u64) {
    sleep(Duration::from_secs(interval)).await;
    println!("UTC now: {}", Utc::now());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        match read_cmd().await {
            Ok(len) => if len == 0 {
                break;
            },
            Err(e) => println!("input error {}", e),
        };
        periodic_task(5).await;
    }
    
    Ok(())
}

