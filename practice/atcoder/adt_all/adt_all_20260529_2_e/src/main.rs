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
    let a_list: Vec<isize> = (0..n).map(|_| sc.next::<isize>() * 2).collect();
    let mut max_count = 0;

    for mask in 0..1usize << n {
        let mut coord: isize = 1;
        let mut count = 0;

        for (i, &a) in a_list.iter().enumerate() {
            let prev_coord = coord;

            // bit が立っている場合は左に進む
            if is_selected(mask, i) {
                coord -= a;
            } else {
                coord += a;
            }

            // 符号が変わる場合はカウントを増加
            if prev_coord < 0 && 0 < coord || 0 < prev_coord && coord < 0 {
                count += 1;
            }
        }

        max_count = max_count.max(count);
    }

    println!("{max_count}");
}

fn is_selected(mask: usize, i: usize) -> bool {
    (mask & (1usize << i)) != 0
}
