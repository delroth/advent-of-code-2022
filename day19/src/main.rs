use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

struct Blueprint {
    id: u8,
    ore_robot_ore_cost: u8,
    clay_robot_ore_cost: u8,
    obsidian_robot_ore_cost: u8,
    obsidian_robot_clay_cost: u8,
    geode_robot_ore_cost: u8,
    geode_robot_obsidian_cost: u8,
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    t: u8,

    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,

    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

impl Blueprint {
    fn from_line(l: &str) -> Blueprint {
        let tokens: Vec<_> = l.split(" ").collect();

        Blueprint {
            id: tokens[1].trim_matches(':').parse::<u8>().unwrap(),
            ore_robot_ore_cost: tokens[6].parse::<u8>().unwrap(),
            clay_robot_ore_cost: tokens[12].parse::<u8>().unwrap(),
            obsidian_robot_ore_cost: tokens[18].parse::<u8>().unwrap(),
            obsidian_robot_clay_cost: tokens[21].parse::<u8>().unwrap(),
            geode_robot_ore_cost: tokens[27].parse::<u8>().unwrap(),
            geode_robot_obsidian_cost: tokens[30].parse::<u8>().unwrap(),
        }
    }

    fn geode_potential(&self) -> u8 {
        let mut queue = VecDeque::<State>::new();
        let mut cache = HashSet::<State>::new();
        let mut max_geode = 0;
        let mut last_t = 0;

        queue.push_back(State::new());

        while let Some(state) = queue.pop_front() {
            if state.t > last_t {
                last_t = state.t;
                cache.clear();
            }
            if state.is_final() {
                max_geode = max_geode.max(state.geode);
            } else if !cache.contains(&state) {
                state.push_all_succ(&mut queue, &self);
                cache.insert(state);
            }
        }

        println!("Blueprint {}: geode potential = {}", self.id, max_geode);
        max_geode
    }

    fn score(&self) -> u8 {
        self.id * self.geode_potential()
    }
}

impl State {
    fn new() -> State {
        State {
            t: 0,

            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn is_final(&self) -> bool {
        self.t == 32
    }

    fn base_succ(&self) -> State {
        State {
            t: self.t + 1,

            ore: (self.ore + self.ore_robots).min(20),
            clay: (self.clay + self.clay_robots).min(40),
            obsidian: (self.obsidian + self.obsidian_robots).min(30),
            geode: self.geode + self.geode_robots,

            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
        }
    }

    fn push_all_succ(&self, queue: &mut VecDeque<State>, bp: &Blueprint) {
        queue.push_back(self.base_succ());

        let max_ore_cost = bp
            .ore_robot_ore_cost
            .max(bp.clay_robot_ore_cost)
            .max(bp.obsidian_robot_ore_cost)
            .max(bp.geode_robot_ore_cost);
        let max_clay_cost = bp.obsidian_robot_clay_cost;
        let max_obsidian_cost = bp.geode_robot_obsidian_cost;

        if self.ore_robots < max_ore_cost && self.ore >= bp.ore_robot_ore_cost {
            let mut succ = self.base_succ();
            succ.ore -= bp.ore_robot_ore_cost;
            succ.ore_robots += 1;
            queue.push_back(succ);
        }
        if self.clay_robots < max_clay_cost && self.ore >= bp.clay_robot_ore_cost {
            let mut succ = self.base_succ();
            succ.ore -= bp.clay_robot_ore_cost;
            succ.clay_robots += 1;
            queue.push_back(succ);
        }
        if self.obsidian_robots < max_obsidian_cost
            && self.ore >= bp.obsidian_robot_ore_cost
            && self.clay >= bp.obsidian_robot_clay_cost
        {
            let mut succ = self.base_succ();
            succ.ore -= bp.obsidian_robot_ore_cost;
            succ.clay -= bp.obsidian_robot_clay_cost;
            succ.obsidian_robots += 1;
            queue.push_back(succ);
        }
        if self.ore >= bp.geode_robot_ore_cost && self.obsidian >= bp.geode_robot_obsidian_cost {
            let mut succ = self.base_succ();
            succ.ore -= bp.geode_robot_ore_cost;
            succ.obsidian -= bp.geode_robot_obsidian_cost;
            succ.geode_robots += 1;
            queue.push_back(succ);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    /*
        let blueprints: Vec<_> = input.trim().split("\n").map(Blueprint::from_line).collect();
        let result: u8 = blueprints.par_iter().map(Blueprint::score).sum();
        println!("Part 1 result: {}", result);
    */

    let blueprints: Vec<_> = input
        .trim()
        .split("\n")
        .map(Blueprint::from_line)
        .take(3)
        .collect();
    let result: u8 = blueprints.iter().map(Blueprint::geode_potential).product();
    println!("Part 2 result: {}", result);
}
