use crate::thread::JoinHandle;
use std::collections::HashMap;
use std::thread;

const CHARS: &'static str = "abcdefghijklmnopqrstuvwxyzéâäàåçêëèïîìæÆôöòûùü";

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 || input.iter().all(|i| i.is_empty()) {
        return HashMap::default();
    }

    let input_as_str: String = input
        .iter()
        .map(|i| i.to_ascii_lowercase().to_string())
        .collect();

    let chunk_size: usize = CHARS.len() / worker_count;
    let mut handles: Vec<JoinHandle<HashMap<char, usize>>> = Vec::default();
    let mut output: HashMap<char, usize> = HashMap::default();

    for worker in 0..worker_count {
        let content = input_as_str.clone();
        let handle = std::thread::spawn(move || {
            let slice = &CHARS[worker * chunk_size..(worker + 1) * chunk_size];
            let mut out: HashMap<char, usize> = HashMap::default();

            content
                .chars()
                .filter(|c| slice.contains(*c))
                .for_each(|c| *out.entry(c).or_insert(0) += 1);

            out
        });

        handles.push(handle);
    }

    for handle in handles {
        output.extend(handle.join().unwrap());
    }

    output
}