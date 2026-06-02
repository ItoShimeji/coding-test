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
    if !n.is_multiple_of(2) {
        return;
    }
    let h = n / 2;

    let s = String::new();
    dfs(s, h, h);
}

fn dfs(s: String, rest_open: usize, rest_close: usize) {
    if rest_open == 0 && rest_close == 0 {
        println!("{s}");
        return;
    }

    if rest_open > 0 {
        let mut s_plus_open = s.clone();
        s_plus_open.push('(');
        dfs(s_plus_open, rest_open - 1, rest_close);
    }

    if rest_open < rest_close {
        let mut s_plus_close = s.clone();
        s_plus_close.push(')');
        dfs(s_plus_close, rest_open, rest_close - 1);
    }
}

// ((((())))) -> 5 5 5 5 5 -> 5 0 0 0 0
// (((()()))) -> 4 5 5 5 5 -> 4 1 0 0 0
// (((())())) -> 4 4 5 5 5 -> 4 0 1 0 0
// ((()(()))) -> 3 5 5 5 5 -> 3 2 0 0 0
// ((()()())) -> 3 4 5 5 5 -> 3 1 1 0 0
// ((())(())) -> 3 3 5 5 5 -> 3 0 2 0 0
// ()()()(()) -> 1 2 3 4 5 -> 1 1 1 2 0
// ()()()()() -> 1 2 3 4 5 -> 1 1 1 1 1
