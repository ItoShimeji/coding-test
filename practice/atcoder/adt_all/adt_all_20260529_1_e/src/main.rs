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

    let mut count = vec![0u8; n];
    let mut result = Vec::with_capacity(n);

    for _ in 0..3 * n {
        let num: usize = sc.next();

        if count[num - 1] == 1 {
            result.push(num);
        }

        count[num - 1] += 1;
    }

    println!(
        "{}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
