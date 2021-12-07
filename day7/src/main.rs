use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: Vec<i32> = fs::read_to_string("day7/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .flat_map(|line| {
            line.split(",")
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    
    let max = input.iter().max().unwrap();
    let mut min_moves = i32::MAX;

    // Brute force calculate min number of moves for each possible position
    for i in 0..*max {
        let moves = input.iter()
            .map(|x| (*x - i).abs())
            .sum::<i32>();
        min_moves = min_moves.min(moves);
    }
    println!("minimal horizontal movement {}", min_moves);
}

fn part_2_solution() {
    let input: Vec<i32> = fs::read_to_string("day7/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .flat_map(|line| {
            line.split(",")
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    
    let max = input.iter().max().unwrap();
    let mut min_moves = i32::MAX;
        
    // Brute force calculate min number of moves for each possible position using n*(n+1)/2
    for i in 0..*max {
        let moves = input.iter()
            .map(|x| ((*x - i).abs() as f32 * ((1 + (*x - i).abs()) as f32 / 2.0)) as i32)
            .sum::<i32>();
        min_moves = min_moves.min(moves);
    }
    println!("minimal fuel cost {}", min_moves);
}