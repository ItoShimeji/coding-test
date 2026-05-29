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
    let h: usize = sc.next();
    let w: usize = sc.next();

    let mut s_list: Vec<Vec<u8>> = (0..h).map(|_| sc.next::<String>().into_bytes()).collect();

    for s in &mut s_list {
        let mut j = 0;
        while j < w - 1 {
            if s[j] == b'T' {
                if s[j + 1] == b'T' {
                    s[j] = b'P';
                    s[j + 1] = b'C';
                }

                j += 2;
            } else {
                j += 1;
            }
        }
    }

    for s in &s_list {
        println!("{}", std::str::from_utf8(s).unwrap());
    }
}
