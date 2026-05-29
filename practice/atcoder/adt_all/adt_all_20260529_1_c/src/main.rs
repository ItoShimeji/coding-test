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

    let mut a: usize = sc.next();
    let mut b: usize = sc.next();

    let mut is_easy = true;

    // 小さい桁から順に見ていけば良い
    while a > 0 || b > 0 {
        if a % 10 + b % 10 >= 10 {
            is_easy = false;
            break;
        }

        a /= 10;
        b /= 10;
    }

    println!("{}", if is_easy { "Easy" } else { "Hard" });
}
