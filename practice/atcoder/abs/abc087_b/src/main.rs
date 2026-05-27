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

// dp で書くなら
// use std::io::{self, Read};

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_to_string(&mut input).unwrap();

//     let nums: Vec<usize> = input
//         .split_whitespace()
//         .map(|s| s.parse().unwrap())
//         .collect();

//     let a = nums[0];
//     let b = nums[1];
//     let c = nums[2];
//     let x = nums[3];

//     let coins = vec![(500, a), (100, b), (50, c)];

//     let mut dp = vec![0usize; x + 1];
//     dp[0] = 1;

//     for (value, limit) in coins {
//         let mut next_dp = vec![0usize; x + 1];

//         for amount in 0..=x {
//             if dp[amount] == 0 {
//                 continue;
//             }

//             for count in 0..=limit {
//                 let next_amount = amount + value * count;

//                 if next_amount <= x {
//                     next_dp[next_amount] += dp[amount];
//                 }
//             }
//         }

//         dp = next_dp;
//     }

//     println!("{}", dp[x]);
// }
