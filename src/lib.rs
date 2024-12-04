use std::process::id;

pub struct ThreadPool {
    workers: Vec
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(id()))
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static, {}
}