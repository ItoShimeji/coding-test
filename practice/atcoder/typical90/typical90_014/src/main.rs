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
    let mut a_list: Vec<usize> = (0..n).map(|_| sc.next()).collect();
    a_list.sort();
    let mut b_list: Vec<usize> = (0..n).map(|_| sc.next()).collect();
    b_list.sort();

    let mut sum = 0;

    for i in 0..n {
        sum += a_list[i].abs_diff(b_list[i]);
    }

    println!("{sum}")
}
