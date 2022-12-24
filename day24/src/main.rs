use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

type Coord = (i32, i32);

struct Cell {
    blizz_up: bool,
    blizz_right: bool,
    blizz_down: bool,
    blizz_left: bool,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        let blizz_up = c == '^';
        let blizz_right = c == '>';
        let blizz_down = c == 'v';
        let blizz_left = c == '<';

        Cell {
            blizz_up,
            blizz_right,
            blizz_down,
            blizz_left,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell {
                blizz_up: false,
                blizz_right: false,
                blizz_down: false,
                blizz_left: false,
            } => '.',
            Cell {
                blizz_up: true,
                blizz_right: false,
                blizz_down: false,
                blizz_left: false,
            } => '^',
            Cell {
                blizz_up: false,
                blizz_right: true,
                blizz_down: false,
                blizz_left: false,
            } => '>',
            Cell {
                blizz_up: false,
                blizz_right: false,
                blizz_down: true,
                blizz_left: false,
            } => 'v',
            Cell {
                blizz_up: false,
                blizz_right: false,
                blizz_down: false,
                blizz_left: true,
            } => '<',
            _ => '*',
        }
    }

    fn traversable(&self) -> bool {
        !self.blizz_up && !self.blizz_right && !self.blizz_left && !self.blizz_down
    }
}

type Grid = HashMap<Coord, Cell>;

type CacheKey = (usize, Coord);
type Cache = HashSet<CacheKey>;

struct State {
    grids: Vec<Grid>,

    width: i32,
    height: i32,
}

impl State {
    fn from_input(input: &str) -> State {
        let lines: Vec<&str> = input.trim().split("\n").collect();
        let lines = &lines[1..lines.len() - 1];

        let width = (lines[0].len() - 2) as i32;
        let height = lines.len() as i32;

        let grid: Grid = lines
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.trim_matches('#')
                    .chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32), Cell::from_char(c)))
            })
            .collect();

        State {
            grids: vec![grid],
            width,
            height,
        }
    }

    fn print_grid(&self, grid: &Grid) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = grid[&(x, y)].to_char();
                print!("{c}");
            }
            println!();
        }
    }

    fn advance_time(&mut self) {
        let last_grid = self.grids.last().unwrap();
        let mut new_grid: Grid = last_grid
            .keys()
            .map(|&k| (k, Cell::from_char('.')))
            .collect();

        for (&(x, y), c) in last_grid.iter() {
            let up_neigh = (x, (y + self.height - 1) % self.height);
            let left_neigh = ((x + self.width - 1) % self.width, y);
            let down_neigh = (x, (y + 1) % self.height);
            let right_neigh = ((x + 1) % self.width, y);

            if c.blizz_up {
                new_grid.get_mut(&up_neigh).unwrap().blizz_up = true;
            }
            if c.blizz_left {
                new_grid.get_mut(&left_neigh).unwrap().blizz_left = true;
            }
            if c.blizz_down {
                new_grid.get_mut(&down_neigh).unwrap().blizz_down = true;
            }
            if c.blizz_right {
                new_grid.get_mut(&right_neigh).unwrap().blizz_right = true;
            }
        }

        self.grids.push(new_grid)
    }

    fn grid_at_time(&self, minute: usize) -> &Grid {
        &self.grids[minute]
    }

    fn solve(&mut self, start_time: usize, start: Coord, goal: Coord) -> usize {
        let mut highest_seen_time = start_time;
        let mut queue = VecDeque::<CacheKey>::new();
        let mut visited = Cache::new();

        queue.push_back((start_time, start));
        while let Some(key) = queue.pop_front() {
            let (time, (x, y)) = key;

            let time = time + 1;
            if time > highest_seen_time {
                highest_seen_time = time;
                self.advance_time();
            }
            let grid = self.grid_at_time(time);

            let neighs = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]
                .iter()
                .map(|&(dx, dy)| (x + dx, y + dy))
                .filter(|&(nx, ny)| {
                    ((nx, ny) == goal)
                        || ((nx, ny) == start)
                        || ((nx >= 0) && (ny >= 0) && (nx < self.width) && (ny < self.height))
                });
            for neigh in neighs {
                if neigh == goal {
                    return time;
                } else if neigh == start || grid[&neigh].traversable() {
                    let nkey = (time, neigh);
                    if !visited.contains(&nkey) {
                        visited.insert(nkey);
                        queue.push_back(nkey);
                    }
                }
            }
        }

        panic!("unreachable! reached time {highest_seen_time}")
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut state = State::from_input(input.as_str());

    let start = (0, -1);
    let goal = (state.width - 1, state.height);

    let steps_needed = state.solve(0, start, goal);
    println!("Steps needed: {steps_needed}");

    let steps_needed = state.solve(steps_needed, goal, start);
    println!("Steps needed roundtrip: {steps_needed}");

    let steps_needed = state.solve(steps_needed, start, goal);
    println!("Steps needed roundtrip and back: {steps_needed}");
}
