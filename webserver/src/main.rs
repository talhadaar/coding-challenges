use std::net::SocketAddr;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const ADDR: [u8; 4] = [127, 0, 0, 1];
pub const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(SocketAddr::from((ADDR, PORT))).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let header = "HTTP/1.1 200 OK";
    let body = tokio::fs::read_to_string("hello.html").await?;
    let body_len = body.len();

    let response = format!("{header}\r\nContent-Length: {body_len}\r\n\r\n{body}");
    let _ = stream.write_all(response.as_bytes()).await;
    Ok(())
}
