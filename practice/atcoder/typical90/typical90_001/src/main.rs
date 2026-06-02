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

    // 左右の端も含めた切れ目の配列
    let mut a_list: Vec<usize> = Vec::with_capacity(n + 2);
    a_list.push(0);
    for _ in 0..n {
        a_list.push(sc.next());
    }
    a_list.push(l);

    let diffs: Vec<usize> = a_list.windows(2).map(|w| w[1] - w[0]).collect();

    let mut score: usize;
    let mut score_min: usize = a_list.iter().min().copied().unwrap();
    let mut score_max: usize = l;

    'outer: loop {
        score = score_min + (score_max.abs_diff(score_min) / 2);

        // ピースの数
        let mut pieces = 0;
        // 一つずつの切れ目の長さ
        let mut lengh = 0;

        for &a in &diffs {
            if lengh + a <= score {
                lengh += a;
            } else {
                lengh = 0;
                pieces += 1;
            }

            if pieces > k {
                score_min = score + 1;
                continue 'outer;
            }
        }

        pieces += 1;

        if score_min + 1 > score_max {
            score = if pieces == k {
                score_min + 1
            } else {
                score_min
            };
            break;
        }
        score_max = score;
    }

    println!("{}", score);
}
