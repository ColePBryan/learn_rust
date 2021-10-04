use std::thread;

pub struct ThreadPool{
    workers: Vec<Worker>    
}

impl ThreadPool{

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// 
    /// The `new` function panics when size is zero.
    pub fn new(size: usize) -> Self {
        // TODO: change assert to PoolCreationError, have new return result instead
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for worker_id in 0..size {
            workers.push(Worker::new(worker_id));
        }

        ThreadPool{ workers }
    }
    pub fn execute<F>(&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

pub struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker{
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(||{});

        Worker { id, thread }
    }
}