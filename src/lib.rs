use std::collections::hash_map;
use std::hash::Hash;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use std::net::TcpStream;
use std::time::Duration;

// use listener::*;
use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;


type Job = Box<dyn FnOnce() + Send + 'static>;

enum ThreadState {
    Doing(Job),
    Terminate,
}
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
                    println!("Worder {}, got a job; executing.", id);
                    job();
                }
                ThreadState::Terminate => {
                    println!("Worker {} was told to terminate.", id);
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
            println!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub fn check_req(buffer: [u8; 1024]) -> (String, String) {
    // TODO: impliment a hashmap to lookup the requests
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
    (status_line.to_string(), filename.to_string())

}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();


    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let (status_line, filename) = check_req(buffer);


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


