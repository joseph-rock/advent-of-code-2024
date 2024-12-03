use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Mul {
    left: usize,
    right: usize,
}

fn main() {
    let input: &str = include_str!("./day03.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
            r"(?x)
            (mul\()
            (?<left>\d+)
            (\,)
            (?<right>\d+)
            (\))",
        )
        .unwrap()
    });

    let mut total: usize = 0;

    for line in input.lines() {
        let instructions: Vec<Mul> = RE.captures_iter(line).map(|mul| {
            let left = mul.name("left").unwrap().as_str().parse::<usize>().unwrap();
            let right = mul.name("right").unwrap().as_str().parse::<usize>().unwrap();
            Mul{
                left,
                right,
            }
        }).collect();

        total += instructions.into_iter().fold(0, |total, inst| total + (inst.left * inst.right));
    }

    total
}
 