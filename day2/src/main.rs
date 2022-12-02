use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn losing_shape(s: Shape) -> Shape {
    match s {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn winning_shape(s: Shape) -> Shape {
    match s {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

enum Goal {
    Lose,
    Draw,
    Win,
}

fn abc_to_choice(abc: &str) -> Shape {
    match abc {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Opponent shape is not A/B/C"),
    }
}

fn xyz_to_goal(xyz: &str) -> Goal {
    match xyz {
        "X" => Goal::Lose,
        "Y" => Goal::Draw,
        "Z" => Goal::Win,
        _ => panic!("Goal is not X/Y/Z"),
    }
}

fn make_choice(opponent: Shape, goal: Goal) -> Shape {
    match goal {
        Goal::Draw => opponent,
        Goal::Win => winning_shape(opponent),
        Goal::Lose => losing_shape(opponent),
    }
}

fn score_round(opponent: Shape, us: Shape) -> u32 {
    let shape_score = match us {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let vs_score = if opponent == us {
        3
    } else if opponent == winning_shape(us) {
        0
    } else {
        6
    };

    shape_score + vs_score
}

fn main() {
    let mut total_score = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut tokens = line.split(' ');

        let opponent = abc_to_choice(tokens.next().unwrap());
        let goal = xyz_to_goal(tokens.next().unwrap());
        let us = make_choice(opponent, goal);

        total_score += score_round(opponent, us);
    }

    println!("{}", total_score);
}
