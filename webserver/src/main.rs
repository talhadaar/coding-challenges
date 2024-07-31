use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn main() {
    let server_socket = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(server_socket).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let header = "HTTP/1.1 200 OK";
    let body = fs::read_to_string("hello.html").unwrap();
    let body_len = body.len();

    let response = format!("{header}\r\nContent-Length: {body_len}\r\n\r\n{body}");
    stream.write_all(response.as_bytes()).unwrap();
}
