#[derive(Debug)]
struct Calibration {
    total: usize,
    values: Vec<usize>,
}

impl Calibration {
    fn new(line: &str) -> Calibration {
        let split_line: Vec<&str> = line.split(":").collect();
        let total = split_line[0].parse::<usize>().expect("Cannot parse total");
        let values: Vec<usize> = split_line[1]
            .split_whitespace()
            .map(|num| num.parse::<usize>().expect("Cannot parse value"))
            .collect();
        Calibration { total, values }
    }

    fn parse_input(input: &str) -> Vec<Calibration> {
        input
            .lines()
            .map(|line: &str| Calibration::new(line))
            .collect()
    }
}

fn add(total: &mut usize, values: &mut Vec<usize>) -> (usize, Vec<usize>) {
    if values.is_empty() {
        return (*total, vec![]);
    }

    let value = values.remove(0);
    (*total + value, values.to_vec())
}

fn multiply(total: &mut usize, values: &mut Vec<usize>) -> (usize, Vec<usize>) {
    if values.is_empty() {
        return (*total, vec![]);
    }
    if *total == 0 {
        *total = 1;
    }

    let value = values.remove(0);
    (*total * value, values.to_vec())
}

fn solve(calibration: &Calibration) -> Option<usize> {
    // sort list
    let values_clone = calibration.values.clone().sort();
    // add
    //  if smaller, multiply first 2 and add again
    //  if larger, return none
    todo!()
}

fn apply_add(calibration: &Calibration) -> Option<usize> {
    let mut total = 0;
    let mut values_clone = calibration.values.clone();
    while !values_clone.is_empty() {
        (total, values_clone) = add(&mut total, &mut values_clone);
    }

    // Lost cause IF adding all the numbers is greater than the total
    if total > calibration.total {
        return None;
    }
    
    // If adding works
    if total == calibration.total {
        return Some(total);
    }

    todo!("figure out what to consume to minimize clone")
}

fn main() {
    let input: &str = include_str!("./day07.txt");
    let pt1 = part_1(input);
    // let pt2 = part_2(input);
    println!("Part 1: {pt1}");
    // println!("Part 2: {pt2}");
}

fn part_1(input: &str) -> usize {
    let calibrations = Calibration::parse_input(&input);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let test = Calibration {
            total: 10,
            values: vec![1, 2, 3, 4],
        };

        let mut values_clone = test.values.clone();
        let mut total = 0;
        while !values_clone.is_empty() {
            (total, values_clone) = add(&mut total, &mut values_clone);
        }

        assert_eq!(test.total, total);
        assert_eq!(test.values, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_multiply() {
        let test = Calibration {
            total: 24,
            values: vec![1, 2, 3, 4],
        };

        let mut values_clone = test.values.clone();
        let mut total = 0;
        while !values_clone.is_empty() {
            (total, values_clone) = multiply(&mut total, &mut values_clone);
        }

        assert_eq!(test.total, total);
        assert_eq!(test.values, vec![1, 2, 3, 4]);
    }
}
