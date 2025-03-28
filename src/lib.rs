use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread
};


type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // place for change here. Use std::thread::Builder instead of std::thread::spawn
        // Why?: if there is not enough resources for creating a thread whole thing will
        // panic and collapse.
        
        let thread = thread::spawn(move || loop {
            // let job = receiver.lock().unwrap().recv().unwrap();
            let job = receiver.lock().expect("Thread was poisoned.").recv();

            match job {
                Ok(job) => { 
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker disconnected. shutting down...");
                    break;
                }
            } 
        });
        
        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 1..=size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { 
            workers, 
            sender: Some(sender) 
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // Good place to try error handle myself
        // Author says that sending a job can fail if we stpo all threads, but
        // we aren't able to do so, because threads will exist as long as the pool exist.
        // Therefore we can to not worry about it, and tell compiler to chill by unwrapping it.
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}
