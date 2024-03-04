use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::error::Error;

async fn handle_client(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0, 80];

    println!("Handling new client");
    loop {
        match stream.read(&mut buf).await {
            Ok(n) => {
                if n == 0 {
                    println!("Bye Bye!");
                    break;
                }
                stream.write_all(&buf[0..n]).await?;
            },
            Err(e) => {
                println!("read failed: {e}");
                return Err(Box::new(e));
            },
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:50000").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        
        // Now process this request without blocking the main loop
        tokio::spawn(async move { handle_client(&mut stream).await.unwrap(); });
    }
}
