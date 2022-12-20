use std::io::{self, BufRead};

fn main() {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap() * 811589153)
        .collect();
    let mut indices: Vec<_> = (0..input.len()).collect();

    for _ in 0..10 {
        for (i, &n) in input.iter().enumerate() {
            let curr_i = indices.iter().position(|&e| e == i).unwrap();
            let new_i = (curr_i as i64 + n).rem_euclid(input.len() as i64 - 1);

            indices.remove(curr_i);
            indices.insert(new_i as usize, i);
        }
    }

    let i_zero = input.iter().position(|&e| e == 0).unwrap();
    let new_i_zero = indices.iter().position(|&e| e == i_zero).unwrap();
    let sum: i64 = [1000, 2000, 3000]
        .iter()
        .map(|e| input[indices[(new_i_zero + e) % input.len()]])
        .sum();

    println!("Sum: {}", sum);
}
