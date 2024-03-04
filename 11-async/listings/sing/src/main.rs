extern crate async_std;

use async_std::task::{block_on, sleep};

use std::time::Duration;

// ANCHOR: sing
struct Song {
    lyrics: String,
}

async fn learn_song() -> Song {
    println!("Learning...");
    sleep(Duration::from_secs(5)).await;
    Song { lyrics: "Doe, a deer, a female deer; Ray, a drop of golden sun...".to_string() }
}

async fn sing_song(song: &Song) {
    println!("Singing! {}", song.lyrics);
}

async fn dance() {
    for _ in 0..5 {
        sleep(Duration::from_secs(1)).await;
        println!("dancity-dance");
    }
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(&song).await;
}

async fn blocking() {
    println!("Learn to sing and dance!\n");
    learn_and_sing().await;
    dance().await;
}
// ANCHOR_END: sing

// ANCHOR: async
#[async_std::main]
async fn main() {
    block_on(blocking());

    println!("\nLet's do both at once!");

    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}
// ANCHOR_END: async

