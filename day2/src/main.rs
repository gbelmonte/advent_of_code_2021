use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: Vec<(String, i32)> = fs::read_to_string("day2/input/part1.txt")
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

fn part_2_solution() {
    let input: Vec<(String, i32)> = fs::read_to_string("day2/input/part1.txt")
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