use std::{fs, collections::HashSet};

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let mut folds: Vec<(char, i32)> = Vec::new();
    let mut coordinates: HashSet<(i32,i32)> = fs::read_to_string("day13/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .filter_map(|line| {
            if line.starts_with("fold") {
                let fold_data = line.replace("fold along ", "");
                let mut itr = fold_data.split("=");
                folds.push((itr.next().unwrap().chars().last().unwrap(), itr.next().unwrap().parse::<i32>().unwrap()));
                None
            } else if line.is_empty() {
                None
            } else {
                let mut itr = line.split(",");
                Some((itr.next().unwrap().parse::<i32>().unwrap(), itr.next().unwrap().parse::<i32>().unwrap()))
            }
        })
        .collect();

    let (direction, fold) = folds[0];
    let edge = fold * 2;
    let mirrored_points;
    if direction == 'y' {
        mirrored_points = coordinates.iter()
            .filter(|(_, y)| *y > fold && *y <= edge)
            .map(|(x, y)| (*x, fold - (*y - fold)))
            .collect::<Vec<(i32, i32)>>();
        coordinates.retain(|(_, y)| *y < fold);
    } else {
        mirrored_points = coordinates.iter()
            .filter(|(x, _)| *x > fold && *x <= edge)
            .map(|(x, y)| (fold - (*x - fold), *y))
            .collect::<Vec<(i32, i32)>>();
        coordinates.retain(|(x, _)| *x < fold);
    }

    for point in mirrored_points {
        coordinates.insert(point);
    }
    println!("Remaining points after first fold {}", coordinates.len());
}

fn part_2_solution() {
    let mut folds: Vec<(char, i32)> = Vec::new();
    let mut coordinates: HashSet<(i32,i32)> = fs::read_to_string("day13/input/part1.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .filter_map(|line| {
            if line.starts_with("fold") {
                let fold_data = line.replace("fold along ", "");
                let mut itr = fold_data.split("=");
                folds.push((itr.next().unwrap().chars().last().unwrap(), itr.next().unwrap().parse::<i32>().unwrap()));
                None
            } else if line.is_empty() {
                None
            } else {
                let mut itr = line.split(",");
                Some((itr.next().unwrap().parse::<i32>().unwrap(), itr.next().unwrap().parse::<i32>().unwrap()))
            }
        })
        .collect();

    let mut width = 0;
    let mut height = 0;
    for (direction, fold) in folds {
        let edge = fold * 2;
        let mirrored_points;
        if direction == 'y' {
            mirrored_points = coordinates.iter()
                .filter(|(_, y)| *y > fold && *y <= edge)
                .map(|(x, y)| (*x, fold - (*y - fold)))
                .collect::<Vec<(i32, i32)>>();
            coordinates.retain(|(_, y)| *y < fold);
            height = fold;
        } else {
            mirrored_points = coordinates.iter()
                .filter(|(x, _)| *x > fold && *x <= edge)
                .map(|(x, y)| (fold - (*x - fold), *y))
                .collect::<Vec<(i32, i32)>>();
            coordinates.retain(|(x, _)| *x < fold);
            width = fold;
        }

        for point in mirrored_points {
            coordinates.insert(point);
        }
    } 
    let grid: Vec<Vec<char>> = coordinates.iter()
        .fold(vec![vec!['.'; width as usize]; height as usize], |mut acc, (x, y)| {
            acc[*y as usize][*x as usize] = '#';
            acc
        });
    for line in grid {
        println!("{:?}", line);
    }
}