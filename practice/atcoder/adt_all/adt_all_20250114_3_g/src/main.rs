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

    let r = sc.next::<String>().into_bytes();
    let c = sc.next::<String>().into_bytes();

    let rows = create_rows(n);

    // 外側の vec は a, b, c
    // 内側はそれぞれの行で使用されたか
    let mut result = vec![vec![b'.'; n]; n];

    // それぞれの列の最も上の記号の vec
    let mut first_c_list: Vec<Option<u8>> = vec![None; n];

    let is_yes = dfs(n, &r, &c, &rows, &mut result, 0, &mut first_c_list);

    if is_yes {
        println!("Yes");
        for r in result {
            println!("{}", String::from_utf8(r).unwrap());
        }
    } else {
        println!("No");
    }
}

fn dfs(
    n: usize,
    r: &Vec<u8>,
    c: &Vec<u8>,
    rows: &Vec<Vec<u8>>,
    result: &mut Vec<Vec<u8>>,
    i: usize,
    first_c_list: &mut Vec<Option<u8>>,
) -> bool {
    if i == n {
        return true;
    }

    let first_c_list_prev = first_c_list.clone();

    for row in rows {
        let mut is_ok = true;
        let mut first_r: Option<u8> = None;
        'column: for (j, &char) in row.iter().enumerate() {
            if char == b'.' {
                continue;
            }

            // 行で初めての文字の場合
            if first_r.is_none() {
                if char != r[i] {
                    is_ok = false;
                    break;
                }
                first_r = Some(char);
            }

            // 列で初めての文字の場合
            if first_c_list[j].is_none() {
                if char != c[j] {
                    is_ok = false;
                    break;
                }
                first_c_list[j] = Some(char);
            }

            // 列でユニークな文字かどうか
            for k in 0..n {
                if result[k][j] == char {
                    is_ok = false;
                    break 'column;
                }
            }
        }

        if is_ok {
            result[i] = row.clone();
            if dfs(n, r, c, rows, result, i + 1, first_c_list) {
                return true;
            }
        }
        *first_c_list = first_c_list_prev.clone();
        result[i] = vec![b'.'; n];
    }

    false
}

fn create_rows(n: usize) -> Vec<Vec<u8>> {
    let mut rows = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }

                let mut row = vec![b'.'; n];
                row[i] = b'A';
                row[j] = b'B';
                row[k] = b'C';
                rows.push(row);
            }
        }
    }

    rows
}
