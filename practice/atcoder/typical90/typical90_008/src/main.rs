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

struct Count {
    a: usize,
    at: usize,
    atc: usize,
    atco: usize,
    atcod: usize,
    atcode: usize,
    atcoder: usize,
}

const MOD: usize = 1_000_000_007;

impl Count {
    fn new() -> Self {
        Count {
            a: 0,
            at: 0,
            atc: 0,
            atco: 0,
            atcod: 0,
            atcode: 0,
            atcoder: 0,
        }
    }

    fn new_char(&mut self, c: u8) {
        match c {
            b'a' => self.a += 1,
            b't' => self.at = (self.at + self.a) % MOD,
            b'c' => self.atc = (self.atc + self.at) % MOD,
            b'o' => self.atco = (self.atco + self.atc) % MOD,
            b'd' => self.atcod = (self.atcod + self.atco) % MOD,
            b'e' => self.atcode = (self.atcode + self.atcod) % MOD,
            b'r' => self.atcoder = (self.atcoder + self.atcode) % MOD,
            _ => {}
        }
    }
}

fn main() {
    let mut sc = Scanner::new();
    let _: usize = sc.next();
    let s: String = sc.next();

    let mut count = Count::new();

    for &c in s.as_bytes() {
        count.new_char(c);
    }

    println!("{}", count.atcoder);
}
