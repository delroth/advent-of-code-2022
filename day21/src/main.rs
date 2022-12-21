use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
enum Monkey {
    Int(i64),
    Add(String, String),
    Mul(String, String),
    Div(String, String),
    Sub(String, String),

    Match(String, String),
}

fn parse_line(l: &str) -> (String, Monkey) {
    let name = l.split(":").next().unwrap().to_owned();
    let expr = l.split(": ").last().unwrap();

    if expr.contains(" ") {
        let mut parts = expr.split(" ");
        let lhs = parts.next().unwrap().to_owned();
        let op = parts.next().unwrap();
        let rhs = parts.next().unwrap().to_owned();

        if name == "root" {
            (name, Monkey::Match(lhs, rhs))
        } else {
            (
                name,
                match op {
                    "+" => Monkey::Add(lhs, rhs),
                    "-" => Monkey::Sub(lhs, rhs),
                    "*" => Monkey::Mul(lhs, rhs),
                    "/" => Monkey::Div(lhs, rhs),
                    _ => panic!("Unknown op: {}", op),
                },
            )
        }
    } else {
        (name, Monkey::Int(expr.parse().unwrap()))
    }
}

fn eval(monkeys: &HashMap<String, Monkey>, name: &str) -> i64 {
    match &monkeys[name] {
        Monkey::Int(i) => *i,
        Monkey::Add(lhs, rhs) => eval(monkeys, lhs.as_str()) + eval(monkeys, rhs.as_str()),
        Monkey::Sub(lhs, rhs) => eval(monkeys, lhs.as_str()) - eval(monkeys, rhs.as_str()),
        Monkey::Mul(lhs, rhs) => eval(monkeys, lhs.as_str()) * eval(monkeys, rhs.as_str()),
        Monkey::Div(lhs, rhs) => eval(monkeys, lhs.as_str()) / eval(monkeys, rhs.as_str()),
        Monkey::Match(_, _) => panic!("Found match in eval"),
    }
}

fn contains_humn(monkeys: &HashMap<String, Monkey>, name: &str) -> bool {
    if name == "humn" {
        true
    } else {
        match &monkeys[name] {
            Monkey::Int(_) => false,
            Monkey::Add(lhs, rhs)
            | Monkey::Sub(lhs, rhs)
            | Monkey::Mul(lhs, rhs)
            | Monkey::Div(lhs, rhs) => {
                contains_humn(monkeys, lhs.as_str()) || contains_humn(monkeys, rhs.as_str())
            }
            Monkey::Match(_, _) => panic!("Found match in humn check"),
        }
    }
}

fn find_humn(monkeys: &HashMap<String, Monkey>, name: &str, expected: i64) -> i64 {
    if name == "humn" {
        expected
    } else {
        match &monkeys[name] {
            Monkey::Add(lhs, rhs)
            | Monkey::Sub(lhs, rhs)
            | Monkey::Mul(lhs, rhs)
            | Monkey::Div(lhs, rhs) => {
                let lhs_humn = contains_humn(&monkeys, lhs.as_str());
                let other_side = if lhs_humn {
                    eval(&monkeys, rhs.as_str())
                } else {
                    eval(&monkeys, lhs.as_str())
                };
                let expected = match &monkeys[name] {
                    // X = H + O => H = X - O
                    Monkey::Add(_, _) => expected - other_side,
                    // X = H - O => H = X + O, X = O - H => H = O - X
                    Monkey::Sub(_, _) => {
                        if lhs_humn {
                            expected + other_side
                        } else {
                            other_side - expected
                        }
                    }
                    // X = H * O => H = X / O
                    Monkey::Mul(_, _) => expected / other_side,
                    // X = H / O => H = X * O, X = O / H => H = O / X
                    Monkey::Div(_, _) => {
                        if lhs_humn {
                            expected * other_side
                        } else {
                            other_side / expected
                        }
                    }
                    _ => panic!("Should never happen"),
                };
                if lhs_humn {
                    find_humn(&monkeys, lhs.as_str(), expected)
                } else {
                    find_humn(&monkeys, rhs.as_str(), expected)
                }
            }
            _ => panic!("Unexpected type in find_humn"),
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let monkeys: HashMap<String, Monkey> = input.trim().split("\n").map(parse_line).collect();

    if let Monkey::Match(lhs, rhs) = &monkeys["root"] {
        let lhs_humn = contains_humn(&monkeys, lhs.as_str());

        let val = if lhs_humn {
            eval(&monkeys, rhs.as_str())
        } else {
            eval(&monkeys, lhs.as_str())
        };
        let humn = if lhs_humn {
            find_humn(&monkeys, lhs.as_str(), val)
        } else {
            find_humn(&monkeys, rhs.as_str(), val)
        };

        println!("Humn value: {}", humn);
    } else {
        panic!("Wrong type for root");
    }
}
