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
    let mut s: Vec<u8> = sc.next::<String>().into_bytes();

    let mut i = 0;
    let mut w_run_len = 0;
    while i < s.len() - 1 {
        if s[i] != b'W' {
            w_run_len = 0;
            i += 1;
            continue;
        } else {
            w_run_len += 1;

            if s[i + 1] == b'A' {
                // i - w_run_len + 1 が A となり、
                // そこから、 i + 1 までは C で埋める
                s[i + 1 - w_run_len] = b'A';
                for j in (i + 2 - w_run_len)..=(i + 1) {
                    s[j] = b'C';
                }

                w_run_len = 0;
                i += 2;
            } else {
                i += 1;
            }
        }
    }

    println!("{}", String::from_utf8(s).unwrap());
}
