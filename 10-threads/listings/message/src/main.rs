extern crate chrono;
use chrono::offset::Local;
use std::sync::mpsc;
use std::thread;
use chrono::DateTime;
use std::time::{Duration, SystemTime};

enum Message {
    Request { rsvp: mpsc::Sender<Message>, op: i32 },
    Response { data: SystemTime },
}

fn main() {
    let (server_tx, server_rx) = mpsc::channel::<Message>();

    let client = thread::spawn(move || {
        let (tx, rx) = mpsc::channel::<Message>();

        for i in 1..4 {
            let msg = Message::Request { rsvp: tx.clone(), op: i };
            server_tx.send(msg).expect("send to server failed");
            if let Ok(msg) = rx.recv() {
                match msg {
                    Message::Request{..} => println!("unexpected request"),
                    Message::Response{ data } => {
                        let datetime: DateTime<Local> = data.into();
                        println!("response: {}", datetime);
                    }
                }
            }
            thread::sleep(Duration::from_secs(i as u64));
        }
    });

    let server = thread::spawn(move || {
        while let Ok(request) = server_rx.recv() {
            match request {
                Message::Request { rsvp, op } => {
                    println!("server got request {}", op);
                    let msg = Message::Response { data: SystemTime::now() };
                    rsvp.send(msg).expect("send to client failed");
                }
                Message::Response{..} => println!("unexpected response"),
            }
        }
    });

    let _ = client.join().expect("unexpected client panic");
    let _ = server.join().expect("unexpected server panic");
}

