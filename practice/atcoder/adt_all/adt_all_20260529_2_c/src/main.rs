use std::io::{self, Read};

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
    let n: usize = sc.next();
    let mut is_called = vec![false; n];

    for i in 0..n {
        let a: usize = sc.next();
        if !is_called[i] {
            is_called[a - 1] = true;
        }
    }

    let is_not_called = is_called
        .iter()
        .enumerate()
        .filter(|(_, a)| !**a)
        .map(|(i, _)| i + 1)
        .collect::<Vec<usize>>();

    println!("{}", is_not_called.len());
    println!(
        "{}",
        is_not_called
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
