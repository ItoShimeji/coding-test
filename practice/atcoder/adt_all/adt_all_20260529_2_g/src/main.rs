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

    let mut login_list: Vec<usize> = Vec::with_capacity(n);
    let mut logout_list: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        let a: usize = sc.next();
        let b: usize = sc.next();
        login_list.push(a);
        logout_list.push(a + b);
    }

    // 取り出しやすいように逆順にソート
    login_list.sort_by(|a, b| b.cmp(a));
    logout_list.sort_by(|a, b| b.cmp(a));

    let mut next_login = login_list.pop();
    let mut next_logout = logout_list.pop();
    let mut player_count = 0;
    // 前回人数が変化したときの日にちを記録
    let mut prev = 0;

    let mut result = vec![0; n];

    loop {
        match (next_login, next_logout) {
            (Some(next_login_day), Some(next_logout_day)) => {
                // logout_day はその日を含まなず、その前の日にログアウトしているため、
                // = はつかない
                if next_login_day < next_logout_day {
                    // 誰かがログインした

                    if player_count > 0 {
                        let duration = next_login_day - prev;
                        result[player_count - 1] += duration;
                    }

                    player_count += 1;
                    prev = next_login_day;
                    next_login = login_list.pop();
                } else {
                    // だれかがログアウトした

                    let duration = next_logout_day - prev;
                    result[player_count - 1] += duration;

                    player_count -= 1;
                    prev = next_logout_day;
                    next_logout = logout_list.pop();
                }
            }
            (None, Some(next_logout_day)) => {
                // だれかがログアウトした

                let duration = next_logout_day - prev;
                result[player_count - 1] += duration;

                player_count -= 1;
                prev = next_logout_day;
                next_logout = logout_list.pop();
            }
            _ => break,
        }
    }

    println!(
        "{}",
        result
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
