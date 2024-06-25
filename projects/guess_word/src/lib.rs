#![allow(dead_code)] // 未使用の警告を抑制
use rand::seq::SliceRandom;
use std::collections::HashSet;

const WORDS: &str = include_str!("words.txt");

pub struct Dictionary {
    words: HashSet<&'static str>,
}

impl Dictionary {
    pub fn new() -> Self {
        let words: HashSet<&str> = WORDS.split("\r\n").collect();
        Self { words }
    }
    pub fn get_random_word(&self) -> String {
        Vec::from_iter(self.words.iter())
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}



