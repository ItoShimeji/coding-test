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

    let mut min = usize::MAX;
    for _ in 0..n {
        let x: usize = sc.next();
        let count = even(x);

        min = if count < min { count } else { min };
    }

    println!("{min}");
}

fn even(x: usize) -> usize {
    if x.is_multiple_of(2) {
        even(x / 2) + 1
    } else {
        0
    }
}
