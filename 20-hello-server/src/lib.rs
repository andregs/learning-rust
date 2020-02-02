use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    sender: mpsc::Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // we have to share this single consumer between many worker threads
        let (sender, receiver) = mpsc::channel();

        // Arc: multiple workers own the receiver
        // Mutex: only one worker gets a job from the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            workers.push(Worker::new(id, receiver));
        }

        ThreadPool { sender, workers }
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Send to transfer the given closure from one thread to another
        // 'static because we don't know how long the thread will take to execute
        let job = Box::new(func);
        let msg = Message::NewJob(job);
        self.sender.send(msg).unwrap();
    }
}

/// when the pool is dropped, main thread join all workers to make sure they finish their work.
/// TODO make this graceful shutdown be called when we ctrl+c to kill the app.
/// currently we can test this by taking only a few tcp messages in "fn main()" loop
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        
        for _ in &mut self.workers {
            // it's no broadcast. Each worker get a message
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // we use 'take' to move the thread out of the Option, leaving None in the worker
            if let Some(thread) = worker.thread.take() {
                // this blocks untill worker finishes its job, then the worker will get a
                // Terminate signal that breaks its infinite loop (see Worker::new)
                thread.join().unwrap();
            }
        }
    }
}

/// ThreadPool sends one of these messages to its workers
enum Message {
    /// job to be executed
    NewJob(Job),
    /// signal worker to stop listening and exit its infinite loop
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize, // that's the recommended type for indexing collections
    thread: Option<thread::JoinHandle<()>>,
}

type Receiver = Arc<Mutex<mpsc::Receiver<Message>>>;

impl Worker {
    fn new(id: usize, receiver: Receiver) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // receiving a message is synchronized, thanks to Mutex, but
                // executing the job is parallel
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} executing a job.", id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} got terminate signal.", id);
                        break; // the outer loop
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
