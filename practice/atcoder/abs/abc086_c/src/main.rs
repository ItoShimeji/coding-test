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

struct Coord {
    t: usize,
    x: usize,
    y: usize,
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.next();

    let plan: Vec<Coord> = (0..=n)
        .map(|i| {
            if i == 0 {
                Coord { t: 0, x: 0, y: 0 }
            } else {
                Coord {
                    t: sc.next(),
                    x: sc.next(),
                    y: sc.next(),
                }
            }
        })
        .collect();

    let mut is_yes = true;
    for i in 0..n {
        let current = &plan[i];
        let next = &plan[i + 1];
        let displacement_x = next.x.abs_diff(current.x);
        let displacement_y = next.y.abs_diff(current.y);
        let duration = next.t - current.t;

        // 最低限必要な時間より時刻の差が小さい場合は不可能
        if duration < displacement_x + displacement_y {
            is_yes = false;
            break;
        }

        // 移動に最低限必要な時間を引いた余分の時間
        let extra = duration - displacement_x - displacement_y;
        if !extra.is_multiple_of(2) {
            is_yes = false;
            break;
        }
    }

    let message = if is_yes { "Yes" } else { "No" };
    println!("{message}");
}
