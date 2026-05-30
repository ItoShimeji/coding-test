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

struct Circle {
    x: isize,
    y: isize,
    r: usize,
}

impl Circle {
    // この中では浮動小数の誤差を避けるためにすべて2乗で計算する。
    fn intersects(&self, other: &Circle) -> bool {
        // isize のため、座標の引き算は可能だが、abs_diff で isize -> usize 変換が行われるため、
        // r といっしょに扱いやすいため、使用している
        let distance = self.x.abs_diff(other.x).pow(2) + self.y.abs_diff(other.y).pow(2);
        let r_sum = (self.r + other.r).pow(2);
        let r_diff = self.r.abs_diff(other.r).pow(2);

        // 距離が半径の合計よりも大きい or
        // 片方が他の円の内部に存在
        if distance < r_diff || r_sum < distance {
            return false;
        }

        true
    }

    fn is_on_circle(&self, x: isize, y: isize) -> bool {
        self.x.abs_diff(x).pow(2) + self.y.abs_diff(y).pow(2) == self.r.pow(2)
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.next();
    let s_x: isize = sc.next();
    let s_y: isize = sc.next();
    let t_x: isize = sc.next();
    let t_y: isize = sc.next();

    let circles: Vec<Circle> = (0..n)
        .map(|_| Circle {
            x: sc.next(),
            y: sc.next(),
            r: sc.next(),
        })
        .collect();

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];

    for i in 0..n {
        for j in i + 1..n {
            if circles[i].intersects(&circles[j]) {
                adj[i].push(j);
                adj[j].push(i);
            }
        }
    }

    let mut start: Option<usize> = None;
    let mut goal: Option<usize> = None;

    for (i, c) in circles.iter().enumerate() {
        if start.is_none() && c.is_on_circle(s_x, s_y) {
            start = Some(i);
        }
        if goal.is_none() && c.is_on_circle(t_x, t_y) {
            goal = Some(i);
        }
    }

    let start = start.unwrap();
    let goal = goal.unwrap();

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);
    let mut is_used = vec![false; n];
    is_used[start] = true;

    let mut is_yes = false;
    while let Some(c) = queue.pop_front() {
        if c == goal {
            is_yes = true;
            break;
        }

        for &a in &adj[c] {
            if !is_used[a] {
                is_used[c] = true;
                queue.push_back(a);
            }
        }
    }

    println!("{}", if is_yes { "Yes" } else { "No" });
}
