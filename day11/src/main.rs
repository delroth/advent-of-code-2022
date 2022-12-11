use std::collections::{HashMap, VecDeque};
use std::io;

struct Item {
    base: u32,
    mods: HashMap<u32, u32>,
}

impl Item {
    fn new(base: u32) -> Item {
        Item { base, mods: HashMap::new() }
    }

    fn add_divider(&mut self, div: u32) {
        self.mods.insert(div, self.base % div);
    }

    fn apply_op(&mut self, op: Operation) {
        for (div, rem) in self.mods.iter_mut() {
            match op {
                Operation::Square => *rem = (*rem * *rem) % div,
                Operation::Add(x) => *rem = (*rem + x) % div,
                Operation::Mul(x) => *rem = (*rem * x) % div,
            }
        }
    }

    fn is_divisible_by(&self, div: u32) -> bool {
        self.mods[&div] == 0
    }
}

struct Monkey {
    items: VecDeque<Item>,
    op: Operation,

    divider: u32,
    if_divides: u32,
    if_not: u32,

    inspected: u64,
}

#[derive(Clone, Copy)]
enum Operation {
    Add(u32),
    Mul(u32),
    Square,
}

fn read_monkey() -> Monkey {
    let mut line = String::new();

    // Monkey N
    io::stdin().read_line(&mut line).unwrap();

    // Starting items: 1, 2, 3
    io::stdin().read_line(&mut line).unwrap();
    line = line.split(": ").nth(1).unwrap().trim().to_string();
    let items: VecDeque<Item> = line.split(", ").map(|i| Item::new(i.parse::<u32>().unwrap())).collect();

    // Operation: new = old + 3
    io::stdin().read_line(&mut line).unwrap();
    line = line.split(" = ").nth(1).unwrap().trim().to_string();
    let op = if line == "old * old" {
        Operation::Square
    } else if line.contains(" * ") {
        Operation::Mul(line.split(" * ").nth(1).unwrap().parse::<u32>().unwrap())
    } else {
        Operation::Add(line.split(" + ").nth(1).unwrap().parse::<u32>().unwrap())
    };

    // Test: divisible by 19
    io::stdin().read_line(&mut line).unwrap();
    line = line.split(" by ").nth(1).unwrap().trim().to_string();
    let divider = line.parse::<u32>().unwrap();

    // If true: throw to monkey 1
    io::stdin().read_line(&mut line).unwrap();
    line = line.split(" monkey ").nth(1).unwrap().trim().to_string();
    let if_divides = line.parse::<u32>().unwrap();

    // If false: throw to monkey 1
    io::stdin().read_line(&mut line).unwrap();
    line = line.split(" monkey ").nth(1).unwrap().trim().to_string();
    let if_not = line.parse::<u32>().unwrap();

    Monkey {
        items,
        op,
        divider,
        if_divides,
        if_not,
        inspected: 0,
    }
}

fn read_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();

    loop {
        monkeys.push(read_monkey());

        let mut line = String::new();
        if !io::stdin().read_line(&mut line).is_ok() || line == "" {
            break;
        }
    }

    monkeys
}

fn run_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            let monkey = &mut monkeys[i];

            monkey.inspected += 1;

            let mut item = monkey.items.pop_front().unwrap();
            item.apply_op(monkey.op);

            let dst = if item.is_divisible_by(monkey.divider) {
                monkey.if_divides
            } else {
                monkey.if_not
            };

            monkeys[dst as usize].items.push_back(item);
        }
    }
}

fn main() {
    let mut monkeys = read_monkeys();

    // Update all items to set dividers.
    let dividers: Vec<u32> = monkeys.iter().map(|m| m.divider).collect();
    for m in &mut monkeys {
        for i in &mut m.items {
            for d in dividers.iter() {
                i.add_divider(*d)
            }
        }
    }

    for _ in 0..10000 {
        run_round(&mut monkeys);
    }

    monkeys.sort_by(|a, b| b.inspected.partial_cmp(&a.inspected).unwrap());
    let result = monkeys[0].inspected * monkeys[1].inspected;

    println!("Result: {}", result);
}
