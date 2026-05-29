use std::{
    collections::VecDeque,
    io::{self, Read},
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

    let a: usize = sc.next();
    let mut digit_list_a: VecDeque<usize> = VecDeque::new();
    split_to_digit(a, &mut digit_list_a);

    let b: usize = sc.next();
    let mut digit_list_b: VecDeque<usize> = VecDeque::new();
    split_to_digit(b, &mut digit_list_b);

    let mut is_easy = true;
    while let (Some(d_a), Some(d_b)) = (digit_list_a.pop_back(), digit_list_b.pop_back()) {
        if d_a + d_b >= 10 {
            is_easy = false;
        }
    }

    println!("{}", if is_easy { "Easy" } else { "Hard" })
}

fn split_to_digit(n: usize, digit_list: &mut VecDeque<usize>) {
    if n < 10 {
        digit_list.push_front(n);
        return;
    }
    digit_list.push_front(n % 10);
    split_to_digit(n / 10, digit_list);
}
