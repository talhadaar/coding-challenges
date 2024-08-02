use std::{net::SocketAddr, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::Mutex,
    task::JoinSet,
    time::{timeout, Duration},
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
    println!("-> receive_clients()");
    let timeout_limit = Duration::from_secs(3);

    // loop while the terminate flag is false
    while !*webserver.terminate.lock().await {
        let (stream, addr) = timeout(timeout_limit, webserver.listener.accept()).await??;
        let clients = Arc::clone(&webserver.clients);

        tokio::spawn(async move {
            let mut clients_lock = clients.lock().await;
            clients_lock.push(Client { stream, addr });
        });
    }

    println!("-> Terminating receive_clients()");
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

async fn process_client(client: Client) -> Result<()> {
    println!("-> Processing client: {:?}", client);
    handle_connection(client.stream).await?;
    Ok(())
}

async fn process_clients(webserver: Arc<WebServer>) -> Result<()> {
    println!("-> process_clients()");
    let mut set = JoinSet::new();

    while !*webserver.terminate.lock().await {
        let client = webserver.clients.lock().await.pop();
        match client {
            Some(c) => {
                set.spawn(async move {
                    timeout(Duration::from_secs(5), process_client(c))
                        .await
                        .unwrap()
                        .unwrap();
                });
            }
            None => {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    set.abort_all();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let saddr = SocketAddr::from((ADDR, PORT));
    let webserver_pool = Arc::new(WebServer::new(&saddr).await?);
    let mut set = JoinSet::new(); // TODO explore using barriers instead of JoinSet?

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
