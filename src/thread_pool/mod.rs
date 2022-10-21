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
