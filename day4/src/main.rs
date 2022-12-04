use std::io::{self, BufRead};

struct Assignment(u32, u32);

impl Assignment {
    fn new(s: &String) -> Assignment {
        let mut parts = s.split("-");
        Assignment(
            parts.next().unwrap().parse::<u32>().unwrap(),
            parts.next().unwrap().parse::<u32>().unwrap(),
        )
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        (self.0 <= other.0 && self.1 >= other.0) || (other.0 <= self.0 && other.1 >= self.0)
    }
}

fn count_overlapping_assignments() -> usize {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().split(",").map(str::to_owned).collect::<Vec<_>>())
        .map(|parts| (Assignment::new(&parts[0]), Assignment::new(&parts[1])))
        .filter(|(set1, set2)| set1.overlaps(set2))
        .count()
}

fn main() {
    println!("{}", count_overlapping_assignments());
}
