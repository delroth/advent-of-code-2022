use std::collections::HashSet;
use std::io::{self, BufRead};

type Sack = HashSet<char>;

fn parse_sack(contents: String) -> Sack {
    let mut sack = Sack::new();

    for c in contents.chars() {
        sack.insert(c);
    }

    sack
}

fn char_to_prio(c: char) -> u32 {
    match c {
        'a'..='z' => 1 + (c as u32) - ('a' as u32),
        'A'..='Z' => 27 + (c as u32) - ('A' as u32),
        _ => panic!("Invalid char for item: {}", c),
    }
}

fn main() {
    let mut prio_sum = 0;

    let mut lines = io::stdin().lock().lines().peekable();
    while lines.peek().is_some() {
        let chunk = [
            lines.next().unwrap().unwrap(),
            lines.next().unwrap().unwrap(),
            lines.next().unwrap().unwrap(),
        ];
        let sacks = chunk.map(parse_sack);

        let common_first_two: Sack = sacks[0].intersection(&sacks[1]).cloned().collect();
        let common = *common_first_two.intersection(&sacks[2]).next().unwrap();

        prio_sum += char_to_prio(common);
    }

    println!("{}", prio_sum);
}
