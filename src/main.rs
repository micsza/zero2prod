use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = "127.0.0.1:8000";
    let listener =
        TcpListener::bind(address).expect(&format!("Failed to bind to {}", address));
    run(listener)?.await
}
