use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type WorkerFunction<T, U> = Box<dyn (FnOnce(T) -> U) + Send + 'static>;

struct Pool<T: Sync + Send + 'static, U: Sync + Send> {
    workers: Vec<Worker<T, U>>,
    worker_sender: mpsc::Sender<T>,
    pool_receiver: Arc<Mutex<mpsc::Receiver<U>>>,
}

impl<T: Sync + Send + 'static, U: Sync + Send> Pool<T, U> {
    fn new(size: usize, function: WorkerFunction<T, U>) -> Pool<T, U> {
        let (pool_sender, pool_receiver): (mpsc::Sender<U>, mpsc::Receiver<U>) = mpsc::channel();
        let pool_receiver = Arc::new(Mutex::new(pool_receiver));

        let workers = Vec::with_capacity(size);

        let (worker_sender, worker_receiver) = mpsc::channel();
        let worker_receiver = Arc::new(Mutex::new(worker_receiver));
        for _ in 0..size {
            workers.push(Worker::new(function, pool_sender, worker_receiver));
        }

        Pool {
            workers: workers,
            worker_sender: worker_sender,
            pool_receiver: pool_receiver,
        }
    }

    fn process(&self, input: T) {
        self.worker_sender.send(input);
    }

    fn result(&self) -> U {
        self.pool_receiver.lock().unwrap().recv().unwrap()
    }
}

struct Worker<T: Sync + Send + 'static, U: Sync + Send> {

    sender: mpsc::Sender<U>,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
}

impl<T: Sync + Send, U: Sync + Send> Worker<T, U> {
    fn new(
        function: WorkerFunction<T, U>,
        sender: mpsc::Sender<U>,
        receiver: Arc<Mutex<mpsc::Receiver<T>>>,
    ) -> Worker<T, U> {
        let sender = sender.clone();

        thread::spawn(move || {
            while let Ok(data) = receiver.lock().unwrap().recv() {
                sender.send(function(data));
            }
        });

        Self {
            sender: sender,
            receiver: receiver,
        }
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool =
        Pool::<String, Box<HashMap<char, usize>>>::new(worker_count, Box::new(line_frequency));

    let mut i = 0;

    for line in input {
        i += 1;
        pool.process(line.to_string());
    }

    let mut result = HashMap::new();

    while let line_result = *pool.result() {
        for (k, v) in line_result.iter() {
            *result.entry(*k).or_insert(0) += v;
        }
        i -= 1;
        if i == 0 {
            break;
        }
    }

    result
}

fn line_frequency(input: String) -> Box<HashMap<char, usize>> {
    let mut result = HashMap::new();
    for c in input.chars() {
        if !c.is_alphabetic() {
            continue;
        }
        *result.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }
    Box::new(result)
}
