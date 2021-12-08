use std::{fs, collections::HashMap};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: Vec<Vec<String>> = fs::read_to_string("day8/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let output = line.split("|").skip(1).next().unwrap();
            output.trim().split(" ").map(String::from).collect::<Vec<String>>()
        })
        .collect();

    let count = input.iter()
        .flat_map(|line| {
            line.iter()
                .filter(|val| val.len() == 2 || val.len() == 3 || val.len() == 4 || val.len() == 7)
                .collect::<Vec<&String>>()
        })
        .count();
    println!("Number of 1,4,7,8 in outputs {}", count);
}

fn part_2_solution() {
    let input: Vec<(Vec<String>, Vec<String>)> = fs::read_to_string("day8/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let mut itr = line.split("|");
            let digits = itr.next().unwrap();
            let output = itr.next().unwrap();
            (digits.trim().split(" ").map(String::from).collect::<Vec<String>>(), 
                output.trim().split(" ").map(String::from).collect::<Vec<String>>())
        })
        .collect();

    let sum = input.iter()
        .map(|(digits, output)| {
            let mut segment_to_digit: Vec<String> = digits.iter()
                .fold(vec![String::new(); 10], |mut acc, segment| {
                    match segment.len() {
                        2 => acc[1] = segment.clone(),
                        3 => acc[7] = segment.clone(),
                        4 => acc[4] = segment.clone(),
                        7 => acc[8] = segment.clone(),
                        _ => (),
                    };
                    acc
                });
            
            // Determine 3
            segment_to_digit[3] = digits.iter()
                .filter(|segment| segment.len() == 5 
                    && segment_to_digit[1].chars().filter(|c| segment.contains(*c)).count() == 2)
                .last()
                .unwrap()
                .clone();
            
            // Determine 5
            let b = segment_to_digit[4].chars()
                .filter(|c| !segment_to_digit[3].contains(*c))
                .last().unwrap();
            segment_to_digit[5] = digits.iter()
                .filter(|segment| segment.len() == 5 && segment.contains(b))
                .last().unwrap().clone();
            
            // Determine 2
            segment_to_digit[2] = digits.iter()
                .filter(|&segment| segment.len() == 5 && *segment != segment_to_digit[3] && *segment != segment_to_digit[5])
                .last().unwrap().clone();
            
            // Determine 6
            let c = segment_to_digit[1].chars()
                .filter(|c| !segment_to_digit[5].contains(*c))
                .last().unwrap();
            segment_to_digit[6] = digits.iter()
                .filter(|segment| segment.len() == 6 && !segment.contains(c))
                .last().unwrap().clone();
            
            // Determine 0
            let d = segment_to_digit[4].chars()
                .filter(|c| !segment_to_digit[1].contains(*c) && *c != b)
                .last().unwrap();
            segment_to_digit[0] = digits.iter()
                .filter(|segment| segment.len() == 6 && !segment.contains(d))
                .last().unwrap().clone();

            // Determine 9
            segment_to_digit[9] = digits.iter()
                .filter(|&segment| segment.len() == 6 && *segment != segment_to_digit[0] && *segment != segment_to_digit[6])
                .last().unwrap().clone();
            
            // Gather into map of sorted segment to value
            let segment_to_value: HashMap<String,i32> = segment_to_digit.iter().enumerate()
                .map(|(i, segment)| {
                    let mut chars = segment.chars().collect::<Vec<char>>();
                    chars.sort();
                    (chars.into_iter().collect::<String>(), i as i32)
                })
                .collect();
            (segment_to_value, output)
        })
        .map(|(mapping, output)| {
            let mut curr_digit = 1;
            output.iter().rev()
                .map(|val| {
                    let mut chars = val.chars().collect::<Vec<char>>();
                    chars.sort();
                    let key = chars.into_iter().collect::<String>();
                    let x = mapping.get(&key).unwrap() * curr_digit;
                    curr_digit *= 10;
                    x
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("Sum of all outputs {}", sum);
}