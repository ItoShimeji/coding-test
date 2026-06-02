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
    let n: usize = sc.next();
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let a: usize = sc.next();
        let b: usize = sc.next();

        adj_list[a - 1].push(b);
        adj_list[b - 1].push(a);
    }

    let (id, _) = bfs(&adj_list, 1);
    let (_, d) = bfs(&adj_list, id);

    println!("{}", d + 1);
}

fn bfs(adj_list: &[Vec<usize>], start: usize) -> (usize, usize) {
    let n = adj_list.len();

    let mut used: Vec<Vec<bool>> = (0..n)
        .map(|idx_a| vec![false; adj_list[idx_a].len()])
        .collect();

    let mut d: Vec<usize> = vec![0; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(start);

    while let Some(a) = queue.pop_front() {
        let idx_a = a - 1;
        for (i, &b) in adj_list[idx_a].iter().enumerate() {
            let idx_b = b - 1;
            if used[idx_a][i] {
                continue;
            }
            if d[idx_b] < d[idx_a] + 1 {
                d[idx_b] = d[idx_a] + 1;
                used[idx_a][i] = true;

                for (j, &c) in adj_list[b - 1].iter().enumerate() {
                    if c == a {
                        used[b - 1][j] = true;
                        break;
                    }
                }
                queue.push_back(b);
            }
        }
    }

    d.iter()
        .enumerate()
        .map(|(i, &x)| (i + 1, x))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
}
