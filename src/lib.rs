mod thread_pool;
mod server;

// pub mod server {
//     use crate::thread_pool;
//     use std::collections::HashMap;
//     use std::fs;
//     use std::io::prelude::*;
//     use std::net::TcpListener;
//     use std::net::TcpStream;

//     #[derive(Clone)]
//     pub enum Content {
//         Fn(fn(String) -> String),
//         Addr(String),
//     }
//     #[derive(Clone)]
//     pub struct Server {
//         pub req_hash: HashMap<String, Content>,
//         pub port: String,
//     }

//     #[warn(dead_code)]
//     pub struct Buffer {
//         req: String,
//         host: String,
//         connection: String,
//         accept_encoding: Vec<String>,
//     }
//     impl Buffer {
//         pub fn new(
//             req: String,
//             host: String,
//             connection: String,
//             accept_encoding: Vec<String>,
//         ) -> Buffer {
//             Buffer {
//                 req,
//                 host,
//                 connection,
//                 accept_encoding,
//             }
//         }
//         pub fn parse_stream(buffer: [u8; 1024]) {}
//     }

//     impl Server {
//         pub fn new() -> Server {
//             Server {
//                 req_hash: HashMap::new(),
//                 port: "8080".to_string(),
//             }
//         }

//         pub fn get(&mut self, path: &str, serve: Content) {
//             match serve {
//                 Content::Fn(f) => {
//                     self.req_hash
//                         .insert(format!("GET {} HTTP/1.1\r\n", path), Content::Fn(f));
//                 }
//                 Content::Addr(serve) => {
//                     self.req_hash.insert(
//                         format!("GET {} HTTP/1.1\r\n", path),
//                         Content::Addr(serve.to_string()),
//                     );
//                 }
//             }
//         }

//         pub fn check_req(&mut self, buffer: [u8; 1024]) -> (String, String) {
//             Buffer::parse_stream(buffer.clone());
//             let buffer = String::from_utf8_lossy(&buffer[..]);
//             for req in self.req_hash.keys() {
//                 if buffer.starts_with(req) {
//                     return (
//                         "HTTP/1.1 200 OK".to_string(),
//                         match self.req_hash.clone().get(&req.clone()) {
//                             Some(content) => match &content {
//                                 Content::Fn(f) => f(buffer.clone().to_string()),
//                                 Content::Addr(s) => fs::read_to_string(s).unwrap(),
//                             },
//                             None => {
//                                 // handle error
//                                 panic!("error while handleing {}", req);
//                             }
//                         },
//                     );
//                 }
//             }
//             (
//                 "HTTP/1.1 404 NOT FOUND".to_string(),
//                 "public/404.html".to_string(),
//             )
//         }

//         fn handle_connection(&mut self, mut stream: TcpStream) {
//             let mut buffer = [0; 1024];
//             stream.read(&mut buffer).unwrap();
//             let (status_line, content) = self.check_req(buffer.clone());

//             let responce = format!(
//                 "{}\r\nContent-Lenght: {}\r\n\r\n{}",
//                 status_line,
//                 content.len(),
//                 content
//             );
//             stream.write(responce.as_bytes()).unwrap();
//             stream.flush().unwrap();
//         }
//         pub fn bind(&mut self, port: &str) {
//             self.port = port.to_string();
//         }
//         pub fn start(&self) {
//             let listener = TcpListener::bind(String::from("127.0.0.1:") + &self.port).unwrap();
//             let pool = thread_pool::ThreadPool::new(4);
//             for stream in listener.incoming() {
//                 let stream = stream.unwrap();
//                 let mut inst = self.clone();
//                 pool.execute(move || {
//                     inst.handle_connection(stream);
//                 });
//             }
//             println!("Shutting the server down");
//         }
//     }
// }
