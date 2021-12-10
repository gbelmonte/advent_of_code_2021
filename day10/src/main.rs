use std::{fs, collections::HashMap};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: Vec<Vec<char>> = fs::read_to_string("day10/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut mapping: HashMap<char, char> = HashMap::new();
    mapping.insert('}', '{');
    mapping.insert(')', '(');
    mapping.insert('>', '<');
    mapping.insert(']', '[');

    let total = input.iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut error = '1';
            for i in 0..line.len() {
                if mapping.contains_key(&line[i]) {
                    if stack.is_empty() || stack.last().unwrap() != mapping.get(&line[i]).unwrap() {
                        error = line[i];
                        break;
                    }
                    stack.pop();
                } else {
                    stack.push(line[i]);
                }
            }
            error
        })
        .filter(|x| *x != '1')
        .map(|x| {
            match x {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        })
        .sum::<i32>();

    println!("{}", total);
}

fn part_2_solution() {
    let input: Vec<Vec<char>> = fs::read_to_string("day10/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut mapping: HashMap<char, char> = HashMap::new();
    mapping.insert('}', '{');
    mapping.insert(')', '(');
    mapping.insert('>', '<');
    mapping.insert(']', '[');

    let mut noncomplete = input.iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            for i in 0..line.len() {
                if mapping.contains_key(&line[i]) {
                    if stack.is_empty() || stack.last().unwrap() != mapping.get(&line[i]).unwrap() {
                        stack = Vec::new();
                        break;
                    }
                    stack.pop();
                } else {
                    stack.push(line[i]);
                }
            }
            stack
        })
        .filter(|x| x.len() > 0)
        .map(|x| {
            let score = x.iter().rev()
                .fold(0_i64, |mut acc, c| {
                    acc *= 5;
                    acc += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    };
                    acc
                });
            score
        })
        .collect::<Vec<i64>>();
    noncomplete.sort();
    let middle = (noncomplete.len() - 1) / 2;
    println!("{}", noncomplete[middle]);
}