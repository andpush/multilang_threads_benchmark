
use std::collections::HashMap;
use std::{fs, thread};
use std::str::Chars;
use std::sync::mpsc;
use std::time::Instant;
use regex::Regex;

const ROUNDS: usize = 10000;
const THREADS: usize = 100;

fn main() {

    let contents = fs::read_to_string("../text.txt")
        .expect("Something went wrong reading the file");

    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::with_capacity(THREADS);
    for i in 0..THREADS {
        let tx1 = tx.clone();
        let c1 = contents.clone();
        let handle = thread::spawn(move || {
            let mut result: CountResult = CountResult { top_words: vec![], top_letters: vec![] };
            const ITER: usize = ROUNDS / THREADS;
            let word_regex : Regex = Regex::new(r"[\W,.?!]+").unwrap();
            for i in 0..ITER {
                result = parse(&c1, &word_regex);
            }
            tx1.send(result).unwrap();
        });
        handles.push(handle);
    }
    let start = Instant::now();
    handles.into_iter().for_each(|h| {
        h.join().unwrap()
    });

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);

    let received = rx.recv().unwrap();
    println!("Top words: {:?}", received.top_words);
    println!("Top letters: {:?}", received.top_letters);
}

fn parse(text: &str, word_regex: &Regex) -> CountResult {
    let text = text.to_ascii_lowercase();
    let mut words: Counter = Counter::new();
    let mut letters: Counter = Counter::new();
    word_regex.split(&text)
        .for_each(|word| {
        words.add(word);
        word.chars().for_each(|ch| letters.add(String::from(ch).as_str()));
    });
    return CountResult{ top_words: words.top(10), top_letters: letters.top(10)};
}

#[derive(Debug)]
pub struct Counter {
    _map: HashMap<String, i32>,
}


impl Counter
{
    fn new() -> Counter  {
        Counter { _map: HashMap::new() }
    }

    fn add(&mut self, key: &str) {
        *self._map.entry(key.to_owned()).or_insert(0) += 1;
    }

    fn top(&self, limit: usize) -> Vec<(String, i32)> {
        let mut values: Vec<(&str, i32)> = (&self._map).iter().map(|(k, v)|(k.as_str(),*v)).collect::<Vec<(&str, i32)>>();
        values.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        values.truncate(limit);
        values.iter().map(|(k, v)|(k.to_string(), *v)).collect()
    }
}

#[derive(Debug)]
pub struct CountResult {
    top_words: Vec<(String, i32)>,
    top_letters: Vec<(String, i32)>,
}
