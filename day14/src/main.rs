use std::{fs, collections::HashMap, time::{Instant}};

fn main() {
    let mut start = Instant::now();
    part_1_solution();
    println!("Part1 {:?}", start.elapsed());
    start = Instant::now();
    part_2_solution();
    println!("Part2 {:?}", start.elapsed());
}

fn part_1_solution() {
    let mut template: Vec<char> = Vec::new();
    let mappings: HashMap<String,char> = fs::read_to_string("day14/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if i == 0 {
                template = line.chars().collect();
                None
            } else if line.is_empty() {
                None
            } else {
                let mut iter = line.split("->");
                Some((iter.next().unwrap().trim().to_string(), iter.next().unwrap().trim().chars().last().unwrap()))
            }
        })
        .collect();
    
    for _ in 0..10 {
        let size = template.len() - 1;
        template = template.windows(2).enumerate()
            .flat_map(|(i, chars)| {
                let mut res = Vec::new();
                if let Some(entry) = mappings.get(&chars.iter().collect::<String>()) {
                    res.push(chars[0]);
                    res.push(*entry);
                } else {
                    res.push(chars[0]);
                }
                if i == size - 1 {
                    res.push(chars[1]);
                }
                res
            })
            .collect();
    }
    let mut occurances: Vec<(char, i32)> = template.iter()
        .fold(HashMap::new(), |mut acc, val| {
            *acc.entry(*val).or_insert(0) += 1;
            acc
        })
        .into_iter().collect();
    occurances.sort_by(|a, b| a.1.cmp(&b.1));
    println!("Most common - least common {}", occurances.last().unwrap().1 - occurances.first().unwrap().1);
}

fn part_2_solution() {
    let mut template: Vec<char> = Vec::new();
    let mappings: HashMap<(char,char),char> = fs::read_to_string("day14/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            if i == 0 {
                template = line.chars().collect();
                None
            } else if line.is_empty() {
                None
            } else {
                let mut iter = line.split("->");
                let key: Vec<char> = iter.next().unwrap().trim().chars().collect();
                Some(((key[0], key[1]), iter.next().unwrap().trim().chars().last().unwrap()))
            }
        })
        .collect();
    
    let mut cache = HashMap::new();
    let size = template.len() - 1;
    let mut occurances: Vec<(char, i64)> = template.windows(2).enumerate()
        .fold(HashMap::new(), |mut acc, (i, chars)| {
            generate_sequence_count(chars[0], chars[1], 40, &mappings, &mut cache)
                .into_iter()
                .for_each(|(key, val)| {
                    *acc.entry(key).or_insert(0_i64) += val;
                });
            *acc.entry(chars[0]).or_insert(0_i64) += 1;
            if i == size - 1 {
                *acc.entry(chars[1]).or_insert(0_i64) += 1;
            }
            acc
        })
        .into_iter().collect();
    occurances.sort_by(|a, b| a.1.cmp(&b.1));
    println!("Most common - least common {}", occurances.last().unwrap().1 - occurances.first().unwrap().1);
}

fn generate_sequence_count(left: char, right: char, step: i8, mappings: &HashMap<(char,char),char>, cache: &mut HashMap<(char,char,i8), HashMap<char, i64>>) -> HashMap<char, i64> {
    if cache.contains_key(&(left,right,step)) {
        return cache.get(&(left,right,step)).unwrap().clone();
    } else if step == 0 {
        return HashMap::new();
    }

    if let Some(entry) = mappings.get(&(left,right)) {
        let mut left_map = generate_sequence_count(left, *entry, step-1, mappings, cache);
        let right_map = generate_sequence_count(*entry, right, step-1, mappings, cache);
        right_map.into_iter().for_each(|(key, val)| {
            *left_map.entry(key).or_insert(0_i64) += val;
        });
        *left_map.entry(*entry).or_insert(0_i64) += 1;
        cache.insert((left,right,step), left_map.clone());
        return left_map;
    }
    return HashMap::new();
} 