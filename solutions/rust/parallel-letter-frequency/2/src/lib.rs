use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    let mut workers: Vec<thread::JoinHandle<HashMap<char, usize>>> = Vec::new();

    let (min_chunk_size, worker_count) = if worker_count < input.len() {
        (input.len(), 1)
    } else {
        (input.len() / worker_count, worker_count)
    };
    let remainder = input.len() % worker_count;

    let lines: Vec<String> = input.iter().map(|l| l.to_string()).collect();
    let mut lines = lines.into_iter();

    for i in 0..worker_count {
        let chunk_size = if i < remainder {
            min_chunk_size + 1
        } else {
            min_chunk_size
        };
        let chunk: Vec<String> = lines.by_ref().take(chunk_size).collect();
        workers.push(thread::spawn(move || {
            let mut result = HashMap::new();
            for line in chunk {
                for c in line.chars().filter(|c| c.is_alphabetic()) {
                    *result.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                }
            }
            result
        }));
    }

    for worker in workers {
        let chunk_result = worker.join().unwrap();
        for (k, v) in chunk_result {
            *result.entry(k).or_insert(0) += v;
        }
    }

    result
}
