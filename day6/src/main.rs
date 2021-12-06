use std::{fs, collections::HashMap};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let mut input: Vec<i32> = fs::read_to_string("day6/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .flat_map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect();

    for _ in 0..80 {
        for fish in 0..input.len() {
            input[fish] -= 1;
            if input[fish] < 0 {
                input.push(8);
                input[fish] = 6;
            }
        }
    }
    println!("Number of fish after 80 days {}", input.len());
}

fn part_2_solution() {
    let input: Vec<i32> = fs::read_to_string("day6/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .flat_map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect();

    let mut fish = input.iter()
        .fold(HashMap::new(), |mut map, val| {
            *map.entry(*val).or_insert(0) += 1_u64;
            map
        });
    let mut day = 0;
    let mut days_past = 0;
    let mut one_day_old = 0;
    let mut two_day_old = 0;
    while days_past < 256 {
        let spawning = fish.get(&day).unwrap_or(&0).clone();
        *fish.entry(day).or_insert(0) += two_day_old;
        two_day_old = one_day_old;
        one_day_old = spawning;
        days_past += 1;
        day = (day + 1) % 7;
    }
    println!("Number of fish after 256 days {}", fish.iter().map(|(_,val)| val).sum::<u64>() + one_day_old + two_day_old);
}