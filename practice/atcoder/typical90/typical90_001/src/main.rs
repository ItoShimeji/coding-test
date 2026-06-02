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
    let l: usize = sc.next();
    let k: usize = sc.next();

    // 切れる位置に右端も含める
    let mut a_list: Vec<usize> = Vec::with_capacity(n + 1);
    for _ in 0..n {
        a_list.push(sc.next());
    }
    a_list.push(l);

    // 長さ x 以上のピースを K+1 個作れるかは単調である
    // この関数では実際にそのような切り方を見つけるのではなく、作れるかどうかのみを判定
    let can_make = |score: usize| -> bool {
        let mut pieces = 0;
        let mut last_cut = 0;

        for &position in &a_list {
            if position - last_cut >= score {
                pieces += 1;
                last_cut = position;
            }
        }

        pieces >= k + 1
    };

    let mut ok = 0;
    let mut ng = l + 1;

    while ng - ok > 1 {
        let score = (ok + ng) / 2;

        // ここで ok + 2 == ng の状態で処理を行うと回答が決まる
        if can_make(score) {
            ok = score;
        } else {
            ng = score;
        }
    }

    println!("{}", ok);
}
