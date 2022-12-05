use std::io::{self, BufRead};

type CrateStack = Vec<char>;
type CrateStacks = Vec<CrateStack>;

fn main() {
    let mut stacks = CrateStacks::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        if line.contains('[') {
            let mut cur_index = 0;
            while cur_index * 4 < line.len() {
                if line.chars().nth(cur_index * 4).unwrap() != '[' {
                    cur_index += 1;
                    continue;
                }

                while cur_index >= stacks.len() {
                    stacks.push(CrateStack::new());
                }

                let stack = &mut stacks[cur_index];
                let cr = line.chars().nth(cur_index * 4 + 1).unwrap();
                stack.insert(0, cr);
                cur_index += 1;
            }
        }

        if line.starts_with("move") {
            let parts: Vec<_> = line.split(' ').collect();

            let count = parts[1].parse::<usize>().unwrap();
            let from = parts[3].parse::<usize>().unwrap();
            let dst = parts[5].parse::<usize>().unwrap();

            let dst_top = stacks[dst - 1].len();
            for _ in 0..count {
                let c = stacks[from - 1].pop().unwrap();
                stacks[dst - 1].insert(dst_top, c);
            }
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    println!("");
}
