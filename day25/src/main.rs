use std::io::{self, BufRead};

fn from_snafu(s: &str) -> i64 {
    let mut res = 0;

    for c in s.chars() {
        let c = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Invalid character: {c}"),
        };
        res = res * 5 + c;
    }

    res
}

fn to_snafu(n: i64) -> String {
    if n == 0 {
        String::new()
    } else {
        match n % 5 {
            0 => to_snafu(n / 5) + "0",
            1 => to_snafu(n / 5) + "1",
            2 => to_snafu(n / 5) + "2",
            3 => to_snafu((n + 2) / 5) + "=",
            4 => to_snafu((n + 1) / 5) + "-",
            _ => panic!("???"),
        }
    }
}

fn main() {
    let sum: i64 = io::stdin()
        .lock()
        .lines()
        .map(|l| from_snafu(&l.unwrap()))
        .sum();

    println!("{sum} -> {}", to_snafu(sum))
}
