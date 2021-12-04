use std::fs;

fn main() {
    part_1_solution();
    part_2_solution();
}

fn part_1_solution() {
    let input: Vec<String> = fs::read_to_string("day3/input/part1.txt")
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

fn part_2_solution() {
    let input: Vec<Vec<char>> = fs::read_to_string("day3/input/part1.txt")
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