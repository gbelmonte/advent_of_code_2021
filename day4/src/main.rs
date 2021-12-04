use std::{fs, collections::HashSet};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let mut input: Vec<String> = fs::read_to_string("day4/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect();

    let called_numbers: Vec<i32> = input.remove(0)
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<i32>>> = input.chunks(5)
        .map(|board| {
            board.iter()
                .map(|row| {
                    row.split_whitespace()
                        .map(|val| val.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect();

    for num in called_numbers {
        let mut found = false;
        for board in boards.iter_mut() {
            
            // Set called number
            board.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|val| {
                    if *val == num {
                        *val = i32::MAX;
                    }
                })
            });

            // validate row win
            let winning_rows = board.iter()
                .filter(|row| {
                    row.iter()
                        .filter(|&val| *val == i32::MAX)
                        .count() == 5
                })
                .count() > 0;

            // validate col win
            let mut winning_col = false;
            for col in 0..board[0].len() {
                winning_col = true;
                for row in 0..board.len() {
                    winning_col &= board[row][col] == i32::MAX
                }
                if winning_col {
                    break;
                }
            }

            // validate diagnols
            let mut winning_diagnol = true;
            for i in 0..5 {
                winning_diagnol &= board[i][i] == i32::MAX;
            }

            let mut winning_diagnol2 = true;
            let mut j: i32 = 4;
            for i in 0..5 {
                winning_diagnol2 &= board[i][j as usize] == i32::MAX;
                j -= 1;
            }

            if winning_col || winning_rows || winning_diagnol || winning_diagnol2 {
                let sum: i32 = board.iter()
                    .map(|row| row.iter().filter(|&x| *x != i32::MAX).sum::<i32>())
                    .sum();
                println!("winning board {}", sum * num);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }
}

fn part_2_solution() {
    let mut input: Vec<String> = fs::read_to_string("day4/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .filter(|x| !x.is_empty())
        .map(String::from)
        .collect();

    let called_numbers: Vec<i32> = input.remove(0)
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<i32>>> = input.chunks(5)
        .map(|board| {
            board.iter()
                .map(|row| {
                    row.split_whitespace()
                        .map(|val| val.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect();

    let mut won: HashSet<usize> = HashSet::new();
    for i in 0..boards.len() {
        won.insert(i);
    }

    for num in called_numbers {
        for (idx, board) in boards.iter_mut().enumerate() {
            if !won.contains(&idx) {
                continue;
            }

            // Set called number
            board.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|val| {
                    if *val == num {
                        *val = i32::MAX;
                    }
                })
            });

            // validate row win
            let winning_rows = board.iter()
                .filter(|row| {
                    row.iter()
                        .filter(|&val| *val == i32::MAX)
                        .count() == 5
                })
                .count() > 0;

            // validate col win
            let mut winning_col = false;
            for col in 0..board[0].len() {
                winning_col = true;
                for row in 0..board.len() {
                    winning_col &= board[row][col] == i32::MAX
                }
                if winning_col {
                    break;
                }
            }

            // validate diagnols
            let mut winning_diagnol = true;
            for i in 0..5 {
                winning_diagnol &= board[i][i] == i32::MAX;
            }

            let mut winning_diagnol2 = true;
            let mut j: i32 = 4;
            for i in 0..5 {
                winning_diagnol2 &= board[i][j as usize] == i32::MAX;
                j -= 1;
            }

            if winning_col || winning_rows || winning_diagnol || winning_diagnol2 {
                if won.len() == 1 {
                    let sum: i32 = board.iter()
                    .map(|row| row.iter().filter(|&x| *x != i32::MAX).sum::<i32>())
                    .sum();
                    println!("last winning board {}", sum * num);
                }
                won.remove(&idx);
            }
        }
    }
}