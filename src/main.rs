use std::net::TcpListener;

use zero2prod::startup;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("port: {}", port);
    startup::run(listener)?.await
}
