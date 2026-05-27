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
    let y: usize = sc.next();

    let mut result: (isize, isize, isize) = (-1, -1, -1);

    // 最大で n + 2 C 2 回の試行となる
    'outer: for n_eikichi in 0..=n {
        // この時点で超えている場合は早期に脱出
        if n_eikichi * 10_000 > y {
            break;
        }
        for n_umeko in 0..=(n - n_eikichi) {
            let n_shibasaburo = n - n_eikichi - n_umeko;

            if n_eikichi * 10_000 + n_umeko * 5_000 + 1_000 * n_shibasaburo == y {
                result = (n_eikichi as isize, n_umeko as isize, n_shibasaburo as isize);
                break 'outer;
            }
        }
    }

    println!("{} {} {}", result.0, result.1, result.2);
}
