use std::collections::HashSet;
use std::io::{self, Read};

type Coord = (i32, i32, i32);

fn parse_coord(l: &str) -> Coord {
    let mut parts = l.split(",");

    (
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let sides = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let lava: HashSet<Coord> = input.trim().split("\n").map(|l| parse_coord(l)).collect();

    let exposed_sides = lava
        .iter()
        .flat_map(|c| sides.map(|s| (c.0 + s.0, c.1 + s.1, c.2 + s.2)))
        .filter(|c| !lava.contains(c))
        .count();
    println!("Part 1: {}", exposed_sides);

    let lower_bound = (
        lava.iter().map(|c| c.0).min().unwrap() - 1,
        lava.iter().map(|c| c.1).min().unwrap() - 1,
        lava.iter().map(|c| c.2).min().unwrap() - 1,
    );
    let upper_bound = (
        lava.iter().map(|c| c.0).max().unwrap() + 1,
        lava.iter().map(|c| c.1).max().unwrap() + 1,
        lava.iter().map(|c| c.2).max().unwrap() + 1,
    );

    let mut stack = vec![lower_bound];
    let mut visited = HashSet::<Coord>::new();

    while let Some(curr) = stack.pop() {
        if curr.0 < lower_bound.0
            || curr.1 < lower_bound.1
            || curr.2 < lower_bound.2
            || curr.0 > upper_bound.0
            || curr.1 > upper_bound.1
            || curr.2 > upper_bound.2
        {
            continue;
        }
        if visited.contains(&curr) {
            continue;
        }
        if lava.contains(&curr) {
            continue;
        }

        visited.insert(curr);

        for (dx, dy, dz) in sides {
            stack.push((curr.0 + dx, curr.1 + dy, curr.2 + dz));
        }
    }

    let exposed_sides = lava
        .iter()
        .flat_map(|c| sides.map(|s| (c.0 + s.0, c.1 + s.1, c.2 + s.2)))
        .filter(|c| !lava.contains(c) && visited.contains(c))
        .count();
    println!("Part 2: {}", exposed_sides);
}
