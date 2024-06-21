use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::VecDeque;

const BACKEND_SERVERS: [&str; 3] = ["127.0.0.1:8081", "127.0.0.1:8082", "127.0.0.1:8083"];

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Load balancer listening on 127.0.0.1:8080");

    let backend_servers = Arc::new(Mutex::new(VecDeque::from(BACKEND_SERVERS)));

    while let Ok((inbound, _)) = listener.accept().await {
        let backend_servers = backend_servers.clone();

        tokio::spawn(async move {
            let mut backend_servers = backend_servers.lock().await;
            
            let backend_server = backend_servers.pop_front().unwrap();
            backend_servers.push_back(backend_server.clone());

            match TcpStream::connect(backend_server).await {
                Ok(mut outbound) => {
                    if let Err(e) = tokio::io::copy(&mut inbound, &mut outbound).await {
                        eprintln!("Error copying data: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error connecting to backend server: {}", e);
                }
            }
        });
    }

    Ok(())
}
