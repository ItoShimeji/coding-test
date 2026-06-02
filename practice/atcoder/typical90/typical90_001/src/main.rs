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
    let l: usize = sc.next();
    let k: usize = sc.next();

    // 左右の端も含めた切れ目の配列
    let mut a_list: Vec<usize> = Vec::with_capacity(n + 2);
    a_list.push(0);
    for _ in 0..n {
        a_list.push(sc.next());
    }
    a_list.push(l);

    // それぞれの切れ目を使用するかどうかのフラグ
    let mut flag = vec![true; n + 2];

    // ある切れ目を取り除いたときに、従来の切れ目の左右の切れ目
    // からなる領域の長さの最小
    let mut min_len = usize::MAX;

    for _ in 0..(n - k) {
        min_len = usize::MAX;
        let mut min_border = 1;

        // 端の切れ目以外を走査
        for i in 1..=n {
            if !flag[i] {
                continue;
            }

            // 新領域の左の border の位置
            let x = {
                let mut left = i - 1;
                while !flag[left] {
                    left -= 1;
                }
                a_list.get(left).copied().unwrap()
            };

            // 新領域の右の border の位置
            let y = {
                let mut right = i + 1;
                while !flag[right] {
                    right += 1
                }
                a_list.get(right).copied().unwrap()
            };

            if y - x < min_len {
                min_len = y - x;
                min_border = i;
            }
        }
        flag[min_border] = false;
    }

    println!("{min_len}");
}
