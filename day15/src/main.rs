use std::io::{self, Read};

type Coord = (i64, i64);

struct Sensor {
    pos: Coord,
    beacon: Coord,
    range: i64,
}

fn dist(a: Coord, b: Coord) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl Sensor {
    fn from_input_line(line: &str) -> Sensor {
        let parts: Vec<_> = line.split(" ").collect();

        let pos = (
            parts[2]
                .trim_matches(&['x', '=', ','] as &[_])
                .parse::<i64>()
                .unwrap(),
            parts[3]
                .trim_matches(&['y', '=', ':'] as &[_])
                .parse::<i64>()
                .unwrap(),
        );

        let beacon = (
            parts[8]
                .trim_matches(&['x', '=', ','] as &[_])
                .parse::<i64>()
                .unwrap(),
            parts[9]
                .trim_matches(&['y', '='] as &[_])
                .parse::<i64>()
                .unwrap(),
        );

        Sensor {
            pos,
            beacon,
            range: dist(pos, beacon),
        }
    }

    fn covered_at_line(&self, y: i64) -> Option<(i64, i64)> {
        let power = self.range - (y - self.pos.1).abs();
        if power < 0 {
            None
        } else {
            Some((self.pos.0 - power, self.pos.0 + power))
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let sensors: Vec<Sensor> = input
        .trim()
        .split("\n")
        .map(Sensor::from_input_line)
        .collect();

    'outer: for line in 0..=4000000 {
        let mut ranges: Vec<_> = sensors
            .iter()
            .map(|s| s.covered_at_line(line))
            .flatten()
            .collect();
        ranges.sort();

        let mut compressed = ranges[0];
        for i in 1..ranges.len() {
            if ranges[i].0 <= compressed.1 + 1 {
                compressed.1 = compressed.1.max(ranges[i].1)
            } else {
                let freq = (compressed.1 + 1) * 4000000 + line;
                println!("Gap found! Freq = {}", freq);
                break 'outer;
            }
        }
    }
}
