use std::{
    io::{self, Read},
    usize,
};

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
    let players: Vec<(usize, usize)> = (0..n).map(|_| (sc.next(), sc.next())).collect();

    let mut min_day = usize::MAX;
    let mut max_day = 1;

    for &p in &players {
        min_day = min_day.min(p.0);
        max_day = max_day.max(p.0 + p.1);
    }

    let mut day_counts = vec![0; n];

    for i in min_day..max_day {
        let mut online_count: usize = 0;
        for &p in &players {
            if p.0 <= i && i < p.0 + p.1 {
                online_count += 1;
            }
        }

        if online_count != 0 {
            day_counts[online_count - 1] += 1;
        }
    }

    println!(
        "{}",
        day_counts
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
