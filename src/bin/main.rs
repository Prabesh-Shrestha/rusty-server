use listener::*;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting the server down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let home_req = b"GET / HTTP/1.1\r\n";
    let sleep_req = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(home_req) {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if buffer.starts_with(sleep_req) {
        thread::sleep(Duration::from_secs(4));
        ("HTTP/1.1 200 OK", "public/sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };
    let content = fs::read_to_string(filename).unwrap();
    let responce = format!(
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(responce.as_bytes()).unwrap();
    stream.flush().unwrap();
}
