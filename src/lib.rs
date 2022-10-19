pub mod thread_pool {
    use std::sync::mpsc;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;
    type Job = Box<dyn FnOnce() + Send + 'static>;

    enum ThreadState {
        Doing(Job),
        Terminate,
    }

    #[warn(dead_code)]
    pub struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<ThreadState>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let state = receiver.lock().unwrap().recv().unwrap();
                match state {
                    ThreadState::Doing(job) => {
                        // println!("Worder {}, got a job; executing.", id);
                        job();
                    }
                    ThreadState::Terminate => {
                        // println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            });
            Worker {
                id,
                thread: Some(thread),
            }
        }
    }
    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: mpsc::Sender<ThreadState>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            ThreadPool { workers, sender }
        }
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.send(ThreadState::Doing(job)).unwrap();
        }
    }
    impl Drop for ThreadPool {
        fn drop(&mut self) {
            for _ in &self.workers {
                self.sender.send(ThreadState::Terminate).unwrap();
            }
            for worker in &mut self.workers {
                // println!("shutting down worker {}", worker.id);
                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
        }
    }
}

pub mod server {
    use crate::thread_pool;
    use std::collections::HashMap;
    use std::fs;
    use std::io::prelude::*;
    use std::net::TcpListener;
    use std::net::TcpStream;

    #[derive(Clone)]
    pub enum Content {
        Fn(fn(String) -> String),
        Addr(String),
    }
    #[derive(Clone)]
    pub struct Server {
        pub req_hash: HashMap<String, Content>,
        pub port: String,
    }

    impl Server {
        pub fn new() -> Server {
            Server {
                req_hash: HashMap::new(),
                port: "8080".to_string(),
            }
        }

        pub fn get(&mut self, path: &str, serve: Content) {
            match serve {
                Content::Fn(f) => {
                    self.req_hash
                        .insert(format!("GET {} HTTP/1.1\r\n", path), Content::Fn(f));
                }
                Content::Addr(serve) => {
                    self.req_hash.insert(
                        format!("GET {} HTTP/1.1\r\n", path),
                        Content::Addr(serve.to_string()),
                    );
                }
            }
        }

        fn check_req(&mut self, buffer: [u8; 1024]) -> (String, String) {
            let buffer = String::from_utf8_lossy(&buffer[..]);
            for req in self.req_hash.keys() {
                if buffer.starts_with(req) {
                    return (
                        "HTTP/1.1 200 OK".to_string(),
                        match self.req_hash.clone().get(&req.clone()) {
                            Some(content) => match &content {
                                Content::Fn(f) => f(buffer.clone().to_string()),
                                Content::Addr(s) => fs::read_to_string(s).unwrap(),
                            },
                            None => {
                                // handle error
                                panic!("error while handleing {}", req);
                            }
                        },
                    );
                }
            }
            (
                "HTTP/1.1 404 NOT FOUND".to_string(),
                "public/404.html".to_string(),
            )
        }

        fn handle_connection(&mut self, mut stream: TcpStream) {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let (status_line, content) = self.check_req(buffer.clone());

            let responce = format!(
                "{}\r\nContent-Lenght: {}\r\n\r\n{}",
                status_line,
                content.len(),
                content
            );
            stream.write(responce.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        pub fn bind(&mut self, port: &str) {
            self.port = port.to_string();
        }
        pub fn start(&self) {
            let listener = TcpListener::bind(String::from("127.0.0.1:") + &self.port).unwrap();
            let pool = thread_pool::ThreadPool::new(4);
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let mut inst = self.clone();
                pool.execute(move || {
                    inst.handle_connection(stream);
                });
            }
            println!("Shutting the server down");
        }
    }
}
