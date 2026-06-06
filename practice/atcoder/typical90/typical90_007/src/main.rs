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
    let mut a_list: Vec<usize> = (0..n).map(|_| sc.next()).collect();
    a_list.sort();

    let q: usize = sc.next();

    'outer: for _ in 0..q {
        let b: usize = sc.next();

        let mut left = 0;
        let mut right = n - 1;

        if b <= a_list[left] {
            println!("{}", a_list[left] - b);
            continue;
        }
        if a_list[right] <= b {
            println!("{}", b - a_list[right]);
            continue;
        }

        while left + 1 < right {
            let mid = left + (right - left) / 2;

            if b < a_list[mid] {
                right = mid;
            } else if b == a_list[mid] {
                println!("0");
                continue 'outer;
            } else {
                left = mid;
            }
        }

        let b_to_mid = b.abs_diff(a_list[left + 1]);
        let b_to_left = b.abs_diff(a_list[left]);

        println!("{}", b_to_mid.min(b_to_left));
    }
}
