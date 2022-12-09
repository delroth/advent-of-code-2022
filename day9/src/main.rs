use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let mut nodes = vec![(0, 0); 10];
    let mut visited = HashSet::<(i32, i32)>::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..count {
            match dir {
                "R" => nodes[0].0 += 1,
                "L" => nodes[0].0 -= 1,
                "U" => nodes[0].1 += 1,
                "D" => nodes[0].1 -= 1,
                _ => panic!("Unknown direction: {}", dir),
            }

            for i in 1..nodes.len() {
                let dx: i32 = nodes[i - 1].0 - nodes[i].0;
                let dy: i32 = nodes[i - 1].1 - nodes[i].1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    if dx != 0 {
                        nodes[i].0 += dx / dx.abs();
                    }
                    if dy != 0 {
                        nodes[i].1 += dy / dy.abs();
                    }
                }
            }

            visited.insert((nodes.last().unwrap().0, nodes.last().unwrap().1));
        }
    }

    println!("Total coords seen: {}", visited.len());
}
