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

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];

        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let root = self.find(self.parent[x]);
            self.parent[x] = root; // 経路圧縮
            root
        }
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        // 小さい集合を大きい集合にくっつける
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];

        true
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

// 0, 1, 2
// 3, 4, 5
// と二次元を１次元的に数える

fn main() {
    let mut sc = Scanner::new();
    let h: usize = sc.next();
    let w: usize = sc.next();
    let q: usize = sc.next();

    let mut uf = UnionFind::new(h * w);
    let mut matrix = vec![vec![false; w]; h];

    let mut output = String::new();

    // uf 用の１次元のインデックスに変換
    let to_uf_index = |r: usize, c: usize| -> usize { (r - 1) * w + (c - 1) };

    for _ in 0..q {
        match sc.next::<usize>() {
            1 => {
                let r: usize = sc.next();
                let c: usize = sc.next();
                let idx_r: usize = r - 1;
                let idx_c: usize = c - 1;

                matrix[idx_r][idx_c] = true;

                // 左
                if idx_c > 0 && matrix[idx_r][idx_c - 1] {
                    uf.union(to_uf_index(r, c), to_uf_index(r, c - 1));
                }
                // 右
                if idx_c < w - 1 && matrix[idx_r][idx_c + 1] {
                    uf.union(to_uf_index(r, c), to_uf_index(r, c + 1));
                }

                // 上
                if idx_r < h - 1 && matrix[idx_r + 1][idx_c] {
                    uf.union(to_uf_index(r, c), to_uf_index(r + 1, c));
                }
                //下
                if idx_r > 0 && matrix[idx_r - 1][idx_c] {
                    uf.union(to_uf_index(r, c), to_uf_index(r - 1, c));
                }
            }
            2 => {
                let ra: usize = sc.next();
                let ca: usize = sc.next();
                let rb: usize = sc.next();
                let cb: usize = sc.next();

                let is_yes =
                    matrix[ra - 1][ca - 1] && uf.same(to_uf_index(ra, ca), to_uf_index(rb, cb));

                output.push_str(if is_yes { "Yes\n" } else { "No\n" });
            }
            _ => unreachable!(),
        }
    }

    println!("{output}")
}
