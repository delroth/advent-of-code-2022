use std::collections::HashSet;
use std::io::{self, BufRead};

type Coords = (usize, usize);
type CoordsSet = HashSet<Coords>;

type Row = Vec<char>;
type Grid = Vec<Row>;

fn read_grid() -> Grid {
    let mut grid = Grid::new();

    for line in io::stdin().lock().lines() {
        let row: Row = line.unwrap().chars().collect();
        grid.push(row);
    }

    grid
}

fn visible_from_point(g: &Grid, x: usize, y: usize, dx: i32, dy: i32) -> CoordsSet {
    let mut coords = CoordsSet::new();
    let height = g.len() as i32;
    let width = g[0].len() as i32;

    let mut x = x as i32;
    let mut y = y as i32;
    let mut currently_visible = 0 as char;

    while x >= 0 && x < width && y >= 0 && y < height {
        let tree = g[y as usize][x as usize];
        if tree > currently_visible {
            currently_visible = tree;
            coords.insert((x as usize, y as usize));
        }
        x += dx;
        y += dy;
    }

    coords
}

fn count_visible_from_outside(g: &Grid) -> usize {
    let mut coords = CoordsSet::new();
    let height = g.len();
    let width = g[0].len();

    for x in 0..width {
        coords.extend(visible_from_point(g, x, 0, 0, 1));
        coords.extend(visible_from_point(g, x, height - 1, 0, -1));
    }
    for y in 0..height {
        coords.extend(visible_from_point(g, 0, y, 1, 0));
        coords.extend(visible_from_point(g, width - 1, y, -1, 0));
    }

    coords.len()
}

fn score_for_direction(g: &Grid, ox: usize, oy: usize, dx: i32, dy: i32) -> usize {
    let height = g.len() as i32;
    let width = g[0].len() as i32;

    let mut x = (ox as i32) + dx;
    let mut y = (oy as i32) + dy;
    let mut score = 0;

    while x >= 0 && x < width && y >= 0 && y < height {
        let tree = g[y as usize][x as usize];
        score += 1;
        if tree >= g[oy][ox] {
            break;
        }
        x += dx;
        y += dy;
    }

    score
}

fn tree_score(g: &Grid, x: usize, y: usize) -> usize {
    let down = score_for_direction(g, x, y, 0, 1);
    let up = score_for_direction(g, x, y, 0, -1);
    let left = score_for_direction(g, x, y, -1, 0);
    let right = score_for_direction(g, x, y, 1, 0);

    down * up * left * right
}

fn max_tree_score(g: &Grid) -> usize {
    let mut max: usize = 0;

    for y in 0..g.len() {
        for x in 0..g[0].len() {
            let score = tree_score(g, x, y);
            if score > max {
                max = score;
            }
        }
    }

    max
}

fn main() {
    let grid = read_grid();

    println!("Visible: {}", count_visible_from_outside(&grid));
    println!("Max tree score: {}", max_tree_score(&grid));
}
