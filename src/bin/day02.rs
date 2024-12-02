
fn main() {
    let input: &str = include_str!("./day02.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut total_safe: usize = 0;

    'outer: for line in input.lines() {
        let report: Vec<i8> = line
            .split_whitespace()
            .map(|num| num.parse::<i8>().unwrap())
            .collect();

        let mut prev: Option<i8> = None;
        for pair in report.windows(2) {
            let difference = pair[1] - pair[0];

            if difference.abs() > 3 {
                continue 'outer;
            }

            if prev.is_some() && prev.unwrap() * difference <= 0 {
                continue 'outer;
            }
            
            prev = Some(difference);
        }

        total_safe += 1;

    }

    total_safe
}