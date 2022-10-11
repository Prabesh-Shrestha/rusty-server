use listener::*;
use std::net::TcpListener;



fn main() {
    const PORT: &str = "7878";
    let listener = TcpListener::bind(String::from("127.0.0.1:")+ PORT).unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting the server down");
}

