use std::{fs, collections::HashSet};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let mut input: Vec<Vec<i32>> = fs::read_to_string("day11/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect();

    let mut total_flashes = 0;
    for _ in 0..100 {
        //Increase all by 1
        input.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|octo| *octo += 1);
        });

        //For each flash, recursively flash
        let mut flashed: HashSet<(i32, i32)> = HashSet::new();
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] > 9 {
                    total_flashes += dfs_flash(&mut input, x as i32, y as i32, &mut flashed);
                }
            }
        }
    }
    println!("Total flashes after 100 steps {}", total_flashes);
}

fn dfs_flash(matrix: &mut Vec<Vec<i32>>, x: i32, y: i32, flashed: &mut HashSet<(i32, i32)>) -> i32 {
    if flashed.contains(&(x, y))
        || x < 0 || x >= matrix[0].len() as i32 || y < 0 || y >= matrix.len() as i32 {
        return 0;
    }

    if matrix[y as usize][x as usize] >= 9 {
        flashed.insert((x, y));
        matrix[y as usize][x as usize] = 0;
        1 + [(1,0),(-1,0),(0,1),(0,-1),(1,1),(-1,1),(1,-1),(-1,-1)].iter()
            .map(|(offset_x, offset_y)| (x + offset_x, y + offset_y))
            .map(|(adj_x, adj_y)| dfs_flash(matrix, adj_x, adj_y, flashed))
            .sum::<i32>()
    } else {
        matrix[y as usize][x as usize] += 1;
        0
    }
}

fn part_2_solution() {
    let mut input: Vec<Vec<i32>> = fs::read_to_string("day11/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect();

    for step in 1..10000 {
        //Increase all by 1
        input.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|octo| *octo += 1);
        });

        //For each flash, recursively flash
        let mut flashed: HashSet<(i32, i32)> = HashSet::new();
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if input[y][x] > 9 {
                    dfs_flash(&mut input, x as i32, y as i32, &mut flashed);
                }
            }
        }
        if flashed.len() == 100 {
            println!("all octopi flashed on step {}", step);
            break;
        }
    }
}