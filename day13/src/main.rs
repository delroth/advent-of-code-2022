use json::{self, JsonValue};
use std::cmp::Ordering;
use std::io::{self, Read};

fn is_right_order(v1: &JsonValue, v2: &JsonValue) -> Ordering {
    match (v1, v2) {
        (JsonValue::Number(_), JsonValue::Number(_)) => {
            if v1.as_u32() < v2.as_u32() {
                Ordering::Less
            } else if v1.as_u32() == v2.as_u32() {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
        (JsonValue::Array(_), JsonValue::Number(_)) => {
            is_right_order(v1, &JsonValue::Array(vec![v2.clone()]))
        }
        (JsonValue::Number(_), JsonValue::Array(_)) => {
            is_right_order(&JsonValue::Array(vec![v1.clone()]), v2)
        }
        (JsonValue::Array(a1), JsonValue::Array(a2)) => {
            let mut i1 = a1.iter();
            let mut i2 = a2.iter();

            loop {
                match (i1.next(), i2.next()) {
                    (None, None) => return Ordering::Equal,
                    (Some(_), None) => return Ordering::Greater,
                    (None, Some(_)) => return Ordering::Less,
                    (Some(e1), Some(e2)) => match is_right_order(e1, e2) {
                        Ordering::Equal => continue,
                        x => return x,
                    },
                }
            }
        }
        _ => panic!("Not supported"),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let count: usize = input
        .split("\n\n")
        .map(|chunks| {
            let mut lines = chunks.split("\n");
            (lines.next().unwrap(), lines.next().unwrap())
        })
        .map(|(l1, l2)| (json::parse(l1).unwrap(), json::parse(l2).unwrap()))
        .enumerate()
        .filter(|(_, (v1, v2))| is_right_order(&v1, &v2) != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum();

    println!("Part 1 result: {}", count);

    let divider1 = json::parse("[[2]]").unwrap();
    let divider2 = json::parse("[[6]]").unwrap();

    let mut packets: Vec<_> = input
        .split("\n")
        .filter(|l| l != &"")
        .map(|l| json::parse(l).unwrap())
        .collect();

    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort_by(is_right_order);

    let i1 = 1 + packets.iter().position(|v| v == &divider1).unwrap();
    let i2 = 1 + packets.iter().position(|v| v == &divider2).unwrap();

    println!("Part 2 result: {}", i1 * i2);
}
