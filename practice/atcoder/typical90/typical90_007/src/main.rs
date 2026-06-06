use std::io::{self, Read, Write};

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

    let q: usize = sc.next();

    // 標準出力を取得
    let stdout = io::stdout();
    // 出力先を取得
    let mut out = io::BufWriter::new(stdout.lock());

    for _ in 0..q {
        let b: usize = sc.next();

        let i = a_list.partition_point(|&a| a < b);
        let mut ans = usize::MAX;

        if i < n {
            ans = ans.min(a_list[i].abs_diff(b));
        }
        if i > 0 {
            ans = ans.min(a_list[i - 1].abs_diff(b));
        }

        // buffer を指定して、記録
        writeln!(out, "{}", ans).unwrap();
    }
}
