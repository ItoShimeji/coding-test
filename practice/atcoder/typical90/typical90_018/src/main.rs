use std::{
    f64,
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
    let t = sc.next::<usize>() as f64;
    let l = sc.next::<usize>() as f64;
    let x = sc.next::<usize>() as f64;
    let y = sc.next::<usize>() as f64;
    let q: usize = sc.next();

    // pi 関連の定数
    let pi = f64::consts::PI;
    let frac_pi_2 = f64::consts::FRAC_PI_2;

    // 半径
    let r = 0.5 * l;

    let mut output = String::new();

    for _ in 0..q {
        let e = sc.next::<usize>() as f64;

        // e から計算できる位相の変化
        let phase = 2.0 * pi * e / t;

        // 観覧車上の座標
        let y_e = -r * phase.sin();
        let z_e = r * ((phase - frac_pi_2).sin() + 1.0);

        // 像と観覧車上の直角三角形を考えて、傾きを求める
        let gradient = z_e / (x.powi(2) + (y - y_e).powi(2)).sqrt();

        let deg = gradient.atan().to_degrees();

        output.push_str(format!("{:.12}\n", deg).as_str());
    }

    print!("{output}");
}
