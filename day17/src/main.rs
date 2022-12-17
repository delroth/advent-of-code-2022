use once_cell::unsync::Lazy;
use std::collections::HashMap;
use std::io::{self, Read};

enum WindDirection {
    Left,
    Right,
}

struct Simulation {
    occupied: Vec<u8>,
    rocks: usize,
    cycle: usize,
    winds: Vec<WindDirection>,
}

const ROCKS: Lazy<[Vec<(i32, i32)>; 5]> = Lazy::new(|| [
    vec![(0,0), (1,0), (2,0), (3,0)],
    vec![(1,0), (0,1), (1,1), (2,1), (1,2)],
    vec![(0,0), (1,0), (2,0), (2,1), (2,2)],
    vec![(0,0), (0,1), (0,2), (0,3)],
    vec![(0,0), (0,1), (1,0), (1,1)],
]);

impl Simulation {
    fn new(winds: Vec<WindDirection>) -> Simulation {
        Simulation {
            occupied: Vec::new(),
            rocks: 0,
            cycle: 0,
            winds
        }
    }

    fn is_occupied(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= 7 {
            true
        } else if (y as usize) >= self.occupied.len() {
            false
        } else {
            (self.occupied[y as usize] & (1 << x)) != 0
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        let y = y as usize;
        if x < 0 || x >= 7 {
            panic!("Can't place off grid")
        }
        while y >= self.occupied.len() {
            self.occupied.push(0)
        }
        self.occupied[y] |= 1 << x;
    }

    fn add_falling_rock(&mut self) {
        let sx = 2;
        let sy = self.occupied.len() as i32 + 3;

        let mut rock: Vec<_> = ROCKS[self.rocks % ROCKS.len()].iter().map(|(x, y)| (x + sx, y + sy)).collect();
        self.rocks += 1;

        loop {
            let wind = &self.winds[self.cycle];
            self.cycle = (self.cycle + 1) % self.winds.len();

            let dx = match wind {
                WindDirection::Left => -1,
                WindDirection::Right => 1,
            };

            let moved: Vec<_> = rock.iter().map(|&(x, y)| (x + dx, y)).collect();
            if moved.iter().all(|&(x, y)| !self.is_occupied(x, y)) {
                rock = moved;
            }

            let moved: Vec<_> = rock.iter().map(|&(x, y)| (x, y - 1)).collect();
            if moved.iter().all(|&(x, y)| !self.is_occupied(x, y)) {
                rock = moved;
            } else {
                for (x, y) in rock {
                    self.place(x, y)
                }
                break;
            }
        }
    }
}

type StateKey = (usize, usize, u32);

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let winds: Vec<_> = input.trim().chars().map(|c| match c {
        '<' => WindDirection::Left,
        '>' => WindDirection::Right,
        _ => panic!("Unknown input char"),
    }).collect();

    let mut sim = Simulation::new(winds);

    for _ in 0..2000 {
        sim.add_falling_rock();
    }

    let mut seen_states = HashMap::<StateKey, usize>::new();
    let mut cycle_len: usize = 0;
    let hdelta: usize;
    loop {
        sim.add_falling_rock();

        let r1 = sim.occupied[sim.occupied.len() - 1] as u32;
        let r2 = sim.occupied[sim.occupied.len() - 2] as u32;
        let r3 = sim.occupied[sim.occupied.len() - 3] as u32;
        let r4 = sim.occupied[sim.occupied.len() - 4] as u32;
        let r = (r1 << 24) | (r2 << 16) | (r3 << 8) | r4;
        let state_key = (sim.rocks % ROCKS.len(), sim.cycle, r);
        if let Some(h) = seen_states.get(&state_key) {
            hdelta = sim.occupied.len() - h;
            break;
        } else {
            seen_states.insert(state_key, sim.occupied.len());
        }

        cycle_len += 1;
    }
    println!("Cycle length: {}, height delta: {}", cycle_len, hdelta);

    let mut target_rocks: u64 = 1000000000000;
    target_rocks -= sim.rocks as u64;
    let cycles_count = target_rocks / (cycle_len as u64);
    let rem = target_rocks % (cycle_len as u64);

    for _ in 0..rem {
        sim.add_falling_rock();
    }

    println!("Height: {}", sim.occupied.len() as u64 + cycles_count * hdelta as u64);
}
