use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::vec;

pub struct ThreadPool {
    size: usize,
    workers: vec::Vec<Worker>,
    tx: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = vec::Vec::with_capacity(size);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        for id in 0..size {
            workers.push(Worker::new(id, rx.clone()));
        }

        let mut tp = ThreadPool { size, workers, tx };

        return tp;
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);

        // TODO don't unwrap
        self.tx.send(Message::Job(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.tx.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // TODO don't unwrap
            let message = rx.lock().unwrap().recv().unwrap();

            match message {
                Message::Job(job) => {
                    println!("Worker {} got a job", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} got termination signal", id);
                    break;
                }
            }
        });

        return Worker {
            id,
            thread: Some(thread),
        };
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Job(Job),
    Terminate,
}
