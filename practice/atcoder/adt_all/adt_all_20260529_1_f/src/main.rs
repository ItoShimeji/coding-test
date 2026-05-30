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
    let k: usize = sc.next();

    let s_list: Vec<Vec<u8>> = (0..n).map(|_| sc.next::<String>().into_bytes()).collect();

    let mut max = 0;
    let mut counts = vec![0usize; 26];

    for mask in 0..1usize << n {
        counts = vec![0; 26];
        for (i, s) in s_list.iter().enumerate() {
            if is_selected(mask, i) {
                add_to_counts(&mut counts, s);
            }
        }

        let result = count(k, &counts);
        max = if max < result { result } else { max };
    }

    println!("{max}");
}

fn count(k: usize, counts: &Vec<usize>) -> usize {
    counts.iter().filter(|&&c| c == k).count()
}

fn is_selected(mask: usize, i: usize) -> bool {
    (mask & (1usize << i)) != 0
}

fn add_to_counts(counts: &mut Vec<usize>, s: &Vec<u8>) {
    for &c in s {
        counts[c as usize - 97] += 1;
    }
}
