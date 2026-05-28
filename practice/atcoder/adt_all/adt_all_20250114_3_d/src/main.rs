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
    let edge_list = (0..n - 1)
        .map(|_| (sc.next::<usize>(), sc.next::<usize>()))
        .collect::<Vec<(usize, usize)>>();

    let first = edge_list[0];
    let second = edge_list[1];

    let root = if first.0 == second.0 || first.0 == second.1 {
        first.0
    } else {
        first.1
    };

    // 与えられる構造は木であるため、2点間の辺の数は1つのため、rootが出てくる回数を単純に
    // 数えて、それが n - 1 になることを示せばよいだけだが、下では調べてしまっている。
    let mut exists = vec![false; n];
    exists[root - 1] = true;
    for &(v1, v2) in &edge_list {
        let another = if v1 == root { v2 } else { v1 };

        exists[another - 1] = true;
    }

    let is_yes = exists.iter().find(|&x| !x).is_none();

    println!("{}", if is_yes { "Yes" } else { "No" });
}
