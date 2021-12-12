use std::{fs, collections::HashMap};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let adj_map: HashMap<String,Vec<String>> = fs::read_to_string("day12/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .fold(HashMap::new(), |mut adj_map, line| {
            let caves: Vec<String> = line.trim().split("-").map(String::from).collect();
            adj_map.entry(caves[0].clone()).or_insert(Vec::new()).push(caves[1].clone());
            adj_map.entry(caves[1].clone()).or_insert(Vec::new()).push(caves[0].clone());
            adj_map
        });
    println!("{}", dfs(&adj_map, "start", HashMap::new()));
}

// Deep copy seen caves every iteration :(
fn dfs<'a>(adj_map: &HashMap<String,Vec<String>>, cave: &'a str, mut seen: HashMap<&'a str,i32>) -> i32 {
    if cave == "end" {
        return 1;
    }

    if cave.to_lowercase().eq(cave) {
        *seen.entry(cave).or_insert(0) += 1;
    }
    adj_map.get(cave).unwrap().iter()
        .filter(|next| !(next.as_str().to_lowercase().eq(*next) && *seen.get(next.as_str()).unwrap_or(&0) >= 1))
        .map(|next| dfs(adj_map, next, seen.clone()))
        .sum::<i32>()
}

fn part_2_solution() {
    let adj_map: HashMap<String,Vec<String>> = fs::read_to_string("day12/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .fold(HashMap::new(), |mut adj_map, line| {
            let caves: Vec<String> = line.trim().split("-").map(String::from).collect();
            adj_map.entry(caves[0].clone()).or_insert(Vec::new()).push(caves[1].clone());
            adj_map.entry(caves[1].clone()).or_insert(Vec::new()).push(caves[0].clone());
            adj_map
        });
    println!("{}", dfs_2(&adj_map, "start", HashMap::new()));
}

// Deep copy seen caves every iteration :(
fn dfs_2<'a>(adj_map: &HashMap<String,Vec<String>>, cave: &'a str, mut seen: HashMap<&'a str,i32>) -> i32 {
    if cave == "end" {
        return 1;
    }

    if cave.to_lowercase().eq(cave) {
        *seen.entry(cave).or_insert(0) += 1;
    }
    let small_cave_with_two_visits = seen.iter().any(|(_, val)| *val > 1);
    adj_map.get(cave).unwrap().iter()
        .filter(|next| *next != "start")
        .filter(|next| !small_cave_with_two_visits || !(next.as_str().to_lowercase().eq(*next) && *seen.get(next.as_str()).unwrap_or(&0) >= 1))
        .map(|next| dfs_2(adj_map, next, seen.clone()))
        .sum::<i32>()
}