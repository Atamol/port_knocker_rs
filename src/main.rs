use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: rust-port-knocker <host> <port1> <port2> ... <portN>");
        return;
    }

    let host = &args[1];
    let ports = &args[2..];

    for port in ports {
        match port.parse::<u16>() {
            Ok(p) => {
                match TcpStream::connect(format!("{}:{}", host, p)).await {
                    Ok(_) => println!("Knocked on port {}", p),
                    Err(e) => eprintln!("Failed to knock on port {}: {}", p, e),
                }
                sleep(Duration::from_millis(200)).await;
            },
            Err(_) => eprintln!("Invalid port: {}", port),
        }
    }

    println!("Port knocking sequence completed.");
}
