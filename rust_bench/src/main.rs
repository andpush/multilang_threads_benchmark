
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use regex::Regex;

fn main() {

    let contents = fs::read_to_string("../text.txt")
        .expect("Something went wrong reading the file");
    parse(&contents);
}

fn parse(text: &str) {
    let r = Regex::new(r"[\W,.?!]+").unwrap();
    let s = text.to_ascii_lowercase();
    let mut words: Counter = Counter::new();
    let mut letters: Counter = Counter::new();

    let start = Instant::now();
    r.split(&s).for_each(|word| {
        // println!("{:?}", word);
        words.add(word.to_owned());
        word.chars().for_each(|ch| letters.add(ch.to_string()));
    });
    // let mut wordList:Vec<Entry<&str, i32>> = words.iter()
    let duration = start.elapsed();
    println!("{:?}", duration);
    println!("Top words: {:?}", words.top(10));
    println!("Too letters: {:?}", letters.top(10));
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

    fn add(&mut self, key: String) {
        *self._map.entry(key).or_insert(0) += 1;
    }

    fn top(&self, limit: usize) -> Vec<(&str, i32)> {
        let mut values: Vec<(&str, i32)> = (&self._map).iter().map(|(k, v)|(k.as_str(),*v)).collect::<Vec<(&str, i32)>>();
        values.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));
        values.truncate(limit);
        values
    }
}
