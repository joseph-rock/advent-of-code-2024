use std::iter::zip;

fn main() {
    let input: &str = include_str!("./day01.txt");
    let pt1 = part_1(input);
    println!("{pt1}");
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

    let sorted_zip: Vec<(i32, i32)> = zip(left.into_iter(), right.into_iter()).collect();
    sorted_zip
        .into_iter()
        .map(|(left, right): (i32, i32)| (left - right).abs())
        .reduce(|sum, num| sum + num).unwrap()
}
