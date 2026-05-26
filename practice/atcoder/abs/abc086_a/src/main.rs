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
    let a: usize = sc.next();
    let b: usize = sc.next();

    match (a % 2, b % 2) {
        (0, 0) => {
            println!("Even");
        }
        (0, 1) => {
            println!("Even");
        }
        (1, 0) => {
            println!("Even");
        }
        (_, _) => {
            println!("Odd");
        }
    }
}
