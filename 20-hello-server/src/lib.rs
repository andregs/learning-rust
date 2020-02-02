use std::sync::{{ Arc, Mutex, mpsc }};
use std::thread;

pub struct ThreadPool {
    sender: mpsc::Sender<Job>,
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
        self.sender.send(job).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize, // that's the recommended type for indexing collections
    thread: thread::JoinHandle<()>,
}

type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

impl Worker {
    fn new(id: usize, receiver: Receiver) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // receiving a message is synchronized, thanks to Mutex, but
                // executing the job is parallel
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} executing a job.", id);
                job();
            }
        });

        Worker { id, thread }
    }
}

// this code isn't ideal, the compilation warnings saying we're not using 'workers', 'id' and 
// 'thread' fields suggest that we're not cleaning up anything. when we ctrl+c to stop the server,
// we kill all threads.
