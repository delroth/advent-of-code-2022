use std::collections::HashMap;
use std::io::{self, Read};

type Coord = (i32, i32);

#[derive(PartialEq)]
enum Cell {
    Empty,
    Wall,
}

type Grid = HashMap<Coord, Cell>;

#[derive(Debug)]
enum Instr {
    Forward(i32),
    Left,
    Right,
}

type Instrs = Vec<Instr>;

const FACINGS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const FACING_RIGHT: usize = 0;
const FACING_DOWN: usize = 1;
const FACING_LEFT: usize = 2;
const FACING_UP: usize = 3;

fn parse_map(s: &str) -> Grid {
    s.split("\n")
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '.' || c == '#')
                .map(move |(x, c)| {
                    (
                        (x as i32, y as i32),
                        if c == '.' { Cell::Empty } else { Cell::Wall },
                    )
                })
        })
        .collect()
}

fn parse_instrs(mut s: &str) -> Instrs {
    let mut instrs: Instrs = Vec::new();

    while !s.is_empty() {
        let num_end = match (s.find('L'), s.find('R')) {
            (None, None) => s.len(),
            (Some(x), None) | (None, Some(x)) => x,
            (Some(x), Some(y)) => x.min(y),
        };

        let num = s[..num_end].parse::<i32>().unwrap();
        instrs.push(Instr::Forward(num));

        s = &s[num_end..];
        if !s.is_empty() {
            instrs.push(match s.chars().next().unwrap() {
                'L' => Instr::Left,
                'R' => Instr::Right,
                _ => panic!("Unknown turn direction found"),
            });
            s = &s[1..];
        }
    }

    instrs
}

fn wrap_around(x: &mut i32, y: &mut i32, facing: &mut usize) {
    // 1
    if *facing == FACING_LEFT && *x == -1 && (150..200).contains(y) {
        *x = 50 + (*y % 50);
        *y = 0;
        *facing = FACING_DOWN;
    // 2
    } else if *facing == FACING_LEFT && *x == -1 && (100..150).contains(y) {
        *y = 49 - (*y % 50);
        *x = 50;
        *facing = FACING_RIGHT;
    // 3
    } else if *facing == FACING_UP && (0..50).contains(x) && *y == 99 {
        *y = 50 + (*x % 50);
        *x = 50;
        *facing = FACING_RIGHT;
    // 4
    } else if *facing == FACING_LEFT && *x == 49 && (50..100).contains(y) {
        *x = *y % 50;
        *y = 100;
        *facing = FACING_DOWN;
    // 5
    } else if *facing == FACING_LEFT && *x == 49 && (0..50).contains(y) {
        *y = 149 - (*y % 50);
        *x = 0;
        *facing = FACING_RIGHT;
    // 6
    } else if *facing == FACING_UP && (50..100).contains(x) && *y == -1 {
        *y = 150 + (*x % 50);
        *x = 0;
        *facing = FACING_RIGHT;
    // 7
    } else if *facing == FACING_UP && (100..150).contains(x) && *y == -1 {
        *x = *x % 50;
        *y = 199;
        *facing = FACING_UP;
    // 8
    } else if *facing == FACING_RIGHT && *x == 150 && (0..50).contains(y) {
        *x = 99;
        *y = 149 - (*y % 50);
        *facing = FACING_LEFT;
    // 9
    } else if *facing == FACING_DOWN && (100..150).contains(x) && *y == 50 {
        *y = 50 + (*x % 50);
        *x = 99;
        *facing = FACING_LEFT;
    // 10
    } else if *facing == FACING_RIGHT && *x == 100 && (50..100).contains(y) {
        *x = 100 + (*y % 50);
        *y = 49;
        *facing = FACING_UP;
    // 11
    } else if *facing == FACING_RIGHT && *x == 100 && (100..150).contains(y) {
        *y = 49 - (*y % 50);
        *x = 149;
        *facing = FACING_LEFT;
    // 12
    } else if *facing == FACING_DOWN && (50..100).contains(x) && *y == 150 {
        *y = 150 + (*x % 50);
        *x = 49;
        *facing = FACING_LEFT;
    // 13
    } else if *facing == FACING_RIGHT && *x == 50 && (150..200).contains(y) {
        *x = 50 + (*y % 50);
        *y = 149;
        *facing = FACING_UP;
    // 14
    } else if *facing == FACING_DOWN && (0..50).contains(x) && *y == 200 {
        *x = 100 + *x % 50;
        *y = 0;
        *facing = FACING_DOWN;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let map = parse_map(input.split("\n\n").next().unwrap());
    let instrs = parse_instrs(input.split("\n\n").last().unwrap().trim());

    let max_x = map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap();

    let (mut px, mut py) = map.keys().min_by_key(|(x, y)| (y, x)).unwrap();
    let mut facing = 0;

    for instr in instrs {
        println!(
            "Instr: {:?}, facing/position before: {} / {:?}",
            instr,
            facing,
            (px, py)
        );
        match instr {
            Instr::Left => {
                facing = (facing + 3) % 4;
            }
            Instr::Right => {
                facing = (facing + 1) % 4;
            }
            Instr::Forward(count) => {
                for _ in 0..count {
                    let (dx, dy) = FACINGS[facing];
                    let (mut nx, mut ny) = (px + dx, py + dy);
                    let mut nfacing = facing;

                    wrap_around(&mut nx, &mut ny, &mut nfacing);

                    match map.get(&(nx, ny)) {
                        None => panic!("Fail to wrap: x={}, y={}", nx, ny),
                        Some(Cell::Wall) => {
                            println!("Hit wall at {} {}", nx, ny);
                            break;
                        }
                        Some(Cell::Empty) => {
                            px = nx;
                            py = ny;
                            facing = nfacing;
                        }
                    }
                }
            }
        }
        println!("Facing/position after: {} / {:?}", facing, (px, py));
    }

    let res = (py + 1) * 1000 + (px + 1) * 4 + (facing as i32);
    println!("Final result: {}", res);
}
