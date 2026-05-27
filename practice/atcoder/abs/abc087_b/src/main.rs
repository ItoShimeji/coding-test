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
    let c: usize = sc.next();
    let x: usize = sc.next();
    let mut count = 0;

    for i_a in 0..=a {
        for i_b in 0..=b {
            for i_c in 0..=c {
                let sum = 500 * i_a + 100 * i_b + 50 * i_c;
                if sum > x {
                    break;
                } else if sum == x {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("{count}");
}
