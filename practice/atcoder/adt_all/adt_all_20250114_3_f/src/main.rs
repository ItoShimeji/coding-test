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
    let t: usize = sc.next();

    let a_list: Vec<usize> = (0..n).map(|_| sc.next::<usize>()).collect();
    let sum: usize = a_list.iter().sum();

    let mut extra = t % sum;

    for (i, &a) in a_list.iter().enumerate() {
        if extra < a {
            println!("{} {}", i + 1, extra);
            break;
        } else {
            extra -= a;
        }
    }
}
