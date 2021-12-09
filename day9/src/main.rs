use std::{fs, collections::{HashSet, BinaryHeap}};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: Vec<Vec<i32>> = fs::read_to_string("day9/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect();
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let mut sum_of_local_minimums = 0;
    for y in 0..height {
        for x in 0..width {
            let is_local_minimum = [(0,1), (0,-1), (1,0), (-1,0)].iter()
                .map(|(delta_x, delta_y)| (x + *delta_x, y + *delta_y))
                .filter(|(adj_x, adj_y)| *adj_x >= 0 && *adj_x < width && *adj_y >= 0 && *adj_y < height)
                .all(|(adj_x, adj_y)| input[y as usize][x as usize] < input[adj_y as usize][adj_x as usize]);
            if is_local_minimum {
                sum_of_local_minimums += input[y as usize][x as usize] + 1;
            }
        }
    }
    println!("Sum of all local minimums {}", sum_of_local_minimums);
}

fn part_2_solution() {
    let input: Vec<Vec<i32>> = fs::read_to_string("day9/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect();
    let height = input.len() as i32;
    let width = input[0].len() as i32;

    let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
    for y in 0..height {
        for x in 0..width {
            let is_local_minimum = [(0,1), (0,-1), (1,0), (-1,0)].iter()
                .map(|(delta_x, delta_y)| (x + *delta_x, y + *delta_y))
                .filter(|(adj_x, adj_y)| *adj_x >= 0 && *adj_x < width && *adj_y >= 0 && *adj_y < height)
                .all(|(adj_x, adj_y)| input[y as usize][x as usize] < input[adj_y as usize][adj_x as usize]);
            if is_local_minimum {
                max_heap.push(dfs(&input, x, y, -1, &mut HashSet::new()));
            }
        }
    }
    println!("product of 3 largest basins {}", max_heap.pop().unwrap() * max_heap.pop().unwrap() * max_heap.pop().unwrap());
}

fn dfs(matrix: &Vec<Vec<i32>>, x: i32, y: i32, prev: i32, visited: &mut HashSet<(i32, i32)>) -> i32 {
    if visited.contains(&(x, y)) {
        return 0;
    } else if x < 0 || x >= matrix[0].len() as i32 || y < 0 || y >= matrix.len() as i32  {
        return 0;
    } else if matrix[y as usize][x as usize] == 9 || matrix[y as usize][x as usize] <= prev {
        return 0;
    }
    visited.insert((x, y));
    1 + [(0,1), (0,-1), (1,0), (-1,0)].iter()
        .map(|(delta_x, delta_y)| {
            dfs(matrix, x + delta_x, y + delta_y, matrix[y as usize][x as usize], visited)
        })
        .sum::<i32>()
}