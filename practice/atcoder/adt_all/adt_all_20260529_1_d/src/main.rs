use std::{
    collections::VecDeque,
    io::{self, Read},
};

struct Scanner {
    input: Vec<u8>,
    index: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();

        Scanner { input, index: 0 }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        while self.index < self.input.len() && self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        let start = self.index;
        while self.index < self.input.len() && !self.input[self.index].is_ascii_whitespace() {
            self.index += 1;
        }

        std::str::from_utf8(&self.input[start..self.index])
            .unwrap()
            .parse::<T>()
            .ok()
            .unwrap()
    }
}

fn main() {
    let mut sc = Scanner::new();
    let q: usize = sc.next();

    let mut queue: VecDeque<usize> = VecDeque::new();

    let mut has_printed = false;
    for _ in 0..q {
        let q_type: usize = sc.next();

        match q_type {
            1 => queue.push_back(sc.next::<usize>()),
            2 => {
                has_printed = true;
                println!("{}", queue.pop_front().unwrap());
            }
            _ => unreachable!(),
        }
    }

    if !has_printed {
        println!()
    }
}
