
use std::collections::HashMap;
use std::{fs, thread};
use std::sync::mpsc;
use std::time::Instant;
use regex::Regex;

const ROUNDS: usize = 100000;
const THREADS: usize = 1000;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("../text.txt")
        .expect("Something went wrong reading the file");

    let (tx, rx) = mpsc::channel();
    let word_regex : Regex = Regex::new(r"[\W,.?!]+").unwrap();
    for _i in 0..THREADS {
        let tx1 = tx.clone();
        let c1 = contents.clone();
        let r1 = word_regex.clone();
        thread::spawn(move || {
            const ITER: usize = ROUNDS / THREADS;
            for _j in 0..ITER {
                tx1.send(parse(&c1, &r1)).unwrap();
            }
        });
    }
    println!("Started in {:?}", start.elapsed());

    // get first result
    let mut res = rx.recv().unwrap();
    println!("First result received in {:?}", start.elapsed());
    // get the rest of results
    for _k in 1..ROUNDS {
        res = rx.recv().unwrap();
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    println!("Top words: {:?}", res.top_words);
    println!("Top letters: {:?}", res.top_letters);
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
    CountResult{ top_words: words.top(10), top_letters: letters.top(10)}
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
        match self._map.get_mut(key) {
            Some(val) => *val += 1,
            _ => {self._map.insert(key.to_owned(), 1);}
        };
    }

    fn top(&self, limit: usize) -> Vec<(String, i32)> {
        let mut values: Vec<(&str, i32)> = self._map.iter()
            .map(|(k, v)|(k.as_str(),*v)).collect::<Vec<(&str, i32)>>();
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
