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
    let a = sc.next::<usize>();
    let b = sc.next::<usize>();
    let c = sc.next::<usize>();

    let is_yes = if a.is_power_of_two() && c.is_power_of_two() {
        let log_a = a.trailing_zeros() as usize;
        let log_c = c.trailing_zeros() as usize;

        // ここでオーバーフローするなら log_a を n * log_c が超えたときに処理をやめるようにする
        log_a < b * log_c
    } else {
        let a = a as f64;
        let b = b as f64;
        let c = c as f64;

        a.log2() < b * c.log2()
    };

    println!("{}", if is_yes { "Yes" } else { "No" });
}
