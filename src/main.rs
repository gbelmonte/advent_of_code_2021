use std::{fs, borrow::BorrowMut, collections::HashSet};

fn main() {
    day_1_part_1_solution();
    day_1_part_2_solution();
    day_2_part_1_solution();
    day_2_part_2_solution();
    day_3_part_1_solution();
    day_3_part_2_solution();
    day_4_part_1_solution();
    day_4_part_2_solution();
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

fn day_3_part_1_solution() {
    let input: Vec<String> = fs::read_to_string("input/day_3/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(String::from)
        .collect();

    let counts = input.iter()
        .fold(vec![0; input[0].len()], |mut acc, x| {
            x.chars().enumerate().for_each(|(i, c)| {
                acc[i] += if c == '0' {1} else {-1}
            });
            acc
        });

    let binary_string: String = counts.iter()
        .map(|x| if *x > 0 {'0'} else {'1'})
        .collect();

    let gamma_rate = u32::from_str_radix(&binary_string, 2).unwrap();
    let epsilon_rate = !gamma_rate & 0b111111111111;
    println!("power consumption {}", gamma_rate * epsilon_rate);
}

fn filter_by_comparator(mut input: Vec<Vec<char>>, comparator: fn(i32, i32) -> char) -> Vec<char> {
    for i in 0..input.len() {
        let last_value = input[input.len() - 1].clone();
        let ones_count = input.iter()
            .filter(|x| x[i] == '1')
            .count();
        
        let filter_by = comparator(ones_count as i32, input.len() as i32);
        input = input.into_iter()
            .filter(|x| x[i] == filter_by)
            .collect();

        if input.len() == 1 {
            break;
        } else if input.is_empty() {
            input.push(last_value.clone());
            break;
        }
    }
    return input[0].clone();
}

fn day_3_part_2_solution() {
    let input: Vec<Vec<char>> = fs::read_to_string("input/day_3/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let oxygen_generator = filter_by_comparator(input.clone(), 
        |ones_count, total_count| if ones_count as f64 >= (total_count as f64 / 2.0).round() {'1'} else {'0'});
    let co2_scrubber = filter_by_comparator(input.clone(), 
        |ones_count, total_count| if ones_count as f64 >= (total_count as f64 / 2.0).round() {'0'} else {'1'});

    let oxygen_generator_rating = u32::from_str_radix(&oxygen_generator.iter().collect::<String>(), 2).unwrap();
    let co2_scrubber_rating = u32::from_str_radix(&co2_scrubber.iter().collect::<String>(), 2).unwrap();
    println!("life support rating {}", oxygen_generator_rating * co2_scrubber_rating);
}

fn day_4_part_1_solution() {
    let mut input: Vec<String> = fs::read_to_string("input/day_4/part1.txt")
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

fn day_4_part_2_solution() {
    let mut input: Vec<String> = fs::read_to_string("input/day_4/part1.txt")
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