use std::{fs, collections::HashMap};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let mut input: Vec<((i32, i32), (i32, i32))> = fs::read_to_string("day5/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| {
            let points: Vec<Vec<i32>> = line.split("->")
                .map(|x| {
                    x.trim().split(",")
                        .map(|val| val.parse::<i32>().unwrap())
                        .collect()
                })
                .collect();
            ((points[0][0], points[0][1]), (points[1][0], points[1][1]))
        })
        .collect();

    let result = input.iter_mut()
        .filter(|((x1, y1), (x2, y2))| *x1 == *x2 || *y1 == *y2)
        .fold(HashMap::new(), |mut map, ((x1, y1),(x2, y2))| {
            let x_modifier = if x1 == x2 {0} else {if x1 < x2 {1} else {-1}};
            let y_modifier = if y1 == y2 {0} else {if y1 < y2 {1} else {-1}};
            while x1 != x2 || y1 != y2 {
                *map.entry((*x1, *y1)).or_insert(0) += 1;
                *x1 += x_modifier;
                *y1 += y_modifier;
            }
            *map.entry((*x1, *y1)).or_insert(0) += 1;
            map
        });
    let count = result.into_iter().filter(|((_,_), x)| *x > 1).count();    
    println!("Number of positions where at least 2 lines intersect {}", count);   
}

fn part_2_solution() {
    let mut input: Vec<((i32, i32), (i32, i32))> = fs::read_to_string("day5/input/part1.txt")
    .expect("Something went wrong reading the file")
    .lines()
    .map(|line| {
        let points: Vec<Vec<i32>> = line.split("->")
            .map(|x| {
                x.trim().split(",")
                    .map(|val| val.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        ((points[0][0], points[0][1]), (points[1][0], points[1][1]))
    })
    .collect();

    let result = input.iter_mut()
        .fold(HashMap::new(), |mut map, ((x1, y1),(x2, y2))| {
            let x_modifier = if x1 == x2 {0} else {if x1 < x2 {1} else {-1}};
            let y_modifier = if y1 == y2 {0} else {if y1 < y2 {1} else {-1}};
            while x1 != x2 || y1 != y2 {
                *map.entry((*x1, *y1)).or_insert(0) += 1;
                *x1 += x_modifier;
                *y1 += y_modifier;
            }
            *map.entry((*x1, *y1)).or_insert(0) += 1;
            map
        });
    let count = result.into_iter().filter(|((_,_), x)| *x > 1).count();    
    println!("Number of positions where at least 2 lines intersect {}", count);   
}
