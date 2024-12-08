use std::iter::zip;

fn main() {
    let input: &str = include_str!("./day01.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let row: Vec<&str> = line.split_whitespace().collect();
        left.push(row[0].parse::<i32>().expect("Left cannot parse"));
        right.push(row[1].parse::<i32>().expect("Right cannot parse"));
    }

    left.sort();
    right.sort();

    zip(left.into_iter(), right.into_iter())
        .into_iter()
        .map(|(left, right): (i32, i32)| (left - right).abs())
        .reduce(|sum, num| sum + num).unwrap()
}

fn part_2(input: &str) -> usize {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let row: Vec<&str> = line.split_whitespace().collect();
        left.push(row[0].parse::<i32>().expect("Left cannot parse"));
        right.push(row[1].parse::<i32>().expect("Right cannot parse"));
    }

    let mut total: usize = 0;
    for left_num in left {
        let num_matches: Vec<i32> = right
            .clone()
            .into_iter()
            .filter(|&right_num| left_num == right_num)
            .collect();
        total += num_matches.len() * left_num as usize;
    }

    total
}
