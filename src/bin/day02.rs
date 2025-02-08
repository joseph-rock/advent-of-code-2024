enum ReportResult {
    Safe,
    Unsafe(usize),
}

fn main() {
    let input: &str = include_str!("./day02.txt");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let mut total_safe: usize = 0;

    for line in input.lines() {
        let report: Vec<i8> = line
            .split_whitespace()
            .map(|num| num.parse::<i8>().unwrap())
            .collect();

        let result: ReportResult = safe(&report);

        match result {
            ReportResult::Safe => total_safe += 1,
            ReportResult::Unsafe(_) => (),
        }
    }

    total_safe
}

fn part_2(input: &str) -> usize {
    let mut total_safe: usize = 0;

    for line in input.lines() {
        let report: Vec<i8> = line
            .split_whitespace()
            .map(|num| num.parse::<i8>().unwrap())
            .collect();

        let result: ReportResult = safe(&report);

        // Righting beautiful Rust is my passion /s
        match result {
            ReportResult::Safe => total_safe += 1,
            ReportResult::Unsafe(index) => {
                let mut remove_left = report.clone();
                remove_left.remove(index);
                let left_result: ReportResult = safe(&remove_left);
                match left_result {
                    ReportResult::Safe => total_safe += 1,
                    ReportResult::Unsafe(_) => {
                        let mut remove_right = report.clone();
                        remove_right.remove(index + 1);
                        let right_result: ReportResult = safe(&remove_right);
                        match right_result {
                            ReportResult::Safe => total_safe += 1,
                            ReportResult::Unsafe(_) => {
                                let mut remove_index_0 = report.clone();
                                remove_index_0.remove(0);
                                let index_0_result = safe(&remove_index_0);
                                match index_0_result {
                                    ReportResult::Safe => total_safe += 1,
                                    ReportResult::Unsafe(_) => (),
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    total_safe
}

fn safe(report: &Vec<i8>) -> ReportResult {
    let mut prev: Option<i8> = None;
    for (index, pair) in report.windows(2).enumerate() {
        let difference = pair[1] - pair[0];

        if difference.abs() > 3 {
            return ReportResult::Unsafe(index);
        }

        if prev.is_some() && prev.unwrap() * difference <= 0 {
            return ReportResult::Unsafe(index);
        }

        prev = Some(difference);
    }
    ReportResult::Safe
}
