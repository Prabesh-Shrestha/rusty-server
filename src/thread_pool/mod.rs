// Thread Pool Handle 
// concurrency and memory safe 
// Gracefull Shutdown Behaviour 

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
type Job = Box<dyn FnOnce() + Send + 'static>;

enum ThreadState {
    Doing(Job),
    Terminate,
}
// Each worker owns its thread handler, 
// jobs are sent through the shared mpsc reciever
#[warn(dead_code)]
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<ThreadState>>>) -> Worker {
        // worker thread continously waits for message. 
        // Design ensures threads remain alive. 
        // removes the overhead of creating new threads for new tasks. 
        let thread = thread::spawn(move || loop {
            let state = receiver.lock().unwrap().recv().unwrap();
            match state {
                ThreadState::Doing(job) => {
                    // println!("Worder {}, got a job; executing.", id);
                    job();
                }
                // Gracefull Terminate signal
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

// ThreadPool manages a list of worker threads 
// jobs are sent to workers through a channel (mpsc), 
// safe communication without shared mutable
// states, preventing deadlocks
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

// Ensures that every worker thread is cleared 
// from the memory when ThreadPool goes out of scope. 
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
