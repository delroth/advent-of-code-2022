use std::collections::HashSet;
use std::io::{self, Read};

type Coord = (i32, i32);
type Grid = HashSet<Coord>;

fn floor_start_line(g: &Grid) -> i32 {
    g.iter().map(|(x, y)| y).max().unwrap() + 2
}

fn add_sand(g: &mut Grid, floor: i32) -> bool {
    let mut xy: Coord = (500, 0);

    if g.contains(&xy) {
        return true;
    }

    loop {
        let mut new_coord = [(0, 1), (-1, 1), (1, 1)]
            .iter()
            .map(|(dx, dy)| (xy.0 + dx, xy.1 + dy))
            .filter(|c| c.1 < floor && !g.contains(c));

        if let Some(nxy) = new_coord.next() {
            xy = nxy;
        } else {
            g.insert(xy);
            return false;
        }
    }
}

fn line_coords(l: &str) -> Vec<Coord> {
    let mut points = l.split(" -> ").map(|point| {
        let mut coords = point.split(",");
        (
            coords.next().unwrap().parse::<i32>().unwrap(),
            coords.next().unwrap().parse::<i32>().unwrap(),
        )
    });

    let mut line = Vec::<Coord>::new();

    let mut xy = points.next().unwrap();
    while let Some(dst) = points.next() {
        loop {
            let dx = (dst.0 - xy.0).signum();
            let dy = (dst.1 - xy.1).signum();

            line.push(xy);

            if dx == 0 && dy == 0 {
                break;
            }

            xy.0 += dx;
            xy.1 += dy;
        }
    }

    line
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: Grid = input
        .trim()
        .split("\n")
        .flat_map(|line| line_coords(line))
        .collect();

    let floor = floor_start_line(&grid);

    let mut count = 0;
    while !add_sand(&mut grid, floor) {
        count += 1;
    }

    println!("Count before spawner hit: {}", count);
}
