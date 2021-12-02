use std::fs;

fn main() {
    day_1_part_1_solution();
    day_1_part_2_solution();
    day_2_part_1_solution();
    day_2_part_2_solution();
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

fn day_2_part_1_solution() {
    let input: Vec<(String, i32)> = fs::read_to_string("input/day_2/part_1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| {
            let cmd: Vec<&str> = x.split(" ").collect();
            (cmd[0].to_string(), cmd[1].parse::<i32>().unwrap())
        })
        .collect();

    let (h, d) = input.iter()
        .fold((0, 0), |(h, d), cmd| {
            match cmd.0.as_str() {
                "forward" => (h + cmd.1, d),
                "up" => (h, 0.max(d - cmd.1)),
                "down" => (h, d + cmd.1),
                _ => (h, d),
            }
        });
    println!("traveled horizontal distance of {} and depth of {} multiplied {}", h, d, h * d);
}

fn day_2_part_2_solution() {
    let input: Vec<(String, i32)> = fs::read_to_string("input/day_2/part_1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| {
            let cmd: Vec<&str> = x.split(" ").collect();
            (cmd[0].to_string(), cmd[1].parse::<i32>().unwrap())
        })
        .collect();

    let (h, d, _) = input.iter()
        .fold((0, 0, 0), |(h, d, aim), cmd| {
            match cmd.0.as_str() {
                "forward" => (h + cmd.1, 0.max(d + (aim * cmd.1)), aim),
                "up" => (h, d, aim - cmd.1),
                "down" => (h, d, aim + cmd.1),
                _ => (h, d, aim),
            }
        });
    println!("traveled horizontal distance of {} and depth of {} multiplied {}", h, d, h * d);
}