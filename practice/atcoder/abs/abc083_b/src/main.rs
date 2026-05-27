use std::io::{self, Read};

struct Scanner {
    input: Vec<String>,
    index: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let input = input.split_whitespace().map(|s| s.to_string()).collect();

        Scanner { input, index: 0 }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        let value = self.input[self.index].parse::<T>().ok().unwrap();
        self.index += 1;
        value
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.next();
    let a: usize = sc.next();
    let b: usize = sc.next();

    let mut result = 0;
    for num in 1..=n {
        let sum = sum_digit(num);
        if a <= sum && sum <= b {
            result += num;
        }
    }

    println!("{result}");
}

fn sum_digit(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    // 最小桁の値
    let digit = n % 10;
    digit + sum_digit(n / 10)
}
