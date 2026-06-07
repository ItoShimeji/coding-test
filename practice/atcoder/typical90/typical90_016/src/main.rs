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
    let a: usize = sc.next();
    let b: usize = sc.next();
    let c: usize = sc.next();

    let mut coins = [a, b, c];
    coins.sort();

    let mut min_count = usize::MAX;
    for i in 0..10000 {
        for j in 0..10000 - i {
            let sum = i * coins[0] + j * coins[1];
            if n < sum {
                break;
            }

            if (n - sum).is_multiple_of(coins[2]) {
                let k = (n - sum) / coins[2];
                min_count = min_count.min(i + j + k);

                // coins は昇順に sort 済みのため、iを固定したまま j を大きくしたとしても今回の最小値よりも小さくなることはない
                // そのため、内側のループを脱出できる
                break;
            }
        }
    }

    println!("{min_count}");
}
