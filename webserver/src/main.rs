use std::{borrow::BorrowMut, net::SocketAddr, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
    task::JoinSet,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const ADDR: [u8; 4] = [127, 0, 0, 1];
pub const PORT: u16 = 8080;

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    addr: SocketAddr,
}

#[derive(Debug)]
struct WebServer {
    listener: Arc<TcpListener>,
    clients: Arc<Mutex<Vec<Client>>>,
    terminate: Arc<Mutex<bool>>,
}

impl WebServer {
    async fn new(addr: &SocketAddr) -> Result<Self> {
        let listener = Arc::new(TcpListener::bind(addr).await?);
        let clients = Arc::new(Mutex::new(Vec::new()));
        let terminate = Arc::new(Mutex::new(false));
        Ok(Self {
            listener,
            clients,
            terminate,
        })
    }
}

async fn receive_clients(webserver: Arc<WebServer>) -> Result<()> {
    println!("-> Waiting for clients to connect");
    Ok(())
}

async fn process_client(client: Client) -> Result<()> {
    println!("-> Processing client: {:?}", client);
    Ok(())
}

async fn process_clients(webserver: Arc<WebServer>) -> Result<()> {
    println!("-> Processing clients");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let saddr = SocketAddr::from((ADDR, PORT));
    let webserver_pool = Arc::new(WebServer::new(&saddr).await?);
    let mut set = JoinSet::new();

    // Terminate signal process
    println!("-> Press Ctrl-C to terminate the server");
    let terminate_flag = Arc::clone(&webserver_pool.terminate);
    ctrlc_async::set_async_handler(async move {
        println!("-> Terminating the server");
        let mut flag = terminate_flag.lock().await;
        *flag = true;
    })?;

    // Listener process
    let webserver_move = Arc::clone(&webserver_pool);
    set.spawn(async move {
        let _ = receive_clients(webserver_move).await;
    });

    // Client processer process
    let webserver_move = Arc::clone(&webserver_pool);
    set.spawn(async move {
        let _ = process_clients(webserver_move).await;
    });

    set.join_next().await;
    set.join_next().await;

    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let header = "HTTP/1.1 200 OK";
    let body = tokio::fs::read_to_string("hello.html").await?;
    let body_len = body.len();

    let response = format!("{header}\r\nContent-Length: {body_len}\r\n\r\n{body}");
    let _ = stream.write_all(response.as_bytes()).await;
    Ok(())
}
