use std::fs;

fn main() {
    day_1_part_1_solution();
    day_1_part_2_solution();
}

fn day_1_part_1_solution() {
    let input: Vec<i32> = fs::read_to_string("input/day_1/part_1.txt")
        .expect("Something went wrong reading the file")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let mut increasing_count = 0;
    let mut itr = input.iter().peekable();
    while let Some(curr) = itr.next() {
        if let Some(next) = itr.peek() {
            increasing_count += if *next > curr {1} else {0};
        }
    }
    println!("Number of increases {}", increasing_count);
}

fn day_1_part_2_solution() {
    let input: Vec<i32> = fs::read_to_string("input/day_1/part_1.txt")
        .expect("Something went wrong reading the file")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    
    let mut increasing_count = 0;
    let mut itr = input
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .peekable();
    while let Some(curr) = itr.next() {
        if let Some(next) = itr.peek() {
            increasing_count += if *next > curr {1} else {0};
        }
    }
    println!("Number of increasing windows {}", increasing_count);
}