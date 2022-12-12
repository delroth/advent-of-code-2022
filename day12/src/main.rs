use std::collections::VecDeque;
use std::io;

type Coord = (i32, i32);
type Queue = VecDeque<(Coord, Dist)>;
type Grid = Vec<Vec<i32>>;
type Dist = u32;

type DistGrid = Vec<Vec<u32>>;

fn read_grid() -> Grid {
    io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .chars()
                .map(|c| match c {
                    'S' => -1,
                    'E' => 26,
                    'a'..='z' => (c as i32) - ('a' as i32),
                    _ => panic!("Invalid char"),
                })
                .collect()
        })
        .collect()
}

fn find_cell_and_patch(g: &mut Grid, tgt: i32, patch: i32) -> Coord {
    for (y, r) in g.iter_mut().enumerate() {
        for (x, c) in r.iter_mut().enumerate() {
            if *c == tgt {
                *c = patch;
                return (y as i32, x as i32);
            }
        }
    }

    panic!("Could not find start");
}

fn flood_fill(g: &Grid, end: Coord) -> DistGrid {
    let mut dists = DistGrid::new();
    let dims = (g.len() as i32, g[0].len() as i32);

    for _ in 0..dims.0 {
        let mut row = Vec::<Dist>::new();
        for _ in 0..dims.1 {
            row.push(100000000);
        }
        dists.push(row);
    }

    let mut queue = Queue::new();
    queue.push_back((end, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if dists[pos.0 as usize][pos.1 as usize] > dist {
            dists[pos.0 as usize][pos.1 as usize] = dist;

            let current = g[pos.0 as usize][pos.1 as usize];

            let neighbours = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|e| (pos.0 + e.0, pos.1 + e.1))
                .filter(|e| (e.0 >= 0) && (e.1 >= 0) && (e.0 < dims.0) && (e.1 < dims.1))
                .filter(|e| (current - g[e.0 as usize][e.1 as usize]) <= 1)
                .map(|e| (e, dist + 1));

            queue.extend(neighbours);
        }
    }

    dists
}

fn main() {
    let mut grid = read_grid();
    let start = find_cell_and_patch(&mut grid, -1, 0);
    let end = find_cell_and_patch(&mut grid, 26, 25);

    let dist_grid = flood_fill(&grid, end);

    println!(
        "Part 1 distance: {}",
        dist_grid[start.0 as usize][start.1 as usize]
    );

    let dims = (grid.len(), grid[0].len());
    let mut best_coords = (0..dims.0)
        .flat_map(|y| (0..dims.1).map(move |x| (y, x)))
        .filter(|(y, x)| grid[*y][*x] == 0)
        .collect::<Vec<(usize, usize)>>();

    best_coords.sort_by(|(y1, x1), (y2, x2)| {
        dist_grid[*y1][*x1]
            .partial_cmp(&dist_grid[*y2][*x2])
            .unwrap()
    });

    println!(
        "Best start: {:?} (dist={})",
        best_coords[0], dist_grid[best_coords[0].0][best_coords[0].1]
    );
}
