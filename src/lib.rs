use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 创建线程池
    ///
    /// 线程池中的线程数量
    ///
    /// # Panics
    /// 'new' 函数在 size 为 0 时会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // 创建一个信道
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        // 创建 size 个 线程
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// 接受一个闭包函数
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 创建一个 job
        let job = Box::new(f);
        // 将该job 推到信道里面
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    // 线程的 id 号
    id: usize,
    // 线程的句柄 可以多线程共享的加锁的信道接受端
    thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // 工人新建以后会在自己的线程里，持续去获取信道的job
        let thread = thread::spawn(move || loop {
            // 获取receiver的锁，读取一个Job内容
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            // Job 是一个 闭包函数，可以直接 () 使用
            job();
        });

        Worker { id, thread }
    }
}

