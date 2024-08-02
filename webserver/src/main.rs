use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream}, task::JoinSet,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub const ADDR: [u8; 4] = [127, 0, 0, 1];
pub const PORT: u16 = 8080;

#[derive(Debug)]
struct Client {
    stream: TcpStream,
    saddr: SocketAddr,
}

#[derive(Debug, Clone)]
struct WebServer {
    ssocket: Arc<TcpListener>,
    cpool: Arc<Mutex<Vec<Client>>>,
    termination_flag: Arc<Mutex<bool>>,
}

impl WebServer {
    async fn new(saddr: &SocketAddr) -> Result<Self> {
        let ssocket = Arc::new(TcpListener::bind(saddr).await?);
        let cpool = Arc::new(Mutex::new(Vec::new()));
        Ok(Self { ssocket, cpool , termination_flag: Arc::new(Mutex::new(false))})
    }
}

async fn processor(ssocket: Arc<TcpListener>, cpool: Arc<Mutex<Vec<Client>>>) -> Result<()>{
    unimplemented!()
}

async fn pool_clients(ssocket: Arc<TcpListener>, cpool: Arc<Mutex<Vec<Client>>>) -> Result<()>{
    unimplemented!()
}

async fn handle_client(client: &mut Client) -> Result<()> {
    unimplemented!()
}

async fn handle_pool_clients(cpool: Arc<Mutex<Vec<Client>>>) -> Result<()> {
    unimplemented!()
}


#[tokio::main]
async fn main() -> Result<()> {
    let server_socket = SocketAddr::from((ADDR, PORT));
    let webserver = WebServer::new(&server_socket).await?;

    let mut set = JoinSet::new();

    println!("-> Setting up listening");
    let ssocket = Arc::clone(&webserver.ssocket);
    let cpool = Arc::clone(&webserver.cpool);

    set.spawn(async move{
        let _ = processor(ssocket, cpool).await;
    });

    println!("-> Setting up client handeling");
    let cpool = Arc::clone(&webserver.cpool);
    set.spawn(async move {
        let _ = handle_pool_clients(cpool).await;
    });

    let termination_flag = Arc::clone(&webserver.termination_flag);
    ctrlc_async::set_async_handler(async move {
        println!("-> Server terminated");
        let mut lock = termination_flag.lock().unwrap();
        *lock = true;
    })?;

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
