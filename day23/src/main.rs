use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Coord = (i32, i32);
type Elves = HashSet<Coord>;
type Moves = HashMap<Coord, Coord>;

fn round(num: usize, elves: &mut Elves) -> bool {
    let mut moves: Moves = Moves::new();
    let mut invalid_moves: Elves = Elves::new();

    for &(ex, ey) in elves.iter() {
        let mut neighs = Vec::<bool>::new();
        for (dx, dy) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ] {
            neighs.push(elves.contains(&(ex + dx, ey + dy)));
        }

        if !neighs.iter().any(|&x| x) {
            continue;
        }

        let move_options = [
            ((-1, -1), (0, -1), (1, -1), (0, -1)),
            ((-1, 1), (0, 1), (1, 1), (0, 1)),
            ((-1, -1), (-1, 0), (-1, 1), (-1, 0)),
            ((1, -1), (1, 0), (1, 1), (1, 0)),
        ];

        let mut dst = None;
        for i in 0..4 {
            let ((n1x, n1y), (n2x, n2y), (n3x, n3y), (dx, dy)) = move_options[(num + i) % 4];

            if !elves.contains(&(ex + n1x, ey + n1y))
                && !elves.contains(&(ex + n2x, ey + n2y))
                && !elves.contains(&(ex + n3x, ey + n3y))
            {
                dst = Some((ex + dx, ey + dy));
                break;
            }
        }

        if let Some(dst) = dst {
            if !invalid_moves.contains(&dst) {
                if moves.contains_key(&dst) {
                    moves.remove(&dst);
                    invalid_moves.insert(dst);
                } else {
                    moves.insert(dst, (ex, ey));
                }
            }
        }
    }

    for (&k, &v) in moves.iter() {
        elves.remove(&v);
        elves.insert(k);
    }

    moves.len() != 0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut elves: Elves = input
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let mut i = 0;
    while round(i, &mut elves) {
        i += 1;
    }

    let minx = elves.iter().map(|(x, _)| x).min().unwrap();
    let maxx = elves.iter().map(|(x, _)| x).max().unwrap();
    let miny = elves.iter().map(|(_, y)| y).min().unwrap();
    let maxy = elves.iter().map(|(_, y)| y).max().unwrap();

    let result = (maxx - minx + 1) * (maxy - miny + 1) - (elves.len() as i32);
    println!("{result} at round {}", i + 1);
}
