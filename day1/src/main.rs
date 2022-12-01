use std::io::{self, BufRead};

struct TopNCounter<const N: usize> {
    topn: [u32; N],
}

impl<const N: usize> TopNCounter<N> {
    fn new() -> Self {
        TopNCounter::<N> { topn: [0; N] }
    }

    fn push(&mut self, elem: u32) {
        for i in 0..self.topn.len() {
            let top = self.topn[i];
            if elem > top {
                for j in ((i + 1)..self.topn.len()).rev() {
                    self.topn[j] = self.topn[j - 1];
                }
                self.topn[i] = elem;
                break;
            }
        }
    }

    fn sum(self) -> u32 {
        let mut sum = 0;
        for e in self.topn {
            sum += e;
        }
        sum
    }
}

fn main() {
    let mut current_sum = 0;

    let mut top3 = TopNCounter::<3>::new();

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            top3.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<u32>().unwrap();
        }
    }

    top3.push(current_sum);

    println!("{}", top3.sum());
}
