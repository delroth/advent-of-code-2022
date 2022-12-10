use std::io::{self, BufRead};

fn main() {
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut tokens = line.split(" ");

        let instr = tokens.next().unwrap();
        let (next_cycle, next_x) = match instr {
            "noop" => (cycle + 1, x),
            "addx" => {
                let imm = tokens.next().unwrap().parse::<i32>().unwrap();
                (cycle + 2, x + imm)
            }
            _ => panic!("Unknown instr {}", instr),
        };

        while cycle < next_cycle {
            let cx = cycle % 40;

            if cx == x || cx == (x - 1) || cx == (x + 1) {
                print!("#");
            } else {
                print!(".");
            }

            cycle += 1;
            if cycle % 40 == 0 {
                println!("");
            }
        }

        x = next_x;
    }
}
